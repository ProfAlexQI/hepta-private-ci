#!/usr/bin/env python3
"""Verify a separately reviewed exact-Servo-source acceptance pointer.

This standard-library-only tool never fetches, builds, links, launches, executes,
or promotes Servo. It can compile a review challenge and verify a manually
authored source-only pointer plus raw GitHub pull-request review evidence. It has
no command that creates or updates the accepted-source pointer itself.
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

REPOSITORY = "ProfAlexQI/hepta-private-ci"
BASE_BRANCH = "integration/vnext-main-20260811"
SERVO_COMMIT = "0a48e298482659817eb50097df23841f2b8e3044"
SERVO_TREE = "b04d2f75b3217374d079d579c270177b57fa1389"

CANDIDATE_SCHEMA = "hepta.servo.exact_source_review_candidate.v2"
POLICY_SCHEMA = "hepta.servo.source_acceptance_review_policy.v1"
CHALLENGE_SCHEMA = "hepta.servo.source_acceptance_review_challenge.v1"
POINTER_SCHEMA = "hepta.servo.accepted_source_pointer.v1"

CANDIDATE_DOMAIN = b"hepta.servo.exact-source-review-candidate.v2"
POLICY_DOMAIN = b"hepta.servo.source-acceptance-review-policy.v1"
CHALLENGE_DOMAIN = b"hepta.servo.source-acceptance-review-challenge.v1"
POINTER_DOMAIN = b"hepta.servo.accepted-source-pointer.v1"

SHA40 = re.compile(r"^[0-9a-f]{40}$")
SHA64 = re.compile(r"^[0-9a-f]{64}$")
UTC_SECONDS = re.compile(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z$")
SAFE_LOGIN = re.compile(r"^[A-Za-z0-9](?:[A-Za-z0-9-]{0,37}[A-Za-z0-9])?$")
MAX_JSON_BYTES = 64 * 1024 * 1024

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

CANDIDATE_CLAIMS = {
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

ACCEPTED_SOURCE_CLAIMS = {
    "exact_servo_source_accepted": True,
    "source_review_candidate_accepted": True,
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

REQUIRED_CANDIDATE_CHECKS = {
    "workflow_dispatch",
    "workflow_success",
    "runner_allocated",
    "required_steps_recorded_and_passed",
    "required_artifacts_present_and_unexpired",
    "portable_sorted_sha256sums",
    "source_bundle_reverified_offline",
    "pinned_git_tree_recomputed",
    "independent_fetch_nonces",
    "machine_local_paths_absent",
    "source_only_negative_authority",
}


class AcceptanceError(RuntimeError):
    """Fail-closed source-acceptance error."""


def fail(message: str) -> None:
    raise AcceptanceError(message)


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
        sort_keys=True,
        separators=(",", ":"),
        ensure_ascii=False,
    ).encode("utf-8")


def sha256_bytes(value: bytes) -> str:
    return hashlib.sha256(value).hexdigest()


def framed(domain: bytes, payload: bytes) -> str:
    digest = hashlib.sha256()
    digest.update(len(domain).to_bytes(8, "big"))
    digest.update(domain)
    digest.update(len(payload).to_bytes(8, "big"))
    digest.update(payload)
    return digest.hexdigest()


def require_sha40(value: Any, label: str) -> str:
    if not isinstance(value, str) or not SHA40.fullmatch(value):
        fail(f"{label} must be lowercase 40-hex")
    return value


def require_sha64(value: Any, label: str) -> str:
    if not isinstance(value, str) or not SHA64.fullmatch(value):
        fail(f"{label} must be lowercase SHA-256")
    return value


def require_positive_int(value: Any, label: str) -> int:
    if isinstance(value, bool) or not isinstance(value, int) or value <= 0:
        fail(f"{label} must be a positive integer")
    return value


def parse_timestamp(value: Any, label: str) -> dt.datetime:
    if not isinstance(value, str) or not UTC_SECONDS.fullmatch(value):
        fail(f"{label} must use whole-second RFC3339 UTC")
    try:
        return dt.datetime.strptime(value, "%Y-%m-%dT%H:%M:%SZ").replace(
            tzinfo=dt.timezone.utc
        )
    except ValueError as error:
        fail(f"{label} is invalid: {error}")


def timestamp(value: str | None) -> str:
    if value is None:
        return dt.datetime.now(dt.timezone.utc).replace(microsecond=0).strftime(
            "%Y-%m-%dT%H:%M:%SZ"
        )
    parse_timestamp(value, "captured_at_utc")
    return value


def validate_repo_path(value: Any, label: str, *, directory: bool = False) -> str:
    if (
        not isinstance(value, str)
        or not value
        or value.startswith(("/", "~"))
        or "\\" in value
        or "\x00" in value
    ):
        fail(f"{label} must be a repository-relative POSIX path")
    path = pathlib.PurePosixPath(value)
    if any(part in {"", ".", ".."} for part in path.parts):
        fail(f"{label} contains an unsafe component")
    if directory and not value.endswith("/"):
        fail(f"{label} directory prefix must end with '/'")
    if not directory and value.endswith("/"):
        fail(f"{label} file path must not end with '/'")
    return value


def require_regular_file(path: pathlib.Path, label: str) -> pathlib.Path:
    if not path.is_absolute():
        fail(f"{label} must be an absolute canonical path")
    try:
        resolved = path.resolve(strict=True)
        metadata = path.lstat()
    except OSError as error:
        fail(f"{label} is unavailable: {error}")
    if resolved != path or stat.S_ISLNK(metadata.st_mode) or not stat.S_ISREG(metadata.st_mode):
        fail(f"{label} must be a canonical non-symlink regular file")
    if getattr(metadata, "st_nlink", 1) != 1:
        fail(f"{label} must have exactly one hard link")
    if metadata.st_mode & (stat.S_IWGRP | stat.S_IWOTH):
        fail(f"{label} must not be group/world writable")
    return path


def load_json(
    path: pathlib.Path,
    label: str,
    *,
    canonical_required: bool = True,
) -> tuple[dict[str, Any], bytes]:
    require_regular_file(path, label)
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


def verify_self_binding(
    value: dict[str, Any],
    *,
    id_key: str,
    prefix: str,
    domain: bytes,
    label: str,
) -> str:
    identifier = value.get(id_key)
    if not isinstance(identifier, str) or not identifier.startswith(prefix):
        fail(f"{label} ID prefix is invalid")
    digest = require_sha64(identifier.removeprefix(prefix), f"{label} ID")
    payload = dict(value)
    payload.pop(id_key)
    if digest != framed(domain, canonical(payload)):
        fail(f"{label} ID does not bind its payload")
    return identifier


def validate_candidate(candidate: dict[str, Any]) -> None:
    if candidate.get("schema") != CANDIDATE_SCHEMA or candidate.get("schema_version") != 2:
        fail("candidate schema/version drifted")
    if candidate.get("phase") != "DEVELOPMENT":
        fail("candidate phase must remain DEVELOPMENT")
    if candidate.get("claim_level") != "SOURCE_EVIDENCE_COMPLETE_SEPARATE_REVIEW_REQUIRED":
        fail("candidate claim level drifted")
    verify_self_binding(
        candidate,
        id_key="candidate_id",
        prefix="hepta-servo-exact-source-review-candidate:v2:",
        domain=CANDIDATE_DOMAIN,
        label="candidate",
    )
    hepta = candidate.get("hepta")
    if not isinstance(hepta, dict) or hepta.get("repository") != REPOSITORY:
        fail("candidate Hepta binding is invalid")
    if not isinstance(hepta.get("ref"), str) or not hepta["ref"]:
        fail("candidate Hepta ref is missing")
    require_sha40(hepta.get("commit"), "candidate Hepta commit")
    require_sha40(hepta.get("tree"), "candidate Hepta tree")
    servo = candidate.get("servo")
    if not isinstance(servo, dict) or servo != {
        "repository": "servo/servo",
        "commit": SERVO_COMMIT,
        "tree": SERVO_TREE,
        "recomputed_tree": SERVO_TREE,
        "fetch_acquisition_nonce_sha256": servo.get("fetch_acquisition_nonce_sha256")
        if isinstance(servo, dict)
        else None,
    }:
        fail("candidate Servo source binding drifted")
    nonces = servo.get("fetch_acquisition_nonce_sha256")
    if (
        not isinstance(nonces, list)
        or len(nonces) != 2
        or any(not isinstance(item, str) or not SHA64.fullmatch(item) for item in nonces)
        or nonces[0] == nonces[1]
    ):
        fail("candidate independent fetch nonces are invalid")
    workflow = candidate.get("workflow")
    if not isinstance(workflow, dict):
        fail("candidate workflow binding is missing")
    require_positive_int(workflow.get("id"), "candidate workflow run ID")
    require_positive_int(workflow.get("attempt"), "candidate workflow run attempt")
    if workflow.get("head_sha") != hepta["commit"] or workflow.get("head_tree") != hepta["tree"]:
        fail("candidate workflow and Hepta bindings differ")
    job = workflow.get("job")
    if not isinstance(job, dict):
        fail("candidate workflow job is missing")
    require_positive_int(job.get("runner_id"), "candidate runner ID")
    if not isinstance(job.get("runner_name"), str) or not job["runner_name"].strip():
        fail("candidate runner name is missing")
    checks = candidate.get("checks")
    if not isinstance(checks, dict) or set(checks) != REQUIRED_CANDIDATE_CHECKS:
        fail("candidate check field set drifted")
    if any(value is not True for value in checks.values()):
        fail("candidate contains a non-passing evidence check")
    evidence = candidate.get("evidence")
    if not isinstance(evidence, dict) or set(evidence) != {"api_json", "source_files"}:
        fail("candidate evidence projection is incomplete")
    for category in ("api_json", "source_files"):
        mapping = evidence.get(category)
        if not isinstance(mapping, dict) or not mapping:
            fail(f"candidate evidence {category} is empty")
        for name, item in mapping.items():
            if not isinstance(name, str) or not name:
                fail(f"candidate evidence {category} has an invalid name")
            if not isinstance(item, dict) or set(item) != {"sha256", "bytes"}:
                fail(f"candidate evidence {category}/{name} is malformed")
            require_sha64(item.get("sha256"), f"candidate evidence {category}/{name}")
            require_positive_int(item.get("bytes"), f"candidate evidence {category}/{name} bytes")
    if candidate.get("review") != {
        "status": "PENDING_SEPARATE_REVIEW",
        "candidate_accepted": False,
        "pointer_update_performed": False,
        "reviewer": None,
        "reviewed_at_utc": None,
    }:
        fail("candidate review posture is not pending and separate")
    if candidate.get("claims") != CANDIDATE_CLAIMS:
        fail("candidate claims posture is open")
    if candidate.get("authority") != AUTHORITY:
        fail("candidate authority posture is open")
    if candidate.get("decision") != "EVIDENCE_COMPLETE_REVIEW_REQUIRED_BUILD_NOT_AUTHORIZED":
        fail("candidate decision overclaims")


def validate_policy(policy: dict[str, Any]) -> None:
    if policy.get("schema") != POLICY_SCHEMA or policy.get("schema_version") != 1:
        fail("review policy schema/version drifted")
    if policy.get("phase") != "DEVELOPMENT":
        fail("review policy phase must remain DEVELOPMENT")
    verify_self_binding(
        policy,
        id_key="policy_id",
        prefix="hepta-servo-source-acceptance-review-policy:v1:",
        domain=POLICY_DOMAIN,
        label="review policy",
    )
    if policy.get("repository") != REPOSITORY or policy.get("base_branch") != BASE_BRANCH:
        fail("review policy repository/base branch drifted")
    pointer_path = validate_repo_path(policy.get("pointer_path"), "review policy pointer path")
    if pointer_path != (
        "docs/hepta-vnext/browser/source-acceptance/ACCEPTED_SOURCE_POINTER.json"
    ):
        fail("review policy pointer path drifted")
    validate_repo_path(
        policy.get("candidate_snapshot_prefix"),
        "review policy candidate snapshot prefix",
        directory=True,
    )
    auxiliary = policy.get("allowed_auxiliary_paths")
    if not isinstance(auxiliary, list) or not auxiliary:
        fail("review policy allowed auxiliary paths are missing")
    if auxiliary != sorted(set(auxiliary), key=lambda item: item.encode("utf-8")):
        fail("review policy auxiliary paths must be unique and bytewise sorted")
    for item in auxiliary:
        validate_repo_path(item, "review policy auxiliary path")
    review = policy.get("review")
    if not isinstance(review, dict) or set(review) != {
        "minimum_approvals",
        "reviewer_must_differ_from_pr_author",
        "require_current_head_commit",
        "reject_changes_requested",
        "allowed_author_associations",
        "required_body_prefix",
        "head_ref_prefix",
        "draft_allowed",
        "codeowner_review_required",
    }:
        fail("review policy review field set drifted")
    require_positive_int(review.get("minimum_approvals"), "minimum approvals")
    for key in (
        "reviewer_must_differ_from_pr_author",
        "require_current_head_commit",
        "reject_changes_requested",
        "codeowner_review_required",
    ):
        if review.get(key) is not True:
            fail(f"review policy must require {key}")
    if review.get("draft_allowed") is not False:
        fail("review policy cannot allow draft acceptance PRs")
    associations = review.get("allowed_author_associations")
    if (
        not isinstance(associations, list)
        or associations != sorted(set(associations))
        or not associations
        or any(item not in {"COLLABORATOR", "MEMBER", "OWNER"} for item in associations)
    ):
        fail("review policy trusted author associations drifted")
    if review.get("required_body_prefix") != "HEPTA_SOURCE_ACCEPT_V1 ":
        fail("review policy body prefix drifted")
    if review.get("head_ref_prefix") != "review/hepta-servo-source-acceptance-":
        fail("review policy head-ref prefix drifted")
    if policy.get("claims_after_acceptance") != ACCEPTED_SOURCE_CLAIMS:
        fail("review policy post-acceptance claims drifted")
    if policy.get("authority") != AUTHORITY:
        fail("review policy authority posture is open")


def compile_challenge(
    candidate: dict[str, Any],
    candidate_raw: bytes,
    policy: dict[str, Any],
    policy_raw: bytes,
    captured_at_utc: str | None,
) -> dict[str, Any]:
    validate_candidate(candidate)
    validate_policy(policy)
    challenge: dict[str, Any] = {
        "schema": CHALLENGE_SCHEMA,
        "schema_version": 1,
        "phase": "DEVELOPMENT",
        "claim_level": "EXACT_SOURCE_ACCEPTANCE_REVIEW_CHALLENGE_ONLY",
        "captured_at_utc": timestamp(captured_at_utc),
        "candidate": {
            "id": candidate["candidate_id"],
            "sha256": sha256_bytes(candidate_raw),
            "bytes": len(candidate_raw),
        },
        "policy": {
            "id": policy["policy_id"],
            "sha256": sha256_bytes(policy_raw),
            "bytes": len(policy_raw),
        },
        "hepta": dict(candidate["hepta"]),
        "servo": {
            "repository": "servo/servo",
            "commit": SERVO_COMMIT,
            "tree": SERVO_TREE,
            "recomputed_tree": SERVO_TREE,
        },
        "workflow": {
            "run_id": candidate["workflow"]["id"],
            "run_attempt": candidate["workflow"]["attempt"],
        },
        "requested_decision": "ACCEPT_EXACT_SERVO_SOURCE_ONLY",
        "claims_after_acceptance": dict(ACCEPTED_SOURCE_CLAIMS),
        "authority": dict(AUTHORITY),
        "decision": "REVIEW_CHALLENGE_CREATED_SOURCE_NOT_ACCEPTED_BUILD_NOT_AUTHORIZED",
    }
    challenge["challenge_id"] = (
        "hepta-servo-source-acceptance-review-challenge:v1:"
        + framed(CHALLENGE_DOMAIN, canonical(challenge))
    )
    return challenge


def validate_challenge(
    challenge: dict[str, Any],
    candidate: dict[str, Any],
    candidate_raw: bytes,
    policy: dict[str, Any],
    policy_raw: bytes,
) -> None:
    if challenge.get("schema") != CHALLENGE_SCHEMA or challenge.get("schema_version") != 1:
        fail("review challenge schema/version drifted")
    verify_self_binding(
        challenge,
        id_key="challenge_id",
        prefix="hepta-servo-source-acceptance-review-challenge:v1:",
        domain=CHALLENGE_DOMAIN,
        label="review challenge",
    )
    if challenge.get("phase") != "DEVELOPMENT" or challenge.get("claim_level") != (
        "EXACT_SOURCE_ACCEPTANCE_REVIEW_CHALLENGE_ONLY"
    ):
        fail("review challenge phase/claim drifted")
    expected = compile_challenge(
        candidate,
        candidate_raw,
        policy,
        policy_raw,
        challenge.get("captured_at_utc"),
    )
    if challenge != expected:
        fail("review challenge does not match candidate and policy")
    if challenge.get("claims_after_acceptance") != ACCEPTED_SOURCE_CLAIMS:
        fail("review challenge post-acceptance claims drifted")
    if challenge.get("authority") != AUTHORITY:
        fail("review challenge authority posture is open")


def validate_candidate_snapshot_path(path: Any, candidate: dict[str, Any], policy: dict[str, Any]) -> str:
    value = validate_repo_path(path, "accepted pointer candidate snapshot path")
    prefix = policy["candidate_snapshot_prefix"]
    suffix = candidate["candidate_id"].rsplit(":", 1)[-1] + ".json"
    expected = prefix + suffix
    if value != expected:
        fail("accepted pointer candidate snapshot path is not deterministic")
    return value


def validate_pointer(
    pointer: dict[str, Any],
    candidate: dict[str, Any],
    candidate_raw: bytes,
    policy: dict[str, Any],
    policy_raw: bytes,
    challenge: dict[str, Any],
    challenge_raw: bytes,
) -> None:
    validate_candidate(candidate)
    validate_policy(policy)
    validate_challenge(challenge, candidate, candidate_raw, policy, policy_raw)
    if pointer.get("schema") != POINTER_SCHEMA or pointer.get("schema_version") != 1:
        fail("accepted source pointer schema/version drifted")
    verify_self_binding(
        pointer,
        id_key="pointer_id",
        prefix="hepta-servo-accepted-source-pointer:v1:",
        domain=POINTER_DOMAIN,
        label="accepted source pointer",
    )
    if pointer.get("phase") != "DEVELOPMENT" or pointer.get("claim_level") != (
        "EXACT_SERVO_SOURCE_ACCEPTED_TOPOLOGY_REVIEW_REQUIRED"
    ):
        fail("accepted source pointer phase/claim drifted")
    parse_timestamp(pointer.get("accepted_at_utc"), "accepted source pointer timestamp")
    candidate_binding = pointer.get("candidate")
    if not isinstance(candidate_binding, dict) or set(candidate_binding) != {
        "id",
        "sha256",
        "bytes",
        "snapshot_path",
    }:
        fail("accepted pointer candidate binding is malformed")
    expected_snapshot = validate_candidate_snapshot_path(
        candidate_binding.get("snapshot_path"),
        candidate,
        policy,
    )
    if candidate_binding != {
        "id": candidate["candidate_id"],
        "sha256": sha256_bytes(candidate_raw),
        "bytes": len(candidate_raw),
        "snapshot_path": expected_snapshot,
    }:
        fail("accepted pointer candidate bytes differ")
    challenge_binding = pointer.get("challenge")
    if challenge_binding != {
        "id": challenge["challenge_id"],
        "sha256": sha256_bytes(challenge_raw),
        "bytes": len(challenge_raw),
    }:
        fail("accepted pointer challenge bytes differ")
    policy_binding = pointer.get("policy")
    if policy_binding != {
        "id": policy["policy_id"],
        "sha256": sha256_bytes(policy_raw),
        "bytes": len(policy_raw),
    }:
        fail("accepted pointer policy bytes differ")
    if pointer.get("hepta") != candidate["hepta"]:
        fail("accepted pointer Hepta binding differs from candidate")
    if pointer.get("servo") != {
        "repository": "servo/servo",
        "commit": SERVO_COMMIT,
        "tree": SERVO_TREE,
        "recomputed_tree": SERVO_TREE,
    }:
        fail("accepted pointer Servo binding drifted")
    if pointer.get("workflow") != {
        "run_id": candidate["workflow"]["id"],
        "run_attempt": candidate["workflow"]["attempt"],
    }:
        fail("accepted pointer workflow binding differs")
    if pointer.get("evidence") != candidate["evidence"]:
        fail("accepted pointer evidence projection differs from candidate")
    if pointer.get("review") != {
        "mode": "GITHUB_PULL_REQUEST_REVIEW",
        "state": "REQUIRES_LIVE_APPROVAL_EVIDENCE",
        "policy_id": policy["policy_id"],
        "challenge_id": challenge["challenge_id"],
    }:
        fail("accepted pointer review posture drifted")
    if pointer.get("claims") != ACCEPTED_SOURCE_CLAIMS:
        fail("accepted pointer claims drifted")
    if pointer.get("authority") != AUTHORITY:
        fail("accepted pointer authority posture is open")
    if pointer.get("decision") != (
        "EXACT_SOURCE_ACCEPTED_TOPOLOGY_REVIEW_REQUIRED_BUILD_NOT_AUTHORIZED"
    ):
        fail("accepted pointer decision overclaims")


def parse_collection(value: Any, label: str, key: str) -> list[dict[str, Any]]:
    if isinstance(value, list):
        items = value
    elif isinstance(value, dict) and isinstance(value.get(key), list):
        items = value[key]
    else:
        fail(f"{label} payload is malformed")
    if any(not isinstance(item, dict) for item in items):
        fail(f"{label} contains a non-object item")
    return items


def verify_live_review(
    pointer: dict[str, Any],
    candidate: dict[str, Any],
    policy: dict[str, Any],
    challenge: dict[str, Any],
    pull_request: dict[str, Any],
    reviews_payload: Any,
    files_payload: Any,
    head_commit: dict[str, Any],
) -> dict[str, Any]:
    review_policy = policy["review"]
    if pull_request.get("state") != "open":
        fail("source acceptance pull request must remain open while reviewed")
    if pull_request.get("draft") is not review_policy["draft_allowed"]:
        fail("source acceptance pull request draft posture is invalid")
    base = pull_request.get("base")
    head = pull_request.get("head")
    if not isinstance(base, dict) or not isinstance(head, dict):
        fail("source acceptance pull request refs are missing")
    base_repo = base.get("repo")
    head_repo = head.get("repo")
    if (
        not isinstance(base_repo, dict)
        or not isinstance(head_repo, dict)
        or base_repo.get("full_name") != REPOSITORY
        or head_repo.get("full_name") != REPOSITORY
        or base.get("ref") != policy["base_branch"]
    ):
        fail("source acceptance pull request repository/base binding drifted")
    head_ref = head.get("ref")
    if not isinstance(head_ref, str) or not head_ref.startswith(review_policy["head_ref_prefix"]):
        fail("source acceptance pull request head ref is outside the review lane")
    head_sha = require_sha40(head.get("sha"), "source acceptance PR head SHA")
    if head_commit.get("sha") != head_sha:
        fail("source acceptance PR head commit payload differs")
    commit_record = head_commit.get("commit")
    if not isinstance(commit_record, dict):
        fail("source acceptance head commit record is missing")
    committer = commit_record.get("committer")
    if not isinstance(committer, dict):
        fail("source acceptance head committer record is missing")
    commit_time = parse_timestamp(committer.get("date"), "source acceptance head commit time")
    author = pull_request.get("user")
    author_login = author.get("login") if isinstance(author, dict) else None
    if not isinstance(author_login, str) or not SAFE_LOGIN.fullmatch(author_login):
        fail("source acceptance PR author login is invalid")

    files = parse_collection(files_payload, "pull request files", "files")
    filenames: list[str] = []
    for item in files:
        filename = item.get("filename", item.get("path"))
        filenames.append(validate_repo_path(filename, "source acceptance changed path"))
    if len(filenames) != len(set(filenames)):
        fail("source acceptance PR contains duplicate changed paths")
    required_paths = {
        policy["pointer_path"],
        pointer["candidate"]["snapshot_path"],
    }
    allowed_paths = required_paths | set(policy["allowed_auxiliary_paths"])
    if not required_paths <= set(filenames):
        fail("source acceptance PR omits the pointer or candidate snapshot")
    unknown = sorted(set(filenames) - allowed_paths)
    if unknown:
        fail(f"source acceptance PR changes non-governed paths: {unknown}")

    reviews = parse_collection(reviews_payload, "pull request reviews", "reviews")
    challenge_line = review_policy["required_body_prefix"] + challenge["challenge_id"]
    approvals: dict[str, dict[str, Any]] = {}
    changes_requested: set[str] = set()
    for review in reviews:
        state = review.get("state")
        commit_id = review.get("commit_id")
        reviewer = review.get("user")
        login = reviewer.get("login") if isinstance(reviewer, dict) else None
        if not isinstance(login, str) or not SAFE_LOGIN.fullmatch(login):
            continue
        if review_policy["require_current_head_commit"] and commit_id != head_sha:
            continue
        if state == "CHANGES_REQUESTED":
            changes_requested.add(login)
            continue
        if state != "APPROVED":
            continue
        if review_policy["reviewer_must_differ_from_pr_author"] and login == author_login:
            fail("source acceptance review cannot be self-approved")
        if review.get("author_association") not in review_policy["allowed_author_associations"]:
            fail(f"source acceptance reviewer {login!r} is not a trusted collaborator")
        body = review.get("body")
        if not isinstance(body, str) or challenge_line not in body.splitlines():
            fail(f"source acceptance reviewer {login!r} did not bind the exact challenge")
        submitted = parse_timestamp(
            review.get("submitted_at"),
            f"source acceptance review time for {login}",
        )
        if submitted < commit_time:
            fail(f"source acceptance review by {login!r} predates the reviewed head")
        approvals[login] = review
    if review_policy["reject_changes_requested"] and changes_requested:
        fail(f"source acceptance has current-head change requests: {sorted(changes_requested)}")
    if len(approvals) < review_policy["minimum_approvals"]:
        fail("source acceptance has insufficient distinct current-head approvals")
    return {
        "schema": "hepta.servo.source_acceptance_live_review.v1",
        "status": "PASS_LIVE_REVIEW_SOURCE_ONLY",
        "pointer_id": pointer["pointer_id"],
        "challenge_id": challenge["challenge_id"],
        "head_sha": head_sha,
        "approvers": sorted(approvals),
        "changed_paths": sorted(filenames, key=lambda item: item.encode("utf-8")),
        "exact_servo_source_accepted": True,
        "worker_source_topology_accepted": False,
        "build_authorized": False,
        "servo_built": False,
        "servo_runtime_qualified": False,
        "authority": "all_false",
    }


def create_only_json(path: pathlib.Path, value: dict[str, Any]) -> None:
    if not path.is_absolute():
        fail("output must be an absolute canonical path")
    try:
        parent = path.parent.resolve(strict=True)
    except OSError as error:
        fail(f"output parent is unavailable: {error}")
    if parent / path.name != path:
        fail("output path is not canonical")
    if path.exists():
        fail("output already exists")
    descriptor = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
    with os.fdopen(descriptor, "wb") as handle:
        handle.write(canonical(value))
        handle.flush()
        os.fsync(handle.fileno())


def contract(policy: dict[str, Any]) -> dict[str, Any]:
    validate_policy(policy)
    return {
        "schema": POINTER_SCHEMA,
        "status": "PASS_CONTRACT_ONLY",
        "policy_id": policy["policy_id"],
        "candidate_schema": CANDIDATE_SCHEMA,
        "minimum_approvals": policy["review"]["minimum_approvals"],
        "reviewer_must_differ_from_pr_author": True,
        "live_current_head_review_required": True,
        "pointer_creation_command": False,
        "exact_servo_source_accepted": False,
        "build_authorized": False,
        "servo_built": False,
        "servo_runtime_qualified": False,
        "authority": "all_false",
    }


def load_bound_inputs(
    candidate_path: pathlib.Path,
    policy_path: pathlib.Path,
    challenge_path: pathlib.Path | None = None,
    pointer_path: pathlib.Path | None = None,
) -> tuple[
    dict[str, Any],
    bytes,
    dict[str, Any],
    bytes,
    dict[str, Any] | None,
    bytes | None,
    dict[str, Any] | None,
    bytes | None,
]:
    candidate, candidate_raw = load_json(candidate_path, "source review candidate")
    policy, policy_raw = load_json(policy_path, "source acceptance review policy")
    validate_candidate(candidate)
    validate_policy(policy)
    challenge = challenge_raw = pointer = pointer_raw = None
    if challenge_path is not None:
        challenge, challenge_raw = load_json(challenge_path, "source acceptance challenge")
        validate_challenge(challenge, candidate, candidate_raw, policy, policy_raw)
    if pointer_path is not None:
        if challenge is None or challenge_raw is None:
            fail("pointer verification requires a challenge")
        pointer, pointer_raw = load_json(pointer_path, "accepted source pointer")
        validate_pointer(
            pointer,
            candidate,
            candidate_raw,
            policy,
            policy_raw,
            challenge,
            challenge_raw,
        )
    return (
        candidate,
        candidate_raw,
        policy,
        policy_raw,
        challenge,
        challenge_raw,
        pointer,
        pointer_raw,
    )


def parse_arguments() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    subparsers = parser.add_subparsers(dest="command", required=True)

    contract_parser = subparsers.add_parser("contract")
    contract_parser.add_argument("--policy", required=True)

    challenge_parser = subparsers.add_parser("challenge")
    challenge_parser.add_argument("--candidate", required=True)
    challenge_parser.add_argument("--policy", required=True)
    challenge_parser.add_argument("--captured-at-utc")
    challenge_parser.add_argument("--output", required=True)

    verify_challenge = subparsers.add_parser("verify-challenge")
    verify_challenge.add_argument("--candidate", required=True)
    verify_challenge.add_argument("--policy", required=True)
    verify_challenge.add_argument("--challenge", required=True)

    verify_pointer_parser = subparsers.add_parser("verify-pointer")
    verify_pointer_parser.add_argument("--candidate", required=True)
    verify_pointer_parser.add_argument("--policy", required=True)
    verify_pointer_parser.add_argument("--challenge", required=True)
    verify_pointer_parser.add_argument("--pointer", required=True)

    live_parser = subparsers.add_parser("verify-live-review")
    live_parser.add_argument("--candidate", required=True)
    live_parser.add_argument("--policy", required=True)
    live_parser.add_argument("--challenge", required=True)
    live_parser.add_argument("--pointer", required=True)
    live_parser.add_argument("--pull-request", required=True)
    live_parser.add_argument("--reviews", required=True)
    live_parser.add_argument("--files", required=True)
    live_parser.add_argument("--head-commit", required=True)

    return parser.parse_args()


def main() -> int:
    try:
        arguments = parse_arguments()
        if arguments.command == "contract":
            policy, _raw = load_json(
                pathlib.Path(arguments.policy),
                "source acceptance review policy",
            )
            result = contract(policy)
        elif arguments.command == "challenge":
            (
                candidate,
                candidate_raw,
                policy,
                policy_raw,
                _challenge,
                _challenge_raw,
                _pointer,
                _pointer_raw,
            ) = load_bound_inputs(
                pathlib.Path(arguments.candidate),
                pathlib.Path(arguments.policy),
            )
            result = compile_challenge(
                candidate,
                candidate_raw,
                policy,
                policy_raw,
                arguments.captured_at_utc,
            )
            create_only_json(pathlib.Path(arguments.output), result)
        elif arguments.command == "verify-challenge":
            load_bound_inputs(
                pathlib.Path(arguments.candidate),
                pathlib.Path(arguments.policy),
                pathlib.Path(arguments.challenge),
            )
            result = {
                "schema": CHALLENGE_SCHEMA,
                "status": "PASS_CHALLENGE_SOURCE_NOT_ACCEPTED",
                "exact_servo_source_accepted": False,
                "build_authorized": False,
                "authority": "all_false",
            }
        elif arguments.command == "verify-pointer":
            (
                _candidate,
                _candidate_raw,
                _policy,
                _policy_raw,
                _challenge,
                _challenge_raw,
                pointer,
                _pointer_raw,
            ) = load_bound_inputs(
                pathlib.Path(arguments.candidate),
                pathlib.Path(arguments.policy),
                pathlib.Path(arguments.challenge),
                pathlib.Path(arguments.pointer),
            )
            assert pointer is not None
            result = {
                "schema": POINTER_SCHEMA,
                "status": "PASS_POINTER_LIVE_REVIEW_STILL_REQUIRED",
                "pointer_id": pointer["pointer_id"],
                "exact_servo_source_accepted": True,
                "worker_source_topology_accepted": False,
                "build_authorized": False,
                "authority": "all_false",
            }
        else:
            (
                candidate,
                _candidate_raw,
                policy,
                _policy_raw,
                challenge,
                _challenge_raw,
                pointer,
                _pointer_raw,
            ) = load_bound_inputs(
                pathlib.Path(arguments.candidate),
                pathlib.Path(arguments.policy),
                pathlib.Path(arguments.challenge),
                pathlib.Path(arguments.pointer),
            )
            assert challenge is not None and pointer is not None
            pull_request, _ = load_json(
                pathlib.Path(arguments.pull_request),
                "source acceptance pull request",
                canonical_required=False,
            )
            reviews, _ = load_json(
                pathlib.Path(arguments.reviews),
                "source acceptance pull request reviews",
                canonical_required=False,
            )
            files_value, _ = load_json(
                pathlib.Path(arguments.files),
                "source acceptance pull request files",
                canonical_required=False,
            )
            head_commit, _ = load_json(
                pathlib.Path(arguments.head_commit),
                "source acceptance head commit",
                canonical_required=False,
            )
            result = verify_live_review(
                pointer,
                candidate,
                policy,
                challenge,
                pull_request,
                reviews,
                files_value,
                head_commit,
            )
    except (AcceptanceError, OSError, UnicodeError, AssertionError) as error:
        print(
            f"HEPTA_SERVO_EXACT_SOURCE_ACCEPTANCE_POINTER_V1=FAIL: {error}",
            file=sys.stderr,
        )
        return 1
    print(json.dumps(result, sort_keys=True, separators=(",", ":"), ensure_ascii=False))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
