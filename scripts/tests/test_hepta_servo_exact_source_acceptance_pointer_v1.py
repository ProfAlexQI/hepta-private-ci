#!/usr/bin/env python3
from __future__ import annotations

import importlib.util
import json
import os
import pathlib
import tempfile
import unittest
from types import ModuleType
from typing import Any

ROOT = pathlib.Path(__file__).resolve().parents[2]
TOOL_PATH = ROOT / "scripts/hepta-servo-exact-source-acceptance-pointer-v1.py"
POLICY_PATH = ROOT / "docs/hepta-vnext/browser/SOURCE_ACCEPTANCE_REVIEW_POLICY_V1.json"


def load_tool() -> ModuleType:
    spec = importlib.util.spec_from_file_location("acceptance_pointer_v1", TOOL_PATH)
    if spec is None or spec.loader is None:
        raise RuntimeError("cannot load acceptance pointer tool")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


TOOL = load_tool()


def write_json(path: pathlib.Path, value: object, *, canonical: bool = True) -> bytes:
    raw = TOOL.canonical(value) if canonical else json.dumps(value, indent=2).encode("utf-8")
    path.write_bytes(raw)
    os.chmod(path, 0o600)
    return raw


def build_candidate() -> dict[str, Any]:
    source_files = {
        "fetch-a.receipt.json": {"sha256": "11" * 32, "bytes": 101},
        "fetch-b.receipt.json": {"sha256": "22" * 32, "bytes": 102},
        "independent-source-bundle.receipt.json": {
            "sha256": "33" * 32,
            "bytes": 103,
        },
        "license-packet.json": {"sha256": "44" * 32, "bytes": 104},
        "servo-source-a.tar.gz": {"sha256": "55" * 32, "bytes": 105},
        "source-bundle.verification.json": {"sha256": "66" * 32, "bytes": 106},
    }
    api_json = {
        "workflow-run.json": {"sha256": "77" * 32, "bytes": 201},
        "workflow-jobs.json": {"sha256": "88" * 32, "bytes": 202},
        "workflow-artifacts.json": {"sha256": "99" * 32, "bytes": 203},
    }
    candidate: dict[str, Any] = {
        "schema": TOOL.CANDIDATE_SCHEMA,
        "schema_version": 2,
        "phase": "DEVELOPMENT",
        "claim_level": "SOURCE_EVIDENCE_COMPLETE_SEPARATE_REVIEW_REQUIRED",
        "captured_at_utc": "2026-08-28T12:00:00Z",
        "hepta": {
            "repository": TOOL.REPOSITORY,
            "ref": "codex/hepta-vnext-plan-browser-c0-c3-20260827",
            "commit": "ab" * 20,
            "tree": "cd" * 20,
        },
        "servo": {
            "repository": "servo/servo",
            "commit": TOOL.SERVO_COMMIT,
            "tree": TOOL.SERVO_TREE,
            "recomputed_tree": TOOL.SERVO_TREE,
            "fetch_acquisition_nonce_sha256": ["aa" * 32, "bb" * 32],
        },
        "workflow": {
            "id": 12345,
            "attempt": 1,
            "name": "hepta-servo independent source qualification v3",
            "path": ".github/workflows/hepta-servo-independent-source-qualification-v3.yml",
            "event": "workflow_dispatch",
            "head_ref": "codex/hepta-vnext-plan-browser-c0-c3-20260827",
            "head_sha": "ab" * 20,
            "head_tree": "cd" * 20,
            "html_url": "https://github.com/ProfAlexQI/hepta-private-ci/actions/runs/12345",
            "job": {
                "id": 98765,
                "name": "Exact source, deterministic archive, Git-order tree reconstruction",
                "runner_id": 42,
                "runner_name": "GitHub Actions 42",
                "required_step_count": 8,
                "recorded_step_count": 9,
            },
        },
        "artifacts": [
            {
                "id": 1,
                "name": "hepta-servo-independent-source-v3-receipts",
                "size_in_bytes": 1000,
                "expired": False,
            },
            {
                "id": 2,
                "name": f"servo-{TOOL.SERVO_COMMIT}-source-v3",
                "size_in_bytes": 2000,
                "expired": False,
            },
        ],
        "evidence": {"api_json": api_json, "source_files": source_files},
        "checks": {key: True for key in TOOL.REQUIRED_CANDIDATE_CHECKS},
        "review": {
            "status": "PENDING_SEPARATE_REVIEW",
            "candidate_accepted": False,
            "pointer_update_performed": False,
            "reviewer": None,
            "reviewed_at_utc": None,
        },
        "claims": dict(TOOL.CANDIDATE_CLAIMS),
        "authority": dict(TOOL.AUTHORITY),
        "decision": "EVIDENCE_COMPLETE_REVIEW_REQUIRED_BUILD_NOT_AUTHORIZED",
    }
    candidate["candidate_id"] = (
        "hepta-servo-exact-source-review-candidate:v2:"
        + TOOL.framed(TOOL.CANDIDATE_DOMAIN, TOOL.canonical(candidate))
    )
    return candidate


def build_pointer(
    candidate: dict[str, Any],
    candidate_raw: bytes,
    policy: dict[str, Any],
    policy_raw: bytes,
    challenge: dict[str, Any],
    challenge_raw: bytes,
) -> dict[str, Any]:
    suffix = candidate["candidate_id"].rsplit(":", 1)[-1]
    pointer: dict[str, Any] = {
        "schema": TOOL.POINTER_SCHEMA,
        "schema_version": 1,
        "phase": "DEVELOPMENT",
        "claim_level": "EXACT_SERVO_SOURCE_ACCEPTED_TOPOLOGY_REVIEW_REQUIRED",
        "accepted_at_utc": "2026-08-28T13:00:00Z",
        "candidate": {
            "id": candidate["candidate_id"],
            "sha256": TOOL.sha256_bytes(candidate_raw),
            "bytes": len(candidate_raw),
            "snapshot_path": policy["candidate_snapshot_prefix"] + suffix + ".json",
        },
        "challenge": {
            "id": challenge["challenge_id"],
            "sha256": TOOL.sha256_bytes(challenge_raw),
            "bytes": len(challenge_raw),
        },
        "policy": {
            "id": policy["policy_id"],
            "sha256": TOOL.sha256_bytes(policy_raw),
            "bytes": len(policy_raw),
        },
        "hepta": dict(candidate["hepta"]),
        "servo": {
            "repository": "servo/servo",
            "commit": TOOL.SERVO_COMMIT,
            "tree": TOOL.SERVO_TREE,
            "recomputed_tree": TOOL.SERVO_TREE,
        },
        "workflow": {
            "run_id": candidate["workflow"]["id"],
            "run_attempt": candidate["workflow"]["attempt"],
        },
        "evidence": candidate["evidence"],
        "review": {
            "mode": "GITHUB_PULL_REQUEST_REVIEW",
            "state": "REQUIRES_LIVE_APPROVAL_EVIDENCE",
            "policy_id": policy["policy_id"],
            "challenge_id": challenge["challenge_id"],
        },
        "claims": dict(TOOL.ACCEPTED_SOURCE_CLAIMS),
        "authority": dict(TOOL.AUTHORITY),
        "decision": (
            "EXACT_SOURCE_ACCEPTED_TOPOLOGY_REVIEW_REQUIRED_BUILD_NOT_AUTHORIZED"
        ),
    }
    pointer["pointer_id"] = (
        "hepta-servo-accepted-source-pointer:v1:"
        + TOOL.framed(TOOL.POINTER_DOMAIN, TOOL.canonical(pointer))
    )
    return pointer


class AcceptancePointerTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temp = tempfile.TemporaryDirectory()
        self.root = pathlib.Path(self.temp.name).resolve()
        self.policy = json.loads(POLICY_PATH.read_text(encoding="utf-8"))
        self.policy_raw = POLICY_PATH.read_bytes()
        self.candidate = build_candidate()
        self.candidate_path = self.root / "candidate.json"
        self.candidate_raw = write_json(self.candidate_path, self.candidate)
        self.policy_path = self.root / "policy.json"
        write_json(self.policy_path, self.policy)
        self.challenge = TOOL.compile_challenge(
            self.candidate,
            self.candidate_raw,
            self.policy,
            self.policy_raw,
            "2026-08-28T12:30:00Z",
        )
        self.challenge_path = self.root / "challenge.json"
        self.challenge_raw = write_json(self.challenge_path, self.challenge)
        self.pointer = build_pointer(
            self.candidate,
            self.candidate_raw,
            self.policy,
            self.policy_raw,
            self.challenge,
            self.challenge_raw,
        )
        self.pointer_path = self.root / "pointer.json"
        self.pointer_raw = write_json(self.pointer_path, self.pointer)

    def tearDown(self) -> None:
        self.temp.cleanup()

    def rewrite_candidate(self) -> None:
        self.candidate_path.unlink()
        self.candidate_raw = write_json(self.candidate_path, self.candidate)

    def rewrite_challenge(self) -> None:
        self.challenge_path.unlink()
        self.challenge_raw = write_json(self.challenge_path, self.challenge)

    def rewrite_pointer(self) -> None:
        self.pointer_path.unlink()
        self.pointer_raw = write_json(self.pointer_path, self.pointer)

    def live_payloads(self) -> tuple[dict[str, Any], dict[str, Any], dict[str, Any], dict[str, Any]]:
        head_sha = "ef" * 20
        pr = {
            "state": "open",
            "draft": False,
            "user": {"login": "proposal-author"},
            "base": {
                "ref": TOOL.BASE_BRANCH,
                "repo": {"full_name": TOOL.REPOSITORY},
            },
            "head": {
                "ref": "review/hepta-servo-source-acceptance-fixture",
                "sha": head_sha,
                "repo": {"full_name": TOOL.REPOSITORY},
            },
        }
        reviews = {
            "reviews": [
                {
                    "state": "APPROVED",
                    "commit_id": head_sha,
                    "user": {"login": "security-reviewer"},
                    "author_association": "COLLABORATOR",
                    "body": (
                        "Reviewed retained exact-source evidence.\n"
                        + self.policy["review"]["required_body_prefix"]
                        + self.challenge["challenge_id"]
                    ),
                    "submitted_at": "2026-08-28T14:05:00Z",
                }
            ]
        }
        files = {
            "files": [
                {"filename": self.policy["pointer_path"]},
                {"filename": self.pointer["candidate"]["snapshot_path"]},
                {
                    "filename": (
                        "docs/hepta-vnext/browser/"
                        "C1_EXACT_SOURCE_ACCEPTANCE_POINTER_V1_STATUS.json"
                    )
                },
            ]
        }
        commit = {
            "sha": head_sha,
            "commit": {"committer": {"date": "2026-08-28T14:00:00Z"}},
        }
        return pr, reviews, files, commit

    def test_contract_is_closed(self) -> None:
        summary = TOOL.contract(self.policy)
        self.assertEqual(summary["status"], "PASS_CONTRACT_ONLY")
        self.assertFalse(summary["pointer_creation_command"])
        self.assertFalse(summary["exact_servo_source_accepted"])
        self.assertFalse(summary["build_authorized"])

    def test_challenge_is_self_bound(self) -> None:
        TOOL.validate_challenge(
            self.challenge,
            self.candidate,
            self.candidate_raw,
            self.policy,
            self.policy_raw,
        )

    def test_challenge_output_is_create_only_0600(self) -> None:
        output = self.root / "challenge-output.json"
        TOOL.create_only_json(output, self.challenge)
        self.assertEqual(output.stat().st_mode & 0o777, 0o600)
        with self.assertRaises(TOOL.AcceptanceError):
            TOOL.create_only_json(output, self.challenge)

    def test_pointer_verifies_but_build_stays_closed(self) -> None:
        TOOL.validate_pointer(
            self.pointer,
            self.candidate,
            self.candidate_raw,
            self.policy,
            self.policy_raw,
            self.challenge,
            self.challenge_raw,
        )
        self.assertTrue(self.pointer["claims"]["exact_servo_source_accepted"])
        self.assertFalse(self.pointer["claims"]["build_authorized"])

    def test_live_review_passes_with_distinct_current_head_approval(self) -> None:
        pr, reviews, files, commit = self.live_payloads()
        result = TOOL.verify_live_review(
            self.pointer,
            self.candidate,
            self.policy,
            self.challenge,
            pr,
            reviews,
            files,
            commit,
        )
        self.assertEqual(result["status"], "PASS_LIVE_REVIEW_SOURCE_ONLY")
        self.assertEqual(result["approvers"], ["security-reviewer"])
        self.assertFalse(result["build_authorized"])

    def test_self_approval_is_rejected(self) -> None:
        pr, reviews, files, commit = self.live_payloads()
        reviews["reviews"][0]["user"]["login"] = "proposal-author"
        with self.assertRaises(TOOL.AcceptanceError):
            TOOL.verify_live_review(
                self.pointer, self.candidate, self.policy, self.challenge,
                pr, reviews, files, commit,
            )

    def test_stale_commit_approval_is_rejected(self) -> None:
        pr, reviews, files, commit = self.live_payloads()
        reviews["reviews"][0]["commit_id"] = "01" * 20
        with self.assertRaises(TOOL.AcceptanceError):
            TOOL.verify_live_review(
                self.pointer, self.candidate, self.policy, self.challenge,
                pr, reviews, files, commit,
            )

    def test_current_head_change_request_is_rejected(self) -> None:
        pr, reviews, files, commit = self.live_payloads()
        reviews["reviews"].append(
            {
                "state": "CHANGES_REQUESTED",
                "commit_id": pr["head"]["sha"],
                "user": {"login": "second-reviewer"},
                "author_association": "MEMBER",
                "body": "changes required",
                "submitted_at": "2026-08-28T14:06:00Z",
            }
        )
        with self.assertRaises(TOOL.AcceptanceError):
            TOOL.verify_live_review(
                self.pointer, self.candidate, self.policy, self.challenge,
                pr, reviews, files, commit,
            )

    def test_review_without_exact_challenge_line_is_rejected(self) -> None:
        pr, reviews, files, commit = self.live_payloads()
        reviews["reviews"][0]["body"] = "approved"
        with self.assertRaises(TOOL.AcceptanceError):
            TOOL.verify_live_review(
                self.pointer, self.candidate, self.policy, self.challenge,
                pr, reviews, files, commit,
            )

    def test_untrusted_review_association_is_rejected(self) -> None:
        pr, reviews, files, commit = self.live_payloads()
        reviews["reviews"][0]["author_association"] = "CONTRIBUTOR"
        with self.assertRaises(TOOL.AcceptanceError):
            TOOL.verify_live_review(
                self.pointer, self.candidate, self.policy, self.challenge,
                pr, reviews, files, commit,
            )

    def test_unknown_changed_path_is_rejected(self) -> None:
        pr, reviews, files, commit = self.live_payloads()
        files["files"].append({"filename": "scripts/unsafe-build.py"})
        with self.assertRaises(TOOL.AcceptanceError):
            TOOL.verify_live_review(
                self.pointer, self.candidate, self.policy, self.challenge,
                pr, reviews, files, commit,
            )

    def test_missing_candidate_snapshot_is_rejected(self) -> None:
        pr, reviews, files, commit = self.live_payloads()
        files["files"] = [
            item for item in files["files"]
            if item["filename"] != self.pointer["candidate"]["snapshot_path"]
        ]
        with self.assertRaises(TOOL.AcceptanceError):
            TOOL.verify_live_review(
                self.pointer, self.candidate, self.policy, self.challenge,
                pr, reviews, files, commit,
            )

    def test_draft_acceptance_pr_is_rejected(self) -> None:
        pr, reviews, files, commit = self.live_payloads()
        pr["draft"] = True
        with self.assertRaises(TOOL.AcceptanceError):
            TOOL.verify_live_review(
                self.pointer, self.candidate, self.policy, self.challenge,
                pr, reviews, files, commit,
            )

    def test_pointer_build_authority_is_rejected(self) -> None:
        self.pointer["claims"]["build_authorized"] = True
        self.pointer.pop("pointer_id")
        self.pointer["pointer_id"] = (
            "hepta-servo-accepted-source-pointer:v1:"
            + TOOL.framed(TOOL.POINTER_DOMAIN, TOOL.canonical(self.pointer))
        )
        with self.assertRaises(TOOL.AcceptanceError):
            TOOL.validate_pointer(
                self.pointer, self.candidate, self.candidate_raw,
                self.policy, self.policy_raw, self.challenge, self.challenge_raw,
            )

    def test_pointer_topology_acceptance_is_rejected(self) -> None:
        self.pointer["claims"]["worker_source_topology_accepted"] = True
        self.pointer.pop("pointer_id")
        self.pointer["pointer_id"] = (
            "hepta-servo-accepted-source-pointer:v1:"
            + TOOL.framed(TOOL.POINTER_DOMAIN, TOOL.canonical(self.pointer))
        )
        with self.assertRaises(TOOL.AcceptanceError):
            TOOL.validate_pointer(
                self.pointer, self.candidate, self.candidate_raw,
                self.policy, self.policy_raw, self.challenge, self.challenge_raw,
            )

    def test_positive_authority_is_rejected(self) -> None:
        self.pointer["authority"]["runtime_authority"] = True
        self.pointer.pop("pointer_id")
        self.pointer["pointer_id"] = (
            "hepta-servo-accepted-source-pointer:v1:"
            + TOOL.framed(TOOL.POINTER_DOMAIN, TOOL.canonical(self.pointer))
        )
        with self.assertRaises(TOOL.AcceptanceError):
            TOOL.validate_pointer(
                self.pointer, self.candidate, self.candidate_raw,
                self.policy, self.policy_raw, self.challenge, self.challenge_raw,
            )

    def test_candidate_acceptance_claim_is_rejected(self) -> None:
        self.candidate["claims"]["exact_servo_source_accepted"] = True
        self.candidate.pop("candidate_id")
        self.candidate["candidate_id"] = (
            "hepta-servo-exact-source-review-candidate:v2:"
            + TOOL.framed(TOOL.CANDIDATE_DOMAIN, TOOL.canonical(self.candidate))
        )
        with self.assertRaises(TOOL.AcceptanceError):
            TOOL.validate_candidate(self.candidate)

    def test_policy_tamper_is_rejected(self) -> None:
        policy = dict(self.policy)
        policy["base_branch"] = "main"
        with self.assertRaises(TOOL.AcceptanceError):
            TOOL.validate_policy(policy)

    def test_challenge_id_tamper_is_rejected(self) -> None:
        challenge = dict(self.challenge)
        challenge["challenge_id"] = (
            "hepta-servo-source-acceptance-review-challenge:v1:" + "00" * 32
        )
        with self.assertRaises(TOOL.AcceptanceError):
            TOOL.validate_challenge(
                challenge, self.candidate, self.candidate_raw,
                self.policy, self.policy_raw,
            )

    def test_pointer_id_tamper_is_rejected(self) -> None:
        pointer = dict(self.pointer)
        pointer["pointer_id"] = "hepta-servo-accepted-source-pointer:v1:" + "00" * 32
        with self.assertRaises(TOOL.AcceptanceError):
            TOOL.validate_pointer(
                pointer, self.candidate, self.candidate_raw,
                self.policy, self.policy_raw, self.challenge, self.challenge_raw,
            )


if __name__ == "__main__":
    unittest.main()
