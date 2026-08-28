#!/usr/bin/env python3
"""Regression tests for the fail-closed Browser runner evidence classifier."""

from __future__ import annotations

import importlib.util
import json
import pathlib
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


def run_payload(
    *,
    status: str = "completed",
    conclusion: str | None = "success",
    head_sha: str = HEAD,
) -> dict[str, object]:
    return {
        "workflow_runs": [
            {
                "conclusion": conclusion,
                "head_sha": head_sha,
                "id": index,
                "name": workflow["workflow_name"],
                "status": status,
            }
            for index, workflow in enumerate(MODULE.EXPECTED_WORKFLOWS, start=101)
        ]
    }


def job_payload(
    *,
    conclusion: str | None = "success",
    runner_id: int = 42,
    status: str = "completed",
    steps: list[dict[str, object]] | None = None,
) -> dict[int, dict[str, object]]:
    if steps is None:
        steps = [
            {
                "conclusion": conclusion,
                "name": "qualification",
                "number": 1,
                "status": status,
            }
        ]
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
            ]
        }
        for index, workflow in enumerate(MODULE.EXPECTED_WORKFLOWS, start=101)
    }


class RunnerEvidenceTests(unittest.TestCase):
    def test_policy_is_canonical_and_all_false(self) -> None:
        policy = MODULE.verify_policy()
        self.assertEqual(
            policy["required_workflows"],
            MODULE.EXPECTED_WORKFLOWS,
        )
        self.assertTrue(
            all(value is False for value in policy["authority"].values())
        )

    def test_completed_success_is_ci_execution_only_pass(self) -> None:
        evidence = MODULE.classify(run_payload(), job_payload(), HEAD)
        self.assertEqual(evidence["disposition"], MODULE.PASS_DISPOSITION)
        self.assertEqual(evidence["exit_code"], 0)
        self.assertTrue(evidence["required_checks_passed"])
        self.assertTrue(
            all(item["execution_observed"] for item in evidence["workflows"])
        )
        self.assertTrue(
            all(value is False for value in evidence["authority"].values())
        )

    def test_zero_jobs_is_environment_blocker_not_pass(self) -> None:
        runs = run_payload(status="pending", conclusion=None)
        jobs = {run_id: {"jobs": []} for run_id in range(101, 104)}
        evidence = MODULE.classify(runs, jobs, HEAD)
        self.assertEqual(evidence["disposition"], MODULE.BLOCKER_DISPOSITION)
        self.assertEqual(evidence["exit_code"], 3)
        self.assertFalse(evidence["required_checks_passed"])
        self.assertEqual(
            evidence["zero_step_workflows"],
            [item["workflow_name"] for item in MODULE.EXPECTED_WORKFLOWS],
        )

    def test_completed_failure_without_steps_is_still_a_gate_failure(self) -> None:
        runs = run_payload(conclusion="failure")
        jobs = {run_id: {"jobs": []} for run_id in range(101, 104)}
        evidence = MODULE.classify(runs, jobs, HEAD)
        self.assertEqual(evidence["disposition"], MODULE.FAILURE_DISPOSITION)
        self.assertEqual(evidence["exit_code"], 1)

    def test_completed_success_without_required_job_is_invalid_evidence(self) -> None:
        runs = run_payload()
        jobs = {run_id: {"jobs": []} for run_id in range(101, 104)}
        evidence = MODULE.classify(runs, jobs, HEAD)
        self.assertEqual(evidence["disposition"], MODULE.INVALID_DISPOSITION)
        self.assertEqual(evidence["exit_code"], 2)
        self.assertFalse(evidence["required_checks_passed"])

    def test_observed_failure_is_gate_failure(self) -> None:
        runs = run_payload(conclusion="failure")
        jobs = job_payload(conclusion="failure")
        evidence = MODULE.classify(runs, jobs, HEAD)
        self.assertEqual(evidence["disposition"], MODULE.FAILURE_DISPOSITION)
        self.assertEqual(evidence["exit_code"], 1)
        self.assertFalse(evidence["required_checks_passed"])

    def test_executed_leaf_failure_is_gate_failure_even_if_required_job_is_absent(self) -> None:
        runs = run_payload(conclusion="failure")
        jobs = {}
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
                            {
                                "conclusion": "failure",
                                "name": "test",
                                "number": 1,
                                "status": "completed",
                            }
                        ],
                    }
                ]
            }
        evidence = MODULE.classify(runs, jobs, HEAD)
        self.assertEqual(evidence["disposition"], MODULE.FAILURE_DISPOSITION)
        self.assertTrue(
            all(item["failed_jobs_count"] == 1 for item in evidence["workflows"])
        )

    def test_in_progress_steps_are_not_a_pass(self) -> None:
        runs = run_payload(status="in_progress", conclusion=None)
        jobs = job_payload(conclusion=None, status="in_progress")
        evidence = MODULE.classify(runs, jobs, HEAD)
        self.assertEqual(evidence["disposition"], MODULE.EXECUTING_DISPOSITION)
        self.assertEqual(evidence["exit_code"], 3)
        self.assertFalse(evidence["required_checks_passed"])

    def test_wrong_head_runs_do_not_count(self) -> None:
        evidence = MODULE.classify(
            run_payload(head_sha="5" * 40),
            {},
            HEAD,
        )
        self.assertEqual(evidence["disposition"], MODULE.BLOCKER_DISPOSITION)
        self.assertEqual(
            evidence["missing_workflows"],
            [item["workflow_name"] for item in MODULE.EXPECTED_WORKFLOWS],
        )

    def test_output_is_create_only(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            output = pathlib.Path(temporary) / "evidence.json"
            MODULE.write_output(
                output,
                MODULE.classify(run_payload(), job_payload(), HEAD),
            )
            value = json.loads(output.read_text(encoding="utf-8"))
            self.assertEqual(value["disposition"], MODULE.PASS_DISPOSITION)
            with self.assertRaises(MODULE.EvidenceError):
                MODULE.write_output(output, value)


if __name__ == "__main__":
    unittest.main()
