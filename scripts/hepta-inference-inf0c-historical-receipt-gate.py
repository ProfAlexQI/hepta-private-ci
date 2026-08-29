#!/usr/bin/env python3
"""Validate immutable INF-0C source receipt without replaying current stage truth."""

from __future__ import annotations

import hashlib
import json
import pathlib
import subprocess
from typing import Any

ROOT = pathlib.Path(__file__).resolve().parents[1]
RECEIPT_PATH = (
    "docs/hepta-vnext/inference/"
    "HEPTA_INFERENCE_INF0C_SOURCE_RECEIPT_2026-08-28.json"
)
EXPECTED = {
    "schema": "hepta.inference.inf0c_protocol_source_receipt.v2",
    "plan_id": "HEPTA-INFERENCE-RUNTIME-V2",
    "plan_version": "2.0.0",
    "source_binding_repository": "ProfAlexQI/hepta-private-ci",
    "source_binding_commit": "fe0889ecd46a5fc89de7b1ff3f28158c133a3502",
    "source_binding_tree": "636341eb865b7c6d669958a96e7959de74fee020",
    "source_candidate_commit": "7f29597d5ebc080320f62cb92d4c21bd6b5a008a",
    "source_candidate_tree": "70703d18590d4cd6baa09fb2310635183075c227",
    "parent_stack_receipt_commit": "0550d2936373d310ecd1ec140910e19cac83526d",
    "claim": "SOURCE_PRESENT_NOT_RUN",
}
AUTHORITY_FALSE_FIELDS = (
    "production_listener",
    "production_writer",
    "provider_effect",
    "external_effect",
    "shared_kg_write",
    "memory_write",
    "route_write",
    "fleet_write",
    "model_npu",
    "remote_inference",
    "automatic_model_install",
    "operator_acceptance",
    "promotion",
    "release",
)


def fail(message: str) -> None:
    raise SystemExit(f"FAIL_HEPTA_INFERENCE_INF0C_HISTORICAL_RECEIPT: {message}")


def git(*arguments: str) -> str:
    return subprocess.check_output(
        ["git", *arguments], cwd=ROOT, text=True
    ).strip()


def load_receipt() -> dict[str, Any]:
    try:
        value = json.loads((ROOT / RECEIPT_PATH).read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        fail(f"cannot parse historical receipt: {error}")
    if not isinstance(value, dict):
        fail("historical receipt must be a JSON object")
    return value


def require_equal(actual: Any, expected: Any, field: str) -> None:
    if actual != expected:
        fail(f"{field} drift: expected {expected!r}, observed {actual!r}")


def require_tracked_checkout_unchanged() -> None:
    for arguments in (
        ("diff", "--quiet", "--ignore-submodules", "--"),
        ("diff", "--cached", "--quiet", "--ignore-submodules", "--"),
    ):
        result = subprocess.run(["git", *arguments], cwd=ROOT, check=False)
        if result.returncode != 0:
            fail("tracked checkout is not clean")


def require_commit_tree(commit: str, expected_tree: str, label: str) -> None:
    try:
        actual_tree = git("rev-parse", f"{commit}^{{tree}}")
    except subprocess.CalledProcessError:
        fail(f"{label} commit object is missing: {commit}")
    require_equal(actual_tree, expected_tree, f"{label} tree")


def require_ancestor(commit: str, label: str) -> None:
    result = subprocess.run(
        ["git", "merge-base", "--is-ancestor", commit, "HEAD"],
        cwd=ROOT,
        check=False,
    )
    if result.returncode != 0:
        fail(f"{label} is not an ancestor of exact HEAD: {commit}")


def main() -> None:
    require_tracked_checkout_unchanged()

    receipt = load_receipt()
    require_equal(receipt.get("schema"), EXPECTED["schema"], "schema")
    require_equal(receipt.get("plan_id"), EXPECTED["plan_id"], "plan_id")
    require_equal(receipt.get("plan_version"), EXPECTED["plan_version"], "plan_version")
    require_equal(receipt.get("claim"), EXPECTED["claim"], "claim")
    require_equal(receipt.get("qualified"), False, "qualified")

    binding = receipt.get("source_binding")
    if not isinstance(binding, dict):
        fail("source_binding must be an object")
    require_equal(
        binding.get("repository"),
        EXPECTED["source_binding_repository"],
        "source_binding.repository",
    )
    require_equal(
        binding.get("commit"),
        EXPECTED["source_binding_commit"],
        "source_binding.commit",
    )
    require_equal(
        binding.get("tree"),
        EXPECTED["source_binding_tree"],
        "source_binding.tree",
    )
    require_equal(
        receipt.get("source_candidate_commit"),
        EXPECTED["source_candidate_commit"],
        "source_candidate_commit",
    )
    require_equal(
        receipt.get("source_candidate_tree"),
        EXPECTED["source_candidate_tree"],
        "source_candidate_tree",
    )
    require_equal(
        receipt.get("parent_stack_receipt_commit"),
        EXPECTED["parent_stack_receipt_commit"],
        "parent_stack_receipt_commit",
    )

    authority = receipt.get("authority")
    if not isinstance(authority, dict):
        fail("authority must be an object")
    require_equal(authority.get("qualification_only"), True, "qualification_only")
    opened = [
        field for field in AUTHORITY_FALSE_FIELDS
        if authority.get(field) is not False
    ]
    if opened:
        fail(f"historical authority is not closed: {opened}")

    components = receipt.get("source_candidate_components")
    if not isinstance(components, dict):
        fail("source_candidate_components must be an object")
    if components.get("inf1_daemon") is not False:
        fail("historical receipt must keep inf1_daemon false")
    source_components = [
        key for key, value in components.items() if key != "inf1_daemon" and value is True
    ]
    if not source_components:
        fail("historical receipt has no positive source components")

    require_commit_tree(
        EXPECTED["source_binding_commit"],
        EXPECTED["source_binding_tree"],
        "source binding",
    )
    require_commit_tree(
        EXPECTED["source_candidate_commit"],
        EXPECTED["source_candidate_tree"],
        "source candidate",
    )
    parent_tree = git(
        "rev-parse", f"{EXPECTED['parent_stack_receipt_commit']}^{{tree}}"
    )
    if len(parent_tree) != 40:
        fail("invalid parent stack tree")
    require_ancestor(EXPECTED["source_binding_commit"], "source binding")
    require_ancestor(EXPECTED["source_candidate_commit"], "source candidate")
    require_ancestor(EXPECTED["parent_stack_receipt_commit"], "parent stack")

    historical_receipt = git(
        "show",
        f"93bdd3245c2f3d0685ceae8e2ce1267c40a63685:{RECEIPT_PATH}",
    )
    current_receipt = (ROOT / RECEIPT_PATH).read_text(encoding="utf-8").rstrip("\n")
    require_equal(current_receipt, historical_receipt, "historical receipt content")

    raw = (ROOT / RECEIPT_PATH).read_bytes()
    result = {
        "schema": "hepta.inference.inf0c.historical_receipt_validation.v1",
        "head": git("rev-parse", "HEAD"),
        "tree": git("rev-parse", "HEAD^{tree}"),
        "receipt_path": RECEIPT_PATH,
        "receipt_sha256": "sha256:" + hashlib.sha256(raw).hexdigest(),
        "source_binding_commit": EXPECTED["source_binding_commit"],
        "source_candidate_commit": EXPECTED["source_candidate_commit"],
        "parent_stack_receipt_commit": EXPECTED["parent_stack_receipt_commit"],
        "historical_receipt_valid": True,
        "ancestor_bound": True,
        "mutable_current_stage_gate_replayed": False,
        "qualification_only": True,
        "real_provider_executed": False,
        "real_native_model_executed": False,
        "backend_cancellation_acknowledged": False,
        "operator_accepted": False,
        "promoted": False,
        "released": False,
    }
    print(json.dumps(result, indent=2, sort_keys=True))


if __name__ == "__main__":
    try:
        main()
    except subprocess.CalledProcessError as error:
        fail(f"command failed with status {error.returncode}: {error.cmd}")
