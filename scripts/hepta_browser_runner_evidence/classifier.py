#!/usr/bin/env python3
"""Exact-head GitHub Actions snapshot classifier and evidence binding."""

from __future__ import annotations

from .common import *  # noqa: F401,F403

from .contracts import verify_policy

def repository_full_name(run: dict[str, Any], label: str) -> str:
    repository = require_object(run.get("repository"), f"{label} repository")
    value = repository.get("full_name")
    if not isinstance(value, str) or not value:
        fail(f"{label} repository.full_name must be a non-empty string")
    return value


def expected_jobs_url(repository: str, run_id: int) -> str:
    return f"https://api.github.com/repos/{repository}/actions/runs/{run_id}/jobs"


def validate_jobs_url(value: Any, repository: str, run_id: int) -> str:
    if not isinstance(value, str):
        fail(f"run {run_id} jobs_url must be a string")
    parsed = urlparse(value)
    if parsed.scheme != "https" or parsed.netloc != "api.github.com":
        fail(f"run {run_id} jobs_url must use https://api.github.com")
    if value != expected_jobs_url(repository, run_id):
        fail(f"run {run_id} jobs_url is not repository/run bound")
    return value


def select_run(
    runs: Iterable[Any],
    workflow_name: str,
    head_sha: str,
    repository: str,
) -> tuple[dict[str, Any] | None, tuple[int, ...]]:
    candidates: list[dict[str, Any]] = []
    all_ids: set[int] = set()
    for raw_run in runs:
        run = require_object(raw_run, "workflow run")
        run_id = integer(run.get("id"), "workflow run id", minimum=1)
        if run_id in all_ids:
            fail(f"duplicate workflow run id {run_id}")
        all_ids.add(run_id)
        if run.get("name") == workflow_name and run.get("head_sha") == head_sha:
            if repository_full_name(run, f"run {run_id}") != repository:
                fail(f"run {run_id} belongs to a different repository")
            candidates.append(run)
    if not candidates:
        return None, ()
    candidates.sort(key=lambda item: int(item["id"]))
    return candidates[-1], tuple(int(item["id"]) for item in candidates)


def job_is_executable(job: dict[str, Any]) -> bool:
    runner_id = job.get("runner_id", 0)
    if isinstance(runner_id, bool) or not isinstance(runner_id, int):
        return False
    steps = job.get("steps")
    return runner_id > 0 and isinstance(steps, list) and len(steps) > 0


def normalize_steps(
    value: Any,
    label: str,
) -> tuple[int, int, int, int, int]:
    if value is None:
        return 0, 0, 0, 0, 0
    steps = require_list(value, label)
    successful = 0
    terminal_nonfailure = 0
    nonterminal = 0
    failed = 0
    for index, raw_step in enumerate(steps):
        step = require_object(raw_step, f"{label}[{index}]")
        status = optional_string(step.get("status"), f"{label}[{index}].status")
        conclusion = optional_string(
            step.get("conclusion"),
            f"{label}[{index}].conclusion",
        )
        if status != "completed":
            nonterminal += 1
        if status == "completed" and conclusion in TERMINAL_NONFAILURE_STEPS:
            terminal_nonfailure += 1
        if status == "completed" and conclusion == "success":
            successful += 1
        if conclusion in TERMINAL_FAILURES:
            failed += 1
    return len(steps), successful, terminal_nonfailure, nonterminal, failed


def empty_observation(
    spec: dict[str, str],
) -> WorkflowObservation:
    return WorkflowObservation(
        workflow_name=spec["workflow_name"],
        required_job=spec["required_job"],
        matching_run_ids=(),
        run_id=None,
        run_attempt=None,
        run_status=None,
        run_conclusion=None,
        run_head_sha=None,
        run_jobs_url=None,
        jobs_count=0,
        jobs_total_count=0,
        executable_jobs_count=0,
        failed_jobs_count=0,
        required_job_present=False,
        required_job_runner_id=0,
        required_job_steps_count=0,
        required_job_successful_steps_count=0,
        required_job_terminal_nonfailure_steps_count=0,
        required_job_nonterminal_steps_count=0,
        required_job_failed_steps_count=0,
        required_job_status=None,
        required_job_conclusion=None,
    )


def observe_workflow(
    spec: dict[str, str],
    runs: list[Any],
    jobs_by_run: dict[int, dict[str, Any]],
    head_sha: str,
    repository: str,
) -> WorkflowObservation:
    workflow_name = spec["workflow_name"]
    required_job_name = spec["required_job"]
    run, matching_run_ids = select_run(
        runs,
        workflow_name,
        head_sha,
        repository,
    )
    if run is None:
        return empty_observation(spec)

    run_id = integer(run.get("id"), f"{workflow_name} run id", minimum=1)
    run_attempt = integer(
        run.get("run_attempt"),
        f"{workflow_name} run attempt",
        minimum=1,
    )
    run_status = optional_string(run.get("status"), f"{workflow_name} run status")
    run_conclusion = optional_string(
        run.get("conclusion"),
        f"{workflow_name} run conclusion",
    )
    run_jobs_url = validate_jobs_url(run.get("jobs_url"), repository, run_id)
    if run_id not in jobs_by_run:
        fail(f"missing jobs API snapshot for exact-head run {run_id}")
    jobs_payload = require_object(
        jobs_by_run[run_id],
        f"jobs payload for run {run_id}",
    )
    jobs = require_list(jobs_payload.get("jobs"), f"jobs for run {run_id}")
    total_count = integer(
        jobs_payload.get("total_count"),
        f"jobs total_count for run {run_id}",
    )
    if total_count != len(jobs):
        fail(
            f"jobs total_count mismatch for run {run_id}: "
            f"{total_count} != {len(jobs)}"
        )
    normalized_jobs: list[dict[str, Any]] = [
        require_object(job, f"job in run {run_id}") for job in jobs
    ]
    job_ids: set[int] = set()
    for job in normalized_jobs:
        job_id = integer(job.get("id"), f"job id in run {run_id}", minimum=1)
        if job_id in job_ids:
            fail(f"run {run_id} contains duplicate job id {job_id}")
        job_ids.add(job_id)

    executable_jobs = sum(job_is_executable(job) for job in normalized_jobs)
    failed_jobs = sum(
        optional_string(job.get("conclusion"), f"job conclusion in run {run_id}")
        in TERMINAL_FAILURES
        for job in normalized_jobs
    )
    matching_jobs = [
        job for job in normalized_jobs if job.get("name") == required_job_name
    ]
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
            matching_run_ids=matching_run_ids,
            run_id=run_id,
            run_attempt=run_attempt,
            run_status=run_status,
            run_conclusion=run_conclusion,
            run_head_sha=head_sha,
            run_jobs_url=run_jobs_url,
            jobs_count=len(normalized_jobs),
            jobs_total_count=total_count,
            executable_jobs_count=executable_jobs,
            failed_jobs_count=failed_jobs,
            required_job_present=False,
            required_job_runner_id=0,
            required_job_steps_count=0,
            required_job_successful_steps_count=0,
            required_job_terminal_nonfailure_steps_count=0,
            required_job_nonterminal_steps_count=0,
            required_job_failed_steps_count=0,
            required_job_status=None,
            required_job_conclusion=None,
        )

    runner_id_raw = required_job.get("runner_id", 0)
    runner_id = (
        runner_id_raw
        if isinstance(runner_id_raw, int) and not isinstance(runner_id_raw, bool)
        else 0
    )
    (
        steps_count,
        successful_steps,
        terminal_nonfailure_steps,
        nonterminal_steps,
        failed_steps,
    ) = normalize_steps(
        required_job.get("steps"),
        f"steps for required job {required_job_name!r} in run {run_id}",
    )
    return WorkflowObservation(
        workflow_name=workflow_name,
        required_job=required_job_name,
        matching_run_ids=matching_run_ids,
        run_id=run_id,
        run_attempt=run_attempt,
        run_status=run_status,
        run_conclusion=run_conclusion,
        run_head_sha=head_sha,
        run_jobs_url=run_jobs_url,
        jobs_count=len(normalized_jobs),
        jobs_total_count=total_count,
        executable_jobs_count=executable_jobs,
        failed_jobs_count=failed_jobs,
        required_job_present=True,
        required_job_runner_id=runner_id,
        required_job_steps_count=steps_count,
        required_job_successful_steps_count=successful_steps,
        required_job_terminal_nonfailure_steps_count=terminal_nonfailure_steps,
        required_job_nonterminal_steps_count=nonterminal_steps,
        required_job_failed_steps_count=failed_steps,
        required_job_status=optional_string(
            required_job.get("status"),
            f"{required_job_name} status",
        ),
        required_job_conclusion=optional_string(
            required_job.get("conclusion"),
            f"{required_job_name} conclusion",
        ),
    )


def bind_evidence_digest(value: dict[str, Any]) -> dict[str, Any]:
    if "evidence_sha256" in value:
        fail("unsigned evidence must not already contain evidence_sha256")
    bound = dict(value)
    bound["evidence_sha256"] = hashlib.sha256(canonical(value)).hexdigest()
    return bound


def verify_evidence_digest(value: Any) -> dict[str, Any]:
    evidence = require_object(value, "evidence")
    digest = evidence.get("evidence_sha256")
    if not isinstance(digest, str) or not re.fullmatch(r"[0-9a-f]{64}", digest):
        fail("evidence_sha256 must be 64 lowercase hexadecimal characters")
    unsigned = dict(evidence)
    del unsigned["evidence_sha256"]
    expected = hashlib.sha256(canonical(unsigned)).hexdigest()
    if digest != expected:
        fail("evidence_sha256 does not match canonical unsigned evidence")
    require_all_false(evidence.get("authority"))
    return evidence


def classify(
    runs_payload: Any,
    jobs_by_run: dict[int, dict[str, Any]],
    head_sha: str,
    policy: dict[str, Any] | None = None,
) -> dict[str, Any]:
    policy = policy or verify_policy()
    if not HEX40.fullmatch(head_sha):
        fail("head SHA must be exactly 40 lowercase hexadecimal characters")
    repository = policy["evidence_contract"]["expected_repository_full_name"]
    runs_object = require_object(runs_payload, "workflow runs payload")
    runs = require_list(runs_object.get("workflow_runs"), "workflow_runs")
    observations = [
        observe_workflow(spec, runs, jobs_by_run, head_sha, repository)
        for spec in policy["required_workflows"]
    ]
    selected_run_ids = {
        observation.run_id
        for observation in observations
        if observation.run_id is not None
    }
    extra_job_snapshots = set(jobs_by_run) - selected_run_ids
    if extra_job_snapshots:
        fail(
            "unused jobs API snapshots are not allowed: "
            f"{sorted(extra_job_snapshots)}"
        )

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
    unsigned_evidence = {
        "authority": policy["authority"],
        "disposition": disposition,
        "exit_code": exit_code,
        "head_sha": head_sha,
        "missing_workflows": missing_workflows,
        "observation_mode": policy["supported_observation_mode"],
        "policy_schema": policy["schema"],
        "repository_full_name": repository,
        "required_checks_passed": disposition == PASS_DISPOSITION,
        "schema": "hepta.browser.runner_qualification_evidence.v2",
        "schema_version": 2,
        "scope": "ci_execution_only_no_browser_or_servo_authority",
        "workflows": [observation.as_json() for observation in observations],
        "zero_step_workflows": zero_step_workflows,
    }
    require_all_false(unsigned_evidence["authority"])
    return bind_evidence_digest(unsigned_evidence)

