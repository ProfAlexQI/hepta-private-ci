#!/usr/bin/env python3
"""Exact-head source/truth gate for HEPTA-INFERENCE-RUNTIME-V3.

This gate proves tracked source and closed authority only. It deliberately emits
negative claims for real provider execution, native model execution, hardware,
product activation, operator acceptance, promotion, and release.
"""

from __future__ import annotations

import hashlib
import json
import pathlib
import subprocess
import sys
from typing import Any

ROOT = pathlib.Path(__file__).resolve().parents[1]
PLAN_ID = "HEPTA-INFERENCE-RUNTIME-V3"
REPOSITORY = "ProfHepta/hepta-private-ci"

CURRENT_CRATES = {
    "codex-hepta-infer-core",
    "codex-hepta-infer-client",
    "codex-hepta-inferd",
}

REQUIRED_PATHS = (
    ".github/workflows/hepta-inference-gap-closure.yml",
    ".github/workflows/hepta-inference-v2-remaining-source.yml",
    "codex-rs/Cargo.toml",
    "codex-rs/Cargo.lock",
    "codex-rs/hepta-infer-core/Cargo.toml",
    "codex-rs/hepta-infer-core/src/controller.rs",
    "codex-rs/hepta-infer-core/src/hashing.rs",
    "codex-rs/hepta-infer-core/src/lib.rs",
    "codex-rs/hepta-infer-core/src/protocol.rs",
    "codex-rs/hepta-infer-core/src/security.rs",
    "codex-rs/hepta-infer-core/src/tests.rs",
    "codex-rs/hepta-infer-client/src/lib.rs",
    "codex-rs/hepta-infer-client/src/tests.rs",
    "codex-rs/hepta-inferd/src/lib.rs",
    "codex-rs/hepta-inferd/src/main.rs",
    "codex-rs/hepta-inferd/src/tests.rs",
    "docs/hepta-vnext/inference/HEPTA_INFERENCE_RUNTIME_DEVELOPMENT_PLAN_V3.md",
    "docs/hepta-vnext/inference/HEPTA_INFERENCE_BLOCKER_LEDGER_V1.json",
    "docs/hepta-vnext/inference/HEPTA_INFERENCE_CURRENT_STATUS_V3.json",
    "docs/hepta-vnext/inference/HEPTA_INFERENCE_IMPLEMENTATION_STATUS_V2.json",
    "docs/hepta-vnext/inference/HEPTA_INFERENCE_STAGE_MATRIX_V4.json",
    "docs/hepta-vnext/inference/HEPTA_INFERENCE_V3_CLOSURE_EVIDENCE_CONTRACT_V1.json",
    "scripts/hepta-inference-v3-source-truth.py",
)

FORBIDDEN_PUBLIC_SHADOW_PATHS = (
    "codex-rs/hepta-inferd/src/shadow.rs",
    "codex-rs/hepta-inferd/src/bin/hepta-infer-admission-shadow.rs",
)

FORBIDDEN_WORKFLOW_PATHS = (
    "tools/hepta-inference-v2-qualification",
    "scripts/hepta-inference-inf0c-cancel-capability-v4.py",
    "codex-rs/hepta-infer-backend-v1",
    "codex-rs/hepta-infer-input-lease",
    "codex-rs/hepta-infer-model-registry",
    "codex-rs/hepta-infer-product-bridge",
    "codex-rs/hepta-infer-router",
    "codex-rs/hepta-infer-scheduler",
    "codex-rs/hepta-infer-semantic",
    "codex-rs/hepta-infer-worker-host",
)

TRUTH_JSONS = (
    "docs/hepta-vnext/inference/HEPTA_INFERENCE_BLOCKER_LEDGER_V1.json",
    "docs/hepta-vnext/inference/HEPTA_INFERENCE_CURRENT_STATUS_V3.json",
    "docs/hepta-vnext/inference/HEPTA_INFERENCE_IMPLEMENTATION_STATUS_V2.json",
    "docs/hepta-vnext/inference/HEPTA_INFERENCE_STAGE_MATRIX_V4.json",
    "docs/hepta-vnext/inference/HEPTA_INFERENCE_V3_CLOSURE_EVIDENCE_CONTRACT_V1.json",
)

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
    raise SystemExit(f"FAIL_HEPTA_INFERENCE_V3_SOURCE_TRUTH: {message}")


def git(*arguments: str) -> str:
    return subprocess.check_output(
        ["git", *arguments], cwd=ROOT, text=True
    ).strip()


def sha256_file(relative: str) -> str:
    return "sha256:" + hashlib.sha256((ROOT / relative).read_bytes()).hexdigest()


def load_json(relative: str) -> dict[str, Any]:
    try:
        value = json.loads((ROOT / relative).read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        fail(f"cannot parse {relative}: {error}")
    if not isinstance(value, dict):
        fail(f"{relative} must contain a JSON object")
    return value


def require_paths() -> None:
    missing = [relative for relative in REQUIRED_PATHS if not (ROOT / relative).is_file()]
    if missing:
        fail(f"missing required tracked files: {missing}")
    present_forbidden = [
        relative for relative in FORBIDDEN_PUBLIC_SHADOW_PATHS if (ROOT / relative).exists()
    ]
    if present_forbidden:
        fail(f"unprivileged all-role shadow entry points remain: {present_forbidden}")


def require_closed_authority(document: dict[str, Any], relative: str) -> None:
    authority = document.get("authority")
    if not isinstance(authority, dict):
        fail(f"{relative} has no authority object")
    if authority.get("qualification_only") is not True:
        fail(f"{relative} qualification_only must be true")
    opened = [field for field in AUTHORITY_FALSE_FIELDS if authority.get(field) is not False]
    if opened:
        fail(f"{relative} authority is not closed: {opened}")


def require_truth_documents() -> dict[str, str]:
    digests: dict[str, str] = {}
    for relative in TRUTH_JSONS:
        document = load_json(relative)
        if document.get("plan_id") != PLAN_ID:
            fail(f"{relative} plan_id drift")
        if document.get("repository") != REPOSITORY:
            fail(f"{relative} repository drift")
        require_closed_authority(document, relative)
        digests[relative] = sha256_file(relative)

    plan_path = "docs/hepta-vnext/inference/HEPTA_INFERENCE_RUNTIME_DEVELOPMENT_PLAN_V3.md"
    plan = (ROOT / plan_path).read_text(encoding="utf-8")
    for marker in (PLAN_ID, REPOSITORY, "qualification_only", "production_listener: false"):
        if marker not in plan:
            fail(f"active plan missing marker {marker!r}")
    digests[plan_path] = sha256_file(plan_path)
    return digests


def require_current_workspace() -> dict[str, Any]:
    raw = subprocess.check_output(
        [
            "cargo",
            "metadata",
            "--locked",
            "--no-deps",
            "--format-version",
            "1",
            "--manifest-path",
            "codex-rs/Cargo.toml",
        ],
        cwd=ROOT,
        text=True,
    )
    metadata = json.loads(raw)
    packages = {package["name"] for package in metadata.get("packages", [])}
    missing = sorted(CURRENT_CRATES - packages)
    if missing:
        fail(f"workspace is missing current inference crates: {missing}")
    return {
        "current_inference_packages": sorted(CURRENT_CRATES),
        "workspace_package_count": len(packages),
        "cargo_lock_sha256": sha256_file("codex-rs/Cargo.lock"),
    }


def require_workflow_convergence() -> dict[str, str]:
    digests: dict[str, str] = {}
    for relative in (
        ".github/workflows/hepta-inference-gap-closure.yml",
        ".github/workflows/hepta-inference-v2-remaining-source.yml",
    ):
        text = (ROOT / relative).read_text(encoding="utf-8")
        forbidden = [entry for entry in FORBIDDEN_WORKFLOW_PATHS if entry in text]
        if forbidden:
            fail(f"{relative} references future untracked paths: {forbidden}")
        for package in CURRENT_CRATES:
            if package not in text:
                fail(f"{relative} does not qualify {package}")
        if "qualification-only" not in text.lower():
            fail(f"{relative} aggregate is not labelled qualification-only")
        digests[relative] = sha256_file(relative)
    return digests


def require_hardening_source() -> None:
    sources = {
        "core": (ROOT / "codex-rs/hepta-infer-core/src/controller.rs").read_text(
            encoding="utf-8"
        ),
        "security": (ROOT / "codex-rs/hepta-infer-core/src/security.rs").read_text(
            encoding="utf-8"
        ),
        "client": (ROOT / "codex-rs/hepta-infer-client/src/lib.rs").read_text(
            encoding="utf-8"
        ),
        "daemon": (ROOT / "codex-rs/hepta-inferd/src/lib.rs").read_text(
            encoding="utf-8"
        ),
    }
    required_markers = {
        "core": (
            "inflight_requests",
            "running_requests",
            "accepted_token_count",
            "accepted_token_bytes",
            "token_chain_digest",
            "WorkerCancellationRequired",
            "forget_terminal",
        ),
        "security": ("PublicClient", "Worker", "Operator", "required_role"),
        "client": ("is_public_client_operation", "RoleNotAuthorized"),
        "daemon": (
            "Semaphore",
            "frame_read_timeout",
            "response_write_timeout",
            "ReceiptStore::open",
            "contains_request_id",
            "RoleNotAuthorized",
            "sync_directory",
        ),
    }
    for source, markers in required_markers.items():
        missing = [marker for marker in markers if marker not in sources[source]]
        if missing:
            fail(f"{source} hardening source missing markers: {missing}")


def main() -> None:
    require_paths()
    truth_digests = require_truth_documents()
    workspace = require_current_workspace()
    workflow_digests = require_workflow_convergence()
    require_hardening_source()

    receipt = {
        "schema": "hepta.inference.v3.exact_source_truth.v1",
        "plan_id": PLAN_ID,
        "repository": REPOSITORY,
        "head": git("rev-parse", "HEAD"),
        "tree": git("rev-parse", "HEAD^{tree}"),
        "parent": git("rev-parse", "HEAD^"),
        "source_truth": "PASS",
        "truth_digests": truth_digests,
        "workflow_digests": workflow_digests,
        **workspace,
        "public_worker_operator_role_source": "PRESENT",
        "inflight_running_bounds_source": "PRESENT",
        "token_integrity_source": "PRESENT",
        "connection_receipt_recovery_source": "PRESENT",
        "qualification_only": True,
        "real_provider_executed": False,
        "real_native_model_executed": False,
        "hardware_qualified": False,
        "product_default_route_changed": False,
        "operator_accepted": False,
        "promoted": False,
        "released": False,
    }
    print(json.dumps(receipt, indent=2, sort_keys=True))


if __name__ == "__main__":
    try:
        main()
    except subprocess.CalledProcessError as error:
        fail(f"command failed with status {error.returncode}: {error.cmd}")
