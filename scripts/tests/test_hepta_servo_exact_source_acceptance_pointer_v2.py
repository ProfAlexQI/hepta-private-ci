#!/usr/bin/env python3
from __future__ import annotations

import importlib.util
import pathlib
import unittest
from types import ModuleType
from typing import Any

ROOT = pathlib.Path(__file__).resolve().parents[2]


def load(path: pathlib.Path, name: str) -> ModuleType:
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load {path}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


TOOL = load(
    ROOT / "scripts/hepta-servo-exact-source-acceptance-pointer-v2.py",
    "acceptance_pointer_v2",
)
OLD = load(
    ROOT / "scripts/tests/test_hepta_servo_exact_source_acceptance_pointer_v1.py",
    "acceptance_pointer_v1_tests",
)
OLD.TOOL = TOOL


def build_pointer(
    candidate: dict[str, Any],
    candidate_raw: bytes,
    policy: dict[str, Any],
    policy_raw: bytes,
    challenge: dict[str, Any],
    challenge_raw: bytes,
) -> dict[str, Any]:
    candidate_suffix = candidate["candidate_id"].rsplit(":", 1)[-1]
    challenge_suffix = challenge["challenge_id"].rsplit(":", 1)[-1]
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
            "snapshot_path": policy["candidate_snapshot_prefix"] + candidate_suffix + ".json",
        },
        "challenge": {
            "id": challenge["challenge_id"],
            "sha256": TOOL.sha256_bytes(challenge_raw),
            "bytes": len(challenge_raw),
            "snapshot_path": policy["challenge_snapshot_prefix"] + challenge_suffix + ".json",
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
            "pull_request_number": 77,
            "head_ref": "review/hepta-servo-source-acceptance-fixture",
        },
        "claims": dict(TOOL.ACCEPTED_SOURCE_CLAIMS),
        "authority": dict(TOOL.AUTHORITY),
        "decision": "EXACT_SOURCE_ACCEPTED_TOPOLOGY_REVIEW_REQUIRED_BUILD_NOT_AUTHORIZED",
    }
    pointer["pointer_id"] = (
        "hepta-servo-accepted-source-pointer:v1:"
        + TOOL.framed(TOOL.POINTER_DOMAIN, TOOL.canonical(pointer))
    )
    return pointer


OLD.build_pointer = build_pointer
ORIGINAL_LIVE = OLD.AcceptancePointerTests.live_payloads


def live_payloads(self: Any):
    pr, reviews, files, commit = ORIGINAL_LIVE(self)
    pr["number"] = 77
    challenge_path = self.pointer["challenge"]["snapshot_path"]
    if all(item.get("filename") != challenge_path for item in files["files"]):
        files["files"].append({"filename": challenge_path})
    return pr, reviews, files, commit


OLD.AcceptancePointerTests.live_payloads = live_payloads


class AcceptancePointerV2Tests(OLD.AcceptancePointerTests):
    def test_missing_challenge_snapshot_is_rejected(self) -> None:
        pr, reviews, files, commit = self.live_payloads()
        files["files"] = [
            item
            for item in files["files"]
            if item["filename"] != self.pointer["challenge"]["snapshot_path"]
        ]
        with self.assertRaises(TOOL.AcceptanceError):
            TOOL.verify_live_review(
                self.pointer,
                self.candidate,
                self.policy,
                self.challenge,
                pr,
                reviews,
                files,
                commit,
            )

    def test_pull_request_number_mismatch_is_rejected(self) -> None:
        pr, reviews, files, commit = self.live_payloads()
        pr["number"] = 78
        with self.assertRaises(TOOL.AcceptanceError):
            TOOL.verify_live_review(
                self.pointer,
                self.candidate,
                self.policy,
                self.challenge,
                pr,
                reviews,
                files,
                commit,
            )


if __name__ == "__main__":
    unittest.main()
