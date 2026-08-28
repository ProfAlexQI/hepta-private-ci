#!/usr/bin/env python3
"""Hardened v2 entrypoint for source-only acceptance review.

The v1 core remains the canonical candidate/challenge parser. This successor
adds deterministic challenge snapshots, binds the proposed pointer to the exact
review PR number and head ref, and verifies live current-head GitHub approvals.
It never creates or updates an accepted-source pointer.
"""
from __future__ import annotations

import importlib.util
import pathlib
import sys
from types import ModuleType
from typing import Any

BASE_SCRIPT = pathlib.Path(__file__).with_name(
    "hepta-servo-exact-source-acceptance-pointer-v1.py"
)


def load_base() -> ModuleType:
    specification = importlib.util.spec_from_file_location(
        "hepta_servo_exact_source_acceptance_pointer_v1_core",
        BASE_SCRIPT,
    )
    if specification is None or specification.loader is None:
        raise RuntimeError("cannot load source acceptance pointer v1 core")
    module = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(module)
    return module


BASE = load_base()


def validate_policy(policy: dict[str, Any]) -> None:
    if policy.get("schema") != BASE.POLICY_SCHEMA or policy.get("schema_version") != 1:
        BASE.fail("review policy schema/version drifted")
    if policy.get("phase") != "DEVELOPMENT":
        BASE.fail("review policy phase must remain DEVELOPMENT")
    BASE.verify_self_binding(
        policy,
        id_key="policy_id",
        prefix="hepta-servo-source-acceptance-review-policy:v1:",
        domain=BASE.POLICY_DOMAIN,
        label="review policy",
    )
    if policy.get("repository") != BASE.REPOSITORY or policy.get("base_branch") != BASE.BASE_BRANCH:
        BASE.fail("review policy repository/base branch drifted")
    if BASE.validate_repo_path(policy.get("pointer_path"), "review policy pointer path") != (
        "docs/hepta-vnext/browser/source-acceptance/ACCEPTED_SOURCE_POINTER.json"
    ):
        BASE.fail("review policy pointer path drifted")
    for key in ("candidate_snapshot_prefix", "challenge_snapshot_prefix"):
        BASE.validate_repo_path(policy.get(key), f"review policy {key}", directory=True)
    auxiliary = policy.get("allowed_auxiliary_paths")
    if not isinstance(auxiliary, list) or not auxiliary:
        BASE.fail("review policy allowed auxiliary paths are missing")
    if auxiliary != sorted(set(auxiliary), key=lambda item: item.encode("utf-8")):
        BASE.fail("review policy auxiliary paths must be unique and bytewise sorted")
    for item in auxiliary:
        BASE.validate_repo_path(item, "review policy auxiliary path")
    review = policy.get("review")
    expected = {
        "minimum_approvals",
        "reviewer_must_differ_from_pr_author",
        "require_current_head_commit",
        "reject_changes_requested",
        "allowed_author_associations",
        "required_body_prefix",
        "head_ref_prefix",
        "draft_allowed",
        "codeowner_review_required",
        "pointer_must_bind_pull_request_number",
        "pointer_must_bind_head_ref",
        "required_status_check",
    }
    if not isinstance(review, dict) or set(review) != expected:
        BASE.fail("review policy review field set drifted")
    BASE.require_positive_int(review.get("minimum_approvals"), "minimum approvals")
    for key in (
        "reviewer_must_differ_from_pr_author",
        "require_current_head_commit",
        "reject_changes_requested",
        "pointer_must_bind_pull_request_number",
        "pointer_must_bind_head_ref",
    ):
        if review.get(key) is not True:
            BASE.fail(f"review policy must require {key}")
    if review.get("draft_allowed") is not False:
        BASE.fail("review policy cannot allow draft acceptance PRs")
    if review.get("codeowner_review_required") is not False:
        BASE.fail("review policy must not claim an unenforced CODEOWNER identity check")
    associations = review.get("allowed_author_associations")
    if (
        not isinstance(associations, list)
        or associations != sorted(set(associations))
        or not associations
        or any(item not in {"COLLABORATOR", "MEMBER", "OWNER"} for item in associations)
    ):
        BASE.fail("review policy trusted author associations drifted")
    if review.get("required_body_prefix") != "HEPTA_SOURCE_ACCEPT_V1 ":
        BASE.fail("review policy body prefix drifted")
    if review.get("head_ref_prefix") != "review/hepta-servo-source-acceptance-":
        BASE.fail("review policy head-ref prefix drifted")
    if review.get("required_status_check") != "Source-only accepted pointer live review":
        BASE.fail("review policy required status check drifted")
    if policy.get("claims_after_acceptance") != BASE.ACCEPTED_SOURCE_CLAIMS:
        BASE.fail("review policy post-acceptance claims drifted")
    if policy.get("authority") != BASE.AUTHORITY:
        BASE.fail("review policy authority posture is open")


def deterministic_snapshot_path(
    value: Any,
    *,
    prefix: str,
    identifier: str,
    label: str,
) -> str:
    path = BASE.validate_repo_path(value, label)
    expected = prefix + identifier.rsplit(":", 1)[-1] + ".json"
    if path != expected:
        BASE.fail(f"{label} is not deterministic")
    return path


def validate_pointer(
    pointer: dict[str, Any],
    candidate: dict[str, Any],
    candidate_raw: bytes,
    policy: dict[str, Any],
    policy_raw: bytes,
    challenge: dict[str, Any],
    challenge_raw: bytes,
) -> None:
    BASE.validate_candidate(candidate)
    validate_policy(policy)
    BASE.validate_challenge(challenge, candidate, candidate_raw, policy, policy_raw)
    if pointer.get("schema") != BASE.POINTER_SCHEMA or pointer.get("schema_version") != 1:
        BASE.fail("accepted source pointer schema/version drifted")
    BASE.verify_self_binding(
        pointer,
        id_key="pointer_id",
        prefix="hepta-servo-accepted-source-pointer:v1:",
        domain=BASE.POINTER_DOMAIN,
        label="accepted source pointer",
    )
    if pointer.get("phase") != "DEVELOPMENT" or pointer.get("claim_level") != (
        "EXACT_SERVO_SOURCE_ACCEPTED_TOPOLOGY_REVIEW_REQUIRED"
    ):
        BASE.fail("accepted source pointer phase/claim drifted")
    BASE.parse_timestamp(pointer.get("accepted_at_utc"), "accepted source pointer timestamp")

    candidate_binding = pointer.get("candidate")
    if not isinstance(candidate_binding, dict) or set(candidate_binding) != {
        "id", "sha256", "bytes", "snapshot_path"
    }:
        BASE.fail("accepted pointer candidate binding is malformed")
    candidate_snapshot = deterministic_snapshot_path(
        candidate_binding.get("snapshot_path"),
        prefix=policy["candidate_snapshot_prefix"],
        identifier=candidate["candidate_id"],
        label="accepted pointer candidate snapshot path",
    )
    if candidate_binding != {
        "id": candidate["candidate_id"],
        "sha256": BASE.sha256_bytes(candidate_raw),
        "bytes": len(candidate_raw),
        "snapshot_path": candidate_snapshot,
    }:
        BASE.fail("accepted pointer candidate bytes differ")

    challenge_binding = pointer.get("challenge")
    if not isinstance(challenge_binding, dict) or set(challenge_binding) != {
        "id", "sha256", "bytes", "snapshot_path"
    }:
        BASE.fail("accepted pointer challenge binding is malformed")
    challenge_snapshot = deterministic_snapshot_path(
        challenge_binding.get("snapshot_path"),
        prefix=policy["challenge_snapshot_prefix"],
        identifier=challenge["challenge_id"],
        label="accepted pointer challenge snapshot path",
    )
    if challenge_binding != {
        "id": challenge["challenge_id"],
        "sha256": BASE.sha256_bytes(challenge_raw),
        "bytes": len(challenge_raw),
        "snapshot_path": challenge_snapshot,
    }:
        BASE.fail("accepted pointer challenge bytes differ")

    if pointer.get("policy") != {
        "id": policy["policy_id"],
        "sha256": BASE.sha256_bytes(policy_raw),
        "bytes": len(policy_raw),
    }:
        BASE.fail("accepted pointer policy bytes differ")
    if pointer.get("hepta") != candidate["hepta"]:
        BASE.fail("accepted pointer Hepta binding differs from candidate")
    if pointer.get("servo") != {
        "repository": "servo/servo",
        "commit": BASE.SERVO_COMMIT,
        "tree": BASE.SERVO_TREE,
        "recomputed_tree": BASE.SERVO_TREE,
    }:
        BASE.fail("accepted pointer Servo binding drifted")
    if pointer.get("workflow") != {
        "run_id": candidate["workflow"]["id"],
        "run_attempt": candidate["workflow"]["attempt"],
    }:
        BASE.fail("accepted pointer workflow binding differs")
    if pointer.get("evidence") != candidate["evidence"]:
        BASE.fail("accepted pointer evidence projection differs from candidate")

    review = pointer.get("review")
    if not isinstance(review, dict) or set(review) != {
        "mode", "state", "policy_id", "challenge_id", "pull_request_number", "head_ref"
    }:
        BASE.fail("accepted pointer review binding is malformed")
    BASE.require_positive_int(review.get("pull_request_number"), "accepted pointer PR number")
    head_ref = review.get("head_ref")
    if not isinstance(head_ref, str) or not head_ref.startswith(policy["review"]["head_ref_prefix"]):
        BASE.fail("accepted pointer review head ref is outside the review lane")
    if {
        "mode": review.get("mode"),
        "state": review.get("state"),
        "policy_id": review.get("policy_id"),
        "challenge_id": review.get("challenge_id"),
    } != {
        "mode": "GITHUB_PULL_REQUEST_REVIEW",
        "state": "REQUIRES_LIVE_APPROVAL_EVIDENCE",
        "policy_id": policy["policy_id"],
        "challenge_id": challenge["challenge_id"],
    }:
        BASE.fail("accepted pointer review posture drifted")
    if pointer.get("claims") != BASE.ACCEPTED_SOURCE_CLAIMS:
        BASE.fail("accepted pointer claims drifted")
    if pointer.get("authority") != BASE.AUTHORITY:
        BASE.fail("accepted pointer authority posture is open")
    if pointer.get("decision") != (
        "EXACT_SOURCE_ACCEPTED_TOPOLOGY_REVIEW_REQUIRED_BUILD_NOT_AUTHORIZED"
    ):
        BASE.fail("accepted pointer decision overclaims")


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
        BASE.fail("source acceptance pull request must remain open while reviewed")
    if pull_request.get("draft") is not review_policy["draft_allowed"]:
        BASE.fail("source acceptance pull request draft posture is invalid")
    base = pull_request.get("base")
    head = pull_request.get("head")
    if not isinstance(base, dict) or not isinstance(head, dict):
        BASE.fail("source acceptance pull request refs are missing")
    base_repo = base.get("repo")
    head_repo = head.get("repo")
    if (
        not isinstance(base_repo, dict)
        or not isinstance(head_repo, dict)
        or base_repo.get("full_name") != BASE.REPOSITORY
        or head_repo.get("full_name") != BASE.REPOSITORY
        or base.get("ref") != policy["base_branch"]
    ):
        BASE.fail("source acceptance pull request repository/base binding drifted")
    head_ref = head.get("ref")
    if not isinstance(head_ref, str) or not head_ref.startswith(review_policy["head_ref_prefix"]):
        BASE.fail("source acceptance pull request head ref is outside the review lane")
    number = BASE.require_positive_int(
        pull_request.get("number"), "source acceptance pull request number"
    )
    if pointer["review"]["pull_request_number"] != number:
        BASE.fail("accepted pointer pull request number differs from live review evidence")
    if pointer["review"]["head_ref"] != head_ref:
        BASE.fail("accepted pointer head ref differs from live review evidence")
    head_sha = BASE.require_sha40(head.get("sha"), "source acceptance PR head SHA")
    if head_commit.get("sha") != head_sha:
        BASE.fail("source acceptance PR head commit payload differs")
    commit_record = head_commit.get("commit")
    committer = commit_record.get("committer") if isinstance(commit_record, dict) else None
    if not isinstance(committer, dict):
        BASE.fail("source acceptance head committer record is missing")
    commit_time = BASE.parse_timestamp(
        committer.get("date"), "source acceptance head commit time"
    )
    author = pull_request.get("user")
    author_login = author.get("login") if isinstance(author, dict) else None
    if not isinstance(author_login, str) or not BASE.SAFE_LOGIN.fullmatch(author_login):
        BASE.fail("source acceptance PR author login is invalid")

    files = BASE.parse_collection(files_payload, "pull request files", "files")
    filenames = [
        BASE.validate_repo_path(
            item.get("filename", item.get("path")), "source acceptance changed path"
        )
        for item in files
    ]
    if len(filenames) != len(set(filenames)):
        BASE.fail("source acceptance PR contains duplicate changed paths")
    required_paths = {
        policy["pointer_path"],
        pointer["candidate"]["snapshot_path"],
        pointer["challenge"]["snapshot_path"],
    }
    if not required_paths <= set(filenames):
        BASE.fail("source acceptance PR omits pointer, candidate, or challenge snapshot")
    unknown = sorted(set(filenames) - (required_paths | set(policy["allowed_auxiliary_paths"])))
    if unknown:
        BASE.fail(f"source acceptance PR changes non-governed paths: {unknown}")

    reviews = BASE.parse_collection(reviews_payload, "pull request reviews", "reviews")
    challenge_line = review_policy["required_body_prefix"] + challenge["challenge_id"]
    approvals: dict[str, dict[str, Any]] = {}
    changes_requested: set[str] = set()
    for review in reviews:
        state = review.get("state")
        reviewer = review.get("user")
        login = reviewer.get("login") if isinstance(reviewer, dict) else None
        if not isinstance(login, str) or not BASE.SAFE_LOGIN.fullmatch(login):
            continue
        if review_policy["require_current_head_commit"] and review.get("commit_id") != head_sha:
            continue
        if state == "CHANGES_REQUESTED":
            changes_requested.add(login)
            continue
        if state != "APPROVED":
            continue
        if review_policy["reviewer_must_differ_from_pr_author"] and login == author_login:
            BASE.fail("source acceptance review cannot be self-approved")
        if review.get("author_association") not in review_policy["allowed_author_associations"]:
            BASE.fail(f"source acceptance reviewer {login!r} is not a trusted collaborator")
        body = review.get("body")
        if not isinstance(body, str) or challenge_line not in body.splitlines():
            BASE.fail(f"source acceptance reviewer {login!r} did not bind the exact challenge")
        submitted = BASE.parse_timestamp(
            review.get("submitted_at"), f"source acceptance review time for {login}"
        )
        if submitted < commit_time:
            BASE.fail(f"source acceptance review by {login!r} predates the reviewed head")
        approvals[login] = review
    if review_policy["reject_changes_requested"] and changes_requested:
        BASE.fail(f"source acceptance has current-head change requests: {sorted(changes_requested)}")
    if len(approvals) < review_policy["minimum_approvals"]:
        BASE.fail("source acceptance has insufficient distinct current-head approvals")
    return {
        "schema": "hepta.servo.source_acceptance_live_review.v1",
        "status": "PASS_LIVE_REVIEW_SOURCE_ONLY",
        "pointer_id": pointer["pointer_id"],
        "challenge_id": challenge["challenge_id"],
        "pull_request_number": number,
        "head_ref": head_ref,
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


def contract(policy: dict[str, Any]) -> dict[str, Any]:
    validate_policy(policy)
    return {
        "schema": BASE.POINTER_SCHEMA,
        "status": "PASS_CONTRACT_ONLY",
        "policy_id": policy["policy_id"],
        "candidate_schema": BASE.CANDIDATE_SCHEMA,
        "minimum_approvals": policy["review"]["minimum_approvals"],
        "reviewer_must_differ_from_pr_author": True,
        "live_current_head_review_required": True,
        "required_status_check": policy["review"]["required_status_check"],
        "codeowner_identity_claimed": False,
        "pointer_creation_command": False,
        "exact_servo_source_accepted": False,
        "build_authorized": False,
        "servo_built": False,
        "servo_runtime_qualified": False,
        "authority": "all_false",
    }


BASE.validate_policy = validate_policy
BASE.validate_pointer = validate_pointer
BASE.verify_live_review = verify_live_review
BASE.contract = contract

AcceptanceError = BASE.AcceptanceError
AUTHORITY = BASE.AUTHORITY
ACCEPTED_SOURCE_CLAIMS = BASE.ACCEPTED_SOURCE_CLAIMS
CANDIDATE_CLAIMS = BASE.CANDIDATE_CLAIMS
REQUIRED_CANDIDATE_CHECKS = BASE.REQUIRED_CANDIDATE_CHECKS
CANDIDATE_DOMAIN = BASE.CANDIDATE_DOMAIN
CHALLENGE_DOMAIN = BASE.CHALLENGE_DOMAIN
POINTER_DOMAIN = BASE.POINTER_DOMAIN
CANDIDATE_SCHEMA = BASE.CANDIDATE_SCHEMA
POINTER_SCHEMA = BASE.POINTER_SCHEMA
REPOSITORY = BASE.REPOSITORY
BASE_BRANCH = BASE.BASE_BRANCH
SERVO_COMMIT = BASE.SERVO_COMMIT
SERVO_TREE = BASE.SERVO_TREE
canonical = BASE.canonical
framed = BASE.framed
sha256_bytes = BASE.sha256_bytes
compile_challenge = BASE.compile_challenge
validate_candidate = BASE.validate_candidate
validate_challenge = BASE.validate_challenge
create_only_json = BASE.create_only_json


def main() -> int:
    return BASE.main()


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except RuntimeError as error:
        print(f"HEPTA_SERVO_EXACT_SOURCE_ACCEPTANCE_POINTER_V2=FAIL: {error}", file=sys.stderr)
        raise SystemExit(1)
