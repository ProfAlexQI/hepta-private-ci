#!/usr/bin/env python3
"""Shared constants, models, and fail-closed helpers for runner evidence."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import pathlib
import re
import sys
from dataclasses import dataclass
from typing import Any, Iterable
from urllib.parse import urlparse

ROOT = pathlib.Path(__file__).resolve().parents[2]
POLICY_PATH = (
    ROOT / "docs/hepta-vnext/browser/RUNNER_QUALIFICATION_POLICY_V2.json"
)
RECOVERY_PATH = (
    ROOT / "docs/hepta-vnext/browser/RUNNER_RECOVERY_CONTRACT_V1.json"
)
CURRENT_PATH = ROOT / "docs/hepta-vnext/browser/C1_CURRENT_V7.json"
QUEUE_PATH = ROOT / "docs/hepta-vnext/browser/NEXT_WORK_QUEUE_C1_V2.json"
DELTA_PATH = (
    ROOT / "docs/hepta-vnext/browser/C1_EXECUTION_DELTA_2026-08-28_V2.json"
)
RUNBOOK_PATH = ROOT / "docs/hepta-vnext/browser/RUNNER_RECOVERY_RUNBOOK.md"
BROWSER_REQUIRED_WORKFLOW_PATH = (
    ROOT / ".github/workflows/hepta-browser-next-required-v9.yml"
)
VNEXT_REQUIRED_WORKFLOW_PATH = (
    ROOT / ".github/workflows/hepta-vnext-qualification.yml"
)
HEX40 = re.compile(r"^[0-9a-f]{40}$")

EXPECTED_REPOSITORY = "ProfAlexQI/hepta-private-ci"
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

CURRENT_AUTHORITY_KEYS = {
    "credential_export",
    "effect_authority",
    "operator_acceptance",
    "production_caller",
    "production_writer",
    "promotion",
    "raw_cookie_export",
    "release_qualified",
    "runtime_authority",
    "runtime_external_network",
}
QUEUE_AUTHORITY_KEYS = {
    "credential_export_allowed",
    "effect_authority",
    "external_effect",
    "external_network_allowed",
    "operator_acceptance",
    "production_caller",
    "production_writer",
    "promotion",
    "release_qualified",
    "runtime_authority",
}

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

TERMINAL_FAILURES = {
    "action_required",
    "cancelled",
    "failure",
    "startup_failure",
    "stale",
    "timed_out",
}
TERMINAL_NONFAILURE_STEPS = {"success", "skipped"}


class EvidenceError(RuntimeError):
    """Raised when a contract or supplied API snapshot is malformed."""


@dataclass(frozen=True)
class WorkflowObservation:
    workflow_name: str
    required_job: str
    matching_run_ids: tuple[int, ...]
    run_id: int | None
    run_attempt: int | None
    run_status: str | None
    run_conclusion: str | None
    run_head_sha: str | None
    run_jobs_url: str | None
    jobs_count: int
    jobs_total_count: int
    executable_jobs_count: int
    failed_jobs_count: int
    required_job_present: bool
    required_job_runner_id: int
    required_job_steps_count: int
    required_job_successful_steps_count: int
    required_job_terminal_nonfailure_steps_count: int
    required_job_nonterminal_steps_count: int
    required_job_failed_steps_count: int
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
    def steps_valid(self) -> bool:
        return (
            self.required_job_steps_count > 0
            and self.required_job_successful_steps_count > 0
            and self.required_job_terminal_nonfailure_steps_count
            == self.required_job_steps_count
            and self.required_job_nonterminal_steps_count == 0
            and self.required_job_failed_steps_count == 0
        )

    @property
    def passed(self) -> bool:
        return (
            self.execution_observed
            and self.steps_valid
            and self.jobs_count == self.jobs_total_count
            and self.run_status == "completed"
            and self.run_conclusion == "success"
            and self.required_job_status == "completed"
            and self.required_job_conclusion == "success"
        )

    @property
    def failed(self) -> bool:
        return (
            self.failed_jobs_count > 0
            or self.required_job_failed_steps_count > 0
            or (
                self.run_status == "completed"
                and self.run_conclusion in TERMINAL_FAILURES
            )
            or (
                self.required_job_conclusion in TERMINAL_FAILURES
            )
        )

    def as_json(self) -> dict[str, Any]:
        return {
            "exact_head": self.exact_head,
            "executable_jobs_count": self.executable_jobs_count,
            "execution_observed": self.execution_observed,
            "failed_jobs_count": self.failed_jobs_count,
            "jobs_count": self.jobs_count,
            "jobs_total_count": self.jobs_total_count,
            "matching_run_ids": list(self.matching_run_ids),
            "passed": self.passed,
            "required_job": self.required_job,
            "required_job_conclusion": self.required_job_conclusion,
            "required_job_failed_steps_count": self.required_job_failed_steps_count,
            "required_job_nonterminal_steps_count": (
                self.required_job_nonterminal_steps_count
            ),
            "required_job_present": self.required_job_present,
            "required_job_runner_id": self.required_job_runner_id,
            "required_job_status": self.required_job_status,
            "required_job_steps_count": self.required_job_steps_count,
            "required_job_successful_steps_count": (
                self.required_job_successful_steps_count
            ),
            "required_job_terminal_nonfailure_steps_count": (
                self.required_job_terminal_nonfailure_steps_count
            ),
            "run_attempt": self.run_attempt,
            "run_conclusion": self.run_conclusion,
            "run_head_sha": self.run_head_sha,
            "run_id": self.run_id,
            "run_jobs_url": self.run_jobs_url,
            "run_status": self.run_status,
            "steps_valid": self.steps_valid,
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
        fail("runner authority must be an object")
    if set(authority) != AUTHORITY_KEYS:
        fail(
            "runner authority keys drifted: "
            f"{sorted(set(authority) ^ AUTHORITY_KEYS)}"
        )
    enabled = sorted(key for key, value in authority.items() if value is not False)
    if enabled:
        fail(f"runner contract attempted to enable authority: {enabled}")


def require_false_mapping(
    value: Any,
    expected_keys: set[str],
    label: str,
) -> None:
    if not isinstance(value, dict):
        fail(f"{label} must be an object")
    if set(value) != expected_keys:
        fail(f"{label} keys drifted: {sorted(set(value) ^ expected_keys)}")
    enabled = sorted(key for key, item in value.items() if item is not False)
    if enabled:
        fail(f"{label} attempted to enable authority: {enabled}")


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

