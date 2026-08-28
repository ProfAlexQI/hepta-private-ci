#!/usr/bin/env python3
from __future__ import annotations

import importlib.util
import json
import os
import pathlib
import tempfile
import unittest
from typing import Any

SCRIPT = pathlib.Path(__file__).resolve().parents[1] / "hepta-servo-exact-source-review-candidate-v2.py"


def load_module():
    spec = importlib.util.spec_from_file_location("source_review_candidate_v2", SCRIPT)
    if spec is None or spec.loader is None:
        raise RuntimeError("cannot load source review candidate module")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


MODULE = load_module()


def canonical(value: object) -> bytes:
    return json.dumps(value, sort_keys=True, separators=(",", ":")).encode()


class CandidateFixture:
    def __init__(self, root: pathlib.Path) -> None:
        self.root = root.resolve()
        self.source = self.root / "source"
        self.source.mkdir(mode=0o700)
        self.run = {
            "id": 123,
            "name": MODULE.WORKFLOW_NAME,
            "path": MODULE.WORKFLOW_PATH,
            "event": "workflow_dispatch",
            "status": "completed",
            "conclusion": "success",
            "head_branch": "codex/hepta-vnext-plan-browser-c0-c3-20260827",
            "head_sha": "1" * 40,
            "run_attempt": 1,
            "html_url": "https://github.com/ProfAlexQI/hepta-private-ci/actions/runs/123",
            "head_commit": {"id": "1" * 40, "tree_id": "2" * 40},
        }
        self.jobs = {
            "total_count": 1,
            "jobs": [
                {
                    "id": 456,
                    "run_id": 123,
                    "name": MODULE.JOB_NAME,
                    "status": "completed",
                    "conclusion": "success",
                    "runner_id": 789,
                    "runner_name": "GitHub Actions 1",
                    "steps": [
                        {"name": name, "status": "completed", "conclusion": "success"}
                        for name in MODULE.REQUIRED_STEPS
                    ],
                }
            ],
        }
        self.artifacts = {
            "total_count": 2,
            "artifacts": [
                {
                    "id": index + 1,
                    "name": name,
                    "expired": False,
                    "size_in_bytes": 1024 + index,
                }
                for index, name in enumerate(MODULE.EXPECTED_ARTIFACTS)
            ],
        }
        source_authority = {
            "runtime_authority": False,
            "effect_authority": False,
            "production_caller": False,
            "production_writer": False,
            "runtime_external_network": False,
            "raw_cookie_export": False,
            "credential_export": False,
            "operator_acceptance": False,
            "promotion": False,
            "release_qualified": False,
        }
        self.source_json: dict[str, dict[str, Any]] = {
            "independent-source-bundle.receipt.json": {
                "source": {
                    "repository": "servo/servo",
                    "commit": MODULE.SERVO_COMMIT,
                    "tree": MODULE.SERVO_TREE,
                },
                "qualification": {
                    "servo_built": False,
                    "servo_runtime_qualified": False,
                    "operator_accepted": False,
                    "release_qualified": False,
                },
                "authority": source_authority,
            },
            "source-bundle.verification.json": {
                "source": {
                    "commit": MODULE.SERVO_COMMIT,
                    "tree": MODULE.SERVO_TREE,
                    "recomputed_tree": MODULE.SERVO_TREE,
                },
                "verification": {
                    "git_tree_recomputed": True,
                    "pinned_tree_matched": True,
                    "license_matched": True,
                    "servo_built": False,
                    "servo_runtime_qualified": False,
                    "release_qualified": False,
                },
                "authority": source_authority,
            },
            "fetch-a.receipt.json": {
                "acquisition": {
                    "acquisition_nonce_sha256": "a" * 64,
                    "standalone_object_store": True,
                    "alternate_object_database": False,
                },
                "authority": source_authority,
            },
            "fetch-b.receipt.json": {
                "acquisition": {
                    "acquisition_nonce_sha256": "b" * 64,
                    "standalone_object_store": True,
                    "alternate_object_database": False,
                },
                "authority": source_authority,
            },
            "license-packet.json": {
                "license": "MPL-2.0",
                "authority": source_authority,
            },
        }
        self.write_all()

    def write_json(self, path: pathlib.Path, value: object, *, canonical_json: bool = False) -> None:
        raw = canonical(value) if canonical_json else json.dumps(value, indent=2).encode()
        path.write_bytes(raw)
        os.chmod(path, 0o600)

    def write_all(self) -> None:
        self.write_json(self.root / "workflow-run.json", self.run)
        self.write_json(self.root / "workflow-jobs.json", self.jobs)
        self.write_json(self.root / "workflow-artifacts.json", self.artifacts)
        for name, value in self.source_json.items():
            self.write_json(self.source / name, value, canonical_json=True)
        archive = self.source / "servo-source-a.tar.gz"
        archive.write_bytes(b"fixture compressed archive")
        os.chmod(archive, 0o600)
        self.write_checksums()

    def write_checksums(self, *, names=None, absolute=False, reverse=False) -> None:
        names = list(names or MODULE.REQUIRED_SOURCE_FILES)
        if reverse:
            names.reverse()
        lines = []
        for name in names:
            path = self.source / name
            digest, _ = MODULE.sha256_file(path)
            rendered = os.fspath(path) if absolute else name
            lines.append(f"{digest}  {rendered}")
        target = self.source / "SHA256SUMS"
        target.write_text("\n".join(lines) + "\n", encoding="ascii")
        os.chmod(target, 0o600)

    def rewrite_source(self, name: str) -> None:
        self.write_json(self.source / name, self.source_json[name], canonical_json=True)
        self.write_checksums()

    def compile(self, *, output=None):
        return MODULE.compile_candidate(
            self.root,
            self.run["head_branch"],
            "2026-08-28T00:00:00Z",
            pathlib.Path(__file__).resolve(),
            output,
            skip_subprocess_for_test=True,
        )


class CandidateTests(unittest.TestCase):
    def fixture(self):
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        return CandidateFixture(pathlib.Path(temporary.name))

    def test_valid_candidate_is_review_pending_and_self_bound(self):
        fixture = self.fixture()
        candidate = fixture.compile()
        MODULE.verify_candidate(candidate)
        self.assertFalse(candidate["review"]["candidate_accepted"])
        self.assertFalse(candidate["claims"]["build_authorized"])

    def test_output_is_create_only_and_private(self):
        fixture = self.fixture()
        output = fixture.root / "candidate.json"
        fixture.compile(output=output)
        self.assertEqual(output.stat().st_mode & 0o777, 0o600)
        with self.assertRaises(MODULE.CandidateError):
            fixture.compile(output=output)

    def test_zero_steps_are_rejected(self):
        fixture = self.fixture()
        fixture.jobs["jobs"][0]["steps"] = []
        fixture.write_json(fixture.root / "workflow-jobs.json", fixture.jobs)
        with self.assertRaisesRegex(MODULE.CandidateError, "no executable steps"):
            fixture.compile()

    def test_runner_id_zero_is_rejected(self):
        fixture = self.fixture()
        fixture.jobs["jobs"][0]["runner_id"] = 0
        fixture.write_json(fixture.root / "workflow-jobs.json", fixture.jobs)
        with self.assertRaisesRegex(MODULE.CandidateError, "positive integer"):
            fixture.compile()

    def test_wrong_head_is_rejected(self):
        fixture = self.fixture()
        fixture.run["head_commit"]["id"] = "3" * 40
        fixture.write_json(fixture.root / "workflow-run.json", fixture.run)
        with self.assertRaisesRegex(MODULE.CandidateError, "head_commit ID"):
            fixture.compile()

    def test_wrong_workflow_path_is_rejected(self):
        fixture = self.fixture()
        fixture.run["path"] = ".github/workflows/other.yml"
        fixture.write_json(fixture.root / "workflow-run.json", fixture.run)
        with self.assertRaisesRegex(MODULE.CandidateError, "name/path"):
            fixture.compile()

    def test_non_dispatch_event_is_rejected(self):
        fixture = self.fixture()
        fixture.run["event"] = "pull_request"
        fixture.write_json(fixture.root / "workflow-run.json", fixture.run)
        with self.assertRaisesRegex(MODULE.CandidateError, "workflow_dispatch"):
            fixture.compile()

    def test_expired_artifact_is_rejected(self):
        fixture = self.fixture()
        fixture.artifacts["artifacts"][0]["expired"] = True
        fixture.write_json(fixture.root / "workflow-artifacts.json", fixture.artifacts)
        with self.assertRaisesRegex(MODULE.CandidateError, "must remain false"):
            fixture.compile()

    def test_missing_archive_artifact_is_rejected(self):
        fixture = self.fixture()
        fixture.artifacts["artifacts"].pop()
        fixture.write_json(fixture.root / "workflow-artifacts.json", fixture.artifacts)
        with self.assertRaisesRegex(MODULE.CandidateError, "artifact names"):
            fixture.compile()

    def test_absolute_checksum_paths_are_rejected(self):
        fixture = self.fixture()
        fixture.write_checksums(absolute=True)
        with self.assertRaisesRegex(MODULE.CandidateError, "non-portable"):
            fixture.compile()

    def test_unsorted_checksum_names_are_rejected(self):
        fixture = self.fixture()
        fixture.write_checksums(reverse=True)
        with self.assertRaisesRegex(MODULE.CandidateError, "not bytewise sorted"):
            fixture.compile()

    def test_checksum_digest_drift_is_rejected(self):
        fixture = self.fixture()
        with (fixture.source / "servo-source-a.tar.gz").open("ab") as handle:
            handle.write(b"drift")
        with self.assertRaisesRegex(MODULE.CandidateError, "digest differs"):
            fixture.compile()

    def test_noncanonical_source_json_is_rejected(self):
        fixture = self.fixture()
        fixture.write_json(
            fixture.source / "license-packet.json",
            fixture.source_json["license-packet.json"],
            canonical_json=False,
        )
        fixture.write_checksums()
        with self.assertRaisesRegex(MODULE.CandidateError, "not compact canonical"):
            fixture.compile()

    def test_recomputed_tree_drift_is_rejected(self):
        fixture = self.fixture()
        fixture.source_json["source-bundle.verification.json"]["source"]["recomputed_tree"] = "4" * 40
        fixture.rewrite_source("source-bundle.verification.json")
        with self.assertRaisesRegex(MODULE.CandidateError, "recompute"):
            fixture.compile()

    def test_open_authority_is_rejected(self):
        fixture = self.fixture()
        fixture.source_json["independent-source-bundle.receipt.json"]["authority"]["promotion"] = True
        fixture.rewrite_source("independent-source-bundle.receipt.json")
        with self.assertRaisesRegex(MODULE.CandidateError, "enable authority"):
            fixture.compile()

    def test_duplicate_json_key_is_rejected(self):
        fixture = self.fixture()
        path = fixture.root / "workflow-run.json"
        path.write_text('{"id":123,"id":124}', encoding="utf-8")
        os.chmod(path, 0o600)
        with self.assertRaisesRegex(MODULE.CandidateError, "duplicate JSON key"):
            fixture.compile()

    def test_tampered_candidate_id_is_rejected(self):
        fixture = self.fixture()
        candidate = fixture.compile()
        candidate["decision"] = "SOURCE_ACCEPTED"
        with self.assertRaisesRegex(MODULE.CandidateError, "does not bind"):
            MODULE.verify_candidate(candidate)

    def test_hardlinked_source_file_is_rejected(self):
        fixture = self.fixture()
        path = fixture.source / "servo-source-a.tar.gz"
        os.link(path, fixture.source / "second-link")
        with self.assertRaisesRegex(MODULE.CandidateError, "exactly one hard link"):
            fixture.compile()


if __name__ == "__main__":
    unittest.main()
