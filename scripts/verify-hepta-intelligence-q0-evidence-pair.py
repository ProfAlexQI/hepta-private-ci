#!/usr/bin/env python3
"""Validate a same-candidate, independent E1/E2 Q0 evidence pair."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
from pathlib import Path
from typing import Any

SCHEMA = "hepta.intelligence.q0.executable_evidence.v1"


def load(path: Path) -> tuple[dict[str, Any], bytes]:
    raw = path.read_bytes()
    value = json.loads(raw)
    if not isinstance(value, dict):
        raise ValueError(f"{path}: receipt must be an object")
    return value, raw


def false_authority(receipt: dict[str, Any]) -> bool:
    return all(
        receipt.get(key) is False
        for key in (
            "source_writeback",
            "runtime_authority",
            "external_effects",
            "production_authority",
            "operator_acceptance",
            "promotion",
            "callers_ratchet",
        )
    )


def validate_one(receipt: dict[str, Any], evidence_class: str) -> list[str]:
    failures: list[str] = []
    if receipt.get("schema") != SCHEMA:
        failures.append(f"{evidence_class}.schema")
    if receipt.get("evidence_class") != evidence_class:
        failures.append(f"{evidence_class}.class")
    if receipt.get("all_gates_zero") is not True:
        failures.append(f"{evidence_class}.all_gates_zero")
    results = receipt.get("results")
    if not isinstance(results, dict) or not results:
        failures.append(f"{evidence_class}.results_nonempty")
    elif any(not isinstance(code, int) or code != 0 for code in results.values()):
        failures.append(f"{evidence_class}.results_zero")
    else:
        expected_manifest = hashlib.sha256(
            json.dumps(results, sort_keys=True, separators=(",", ":")).encode()
        ).hexdigest()
        if receipt.get("result_manifest_sha256") != expected_manifest:
            failures.append(f"{evidence_class}.result_manifest")
    expected_status = {
        "E1_LOCAL_EXECUTABLE": "PASS_Q0_E1_LOCAL_EXECUTABLE",
        "E2_INDEPENDENT_RUNNER": "PASS_Q0_E2_INDEPENDENT_RUNNER",
    }[evidence_class]
    if receipt.get("status") != expected_status:
        failures.append(f"{evidence_class}.status")
    candidate = receipt.get("candidate")
    if not isinstance(candidate, dict) or any(
        not isinstance(candidate.get(key), str)
        or re.fullmatch(r"[0-9a-f]{40}", candidate[key]) is None
        for key in ("head", "tree", "parent")
    ):
        failures.append(f"{evidence_class}.candidate")
    if receipt.get("qualified_candidate") is not False:
        failures.append(f"{evidence_class}.premature_qualification")
    workflow = receipt.get("workflow")
    if not isinstance(workflow, dict) or not isinstance(workflow.get("run_id"), int):
        failures.append(f"{evidence_class}.workflow")
    runner = receipt.get("runner")
    if not isinstance(runner, dict) or not runner.get("name"):
        failures.append(f"{evidence_class}.runner")
    if not false_authority(receipt):
        failures.append(f"{evidence_class}.authority")
    return failures


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--e1", type=Path, required=True)
    parser.add_argument("--e2", type=Path, required=True)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()

    e1, e1_raw = load(args.e1)
    e2, e2_raw = load(args.e2)
    failures = validate_one(e1, "E1_LOCAL_EXECUTABLE")
    failures.extend(validate_one(e2, "E2_INDEPENDENT_RUNNER"))

    if e1.get("candidate") != e2.get("candidate"):
        failures.append("pair.candidate_identity")

    e1_workflow = e1.get("workflow", {})
    e2_workflow = e2.get("workflow", {})
    if e1_workflow.get("run_id") == e2_workflow.get("run_id"):
        failures.append("pair.distinct_run_id")
    if e1_workflow.get("name") == e2_workflow.get("name"):
        failures.append("pair.distinct_workflow")

    e1_runner = e1.get("runner", {})
    e2_runner = e2.get("runner", {})
    e1_arch = str(e1_runner.get("arch", "")).upper()
    e2_arch = str(e2_runner.get("arch", "")).upper()
    if e1_arch not in {"X64", "X86_64", "AMD64"}:
        failures.append("pair.e1_x64")
    if e2_arch not in {"ARM64", "AARCH64"}:
        failures.append("pair.e2_arm64")
    if e1_runner.get("name") == e2_runner.get("name"):
        failures.append("pair.distinct_runner")
    if str(e1_runner.get("os", "")).lower() != "linux":
        failures.append("pair.e1_linux")
    if str(e2_runner.get("os", "")).lower() != "linux":
        failures.append("pair.e2_linux")
    e1_results = e1.get("results")
    e2_results = e2.get("results")
    if isinstance(e1_results, dict) and isinstance(e2_results, dict):
        if set(e1_results) != set(e2_results):
            failures.append("pair.result_surface")

    failures = sorted(set(failures))
    candidate = e1.get("candidate") if not failures else None
    output = {
        "schema": "hepta.intelligence.q0.evidence_pair.v1",
        "status": (
            "PASS_Q0_E1_E2_EVIDENCE_PAIR"
            if not failures
            else "FAIL_Q0_E1_E2_EVIDENCE_PAIR"
        ),
        "candidate": candidate,
        "e1_receipt_sha256": hashlib.sha256(e1_raw).hexdigest(),
        "e2_receipt_sha256": hashlib.sha256(e2_raw).hexdigest(),
        "e1_run_id": e1_workflow.get("run_id"),
        "e2_run_id": e2_workflow.get("run_id"),
        "evidence_pair_binding_sha256": hashlib.sha256(
            json.dumps(
                {
                    "candidate": candidate,
                    "e1_receipt_sha256": hashlib.sha256(e1_raw).hexdigest(),
                    "e2_receipt_sha256": hashlib.sha256(e2_raw).hexdigest(),
                    "e1_run_id": e1_workflow.get("run_id"),
                    "e2_run_id": e2_workflow.get("run_id"),
                },
                sort_keys=True,
                separators=(",", ":"),
            ).encode()
        ).hexdigest(),
        "independent_architectures": not failures,
        "qualified_candidate": not failures,
        "q0_executable_qualified": not failures,
        "runtime_wired": False,
        "external_effects": False,
        "production_authority": False,
        "operator_acceptance": False,
        "promotion": False,
        "callers_ratchet": False,
        "failures": failures,
    }
    encoded = json.dumps(output, indent=2, sort_keys=True) + "\n"
    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(encoded, encoding="utf-8")
    print(encoded, end="")
    return 0 if not failures else 1


if __name__ == "__main__":
    raise SystemExit(main())
