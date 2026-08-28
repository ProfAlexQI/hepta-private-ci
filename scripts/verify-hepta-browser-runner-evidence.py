#!/usr/bin/env python3
"""Verify and classify exact-head GitHub Actions runner evidence.

The default invocation verifies only the version-controlled policy.  The
classification mode consumes previously captured GitHub REST API JSON.  It
never performs a network request and never grants Browser, Servo, production,
operator, promotion, or release authority.
"""

from __future__ import annotations

import argparse
import json
import os
import pathlib
import re
import sys
from dataclasses import dataclass
from typing import Any, Iterable

ROOT = pathlib.Path(__file__).resolve().parents[1]
POLICY_PATH = (
    ROOT / "docs/hepta-vnext/browser/RUNNER_QUALIFICATION_POLICY_V1.json"
)
HEX40 = re.compile(r"^[0-9a-f]{40}$")

EXPECTED_WORKFLOWS = [
    {
        "required_job": "CI required",
        "workflow_name": "blocking-ci",
    },
    {
        "required_job": "Hepta Browser next required v9",
        "workflow_name": "hepta-browser next required v9",
    },
    {
        "required_job": "Hepta vNext required",
        "workflow_name": "hepta-vnext qualification",
    },
]

AUTHORITY_KEYS = {
    "artifact_created",
    "build_authorized",
    "build_run",
    "credential_export_allowed",
    "effect_authority",
    "exact_servo_source_accepted",
    "execute_allowed",
    "external_effect",
    "external_network_allowed",
    "machine_authority",
    "merge_authorized",
    "operator_acceptance",
    "production_caller",
    "production_writer",
    "promotion",
    "real_build_inputs_sealed",
    "release_qualified",
    "runtime_authority",
    "servo_runtime_qualified",
    "source_review_candidate_accepted",
    "worker_source_topology_accepted",
}

PASS_DISPOSITION = "PASS_CI_EXECUTION_ONLY"
BLOCKER_DISPOSITION = "ENVIRONMENT_BLOCKER_ZERO_STEPS"
EXECUTING_DISPOSITION = "EXECUTING_NOT_QUALIFIED"
FAILURE_DISPOSITION = "GATE_FAILURE"
INVALID_DISPOSITION = "INVALID_EVIDENCE"


class EvidenceError(RuntimeError):
    """Raised when the policy or supplied API snapshots are malformed."""


@dataclass(frozen=True)
class WorkflowObservation:
    workflow_name: str
    required_job: str
    run_id: int | None
    run_status: str | None
    run_conclusion: str | None
    run_head_sha: str | None
    jobs_count: int
    executable_jobs_count: int
    failed_jobs_count: int
    required_job_present: bool
    required_job_runner_id: int
    required_job_steps_count: int
    required_job_status: str | None
    required_job_conclusion: str | None

    @property
    def exact_head(self) -> bool:
        return self.run_head_sha is not None

    @property
    def execution_observed(self) -> bool:
        return (
            self.required_job_present
            and self.required_job_runner_id > 0
            and self.required_job_steps_count > 0
        )

    @property
    def passed(self) -> bool:
        return (
            self.execution_observed
            and self.run_status == "completed"
            and self.run_conclusion == "success"
            and self.required_job_status == "completed"
            and self.required_job_conclusion == "success"
        )

    @property
    def failed(self) -> bool:
        terminal_failures = {"failure", "cancelled", "timed_out", "action_required"}
        return (
            self.failed_jobs_count > 0
            or (
                self.run_status == "completed"
                and self.run_conclusion in terminal_failures
            )
            or (
                self.execution_observed
                and self.required_job_conclusion in terminal_failures
            )
        )

    def as_json(self) -> dict[str, Any]:
        return {
            "exact_head": self.exact_head,
            "executable_jobs_count": self.executable_jobs_count,
            "execution_observed": self.execution_observed,
            "failed_jobs_count": self.failed_jobs_count,
            "jobs_count": self.jobs_count,
            "passed": self.passed,
            "required_job": self.required_job,
            "required_job_conclusion": self.required_job_conclusion,
            "required_job_present": self.required_job_present,
            "required_job_runner_id": self.required_job_runner_id,
            "required_job_status": self.required_job_status,
            "required_job_steps_count": self.required_job_steps_count,
            "run_conclusion": self.run_conclusion,
            "run_head_sha": self.run_head_sha,
            "run_id": self.run_id,
            "run_status": self.run_status,
            "workflow_name": self.workflow_name,
        }


def fail(message: str) -> None:
    raise EvidenceError(message)


def canonical(value: object) -> bytes:
    return json.dumps(
        value,
        sort_keys=True,
        separators=(",", ":"),
        ensure_ascii=False,
    ).encode("utf-8")


def load_json(path: pathlib.Path) -> Any:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        fail(f"cannot parse JSON from {path}: {error}")


def require_all_false(authority: Any) -> None:
    if not isinstance(authority, dict):
        fail("runner qualification authority must be an object")
    if set(authority) != AUTHORITY_KEYS:
        fail(
            "runner qualification authority keys drifted: "
            f"{sorted(set(authority) ^ AUTHORITY_KEYS)}"
        )
    enabled = sorted(key for key, value in authority.items() if value is not False)
    if enabled:
        fail(f"runner qualification policy attempted to enable authority: {enabled}")


def verify_policy(path: pathlib.Path = POLICY_PATH) -> dict[str, Any]:
    if not path.is_file():
        fail(f"missing runner qualification policy: {path.relative_to(ROOT)}")
    raw = path.read_bytes()
    policy = load_json(path)
    if not isinstance(policy, dict):
        fail("runner qualification policy must be an object")
    if raw != canonical(policy):
        fail("runner qualification policy is not compact canonical JSON")
    if policy.get("schema") != "hepta.browser.runner_qualification_policy.v1":
        fail("runner qualification policy schema drifted")
    if policy.get("schema_version") != 1:
        fail("runner qualification policy version drifted")
    if policy.get("phase") != "DEVELOPMENT":
        fail("runner qualification policy must remain DEVELOPMENT")
    if policy.get("claim_level") != "L1_QUALIFICATION_ONLY":
        fail("runner qualification policy must remain qualification-only")
    if policy.get("scope") != "exact_head_ci_execution_only":
        fail("runner qualification scope drifted")
    if policy.get("supported_observation_mode") != "github_actions_api_snapshot":
        fail("runner qualification observation mode drifted")
    if policy.get("required_workflows") != EXPECTED_WORKFLOWS:
        fail("runner qualification workflow set or ordering drifted")
    expected_contract = {
        "api_snapshots_are_inputs_not_authority": True,
        "completed_success_required": True,
        "evidence_output_create_only": True,
        "evidence_output_fsync": True,
        "evidence_output_mode": "0600",
        "exact_head_required": True,
        "job_list_required": True,
        "minimum_runner_id": 1,
        "run_and_required_job_conclusions": "success",
        "steps_required": True,
        "zero_jobs_disposition": "environment_blocker_not_pass",
        "zero_runner_disposition": "environment_blocker_not_pass",
        "zero_steps_disposition": "environment_blocker_not_pass",
    }
    if policy.get("evidence_contract") != expected_contract:
        fail("runner qualification evidence contract drifted")
    require_all_false(policy.get("authority"))
    return policy


def require_object(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        fail(f"{label} must be an object")
    return value


def require_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        fail(f"{label} must be a list")
    return value


def integer(value: Any, label: str, *, minimum: int = 0) -> int:
    if isinstance(value, bool) or not isinstance(value, int) or value < minimum:
        fail(f"{label} must be an integer >= {minimum}")
    return value


def optional_string(value: Any, label: str) -> str | None:
    if value is None:
        return None
    if not isinstance(value, str):
        fail(f"{label} must be a string or null")
    return value


def select_run(
    runs: Iterable[Any],
    workflow_name: str,
    head_sha: str,
) -> dict[str, Any] | None:
    candidates: list[dict[str, Any]] = []
    for raw_run in runs:
        run = require_object(raw_run, "workflow run")
        if run.get("name") == workflow_name and run.get("head_sha") == head_sha:
            integer(run.get("id"), f"{workflow_name} run id", minimum=1)
            candidates.append(run)
    if not candidates:
        return None
    return max(candidates, key=lambda item: int(item["id"]))


def job_is_executable(job: dict[str, Any]) -> bool:
    runner_id = job.get("runner_id", 0)
    if isinstance(runner_id, bool) or not isinstance(runner_id, int):
        return False
    steps = job.get("steps")
    return runner_id > 0 and isinstance(steps, list) and len(steps) > 0


def observe_workflow(
    spec: dict[str, str],
    runs: list[Any],
    jobs_by_run: dict[int, dict[str, Any]],
    head_sha: str,
) -> WorkflowObservation:
    workflow_name = spec["workflow_name"]
    required_job_name = spec["required_job"]
    run = select_run(runs, workflow_name, head_sha)
    if run is None:
        return WorkflowObservation(
            workflow_name=workflow_name,
            required_job=required_job_name,
            run_id=None,
            run_status=None,
            run_conclusion=None,
            run_head_sha=None,
            jobs_count=0,
            executable_jobs_count=0,
            failed_jobs_count=0,
            required_job_present=False,
            required_job_runner_id=0,
            required_job_steps_count=0,
            required_job_status=None,
            required_job_conclusion=None,
        )

    run_id = integer(run.get("id"), f"{workflow_name} run id", minimum=1)
    run_status = optional_string(run.get("status"), f"{workflow_name} run status")
    run_conclusion = optional_string(
        run.get("conclusion"), f"{workflow_name} run conclusion"
    )
    if run_id not in jobs_by_run:
        fail(f"missing jobs API snapshot for exact-head run {run_id}")
    jobs_payload = jobs_by_run[run_id]
    jobs = require_list(
        require_object(jobs_payload, f"jobs payload for run {run_id}").get("jobs"),
        f"jobs for run {run_id}",
    )
    normalized_jobs: list[dict[str, Any]] = [
        require_object(job, f"job in run {run_id}") for job in jobs
    ]
    executable_jobs = sum(job_is_executable(job) for job in normalized_jobs)
    terminal_failures = {"failure", "cancelled", "timed_out", "action_required"}
    failed_jobs = sum(
        job_is_executable(job) and job.get("conclusion") in terminal_failures
        for job in normalized_jobs
    )
    matching_jobs = [job for job in normalized_jobs if job.get("name") == required_job_name]
    if len(matching_jobs) > 1:
        fail(
            f"run {run_id} contains duplicate required job name "
            f"{required_job_name!r}"
        )
    required_job = matching_jobs[0] if matching_jobs else None
    if required_job is None:
        return WorkflowObservation(
            workflow_name=workflow_name,
            required_job=required_job_name,
            run_id=run_id,
            run_status=run_status,
            run_conclusion=run_conclusion,
            run_head_sha=head_sha,
            jobs_count=len(normalized_jobs),
            executable_jobs_count=executable_jobs,
            failed_jobs_count=failed_jobs,
            required_job_present=False,
            required_job_runner_id=0,
            required_job_steps_count=0,
            required_job_status=None,
            required_job_conclusion=None,
        )

    runner_id_raw = required_job.get("runner_id", 0)
    runner_id = (
        runner_id_raw
        if isinstance(runner_id_raw, int) and not isinstance(runner_id_raw, bool)
        else 0
    )
    steps = required_job.get("steps")
    steps_count = len(steps) if isinstance(steps, list) else 0
    return WorkflowObservation(
        workflow_name=workflow_name,
        required_job=required_job_name,
        run_id=run_id,
        run_status=run_status,
        run_conclusion=run_conclusion,
        run_head_sha=head_sha,
        jobs_count=len(normalized_jobs),
        executable_jobs_count=executable_jobs,
        failed_jobs_count=failed_jobs,
        required_job_present=True,
        required_job_runner_id=runner_id,
        required_job_steps_count=steps_count,
        required_job_status=optional_string(
            required_job.get("status"),
            f"{required_job_name} status",
        ),
        required_job_conclusion=optional_string(
            required_job.get("conclusion"),
            f"{required_job_name} conclusion",
        ),
    )


def classify(
    runs_payload: Any,
    jobs_by_run: dict[int, dict[str, Any]],
    head_sha: str,
    policy: dict[str, Any] | None = None,
) -> dict[str, Any]:
    policy = policy or verify_policy()
    if not HEX40.fullmatch(head_sha):
        fail("head SHA must be exactly 40 lowercase hexadecimal characters")
    runs_object = require_object(runs_payload, "workflow runs payload")
    runs = require_list(runs_object.get("workflow_runs"), "workflow_runs")
    observations = [
        observe_workflow(spec, runs, jobs_by_run, head_sha)
        for spec in policy["required_workflows"]
    ]

    if all(observation.passed for observation in observations):
        disposition = PASS_DISPOSITION
        exit_code = 0
    elif any(observation.failed for observation in observations):
        disposition = FAILURE_DISPOSITION
        exit_code = 1
    elif any(
        observation.run_status == "completed"
        and observation.run_conclusion == "success"
        and not observation.passed
        for observation in observations
    ):
        disposition = INVALID_DISPOSITION
        exit_code = 2
    elif any(
        observation.execution_observed
        or observation.executable_jobs_count > 0
        or observation.run_status == "in_progress"
        for observation in observations
    ):
        disposition = EXECUTING_DISPOSITION
        exit_code = 3
    else:
        disposition = BLOCKER_DISPOSITION
        exit_code = 3

    missing_workflows = [
        observation.workflow_name
        for observation in observations
        if observation.run_id is None
    ]
    zero_step_workflows = [
        observation.workflow_name
        for observation in observations
        if not observation.execution_observed
    ]
    evidence = {
        "authority": policy["authority"],
        "disposition": disposition,
        "exit_code": exit_code,
        "head_sha": head_sha,
        "missing_workflows": missing_workflows,
        "observation_mode": policy["supported_observation_mode"],
        "required_checks_passed": disposition == PASS_DISPOSITION,
        "schema": "hepta.browser.runner_qualification_evidence.v1",
        "schema_version": 1,
        "scope": "ci_execution_only_no_browser_or_servo_authority",
        "workflows": [observation.as_json() for observation in observations],
        "zero_step_workflows": zero_step_workflows,
    }
    require_all_false(evidence["authority"])
    return evidence


def parse_jobs_arguments(values: list[str]) -> dict[int, dict[str, Any]]:
    jobs_by_run: dict[int, dict[str, Any]] = {}
    for value in values:
        run_id_text, separator, path_text = value.partition("=")
        if not separator or not run_id_text.isdigit() or not path_text:
            fail("--jobs-json entries must use RUN_ID=PATH")
        run_id = int(run_id_text)
        if run_id <= 0 or run_id in jobs_by_run:
            fail("--jobs-json run ids must be positive and unique")
        payload = load_json(pathlib.Path(path_text))
        jobs_by_run[run_id] = require_object(
            payload,
            f"jobs payload for run {run_id}",
        )
    return jobs_by_run


def write_output(path: pathlib.Path | None, value: dict[str, Any]) -> None:
    body = canonical(value) + b"\n"
    if path is None:
        sys.stdout.buffer.write(body)
        return
    path.parent.mkdir(parents=True, exist_ok=True)
    try:
        descriptor = os.open(
            path,
            os.O_CREAT | os.O_EXCL | os.O_WRONLY,
            0o600,
        )
    except FileExistsError:
        fail(f"refusing to overwrite evidence output: {path}")
    try:
        with os.fdopen(descriptor, "wb") as output:
            output.write(body)
            output.flush()
            os.fsync(output.fileno())
    except BaseException:
        try:
            path.unlink()
        except OSError:
            pass
        raise
    if os.name == "posix":
        directory_descriptor = os.open(path.parent, os.O_RDONLY)
        try:
            os.fsync(directory_descriptor)
        finally:
            os.close(directory_descriptor)


def parser() -> argparse.ArgumentParser:
    result = argparse.ArgumentParser(description=__doc__)
    result.add_argument("--head-sha")
    result.add_argument("--runs-json", type=pathlib.Path)
    result.add_argument(
        "--jobs-json",
        action="append",
        default=[],
        metavar="RUN_ID=PATH",
    )
    result.add_argument("--output", type=pathlib.Path)
    return result


def main(argv: list[str] | None = None) -> int:
    args = parser().parse_args(argv)
    try:
        policy = verify_policy()
        classify_requested = any(
            (
                args.head_sha is not None,
                args.runs_json is not None,
                bool(args.jobs_json),
                args.output is not None,
            )
        )
        if not classify_requested:
            print(
                json.dumps(
                    {
                        "authority": "all_false",
                        "required_workflows": [
                            item["workflow_name"]
                            for item in policy["required_workflows"]
                        ],
                        "schema": "hepta.browser.runner_qualification_policy.verification.v1",
                        "status": "PASS_FAIL_CLOSED_RUNNER_EVIDENCE_CONTRACT",
                    },
                    sort_keys=True,
                    separators=(",", ":"),
                )
            )
            return 0
        if args.head_sha is None or args.runs_json is None:
            fail("classification requires --head-sha and --runs-json")
        runs_payload = load_json(args.runs_json)
        jobs_by_run = parse_jobs_arguments(args.jobs_json)
        evidence = classify(
            runs_payload,
            jobs_by_run,
            args.head_sha,
            policy,
        )
        write_output(args.output, evidence)
        return int(evidence["exit_code"])
    except EvidenceError as error:
        print(f"HEPTA_RUNNER_EVIDENCE=FAIL: {error}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
