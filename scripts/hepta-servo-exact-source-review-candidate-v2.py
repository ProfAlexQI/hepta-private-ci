#!/usr/bin/env python3
"""Compile a review-only candidate from exact Servo source qualification evidence.

The compiler is standard-library only. It does not fetch, accept, build, link,
launch, or execute Servo. A successful result means that one source-only
workflow evidence packet is internally consistent and ready for a separate
reviewed pointer update. It never updates that pointer itself.
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
import subprocess
import sys
from typing import Any

SERVO_COMMIT = "0a48e298482659817eb50097df23841f2b8e3044"
SERVO_TREE = "b04d2f75b3217374d079d579c270177b57fa1389"
WORKFLOW_NAME = "hepta-servo independent source qualification v3"
WORKFLOW_PATH = ".github/workflows/hepta-servo-independent-source-qualification-v3.yml"
JOB_NAME = "Exact source, deterministic archive, Git-order tree reconstruction"
CANDIDATE_SCHEMA = "hepta.servo.exact_source_review_candidate.v2"
CANDIDATE_DOMAIN = b"hepta.servo.exact-source-review-candidate.v2"
EXPECTED_ARTIFACTS = (
    "hepta-servo-independent-source-v3-receipts",
    f"servo-{SERVO_COMMIT}-source-v3",
)
REQUIRED_STEPS = (
    "Require explicit source-only acknowledgement",
    "Verify v3 tooling before network access",
    "Acquire exact Servo source twice and create deterministic bundle",
    "Reconstruct the exact Git tree using canonical Git ordering",
    "Assert exact source-only negative authority",
    "Upload canonical source-only receipts",
    "Upload deterministic compressed source archive",
    "Final source-only summary",
)
REQUIRED_SOURCE_FILES = (
    "fetch-a.receipt.json",
    "fetch-b.receipt.json",
    "independent-source-bundle.receipt.json",
    "license-packet.json",
    "servo-source-a.tar.gz",
    "source-bundle.verification.json",
)
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
CLAIMS = {
    "exact_servo_source_accepted": False,
    "worker_source_topology_accepted": False,
    "build_recipe_accepted": False,
    "build_authorized": False,
    "servo_built": False,
    "worker_artifact_created": False,
    "servo_runtime_qualified": False,
    "operator_acceptance": False,
    "promotion": False,
    "release_qualified": False,
}
SHA40 = re.compile(r"^[0-9a-f]{40}$")
SHA64 = re.compile(r"^[0-9a-f]{64}$")
UTC_SECONDS = re.compile(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z$")
MAX_JSON_BYTES = 64 * 1024 * 1024
MAX_SOURCE_FILE_BYTES = 8 * 1024 * 1024 * 1024
LOCAL_PATH_PATTERNS = (
    re.compile(r"(?:^|[\"'])/(?:home|Users|Volumes|tmp|private)/"),
    re.compile(r"[A-Za-z]:\\\\"),
)


class CandidateError(RuntimeError):
    """Fail-closed evidence compilation error."""


def fail(message: str) -> None:
    raise CandidateError(message)


def reject_duplicate_keys(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    value: dict[str, Any] = {}
    for key, item in pairs:
        if key in value:
            fail(f"duplicate JSON key {key!r}")
        value[key] = item
    return value


def canonical(value: object) -> bytes:
    return json.dumps(
        value,
        sort_keys=True,
        separators=(",", ":"),
        ensure_ascii=False,
    ).encode("utf-8")


def sha256_bytes(value: bytes) -> str:
    return hashlib.sha256(value).hexdigest()


def sha256_file(path: pathlib.Path, maximum_bytes: int = MAX_SOURCE_FILE_BYTES) -> tuple[str, int]:
    digest = hashlib.sha256()
    length = 0
    try:
        with path.open("rb") as handle:
            while True:
                block = handle.read(1024 * 1024)
                if not block:
                    break
                length += len(block)
                if length > maximum_bytes:
                    fail(f"file exceeds byte bound: {path.name}")
                digest.update(block)
    except OSError as error:
        fail(f"cannot hash {path.name}: {error}")
    return digest.hexdigest(), length


def load_json(
    path: pathlib.Path,
    label: str,
    *,
    canonical_required: bool = False,
) -> tuple[dict[str, Any], bytes]:
    try:
        raw = path.read_bytes()
    except OSError as error:
        fail(f"cannot read {label}: {error}")
    if not raw or len(raw) > MAX_JSON_BYTES:
        fail(f"{label} is empty or exceeds its byte bound")
    try:
        value = json.loads(
            raw.decode("utf-8", "strict"),
            object_pairs_hook=reject_duplicate_keys,
        )
    except (UnicodeError, json.JSONDecodeError) as error:
        fail(f"cannot decode {label}: {error}")
    if not isinstance(value, dict):
        fail(f"{label} must contain one JSON object")
    if canonical_required and raw != canonical(value):
        fail(f"{label} is not compact canonical JSON")
    return value, raw


def require_bool_false(value: Any, label: str) -> None:
    if value is not False:
        fail(f"{label} must remain false")


def require_closed_mapping(value: Any, label: str) -> None:
    if not isinstance(value, dict) or not value:
        fail(f"{label} must be a non-empty object")
    enabled = sorted(key for key, item in value.items() if item is not False)
    if enabled:
        fail(f"{label} attempted to enable authority: {enabled}")


def require_positive_integer(value: Any, label: str) -> int:
    if isinstance(value, bool) or not isinstance(value, int) or value <= 0:
        fail(f"{label} must be a positive integer")
    return value


def require_sha40(value: Any, label: str) -> str:
    if not isinstance(value, str) or not SHA40.fullmatch(value):
        fail(f"{label} must be lowercase 40-hex Git object")
    return value


def require_sha64(value: Any, label: str) -> str:
    if not isinstance(value, str) or not SHA64.fullmatch(value):
        fail(f"{label} must be lowercase SHA-256")
    return value


def timestamp(value: str | None) -> str:
    value = value or dt.datetime.now(dt.timezone.utc).replace(microsecond=0).strftime(
        "%Y-%m-%dT%H:%M:%SZ"
    )
    if not UTC_SECONDS.fullmatch(value):
        fail("captured_at_utc must use whole-second RFC3339 UTC")
    try:
        dt.datetime.strptime(value, "%Y-%m-%dT%H:%M:%SZ")
    except ValueError as error:
        fail(f"captured_at_utc is invalid: {error}")
    return value


def require_root(path: pathlib.Path, label: str) -> pathlib.Path:
    if not path.is_absolute():
        fail(f"{label} must be an absolute canonical directory")
    try:
        resolved = path.resolve(strict=True)
        metadata = path.lstat()
    except OSError as error:
        fail(f"{label} is unavailable: {error}")
    if resolved != path or stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode):
        fail(f"{label} must be a canonical non-symlink directory")
    return path


def require_regular_file(root: pathlib.Path, name: str, label: str) -> pathlib.Path:
    if (
        not name
        or pathlib.PurePosixPath(name).name != name
        or "/" in name
        or "\\" in name
        or "\x00" in name
    ):
        fail(f"{label} filename is unsafe")
    path = root / name
    try:
        resolved = path.resolve(strict=True)
        metadata = path.lstat()
    except OSError as error:
        fail(f"{label} is unavailable: {error}")
    try:
        resolved.relative_to(root)
    except ValueError:
        fail(f"{label} escaped evidence root")
    if resolved != path or stat.S_ISLNK(metadata.st_mode) or not stat.S_ISREG(metadata.st_mode):
        fail(f"{label} must be a canonical non-symlink regular file")
    if getattr(metadata, "st_nlink", 1) != 1:
        fail(f"{label} must have exactly one hard link")
    if metadata.st_mode & (stat.S_IWGRP | stat.S_IWOTH):
        fail(f"{label} must not be group/world writable")
    return path


def reject_local_paths(raw: bytes, label: str) -> None:
    text = raw.decode("utf-8", "strict")
    for pattern in LOCAL_PATH_PATTERNS:
        if pattern.search(text):
            fail(f"{label} contains a machine-local path")


def workflow_run_projection(
    run: dict[str, Any],
    expected_ref: str,
) -> dict[str, Any]:
    if run.get("name") != WORKFLOW_NAME or run.get("path") != WORKFLOW_PATH:
        fail("workflow run name/path drifted")
    if run.get("event") != "workflow_dispatch":
        fail("exact-source evidence must come from workflow_dispatch")
    if run.get("status") != "completed" or run.get("conclusion") != "success":
        fail("workflow run did not complete successfully")
    if run.get("head_branch") != expected_ref:
        fail("workflow run head branch differs from requested ref")
    head_sha = require_sha40(run.get("head_sha"), "workflow head SHA")
    run_id = require_positive_integer(run.get("id"), "workflow run ID")
    run_attempt = require_positive_integer(run.get("run_attempt"), "workflow run attempt")
    head_commit = run.get("head_commit")
    if not isinstance(head_commit, dict):
        fail("workflow run head_commit is missing")
    if head_commit.get("id") != head_sha:
        fail("workflow head_commit ID differs from head_sha")
    tree = require_sha40(head_commit.get("tree_id"), "workflow head tree")
    html_url = run.get("html_url")
    if not isinstance(html_url, str) or not html_url.startswith(
        "https://github.com/ProfAlexQI/hepta-private-ci/actions/runs/"
    ):
        fail("workflow run URL is missing or outside the repository")
    return {
        "id": run_id,
        "attempt": run_attempt,
        "name": WORKFLOW_NAME,
        "path": WORKFLOW_PATH,
        "event": "workflow_dispatch",
        "head_ref": expected_ref,
        "head_sha": head_sha,
        "head_tree": tree,
        "html_url": html_url,
    }


def workflow_jobs_projection(jobs_payload: dict[str, Any], run_id: int) -> dict[str, Any]:
    jobs = jobs_payload.get("jobs")
    if not isinstance(jobs, list) or len(jobs) != 1:
        fail("workflow evidence must contain exactly one exact-source job")
    job = jobs[0]
    if not isinstance(job, dict):
        fail("workflow job is not an object")
    if job.get("name") != JOB_NAME:
        fail("workflow job name drifted")
    if job.get("run_id") not in (None, run_id):
        fail("workflow job run_id differs")
    if job.get("status") != "completed" or job.get("conclusion") != "success":
        fail("exact-source job did not complete successfully")
    runner_id = require_positive_integer(job.get("runner_id"), "workflow runner ID")
    runner_name = job.get("runner_name")
    if not isinstance(runner_name, str) or not runner_name.strip():
        fail("workflow runner name is missing")
    steps = job.get("steps")
    if not isinstance(steps, list) or not steps:
        fail("workflow job recorded no executable steps")
    by_name: dict[str, dict[str, Any]] = {}
    for step in steps:
        if not isinstance(step, dict) or not isinstance(step.get("name"), str):
            fail("workflow step is malformed")
        if step["name"] in by_name:
            fail(f"duplicate workflow step {step['name']!r}")
        by_name[step["name"]] = step
    missing = [name for name in REQUIRED_STEPS if name not in by_name]
    if missing:
        fail(f"workflow job is missing required steps: {missing}")
    failed = [
        name
        for name in REQUIRED_STEPS
        if by_name[name].get("status") != "completed"
        or by_name[name].get("conclusion") != "success"
    ]
    if failed:
        fail(f"required workflow steps did not pass: {failed}")
    return {
        "id": require_positive_integer(job.get("id"), "workflow job ID"),
        "name": JOB_NAME,
        "runner_id": runner_id,
        "runner_name": runner_name,
        "required_step_count": len(REQUIRED_STEPS),
        "recorded_step_count": len(steps),
    }


def workflow_artifacts_projection(payload: dict[str, Any]) -> list[dict[str, Any]]:
    artifacts = payload.get("artifacts")
    if not isinstance(artifacts, list):
        fail("workflow artifacts payload is malformed")
    by_name: dict[str, dict[str, Any]] = {}
    for artifact in artifacts:
        if not isinstance(artifact, dict) or not isinstance(artifact.get("name"), str):
            fail("workflow artifact record is malformed")
        name = artifact["name"]
        if name in by_name:
            fail(f"duplicate workflow artifact {name!r}")
        by_name[name] = artifact
    if set(by_name) != set(EXPECTED_ARTIFACTS):
        fail("workflow artifact names differ from the exact source evidence contract")
    projection: list[dict[str, Any]] = []
    for name in EXPECTED_ARTIFACTS:
        artifact = by_name[name]
        require_bool_false(artifact.get("expired"), f"artifact {name} expired")
        size = require_positive_integer(artifact.get("size_in_bytes"), f"artifact {name} size")
        projection.append(
            {
                "id": require_positive_integer(artifact.get("id"), f"artifact {name} ID"),
                "name": name,
                "size_in_bytes": size,
                "expired": False,
            }
        )
    return projection


def parse_checksums(path: pathlib.Path) -> dict[str, str]:
    try:
        raw = path.read_bytes()
    except OSError as error:
        fail(f"cannot read SHA256SUMS: {error}")
    if not raw or len(raw) > 1024 * 1024:
        fail("SHA256SUMS is empty or oversized")
    try:
        text = raw.decode("ascii", "strict")
    except UnicodeError as error:
        fail(f"SHA256SUMS is not ASCII: {error}")
    if "\r" in text or not text.endswith("\n"):
        fail("SHA256SUMS must use LF and end with one newline")
    result: dict[str, str] = {}
    order: list[str] = []
    for line in text.splitlines():
        match = re.fullmatch(r"([0-9a-f]{64})  ([A-Za-z0-9][A-Za-z0-9._-]*)", line)
        if match is None:
            fail(f"SHA256SUMS line is non-portable: {line!r}")
        digest, name = match.groups()
        if "/" in name or "\\" in name or name in result:
            fail("SHA256SUMS contains a path or duplicate filename")
        result[name] = digest
        order.append(name)
    if order != sorted(order, key=lambda item: item.encode("utf-8")):
        fail("SHA256SUMS filenames are not bytewise sorted")
    if set(result) != set(REQUIRED_SOURCE_FILES):
        fail("SHA256SUMS does not bind the exact required source evidence set")
    return result


def verify_source_files(source_root: pathlib.Path) -> dict[str, dict[str, Any]]:
    sums_path = require_regular_file(source_root, "SHA256SUMS", "checksum manifest")
    expected = parse_checksums(sums_path)
    projection: dict[str, dict[str, Any]] = {}
    for name in REQUIRED_SOURCE_FILES:
        path = require_regular_file(source_root, name, name)
        digest, length = sha256_file(path)
        if digest != expected[name]:
            fail(f"{name} digest differs from SHA256SUMS")
        projection[name] = {"sha256": digest, "bytes": length}
    sums_digest, sums_bytes = sha256_file(sums_path, 1024 * 1024)
    projection["SHA256SUMS"] = {"sha256": sums_digest, "bytes": sums_bytes}
    return projection


def verify_source_json(source_root: pathlib.Path) -> dict[str, Any]:
    bundle, bundle_raw = load_json(
        source_root / "independent-source-bundle.receipt.json",
        "source bundle receipt",
        canonical_required=True,
    )
    verification, verification_raw = load_json(
        source_root / "source-bundle.verification.json",
        "source bundle verification",
        canonical_required=True,
    )
    fetch_a, fetch_a_raw = load_json(
        source_root / "fetch-a.receipt.json",
        "fetch A receipt",
        canonical_required=True,
    )
    fetch_b, fetch_b_raw = load_json(
        source_root / "fetch-b.receipt.json",
        "fetch B receipt",
        canonical_required=True,
    )
    _license, license_raw = load_json(
        source_root / "license-packet.json",
        "license packet",
        canonical_required=True,
    )
    for label, raw in (
        ("source bundle receipt", bundle_raw),
        ("source bundle verification", verification_raw),
        ("fetch A receipt", fetch_a_raw),
        ("fetch B receipt", fetch_b_raw),
        ("license packet", license_raw),
    ):
        reject_local_paths(raw, label)
    source = bundle.get("source")
    if not isinstance(source, dict):
        fail("source bundle source projection is missing")
    if source.get("repository") != "servo/servo":
        fail("source bundle repository drifted")
    if source.get("commit") != SERVO_COMMIT or source.get("tree") != SERVO_TREE:
        fail("source bundle commit/tree drifted")
    qualification = bundle.get("qualification")
    if not isinstance(qualification, dict):
        fail("source bundle qualification projection is missing")
    for key in ("servo_built", "servo_runtime_qualified", "operator_accepted", "release_qualified"):
        require_bool_false(qualification.get(key), f"source bundle qualification {key}")
    require_closed_mapping(bundle.get("authority"), "source bundle authority")
    verification_source = verification.get("source")
    if not isinstance(verification_source, dict):
        fail("source verification source projection is missing")
    if verification_source.get("commit") != SERVO_COMMIT:
        fail("source verification commit drifted")
    if verification_source.get("tree") != SERVO_TREE:
        fail("source verification tree drifted")
    if verification_source.get("recomputed_tree") != SERVO_TREE:
        fail("source verification did not recompute the pinned tree")
    checks = verification.get("verification")
    if not isinstance(checks, dict):
        fail("source verification check projection is missing")
    for key in ("git_tree_recomputed", "pinned_tree_matched", "license_matched"):
        if checks.get(key) is not True:
            fail(f"source verification did not prove {key}")
    for key in ("servo_built", "servo_runtime_qualified", "release_qualified"):
        require_bool_false(checks.get(key), f"source verification {key}")
    require_closed_mapping(verification.get("authority"), "source verification authority")
    acquisitions: list[str] = []
    for label, receipt in (("fetch A", fetch_a), ("fetch B", fetch_b)):
        acquisition = receipt.get("acquisition")
        if not isinstance(acquisition, dict):
            fail(f"{label} acquisition projection is missing")
        nonce = require_sha64(acquisition.get("acquisition_nonce_sha256"), f"{label} nonce")
        if acquisition.get("standalone_object_store") is not True:
            fail(f"{label} did not use a standalone object store")
        if acquisition.get("alternate_object_database") is not False:
            fail(f"{label} used an alternate object database")
        acquisitions.append(nonce)
        require_closed_mapping(receipt.get("authority"), f"{label} authority")
    if acquisitions[0] == acquisitions[1]:
        fail("independent fetch receipts reused one acquisition nonce")
    return {
        "repository": "servo/servo",
        "commit": SERVO_COMMIT,
        "tree": SERVO_TREE,
        "recomputed_tree": SERVO_TREE,
        "fetch_acquisition_nonce_sha256": acquisitions,
    }


def run_offline_verifier(source_root: pathlib.Path, verifier: pathlib.Path) -> None:
    if not verifier.is_absolute():
        fail("--offline-verifier must be an absolute path")
    try:
        resolved = verifier.resolve(strict=True)
        metadata = verifier.lstat()
    except OSError as error:
        fail(f"offline verifier is unavailable: {error}")
    if resolved != verifier or stat.S_ISLNK(metadata.st_mode) or not stat.S_ISREG(metadata.st_mode):
        fail("offline verifier must be a canonical non-symlink file")
    command = [
        sys.executable,
        os.fspath(verifier),
        "--bundle-dir",
        os.fspath(source_root),
    ]
    environment = {
        "LC_ALL": "C",
        "LANG": "C",
        "TZ": "UTC",
        "PATH": os.environ.get("PATH", ""),
    }
    try:
        result = subprocess.run(
            command,
            env=environment,
            capture_output=True,
            text=True,
            timeout=300,
            check=False,
        )
    except (OSError, subprocess.TimeoutExpired) as error:
        fail(f"offline source verifier could not run: {error}")
    if result.returncode != 0:
        detail = (result.stderr or result.stdout).strip()[-1000:]
        fail(f"offline source verifier rejected retained bytes: {detail}")
    try:
        summary = json.loads(result.stdout.strip().splitlines()[-1])
    except (IndexError, json.JSONDecodeError) as error:
        fail(f"offline source verifier returned invalid summary: {error}")
    if (
        summary.get("status") != "PASS_SOURCE_BUNDLE_ONLY"
        or summary.get("commit") != SERVO_COMMIT
        or summary.get("tree") != SERVO_TREE
        or summary.get("recomputed_tree") != SERVO_TREE
        or summary.get("servo_built") is not False
        or summary.get("servo_runtime_qualified") is not False
        or summary.get("authority") != "all_false"
    ):
        fail("offline source verifier summary drifted")


def framed(domain: bytes, payload: bytes) -> str:
    digest = hashlib.sha256()
    digest.update(len(domain).to_bytes(8, "big"))
    digest.update(domain)
    digest.update(len(payload).to_bytes(8, "big"))
    digest.update(payload)
    return digest.hexdigest()


def compile_candidate(
    evidence_root: pathlib.Path,
    expected_ref: str,
    captured_at_utc: str | None,
    offline_verifier: pathlib.Path,
    output: pathlib.Path | None,
    *,
    skip_subprocess_for_test: bool = False,
) -> dict[str, Any]:
    evidence_root = require_root(evidence_root, "evidence root")
    source_root = require_root(evidence_root / "source", "source evidence root")
    run_path = require_regular_file(evidence_root, "workflow-run.json", "workflow run JSON")
    jobs_path = require_regular_file(evidence_root, "workflow-jobs.json", "workflow jobs JSON")
    artifacts_path = require_regular_file(
        evidence_root,
        "workflow-artifacts.json",
        "workflow artifacts JSON",
    )
    run, run_raw = load_json(run_path, "workflow run JSON")
    jobs, jobs_raw = load_json(jobs_path, "workflow jobs JSON")
    artifacts, artifacts_raw = load_json(artifacts_path, "workflow artifacts JSON")
    workflow = workflow_run_projection(run, expected_ref)
    job = workflow_jobs_projection(jobs, workflow["id"])
    artifact_projection = workflow_artifacts_projection(artifacts)
    files = verify_source_files(source_root)
    source = verify_source_json(source_root)
    if not skip_subprocess_for_test:
        run_offline_verifier(source_root, offline_verifier)
    raw_evidence = {
        "workflow-run.json": {
            "sha256": sha256_bytes(run_raw),
            "bytes": len(run_raw),
        },
        "workflow-jobs.json": {
            "sha256": sha256_bytes(jobs_raw),
            "bytes": len(jobs_raw),
        },
        "workflow-artifacts.json": {
            "sha256": sha256_bytes(artifacts_raw),
            "bytes": len(artifacts_raw),
        },
    }
    candidate: dict[str, Any] = {
        "schema": CANDIDATE_SCHEMA,
        "schema_version": 2,
        "phase": "DEVELOPMENT",
        "claim_level": "SOURCE_EVIDENCE_COMPLETE_SEPARATE_REVIEW_REQUIRED",
        "captured_at_utc": timestamp(captured_at_utc),
        "hepta": {
            "repository": "ProfAlexQI/hepta-private-ci",
            "ref": expected_ref,
            "commit": workflow["head_sha"],
            "tree": workflow["head_tree"],
        },
        "servo": source,
        "workflow": {
            **workflow,
            "job": job,
        },
        "artifacts": artifact_projection,
        "evidence": {
            "api_json": raw_evidence,
            "source_files": files,
        },
        "checks": {
            "workflow_dispatch": True,
            "workflow_success": True,
            "runner_allocated": True,
            "required_steps_recorded_and_passed": True,
            "required_artifacts_present_and_unexpired": True,
            "portable_sorted_sha256sums": True,
            "source_bundle_reverified_offline": True,
            "pinned_git_tree_recomputed": True,
            "independent_fetch_nonces": True,
            "machine_local_paths_absent": True,
            "source_only_negative_authority": True,
        },
        "review": {
            "status": "PENDING_SEPARATE_REVIEW",
            "candidate_accepted": False,
            "pointer_update_performed": False,
            "reviewer": None,
            "reviewed_at_utc": None,
        },
        "claims": dict(CLAIMS),
        "authority": dict(AUTHORITY),
        "decision": "EVIDENCE_COMPLETE_REVIEW_REQUIRED_BUILD_NOT_AUTHORIZED",
    }
    candidate["candidate_id"] = (
        "hepta-servo-exact-source-review-candidate:v2:"
        + framed(CANDIDATE_DOMAIN, canonical(candidate))
    )
    if output is not None:
        if not output.is_absolute():
            fail("--output must be an absolute canonical path")
        try:
            parent = output.parent.resolve(strict=True)
        except OSError as error:
            fail(f"output parent is unavailable: {error}")
        if parent / output.name != output:
            fail("--output path is not canonical")
        if output.exists():
            fail("candidate output already exists")
        encoded = canonical(candidate)
        descriptor = os.open(output, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
        with os.fdopen(descriptor, "wb") as handle:
            handle.write(encoded)
            handle.flush()
            os.fsync(handle.fileno())
    return candidate


def verify_candidate(candidate: dict[str, Any]) -> None:
    if candidate.get("schema") != CANDIDATE_SCHEMA or candidate.get("schema_version") != 2:
        fail("candidate schema/version drifted")
    identifier = candidate.get("candidate_id")
    prefix = "hepta-servo-exact-source-review-candidate:v2:"
    if not isinstance(identifier, str) or not identifier.startswith(prefix):
        fail("candidate ID prefix is invalid")
    digest = require_sha64(identifier.removeprefix(prefix), "candidate ID")
    without_id = dict(candidate)
    without_id.pop("candidate_id")
    if digest != framed(CANDIDATE_DOMAIN, canonical(without_id)):
        fail("candidate ID does not bind its payload")
    if candidate.get("claims") != CLAIMS:
        fail("candidate claims posture is open")
    if candidate.get("authority") != AUTHORITY:
        fail("candidate authority posture is open")
    review = candidate.get("review")
    if review != {
        "status": "PENDING_SEPARATE_REVIEW",
        "candidate_accepted": False,
        "pointer_update_performed": False,
        "reviewer": None,
        "reviewed_at_utc": None,
    }:
        fail("candidate review posture drifted")
    if candidate.get("decision") != "EVIDENCE_COMPLETE_REVIEW_REQUIRED_BUILD_NOT_AUTHORIZED":
        fail("candidate decision overclaims")


def contract() -> dict[str, Any]:
    if len(set(REQUIRED_STEPS)) != len(REQUIRED_STEPS):
        fail("required workflow steps are not unique")
    if tuple(sorted(REQUIRED_SOURCE_FILES, key=lambda item: item.encode("utf-8"))) != REQUIRED_SOURCE_FILES:
        fail("required source files are not bytewise sorted")
    if set(AUTHORITY) != {
        "machine_authority",
        "runtime_authority",
        "production_caller",
        "production_writer",
        "effect_authority",
        "external_effect",
        "external_network_allowed",
        "credential_export_allowed",
        "operator_acceptance",
        "g5_allowed",
        "execute_allowed",
        "promotion",
        "release_qualified",
    }:
        fail("authority field set drifted")
    return {
        "schema": CANDIDATE_SCHEMA,
        "status": "PASS_CONTRACT_ONLY",
        "workflow": WORKFLOW_PATH,
        "servo_commit": SERVO_COMMIT,
        "servo_tree": SERVO_TREE,
        "required_steps": len(REQUIRED_STEPS),
        "required_source_files": list(REQUIRED_SOURCE_FILES),
        "source_accepted": False,
        "build_authorized": False,
        "authority": "all_false",
    }


def parse_arguments() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    subparsers = parser.add_subparsers(dest="command", required=True)
    subparsers.add_parser("contract")
    compile_parser = subparsers.add_parser("compile")
    compile_parser.add_argument("--evidence-root", required=True)
    compile_parser.add_argument("--expected-ref", required=True)
    compile_parser.add_argument("--offline-verifier", required=True)
    compile_parser.add_argument("--captured-at-utc")
    compile_parser.add_argument("--output", required=True)
    verify_parser = subparsers.add_parser("verify")
    verify_parser.add_argument("--candidate", required=True)
    return parser.parse_args()


def main() -> int:
    try:
        arguments = parse_arguments()
        if arguments.command == "contract":
            result = contract()
        elif arguments.command == "compile":
            result = compile_candidate(
                pathlib.Path(arguments.evidence_root),
                arguments.expected_ref,
                arguments.captured_at_utc,
                pathlib.Path(arguments.offline_verifier),
                pathlib.Path(arguments.output),
            )
            verify_candidate(result)
        else:
            candidate, _raw = load_json(
                pathlib.Path(arguments.candidate),
                "source review candidate",
                canonical_required=True,
            )
            verify_candidate(candidate)
            result = {
                "schema": CANDIDATE_SCHEMA,
                "status": "PASS_CANDIDATE_REVIEW_PENDING",
                "candidate_id": candidate["candidate_id"],
                "source_accepted": False,
                "build_authorized": False,
                "authority": "all_false",
            }
    except (CandidateError, OSError, UnicodeError) as error:
        print(f"HEPTA_SERVO_EXACT_SOURCE_REVIEW_CANDIDATE_V2=FAIL: {error}", file=sys.stderr)
        return 1
    print(json.dumps(result, sort_keys=True, separators=(",", ":"), ensure_ascii=False))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
