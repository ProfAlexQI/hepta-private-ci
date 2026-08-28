#!/usr/bin/env python3
"""Fail-closed source gate for Hepta inference INF-0.

A PASS proves only that the exact source/receipt shape is internally consistent.
It does not prove Rust compilation, runtime behavior, model efficacy, hardware
performance, operator acceptance or production authority.
"""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
PLAN = ROOT / "docs/hepta-vnext/inference/HEPTA_INFERENCE_RUNTIME_DEVELOPMENT_PLAN_V2.md"
MATRIX = ROOT / "docs/hepta-vnext/inference/HEPTA_INFERENCE_STAGE_MATRIX_V2.json"
STATUS = ROOT / "docs/hepta-vnext/inference/HEPTA_INFERENCE_IMPLEMENTATION_STATUS_V1.json"
RECEIPT = (
    ROOT
    / "docs/hepta-vnext/inference/HEPTA_INFERENCE_INF0_SOURCE_RECEIPT_2026-08-28.json"
)
CRATE = ROOT / "tools/hepta-inference-inf0"
RUST = CRATE / "src/lib.rs"
WORKFLOW = ROOT / ".github/workflows/hepta-inference-inf0.yml"
LMSTUDIO = ROOT / "codex-rs/lmstudio/src/lib.rs"
OLLAMA = ROOT / "codex-rs/ollama/src/lib.rs"
OSS_UTIL = ROOT / "codex-rs/utils/oss/src/lib.rs"

EXPECTED_BASE_COMMIT = "fe0889ecd46a5fc89de7b1ff3f28158c133a3502"
EXPECTED_BASE_TREE = "636341eb865b7c6d669958a96e7959de74fee020"
EXPECTED_BRANCH = "codex/hepta-inference-runtime-v2-20260828"
EXPECTED_PLAN_BLOB = "4381207acce1bf6371c248dc3280fff1f2ae59ce"
PASS = "PASS_HEPTA_INFERENCE_INF0_SOURCE_ONLY"

FALSE_AUTHORITY_FIELDS = {
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
}


class GateError(RuntimeError):
    pass


def require(condition: bool, message: str) -> None:
    if not condition:
        raise GateError(message)


def read_text(path: Path) -> str:
    require(path.is_file(), f"required file is missing: {path.relative_to(ROOT)}")
    return path.read_text(encoding="utf-8")


def read_json(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(read_text(path))
    except json.JSONDecodeError as error:
        raise GateError(f"invalid JSON in {path.relative_to(ROOT)}: {error}") from error
    require(isinstance(value, dict), f"{path.relative_to(ROOT)} must contain a JSON object")
    return value


def check_authority(value: dict[str, Any], label: str) -> None:
    authority = value.get("authority")
    require(isinstance(authority, dict), f"{label}.authority must be an object")
    require(authority.get("qualification_only") is True, f"{label} is not qualification-only")
    for field in sorted(FALSE_AUTHORITY_FIELDS):
        require(authority.get(field) is False, f"{label}.authority.{field} must be false")


def git(*args: str) -> str:
    completed = subprocess.run(
        ["git", *args],
        cwd=ROOT,
        check=False,
        capture_output=True,
        text=True,
    )
    if completed.returncode != 0:
        raise GateError(
            f"git {' '.join(args)} failed: {completed.stderr.strip() or completed.stdout.strip()}"
        )
    return completed.stdout.strip()


def main() -> int:
    plan = read_text(PLAN)
    matrix = read_json(MATRIX)
    status = read_json(STATUS)
    receipt = read_json(RECEIPT)
    rust = read_text(RUST)
    cargo = read_text(CRATE / "Cargo.toml")
    lock = read_text(CRATE / "Cargo.lock")
    workflow = read_text(WORKFLOW)
    lmstudio = read_text(LMSTUDIO)
    ollama = read_text(OLLAMA)
    oss_util = read_text(OSS_UTIL)

    require(
        matrix.get("plan_git_blob_sha1") == EXPECTED_PLAN_BLOB,
        "stage matrix plan blob drift",
    )
    require(
        status.get("plan_git_blob_sha1") == EXPECTED_PLAN_BLOB,
        "implementation status plan blob drift",
    )
    require(
        receipt.get("plan_git_blob_sha1") == EXPECTED_PLAN_BLOB,
        "source receipt plan blob drift",
    )
    require(
        git("rev-parse", f"HEAD:{PLAN.relative_to(ROOT)}") == EXPECTED_PLAN_BLOB,
        "checked-out plan blob drift",
    )

    for label, value in (("matrix", matrix), ("status", status), ("receipt", receipt)):
        binding = value.get("source_binding")
        require(isinstance(binding, dict), f"{label}.source_binding must be an object")
        require(binding.get("commit") == EXPECTED_BASE_COMMIT, f"{label} base commit drift")
        require(binding.get("tree") == EXPECTED_BASE_TREE, f"{label} base tree drift")
        check_authority(value, label)

    require(matrix.get("branch") == EXPECTED_BRANCH, "stage matrix branch drift")
    require(status.get("development_branch") == EXPECTED_BRANCH, "status branch drift")
    require(matrix.get("overall_status") == "SOURCE_PRESENT_NOT_RUN", "invalid matrix status")
    require(status.get("status") == "SOURCE_PRESENT_NOT_RUN", "invalid implementation status")
    require(status.get("qualified") is False, "implementation status must not claim qualified")
    require(receipt.get("qualified") is False, "source receipt must not claim qualified")

    stages = matrix.get("stages")
    require(isinstance(stages, list), "stage matrix stages must be an array")
    stage_status = {
        item.get("id"): item.get("status")
        for item in stages
        if isinstance(item, dict)
    }
    require(stage_status.get("INF-0A") == "SOURCE_PRESENT_NOT_RUN", "INF-0A status drift")
    require(stage_status.get("INF-0B") == "SOURCE_PRESENT_NOT_RUN", "INF-0B status drift")
    require(
        stage_status.get("INF-0C") == "SOURCE_SUBSET_PRESENT_NOT_RUN",
        "INF-0C must remain a source subset",
    )

    require("[workspace]" in cargo, "reference crate must remain an isolated workspace")
    require("[dependencies]" not in cargo, "reference crate must remain dependency-free")
    require('name = "hepta-inference-inf0"' in cargo, "reference crate package drift")
    require('name = "hepta-inference-inf0"' in lock, "standalone lockfile package drift")

    banned_rust = (
        "std::net",
        "TcpListener",
        "UdpSocket",
        "reqwest",
        "tokio",
        "unsafe {",
        "raw_prompt",
        "prompt: String",
        "MemoryWrite",
        "KgWrite",
    )
    for token in banned_rust:
        require(token not in rust, f"reference crate contains banned token: {token}")
    for token in (
        "AuthorityEscalation",
        "UnknownModelTuple",
        "StaleRequestGeneration",
        "StaleBackendGeneration",
        "StaleCancelGeneration",
        "CacheScope",
        "ReferenceLoopbackBackend",
    ):
        require(token in rust, f"reference contract is missing {token}")

    require("tokio::spawn" not in lmstudio, "LM Studio readiness still detaches model load")
    require(
        "lmstudio_client.load_model(model).await" in lmstudio,
        "LM Studio readiness does not await the load probe",
    )
    require(
        "let models = client.fetch_models().await?;" in ollama,
        "Ollama model discovery is not fail-closed",
    )
    require(
        "Unable to determine the Ollama version" in ollama,
        "Ollama unknown-version fence is missing",
    )
    require(
        "refusing to skip readiness checks" in oss_util,
        "unknown OSS providers do not fail closed",
    )

    for token in (
        "cargo fmt --manifest-path tools/hepta-inference-inf0/Cargo.toml",
        "cargo test --manifest-path tools/hepta-inference-inf0/Cargo.toml",
        "cargo clippy --manifest-path tools/hepta-inference-inf0/Cargo.toml",
        "cargo check --locked -p codex-lmstudio -p codex-ollama -p codex-utils-oss",
    ):
        require(token in workflow, f"workflow is missing gate command: {token}")

    source_commit = git("rev-parse", "HEAD^")
    source_tree = git("rev-parse", "HEAD^^{tree}")
    require(
        receipt.get("source_candidate_commit") == source_commit,
        "source receipt does not bind the receipt parent commit",
    )
    require(
        receipt.get("source_candidate_tree") == source_tree,
        "source receipt does not bind the receipt parent tree",
    )
    changed = {
        line
        for line in git("diff", "--name-only", "HEAD^", "HEAD").splitlines()
        if line
    }
    require(changed == {str(RECEIPT.relative_to(ROOT))}, "receipt commit must add only receipt")

    require("SOURCE_PRESENT_NOT_RUN" in plan, "plan must retain the non-qualified status")
    require("qualified=false" in plan, "plan must retain qualified=false")
    print(PASS)
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except GateError as error:
        print(f"FAIL_HEPTA_INFERENCE_INF0_SOURCE_GATE: {error}", file=sys.stderr)
        raise SystemExit(1) from error
