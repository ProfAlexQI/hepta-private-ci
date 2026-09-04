"""Deterministic engineering scheduling and integration eligibility.

This module coordinates bounded work envelopes. It deliberately exposes no API
for merging a pull request, modifying runtime authority, deploying, promoting,
or releasing a candidate.
"""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import PurePosixPath
from typing import Iterable

MAX_PACKAGES = 4096
MAX_LEASES = 4096
MAX_ASSIGNMENTS = 128


@dataclass(frozen=True, order=True)
class WorkPackage:
    priority: int
    package_id: str
    predecessors: tuple[str, ...]
    write_paths: tuple[str, ...]


@dataclass(frozen=True, order=True)
class PathLease:
    package_id: str
    write_paths: tuple[str, ...]


@dataclass(frozen=True)
class ScheduleReceipt:
    assigned: tuple[str, ...]
    blocked: tuple[tuple[str, str], ...]
    runtime_authority: bool = False
    merge_authority: bool = False


@dataclass(frozen=True)
class IntegrationEvidence:
    candidate_head: str
    exact_head: str
    merge_candidate_head: str
    source_inventory_ok: bool
    static_verification_ok: bool
    focused_tests_ok: bool
    package_tests_ok: bool
    all_target_check_ok: bool
    strict_lint_ok: bool
    clean_worktree_ok: bool
    authority_delta: bool


@dataclass(frozen=True)
class IntegrationDecision:
    eligible_for_independent_review: bool
    reasons: tuple[str, ...]
    runtime_authority: bool = False
    merge_authority: bool = False
    promotion_authority: bool = False
    release_authority: bool = False


def schedule(
    packages: Iterable[WorkPackage],
    completed: Iterable[str],
    active_leases: Iterable[PathLease],
    maximum_assignments: int = MAX_ASSIGNMENTS,
) -> ScheduleReceipt:
    package_list = tuple(packages)
    lease_list = tuple(active_leases)
    if len(package_list) > MAX_PACKAGES:
        raise ValueError("package limit exceeded")
    if len(lease_list) > MAX_LEASES:
        raise ValueError("lease limit exceeded")
    if maximum_assignments < 1 or maximum_assignments > MAX_ASSIGNMENTS:
        raise ValueError("maximum assignments is outside the bounded range")

    package_ids = [package.package_id for package in package_list]
    if len(package_ids) != len(set(package_ids)):
        raise ValueError("duplicate package identity")
    completed_set = frozenset(completed)
    leased_paths = tuple(
        path
        for lease in lease_list
        for path in _normalized_paths(lease.write_paths)
    )

    selected_paths: list[str] = []
    assigned: list[str] = []
    blocked: list[tuple[str, str]] = []
    for package in sorted(package_list):
        normalized = _normalized_paths(package.write_paths)
        if not normalized:
            blocked.append((package.package_id, "empty_write_envelope"))
            continue
        missing = sorted(set(package.predecessors) - completed_set)
        if missing:
            blocked.append((package.package_id, f"missing_predecessor:{missing[0]}"))
            continue
        if _has_overlap(normalized, leased_paths):
            blocked.append((package.package_id, "active_path_lease"))
            continue
        if _has_overlap(normalized, tuple(selected_paths)):
            blocked.append((package.package_id, "batch_path_conflict"))
            continue
        if len(assigned) >= maximum_assignments:
            blocked.append((package.package_id, "assignment_limit"))
            continue
        assigned.append(package.package_id)
        selected_paths.extend(normalized)

    return ScheduleReceipt(tuple(assigned), tuple(blocked))


def decide_integration(evidence: IntegrationEvidence) -> IntegrationDecision:
    reasons: list[str] = []
    if evidence.candidate_head != evidence.exact_head:
        reasons.append("exact_head_mismatch")
    if evidence.candidate_head != evidence.merge_candidate_head:
        reasons.append("merge_candidate_head_mismatch")
    checks = {
        "source_inventory": evidence.source_inventory_ok,
        "static_verification": evidence.static_verification_ok,
        "focused_tests": evidence.focused_tests_ok,
        "package_tests": evidence.package_tests_ok,
        "all_target_check": evidence.all_target_check_ok,
        "strict_lint": evidence.strict_lint_ok,
        "clean_worktree": evidence.clean_worktree_ok,
    }
    reasons.extend(name for name, passed in checks.items() if not passed)
    if evidence.authority_delta:
        reasons.append("authority_delta")
    return IntegrationDecision(not reasons, tuple(reasons))


def _normalized_paths(paths: Iterable[str]) -> tuple[str, ...]:
    normalized: list[str] = []
    for raw_path in paths:
        if not raw_path or raw_path.startswith("/") or ".." in PurePosixPath(raw_path).parts:
            raise ValueError(f"invalid write path: {raw_path!r}")
        path = raw_path.removesuffix("/**").rstrip("/")
        if not path:
            raise ValueError("write path must not resolve to repository root")
        normalized.append(path)
    return tuple(sorted(set(normalized)))


def _has_overlap(left: tuple[str, ...], right: tuple[str, ...]) -> bool:
    return any(_paths_overlap(a, b) for a in left for b in right)


def _paths_overlap(left: str, right: str) -> bool:
    return (
        left == right
        or left.startswith(f"{right}/")
        or right.startswith(f"{left}/")
    )
