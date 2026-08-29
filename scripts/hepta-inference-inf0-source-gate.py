#!/usr/bin/env python3
"""Descendant-safe qualification-only source gate for Hepta inference.

Historical INF-0 documents are immutable evidence, not the mutable current
status. This gate validates their exact blobs and validates the current v3
matrix/runtime-closure files independently, so later stages do not invalidate
an earlier append-only receipt merely by adding descendant commits.
"""
from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
DOCS = ROOT / "docs/hepta-vnext/inference"
PLAN = DOCS / "HEPTA_INFERENCE_RUNTIME_DEVELOPMENT_PLAN_V2.md"
HISTORICAL_MATRIX = DOCS / "HEPTA_INFERENCE_STAGE_MATRIX_V2.json"
HISTORICAL_STATUS = DOCS / "HEPTA_INFERENCE_IMPLEMENTATION_STATUS_V1.json"
INF0_RECEIPT = DOCS / "HEPTA_INFERENCE_INF0C_SOURCE_RECEIPT_2026-08-28.json"
CURRENT_MATRIX = DOCS / "HEPTA_INFERENCE_STAGE_MATRIX_V3.json"
CURRENT_STATUS = DOCS / "HEPTA_INFERENCE_CURRENT_STATUS_V2.json"
RUNTIME_CLOSURE = DOCS / "HEPTA_INFERENCE_INF0C_RUNTIME_CLOSURE_RECEIPT_2026-08-29.json"
RECLASSIFIER = ROOT / "scripts/hepta-inference-inf0c-cancel-reclassifier.py"
PLAN_BLOB = "4381207acce1bf6371c248dc3280fff1f2ae59ce"
INF0_RECEIPT_BLOB = "79238e9af6f012b2fc4079f47eeb0c63751b9eb1"
BASE = "fe0889ecd46a5fc89de7b1ff3f28158c133a3502"
BASE_TREE = "636341eb865b7c6d669958a96e7959de74fee020"
PASS = "PASS_HEPTA_INFERENCE_DESCENDANT_SAFE_SOURCE_GATE"
FALSE_AUTHORITY = (
    "production_listener", "production_writer", "provider_effect", "external_effect",
    "shared_kg_write", "memory_write", "route_write", "fleet_write", "model_npu",
    "remote_inference", "automatic_model_install", "operator_acceptance", "promotion", "release",
)


class GateError(RuntimeError):
    pass


def need(condition: bool, message: str) -> None:
    if not condition:
        raise GateError(message)


def read(path: Path) -> str:
    need(path.is_file(), f"missing {path.relative_to(ROOT)}")
    return path.read_text(encoding="utf-8")


def obj(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(read(path))
    except json.JSONDecodeError as error:
        raise GateError(f"invalid JSON in {path.relative_to(ROOT)}: {error}") from error
    need(isinstance(value, dict), f"{path.relative_to(ROOT)} must be an object")
    return value


def git(*args: str) -> str:
    run = subprocess.run(["git", *args], cwd=ROOT, capture_output=True, text=True, check=False)
    if run.returncode:
        raise GateError(f"git {' '.join(args)} failed: {run.stderr.strip()}")
    return run.stdout.strip()


def candidate_head() -> str:
    parents = git("rev-list", "--parents", "-n", "1", "HEAD").split()
    need(len(parents) in (2, 3), "unexpected checkout parent shape")
    return git("rev-parse", "HEAD^2") if len(parents) == 3 else git("rev-parse", "HEAD")


def closed(value: dict[str, Any], label: str) -> None:
    authority = value.get("authority")
    need(isinstance(authority, dict), f"{label}.authority missing")
    need(authority.get("qualification_only") is True, f"{label} not qualification-only")
    for field in FALSE_AUTHORITY:
        need(authority.get(field) is False, f"{label}.{field} must be false")


def base_bound(value: dict[str, Any], label: str) -> None:
    binding = value.get("source_binding")
    need(isinstance(binding, dict), f"{label}.source_binding missing")
    need(binding.get("commit") == BASE, f"{label} base commit drift")
    need(binding.get("tree") == BASE_TREE, f"{label} base tree drift")
    need(value.get("plan_git_blob_sha1") == PLAN_BLOB, f"{label} plan drift")


def main() -> int:
    candidate = candidate_head()
    need(git("rev-parse", f"{candidate}:{PLAN.relative_to(ROOT)}") == PLAN_BLOB, "plan blob drift")
    need(
        git("rev-parse", f"{candidate}:{INF0_RECEIPT.relative_to(ROOT)}") == INF0_RECEIPT_BLOB,
        "historical INF-0C receipt mutated",
    )

    historical_matrix = obj(HISTORICAL_MATRIX)
    historical_status = obj(HISTORICAL_STATUS)
    historical_receipt = obj(INF0_RECEIPT)
    for label, value in (
        ("historical_matrix", historical_matrix),
        ("historical_status", historical_status),
        ("historical_receipt", historical_receipt),
    ):
        base_bound(value, label)
        closed(value, label)
    need(historical_receipt.get("claim") == "SOURCE_PRESENT_NOT_RUN", "historical receipt claim drift")
    need(historical_receipt.get("qualified") is False, "historical receipt promoted")

    matrix = obj(CURRENT_MATRIX)
    status = obj(CURRENT_STATUS)
    closure = obj(RUNTIME_CLOSURE)
    for label, value in (("current_matrix", matrix), ("current_status", status), ("runtime_closure", closure)):
        closed(value, label)
    need(matrix.get("schema") == "hepta.inference.stage_matrix.v3", "current matrix schema drift")
    need(status.get("schema") == "hepta.inference.current_status.v2", "current status schema drift")
    need(matrix.get("current_stage") == status.get("current_stage") == "INF-2A", "current stage drift")
    need(matrix.get("current_stage_status") == status.get("current_stage_status") == "NOT_STARTED", "stage status drift")
    stages = {stage.get("id"): stage for stage in matrix.get("stages", []) if isinstance(stage, dict)}
    for stage_id in ("INF-0A", "INF-0B", "INF-0C", "INF-1"):
        stage = stages.get(stage_id, {})
        need(stage.get("status") == "EXECUTED_PASSED_QUALIFICATION_ONLY", f"{stage_id} status drift")
        need(stage.get("qualified") is True, f"{stage_id} qualification missing")
        need(stage.get("operator_accepted") is False and stage.get("promoted") is False, f"{stage_id} authority drift")
    for stage_id in ("INF-2A", "INF-2B", "INF-3", "INF-4", "INF-5", "INF-6", "INF-7", "INF-8"):
        need(stages.get(stage_id, {}).get("status") == "NOT_STARTED", f"{stage_id} activated early")

    evidence = closure.get("immutable_evidence")
    need(isinstance(evidence, dict), "runtime immutable evidence missing")
    need(evidence.get("workflow_run_id") == 33239546304, "runtime run drift")
    need(evidence.get("job_id") == 99066422684, "runtime job drift")
    need(evidence.get("artifact_id") == 9711096479, "runtime artifact drift")
    need(
        evidence.get("artifact_sha256") == "83f5346ee3d7107b794e6466a5aef007d3b087f30ff2850074a73d8a71e353ad",
        "runtime artifact digest drift",
    )
    runtime = closure.get("runtime_results")
    need(isinstance(runtime, dict), "runtime results missing")
    need(runtime.get("semantic_output_verified") is True, "semantic output evidence missing")
    for section in ("transport_disconnect", "controlled_restart"):
        value = runtime.get(section)
        need(isinstance(value, dict), f"{section} missing")
        need(value.get("ollama") == "PASS" and value.get("lmstudio") == "PASS", f"{section} incomplete")
    need(runtime.get("implicit_download") is False, "implicit download enabled")

    cancellation = closure.get("explicit_provider_cancellation")
    need(isinstance(cancellation, dict), "cancellation evidence missing")
    providers = cancellation.get("providers")
    need(isinstance(providers, dict) and set(providers) == {"ollama", "lmstudio"}, "cancel providers drift")
    for provider, value in providers.items():
        need(value.get("classification") == "UNSUPPORTED_FAIL_CLOSED", f"{provider} cancel class drift")
        need(value.get("provider_cancel_capability_classified") is True, f"{provider} cancel unclassified")
        need(value.get("provider_cancel_acknowledged") is False, f"{provider} ack invented")
        need(value.get("transport_disconnect_used") is False, f"{provider} disconnect conflation")
    need(cancellation.get("backend_cancellation_acknowledged") is False, "backend ack invented")
    need(cancellation.get("transport_disconnect_used_as_ack") is False, "disconnect promoted")
    policy = cancellation.get("dispatch_policy")
    need(isinstance(policy, dict), "cancel dispatch policy missing")
    need(policy.get("cancel_required_request") == "REJECT_BEFORE_BACKEND_DISPATCH", "cancel dispatch policy drift")

    for path in (
        ROOT / "tools/hepta-inference-inf0/Cargo.toml",
        ROOT / "tools/hepta-inference-inf0/src/lib.rs",
        ROOT / "codex-rs/hepta-infer-core/src/lib.rs",
        ROOT / "codex-rs/hepta-inferd/src/lib.rs",
        RECLASSIFIER,
    ):
        read(path)
    core = read(ROOT / "codex-rs/hepta-infer-core/src/lib.rs")
    for banned in ("raw_prompt", "MemoryWrite", "KgWrite", "remote_inference_endpoint"):
        need(banned not in core, f"core contains banned authority surface {banned}")
    inferd = read(ROOT / "codex-rs/hepta-inferd/src/lib.rs")
    need("TcpListener" not in inferd and "0.0.0.0" not in inferd, "inferd TCP surface detected")
    print(PASS)
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except GateError as error:
        print(f"FAIL_HEPTA_INFERENCE_SOURCE_GATE: {error}", file=sys.stderr)
        raise SystemExit(1) from error
