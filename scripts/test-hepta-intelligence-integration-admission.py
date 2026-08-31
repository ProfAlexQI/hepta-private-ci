#!/usr/bin/env python3
from __future__ import annotations

import copy
import importlib.util
from pathlib import Path
import unittest
from unittest.mock import patch

SCRIPT = Path(__file__).with_name(
    "verify-hepta-intelligence-integration-admission.py"
)
SPEC = importlib.util.spec_from_file_location(
    "verify_hepta_intelligence_integration_admission",
    SCRIPT,
)
if SPEC is None or SPEC.loader is None:
    raise RuntimeError(f"cannot load admission verifier: {SCRIPT}")
subject = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(subject)

HEAD = "a" * 40
HEAD_TREE = "b" * 40
A0 = "c" * 40
A0_TREE = "d" * 40
Q0 = "e" * 40
Q0_TREE = "f" * 40
A0_PARENT = "1" * 40
Q0_PARENT = "2" * 40
BASE_BRANCH = "codex/hepta-intelligence-a0-authority-gap-closure-20260829"
HEAD_BRANCH = "integration/hepta-intelligence-a0-q0-fixture"
ALLOWLIST = [
    "plans/hepta-intelligence/a.json",
    "plans/hepta-intelligence/b.json",
]
Q0_DELTA = [
    ".github/workflows/hepta-intelligence-integration-admission.yml",
    "scripts/verify-hepta-intelligence-integration-admission.py",
]


def fixture_manifest() -> dict[str, object]:
    return {
        "branch": BASE_BRANCH,
        "classification": "SOURCE_ONLY_GOVERNANCE_CANDIDATE",
        "candidate_provenance_policy": {
            "commit_must_have_exactly_one_parent": True,
            "candidate_workflow_may_write_source": False,
        },
        "expected_parent": A0_PARENT,
        "allowed_changed_paths": list(ALLOWLIST),
        "expected_changed_path_count": len(ALLOWLIST),
        "authority": {
            "runtime_wired": False,
            "production_authority": False,
            "release_authority": False,
        },
    }


def fixture_api() -> dict[str, object]:
    return {
        "sha": HEAD,
        "parents": [{"sha": A0}, {"sha": Q0}],
        "commit": {
            "tree": {"sha": HEAD_TREE},
            "verification": {
                "verified": True,
                "reason": "valid",
                "signature": "fixture-signature",
                "payload": "fixture-payload",
                "verified_at": "2026-08-30T16:18:46Z",
            },
        },
    }


class AdmissionIdentityQualificationTest(unittest.TestCase):
    def evaluate(
        self,
        *,
        head_parents: list[str] | None = None,
        q0_parents: list[str] | None = None,
        manifest: dict[str, object] | None = None,
        api: dict[str, object] | None = None,
        q0_overlay: list[str] | None = None,
        q0_delta: list[str] | None = None,
        dirty: bool = False,
        mutate_a0_entry: bool = False,
    ) -> dict[str, object]:
        head_parents = [A0, Q0] if head_parents is None else head_parents
        q0_parents = [Q0_PARENT] if q0_parents is None else q0_parents
        manifest = fixture_manifest() if manifest is None else manifest
        api = fixture_api() if api is None else api
        q0_overlay = list(ALLOWLIST) if q0_overlay is None else q0_overlay
        q0_delta = list(Q0_DELTA) if q0_delta is None else q0_delta

        def fake_run_git(root: Path, *args: str) -> str:
            del root
            if args == ("rev-parse", "HEAD"):
                return f"{HEAD}\n"
            if args == ("status", "--porcelain", "--untracked-files=no"):
                return " M tracked.txt\n" if dirty else ""
            raise AssertionError(f"unexpected run_git call: {args!r}")

        def fake_tree_sha(root: Path, commit: str) -> str:
            del root
            return {
                HEAD: HEAD_TREE,
                A0: A0_TREE,
                Q0: Q0_TREE,
            }[commit]

        def fake_parents(root: Path, commit: str) -> list[str]:
            del root
            return {
                HEAD: list(head_parents),
                A0: [A0_PARENT],
                Q0: list(q0_parents),
            }[commit]

        def fake_changed_paths(root: Path, base: str, head: str) -> list[str]:
            del root
            mapping = {
                (A0_PARENT, A0): list(ALLOWLIST),
                (Q0, HEAD): list(q0_overlay),
                (A0, HEAD): list(q0_delta),
            }
            return mapping[(base, head)]

        def fake_tree_entry(
            root: Path,
            commit: str,
            path: str,
        ) -> tuple[str, str, str]:
            del root
            suffix = ALLOWLIST.index(path) + 3
            blob = (f"{suffix:x}" * 40)[:40]
            if mutate_a0_entry and commit == HEAD and path == ALLOWLIST[0]:
                blob = "9" * 40
            return ("100644", "blob", blob)

        with (
            patch.object(subject, "run_git", side_effect=fake_run_git),
            patch.object(subject, "tree_sha", side_effect=fake_tree_sha),
            patch.object(subject, "commit_parents", side_effect=fake_parents),
            patch.object(subject, "read_json_at", return_value=manifest),
            patch.object(subject, "changed_paths", side_effect=fake_changed_paths),
            patch.object(subject, "tree_entry", side_effect=fake_tree_entry),
            patch.object(subject, "fetch_commit_metadata", return_value=api),
        ):
            return subject.verify(
                root=Path("/fixture/repository"),
                repository="ProfHepta/hepta-private-ci",
                expected_head=HEAD,
                expected_base=A0,
                expected_base_branch=BASE_BRANCH,
                expected_head_branch=HEAD_BRANCH,
                token="fixture-token",
            )

    def test_valid_identity_is_deterministic_and_binds_both_parents(self) -> None:
        first = self.evaluate()
        second = self.evaluate()
        self.assertEqual(first, second)
        self.assertEqual(first["first_parent_a0"], A0)
        self.assertEqual(first["second_parent_q0"], Q0)
        self.assertEqual(
            first["status"],
            "PASS_HEPTA_INTELLIGENCE_INTEGRATION_ADMISSION_IDENTITY_ONLY",
        )

    def test_three_parent_head_fails_closed(self) -> None:
        with self.assertRaisesRegex(
            subject.AdmissionError,
            "exactly two parents",
        ):
            self.evaluate(head_parents=[A0, Q0, "3" * 40])

    def test_swapped_a0_parent_fails_closed(self) -> None:
        with self.assertRaisesRegex(subject.AdmissionError, "A0 must be the first"):
            self.evaluate(head_parents=[Q0, A0])

    def test_selected_q0_merge_commit_fails_closed(self) -> None:
        with self.assertRaisesRegex(subject.AdmissionError, "must not be a merge"):
            self.evaluate(q0_parents=[Q0_PARENT, "3" * 40])

    def test_positive_a0_authority_fails_closed(self) -> None:
        manifest = fixture_manifest()
        authority = copy.deepcopy(manifest["authority"])
        assert isinstance(authority, dict)
        authority["runtime_wired"] = True
        manifest["authority"] = authority
        with self.assertRaisesRegex(subject.AdmissionError, "authority escaped"):
            self.evaluate(manifest=manifest)

    def test_changed_a0_blob_fails_closed(self) -> None:
        with self.assertRaisesRegex(subject.AdmissionError, "changed canonical A0"):
            self.evaluate(mutate_a0_entry=True)

    def test_noncanonical_q0_overlay_fails_closed(self) -> None:
        with self.assertRaisesRegex(subject.AdmissionError, "exact canonical-A0 overlay"):
            self.evaluate(q0_overlay=[*ALLOWLIST, "unexpected.txt"])

    def test_a0_allowlist_leak_fails_closed(self) -> None:
        with self.assertRaisesRegex(subject.AdmissionError, "A0 paths changed"):
            self.evaluate(q0_delta=[*Q0_DELTA, ALLOWLIST[0]])

    def test_api_parent_order_drift_fails_closed(self) -> None:
        api = fixture_api()
        api["parents"] = [{"sha": Q0}, {"sha": A0}]
        with self.assertRaisesRegex(subject.AdmissionError, "parent order differs"):
            self.evaluate(api=api)

    def test_unverified_api_signature_fails_closed(self) -> None:
        api = fixture_api()
        commit = copy.deepcopy(api["commit"])
        assert isinstance(commit, dict)
        verification = copy.deepcopy(commit["verification"])
        assert isinstance(verification, dict)
        verification["verified"] = False
        commit["verification"] = verification
        api["commit"] = commit
        with self.assertRaisesRegex(subject.AdmissionError, "not verified"):
            self.evaluate(api=api)

    def test_dirty_tracked_worktree_fails_closed(self) -> None:
        with self.assertRaisesRegex(subject.AdmissionError, "worktree is not clean"):
            self.evaluate(dirty=True)

    def test_duplicate_a0_manifest_key_fails_closed(self) -> None:
        with self.assertRaisesRegex(subject.AdmissionError, "duplicate JSON key"):
            subject.reject_duplicate_keys([("authority", {}), ("authority", {})])


if __name__ == "__main__":
    unittest.main(verbosity=2)
