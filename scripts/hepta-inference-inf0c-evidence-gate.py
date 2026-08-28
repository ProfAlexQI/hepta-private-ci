#!/usr/bin/env python3
"""Fail-closed source gate for the stacked INF-0C evidence lane."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
DOCS = ROOT / "docs/hepta-vnext/inference"
PLAN = DOCS / "HEPTA_INFERENCE_INF0C_EVIDENCE_PLAN_V1.md"
STATUS = DOCS / "HEPTA_INFERENCE_INF0C_EVIDENCE_STATUS_V1.json"
RECEIPT = DOCS / "HEPTA_INFERENCE_INF0C_EVIDENCE_SOURCE_RECEIPT_2026-08-28.json"
PARENT_MATRIX = DOCS / "HEPTA_INFERENCE_STAGE_MATRIX_V2.json"
SCRIPT_GLOB = "hepta-inference-inf0c-cancel-restart*.py"
WORKFLOW = ROOT / ".github/workflows/hepta-inference-inf0c-evidence.yml"

PARENT_COMMIT = "68c97b7d1211c8e319df3b850182401ab541eea4"
PARENT_TREE = "3a0d8795db033f780f2e9715b2bc20a2cafa627a"
BRANCH = "codex/hepta-inference-inf0c-evidence-20260828"
PASS = "PASS_HEPTA_INFERENCE_INF0C_EVIDENCE_SOURCE_ONLY"

FALSE_AUTHORITY = (
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


class GateError(RuntimeError):
    pass


def require(condition: bool, message: str) -> None:
    if not condition:
        raise GateError(message)


def text(path: Path) -> str:
    require(path.is_file(), f"missing file: {path.relative_to(ROOT)}")
    return path.read_text(encoding="utf-8")


def object_json(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(text(path))
    except json.JSONDecodeError as error:
        raise GateError(f"invalid JSON in {path.relative_to(ROOT)}: {error}") from error
    require(isinstance(value, dict), f"{path.relative_to(ROOT)} must contain an object")
    return value


def git(*args: str, check: bool = True) -> str:
    completed = subprocess.run(
        ["git", *args],
        cwd=ROOT,
        check=False,
        capture_output=True,
        text=True,
    )
    if check and completed.returncode != 0:
        raise GateError(
            f"git {' '.join(args)} failed: "
            f"{completed.stderr.strip() or completed.stdout.strip()}"
        )
    return completed.stdout.strip() if completed.returncode == 0 else ""


def candidate_head() -> str:
    parents = git("rev-list", "--parents", "-n", "1", "HEAD").split()
    require(len(parents) in (2, 3), "unexpected checkout parent shape")
    return git("rev-parse", "HEAD^2") if len(parents) == 3 else git("rev-parse", "HEAD")


def check_authority(value: dict[str, Any], label: str) -> None:
    authority = value.get("authority")
    require(isinstance(authority, dict), f"{label}.authority missing")
    require(authority.get("qualification_only") is True, f"{label} not qualification-only")
    for field in FALSE_AUTHORITY:
        require(authority.get(field) is False, f"{label}.authority.{field} must be false")


def require_tokens(source: str, required: tuple[str, ...], label: str) -> None:
    for token in required:
        require(token in source, f"{label} missing token: {token}")


def main() -> int:
    candidate = candidate_head()
    plan = text(PLAN)
    status = object_json(STATUS)
    receipt = object_json(RECEIPT)
    parent_matrix = object_json(PARENT_MATRIX)
    script_paths = sorted((ROOT / "scripts").glob(SCRIPT_GLOB))
    require(script_paths, "evidence harness source parts missing")
    script = "\n".join(text(path) for path in script_paths)
    workflow = text(WORKFLOW)

    binding = status.get("source_binding")
    require(isinstance(binding, dict), "status source binding missing")
    require(binding.get("commit") == PARENT_COMMIT, "status parent commit drift")
    require(binding.get("tree") == PARENT_TREE, "status parent tree drift")
    require(status.get("development_branch") == BRANCH, "status branch drift")
    require(status.get("status") == "SOURCE_PRESENT_NOT_RUN", "status claim drift")
    require(status.get("qualified") is False, "status qualified early")
    check_authority(status, "status")
    check_authority(receipt, "receipt")

    parent_stages = {
        item.get("id"): item
        for item in parent_matrix.get("stages", [])
        if isinstance(item, dict)
    }
    require(parent_matrix.get("current_stage") == "INF-0C", "parent stage is not INF-0C")
    require(parent_stages.get("INF-1", {}).get("status") == "NOT_STARTED", "INF-1 activated")
    check_authority(parent_matrix, "parent_matrix")

    require_tokens(
        script,
        (
            "TRANSPORT_DISCONNECT_WITH_POST_HEALTH_V1",
            '"backend_acknowledged": False',
            "HEPTA_INF0C_CONTROL_HELPER",
            "HEPTA_INF0C_CONTROL_HELPER_SHA256",
            "path.lstat()",
            "group/world writable",
            "hash_file(canonical)",
            '[str(helper.path), "restart", provider]',
            "sanitized_subprocess_environment",
            "service_unavailable_observed",
            "service_recovered_observed",
            "post_restart_health",
            "urllib.request.ProxyHandler({})",
            "NoRedirect",
            "shell=False",
            "raw_model_output_persisted",
            "PASS_HEPTA_INFERENCE_INF0C_CANCEL_RESTART_SELF_TEST",
            "exec(compile(_SOURCE",
        ),
        "evidence harness",
    )
    for banned in (
        "shell=True",
        "shell = True",
        "os.system(",
        "subprocess.call(",
        "automatic_model_install",
    ):
        require(banned not in script, f"evidence harness contains banned token: {banned}")

    require_tokens(
        workflow,
        (
            "hepta-inference-inf0c-cancel-restart.py --self-test",
            "hepta-inference-inf0c-evidence-gate.py",
            "runs-on: [self-hosted, hepta-inference-e2e]",
            "--execute",
            "--controlled-restart",
            "HEPTA_INF0C_CONTROL_HELPER",
            "HEPTA_INF0C_CONTROL_HELPER_SHA256",
            "if-no-files-found: error",
        ),
        "evidence workflow",
    )
    require_tokens(
        plan,
        (
            "TRANSPORT_DISCONNECT_WITH_POST_HEALTH_V1",
            "backend_acknowledged=false",
            "<helper> restart ollama",
            "<helper> restart lmstudio",
            "INF-1",
            "qualified",
        ),
        "evidence plan",
    )

    require(
        git("rev-parse", f"{candidate}:{RECEIPT.relative_to(ROOT)}")
        == git("rev-parse", f"HEAD:{RECEIPT.relative_to(ROOT)}"),
        "merge checkout altered the evidence receipt",
    )
    source_commit = git("rev-parse", f"{candidate}^")
    source_tree = git("show", "-s", "--format=%T", source_commit)
    require(receipt.get("source_candidate_commit") == source_commit, "receipt parent mismatch")
    require(receipt.get("source_candidate_tree") == source_tree, "receipt tree mismatch")
    require(receipt.get("claim") == "SOURCE_PRESENT_NOT_RUN", "receipt claim drift")
    require(receipt.get("qualified") is False, "receipt qualified early")
    changed = {
        line
        for line in git("diff", "--name-only", source_commit, candidate).splitlines()
        if line
    }
    require(
        changed == {str(RECEIPT.relative_to(ROOT))},
        "final receipt commit must change only the evidence receipt",
    )
    require(
        git("merge-base", PARENT_COMMIT, candidate) == PARENT_COMMIT,
        "evidence branch is not descended from the frozen parent receipt",
    )

    print(PASS)
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except GateError as error:
        print(f"FAIL_HEPTA_INFERENCE_INF0C_EVIDENCE_GATE: {error}", file=sys.stderr)
        raise SystemExit(1) from error
