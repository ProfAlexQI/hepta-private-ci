#!/usr/bin/env python3
"""Regression tests for fail-closed Browser runner evidence v2."""

from __future__ import annotations

import importlib.util
import json
import os
import pathlib
import stat
import sys
import tempfile
import unittest

ROOT = pathlib.Path(__file__).resolve().parents[2]
SCRIPT = ROOT / "scripts/verify-hepta-browser-runner-evidence.py"

SPEC = importlib.util.spec_from_file_location("hepta_runner_evidence", SCRIPT)
if SPEC is None or SPEC.loader is None:
    raise RuntimeError("cannot load runner evidence verifier")
MODULE = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = MODULE
SPEC.loader.exec_module(MODULE)

HEAD = "4" * 40
OTHER_HEAD = "5" * 40
REPOSITORY = MODULE.EXPECTED_REPOSITORY


def workflow_run(
    workflow: dict[str, str],
    run_id: int,
    *,
    status: str = "completed",
    conclusion: str | None = "success",
    head_sha: str = HEAD,
    repository: str = REPOSITORY,
    run_attempt: int = 1,
) -> dict[str, object]:
    return {
        "conclusion": conclusion,
        "head_sha": head_sha,
        "id": run_id,
        "jobs_url": (
            f"https://api.github.com/repos/{repository}/actions/runs/"
            f"{run_id}/jobs"
        ),
        "name": workflow["workflow_name"],
        "repository": {"full_name": repository},
        "run_attempt": run_attempt,
        "status": status,
    }


def runs_payload(
    *,
    status: str = "completed",
    conclusion: str | None = "success",
    head_sha: str = HEAD,
    repository: str = REPOSITORY,
    run_attempt: int = 1,
) -> dict[str, object]:
    return {
        "workflow_runs": [
            workflow_run(
                workflow,
                index,
                status=status,
                conclusion=conclusion,
                head_sha=head_sha,
                repository=repository,
                run_attempt=run_attempt,
            )
            for index, workflow in enumerate(
                MODULE.EXPECTED_WORKFLOWS,
                start=101,
            )
        ]
    }


def step(
    *,
    status: str = "completed",
    conclusion: str | None = "success",
    name: str = "qualification",
    number: int = 1,
) -> dict[str, object]:
    return {
        "conclusion": conclusion,
        "name": name,
        "number": number,
        "status": status,
    }


def job_payload(
    *,
    conclusion: str | None = "success",
    runner_id: int = 42,
    status: str = "completed",
    steps: list[dict[str, object]] | None = None,
) -> dict[int, dict[str, object]]:
    if steps is None:
        steps = [step()]
    return {
        index: {
            "jobs": [
                {
                    "conclusion": conclusion,
                    "id": index * 10,
                    "name": workflow["required_job"],
                    "runner_id": runner_id,
                    "status": status,
                    "steps": steps,
                }
            ],
            "total_count": 1,
        }
        for index, workflow in enumerate(
            MODULE.EXPECTED_WORKFLOWS,
            start=101,
        )
    }


class RunnerEvidenceTests(unittest.TestCase):
    def test_policy_and_recovery_contract_are_canonical_and_all_false(self) -> None:
        policy = MODULE.verify_policy()
        recovery = MODULE.verify_recovery_contract(policy=policy)
        self.assertEqual(policy["required_workflows"], MODULE.EXPECTED_WORKFLOWS)
        self.assertEqual(recovery["required_workflows"], MODULE.EXPECTED_WORKFLOWS)
        self.assertTrue(all(value is False for value in policy["authority"].values()))
        self.assertTrue(
            all(value is False for value in recovery["authority"].values())
        )

    def test_c1_plan_bindings_are_current_and_fail_closed(self) -> None:
        result = MODULE.verify_plan_bindings()
        self.assertEqual(
            result["status"],
            "PASS_C1_RUNNER_PLAN_BINDINGS_V2",
        )
        queue = json.loads(MODULE.QUEUE_PATH.read_text(encoding="utf-8"))
        blocker = queue["environment_blocker"]
        self.assertFalse(blocker["historical_snapshots_are_authority"])
        self.assertIn(MODULE.PASS_DISPOSITION, blocker["closure"])
        self.assertIn(MODULE.PASS_DISPOSITION, queue["next_authorized_step"])
        self.assertTrue(
            all(value is False for value in queue["authority"].values())
        )

    def test_completed_success_is_ci_execution_only_pass(self) -> None:
        evidence = MODULE.classify(runs_payload(), job_payload(), HEAD)
        self.assertEqual(evidence["disposition"], MODULE.PASS_DISPOSITION)
        self.assertEqual(evidence["exit_code"], 0)
        self.assertTrue(evidence["required_checks_passed"])
        self.assertTrue(
            all(item["execution_observed"] for item in evidence["workflows"])
        )
        self.assertTrue(all(item["steps_valid"] for item in evidence["workflows"]))
        self.assertTrue(
            all(value is False for value in evidence["authority"].values())
        )
        MODULE.verify_evidence_digest(evidence)

    def test_success_and_skipped_steps_are_valid_with_one_success(self) -> None:
        evidence = MODULE.classify(
            runs_payload(),
            job_payload(
                steps=[
                    step(name="qualification", number=1),
                    step(
                        name="conditional cleanup",
                        number=2,
                        conclusion="skipped",
                    ),
                ]
            ),
            HEAD,
        )
        self.assertEqual(evidence["disposition"], MODULE.PASS_DISPOSITION)
        self.assertTrue(all(item["steps_valid"] for item in evidence["workflows"]))

    def test_zero_jobs_is_environment_blocker_not_pass(self) -> None:
        runs = runs_payload(status="pending", conclusion=None)
        jobs = {
            run_id: {"jobs": [], "total_count": 0}
            for run_id in range(101, 104)
        }
        evidence = MODULE.classify(runs, jobs, HEAD)
        self.assertEqual(evidence["disposition"], MODULE.BLOCKER_DISPOSITION)
        self.assertEqual(evidence["exit_code"], 3)
        self.assertFalse(evidence["required_checks_passed"])
        self.assertEqual(
            evidence["zero_step_workflows"],
            [item["workflow_name"] for item in MODULE.EXPECTED_WORKFLOWS],
        )

    def test_completed_failure_without_steps_is_still_gate_failure(self) -> None:
        runs = runs_payload(conclusion="failure")
        jobs = {
            run_id: {"jobs": [], "total_count": 0}
            for run_id in range(101, 104)
        }
        evidence = MODULE.classify(runs, jobs, HEAD)
        self.assertEqual(evidence["disposition"], MODULE.FAILURE_DISPOSITION)
        self.assertEqual(evidence["exit_code"], 1)

    def test_completed_success_without_required_job_is_invalid(self) -> None:
        runs = runs_payload()
        jobs = {
            run_id: {"jobs": [], "total_count": 0}
            for run_id in range(101, 104)
        }
        evidence = MODULE.classify(runs, jobs, HEAD)
        self.assertEqual(evidence["disposition"], MODULE.INVALID_DISPOSITION)
        self.assertEqual(evidence["exit_code"], 2)
        self.assertFalse(evidence["required_checks_passed"])

    def test_observed_job_failure_is_gate_failure(self) -> None:
        runs = runs_payload(conclusion="failure")
        jobs = job_payload(conclusion="failure")
        evidence = MODULE.classify(runs, jobs, HEAD)
        self.assertEqual(evidence["disposition"], MODULE.FAILURE_DISPOSITION)
        self.assertEqual(evidence["exit_code"], 1)

    def test_failed_step_cannot_hide_behind_successful_job(self) -> None:
        jobs = job_payload(
            steps=[
                step(),
                step(
                    name="hidden failure",
                    number=2,
                    conclusion="failure",
                ),
            ]
        )
        evidence = MODULE.classify(runs_payload(), jobs, HEAD)
        self.assertEqual(evidence["disposition"], MODULE.FAILURE_DISPOSITION)
        self.assertTrue(
            all(
                item["required_job_failed_steps_count"] == 1
                for item in evidence["workflows"]
            )
        )

    def test_nonterminal_step_cannot_hide_behind_successful_job(self) -> None:
        jobs = job_payload(
            steps=[
                step(
                    status="in_progress",
                    conclusion=None,
                )
            ]
        )
        evidence = MODULE.classify(runs_payload(), jobs, HEAD)
        self.assertEqual(evidence["disposition"], MODULE.INVALID_DISPOSITION)
        self.assertTrue(
            all(
                item["required_job_nonterminal_steps_count"] == 1
                for item in evidence["workflows"]
            )
        )

    def test_all_skipped_steps_are_invalid(self) -> None:
        evidence = MODULE.classify(
            runs_payload(),
            job_payload(
                steps=[
                    step(conclusion="skipped"),
                    step(name="cleanup", number=2, conclusion="skipped"),
                ]
            ),
            HEAD,
        )
        self.assertEqual(evidence["disposition"], MODULE.INVALID_DISPOSITION)
        self.assertTrue(
            all(
                item["required_job_successful_steps_count"] == 0
                for item in evidence["workflows"]
            )
        )

    def test_executed_leaf_failure_is_gate_failure_without_required_job(self) -> None:
        runs = runs_payload(conclusion="failure")
        jobs: dict[int, dict[str, object]] = {}
        for run_id in range(101, 104):
            jobs[run_id] = {
                "jobs": [
                    {
                        "conclusion": "failure",
                        "id": run_id * 10,
                        "name": "failing dependency",
                        "runner_id": 42,
                        "status": "completed",
                        "steps": [
                            step(
                                name="test",
                                conclusion="failure",
                            )
                        ],
                    }
                ],
                "total_count": 1,
            }
        evidence = MODULE.classify(runs, jobs, HEAD)
        self.assertEqual(evidence["disposition"], MODULE.FAILURE_DISPOSITION)
        self.assertTrue(
            all(item["failed_jobs_count"] == 1 for item in evidence["workflows"])
        )

    def test_in_progress_steps_are_not_pass(self) -> None:
        runs = runs_payload(status="in_progress", conclusion=None)
        jobs = job_payload(
            conclusion=None,
            status="in_progress",
            steps=[step(status="in_progress", conclusion=None)],
        )
        evidence = MODULE.classify(runs, jobs, HEAD)
        self.assertEqual(evidence["disposition"], MODULE.EXECUTING_DISPOSITION)
        self.assertEqual(evidence["exit_code"], 3)
        self.assertFalse(evidence["required_checks_passed"])

    def test_wrong_head_runs_do_not_count(self) -> None:
        evidence = MODULE.classify(
            runs_payload(head_sha=OTHER_HEAD),
            {},
            HEAD,
        )
        self.assertEqual(evidence["disposition"], MODULE.BLOCKER_DISPOSITION)
        self.assertEqual(
            evidence["missing_workflows"],
            [item["workflow_name"] for item in MODULE.EXPECTED_WORKFLOWS],
        )

    def test_latest_exact_head_run_is_selected_fail_closed(self) -> None:
        runs = runs_payload()
        older_jobs = job_payload()
        pending_runs = []
        pending_jobs: dict[int, dict[str, object]] = {}
        for offset, workflow in enumerate(MODULE.EXPECTED_WORKFLOWS, start=201):
            pending_runs.append(
                workflow_run(
                    workflow,
                    offset,
                    status="pending",
                    conclusion=None,
                )
            )
            pending_jobs[offset] = {"jobs": [], "total_count": 0}
        runs["workflow_runs"].extend(pending_runs)
        evidence = MODULE.classify(runs, pending_jobs, HEAD)
        self.assertEqual(evidence["disposition"], MODULE.BLOCKER_DISPOSITION)
        for item in evidence["workflows"]:
            self.assertEqual(len(item["matching_run_ids"]), 2)
            self.assertGreaterEqual(item["run_id"], 201)

    def test_jobs_total_count_mismatch_is_rejected(self) -> None:
        jobs = job_payload()
        jobs[101]["total_count"] = 2
        with self.assertRaises(MODULE.EvidenceError):
            MODULE.classify(runs_payload(), jobs, HEAD)

    def test_cross_repository_run_is_rejected(self) -> None:
        with self.assertRaises(MODULE.EvidenceError):
            MODULE.classify(
                runs_payload(repository="OtherOwner/other-repo"),
                job_payload(),
                HEAD,
            )

    def test_cross_repository_jobs_url_is_rejected(self) -> None:
        runs = runs_payload()
        runs["workflow_runs"][0]["jobs_url"] = (
            "https://api.github.com/repos/OtherOwner/other-repo/"
            "actions/runs/101/jobs"
        )
        with self.assertRaises(MODULE.EvidenceError):
            MODULE.classify(runs, job_payload(), HEAD)

    def test_zero_run_attempt_is_rejected(self) -> None:
        with self.assertRaises(MODULE.EvidenceError):
            MODULE.classify(
                runs_payload(run_attempt=0),
                job_payload(),
                HEAD,
            )

    def test_unused_jobs_snapshot_is_rejected(self) -> None:
        jobs = job_payload()
        jobs[999] = {"jobs": [], "total_count": 0}
        with self.assertRaises(MODULE.EvidenceError):
            MODULE.classify(runs_payload(), jobs, HEAD)

    def test_tampered_digest_is_rejected(self) -> None:
        evidence = MODULE.classify(runs_payload(), job_payload(), HEAD)
        evidence["disposition"] = MODULE.FAILURE_DISPOSITION
        with self.assertRaises(MODULE.EvidenceError):
            MODULE.verify_evidence_digest(evidence)

    def test_output_is_create_only_mode_0600_and_bound(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            output = pathlib.Path(temporary) / "evidence.json"
            value = MODULE.classify(runs_payload(), job_payload(), HEAD)
            MODULE.write_output(output, value)
            loaded = json.loads(output.read_text(encoding="utf-8"))
            MODULE.verify_evidence_digest(loaded)
            self.assertEqual(loaded["disposition"], MODULE.PASS_DISPOSITION)
            if os.name == "posix":
                self.assertEqual(
                    stat.S_IMODE(output.stat().st_mode),
                    0o600,
                )
            with self.assertRaises(MODULE.EvidenceError):
                MODULE.write_output(output, value)


if __name__ == "__main__":
    unittest.main()
