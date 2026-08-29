#!/usr/bin/env python3
"""Exact-head source and closed-authority gate for HEPTA-INFERENCE-RUNTIME-V4.

This gate proves only tracked repository source. It never promotes fixture,
provider, native-model, hardware, product, operator, promotion, or release
claims.
"""

from __future__ import annotations

import hashlib
import json
import pathlib
import subprocess
import sys
import tomllib
from typing import Any

ROOT = pathlib.Path(__file__).resolve().parents[1]
PLAN_ID = "HEPTA-INFERENCE-RUNTIME-V4"
PLAN_VERSION = "4.0.0"
REPOSITORY = "ProfHepta/hepta-private-ci"
DEVELOPMENT_PR = 73
DEVELOPMENT_BRANCH = "codex/hepta-inference-gap-closure-20260829"

CURRENT_POINTER = (
    "docs/hepta-vnext/inference/HEPTA_INFERENCE_CURRENT_PLAN_V1.json"
)
PLAN_PATH = (
    "docs/hepta-vnext/inference/HEPTA_INFERENCE_RUNTIME_DEVELOPMENT_PLAN_V4.md"
)
CURRENT_TRUTH = {
    "status": (
        "docs/hepta-vnext/inference/HEPTA_INFERENCE_CURRENT_STATUS_V4.json"
    ),
    "implementation": (
        "docs/hepta-vnext/inference/"
        "HEPTA_INFERENCE_IMPLEMENTATION_STATUS_V3.json"
    ),
    "stage_matrix": (
        "docs/hepta-vnext/inference/HEPTA_INFERENCE_STAGE_MATRIX_V5.json"
    ),
    "blocker_ledger": (
        "docs/hepta-vnext/inference/HEPTA_INFERENCE_BLOCKER_LEDGER_V2.json"
    ),
    "evidence_contract": (
        "docs/hepta-vnext/inference/"
        "HEPTA_INFERENCE_V4_CLOSURE_EVIDENCE_CONTRACT_V1.json"
    ),
}
WORKFLOWS = (
    ".github/workflows/hepta-inference-gap-closure.yml",
    ".github/workflows/hepta-inference-v2-remaining-source.yml",
    ".github/workflows/hepta-inference-inf0.yml",
    ".github/workflows/hepta-inference-inf0c-evidence-v2.yml",
    ".github/workflows/hepta-inference-inf0c-protocol-evidence.yml",
)
SCRIPTS = (
    "scripts/hepta-inference-v4-source-truth.py",
    "scripts/hepta-inference-inf0c-historical-receipt-gate.py",
)
INFERENCE_PACKAGES = {
    "hepta-infer-core": "codex-hepta-infer-core",
    "hepta-infer-client": "codex-hepta-infer-client",
    "hepta-inferd": "codex-hepta-inferd",
}
REQUIRED_PATHS = (
    *WORKFLOWS,
    *SCRIPTS,
    "codex-rs/Cargo.toml",
    "codex-rs/Cargo.lock",
    "codex-rs/rust-toolchain.toml",
    "codex-rs/hepta-infer-core/Cargo.toml",
    "codex-rs/hepta-infer-core/src/controller.rs",
    "codex-rs/hepta-infer-core/src/hashing.rs",
    "codex-rs/hepta-infer-core/src/lib.rs",
    "codex-rs/hepta-infer-core/src/protocol.rs",
    "codex-rs/hepta-infer-core/src/security.rs",
    "codex-rs/hepta-infer-core/src/tests.rs",
    "codex-rs/hepta-infer-client/Cargo.toml",
    "codex-rs/hepta-infer-client/src/lib.rs",
    "codex-rs/hepta-infer-client/src/tests.rs",
    "codex-rs/hepta-inferd/Cargo.toml",
    "codex-rs/hepta-inferd/src/lib.rs",
    "codex-rs/hepta-inferd/src/main.rs",
    "codex-rs/hepta-inferd/src/tests.rs",
    CURRENT_POINTER,
    PLAN_PATH,
    *CURRENT_TRUTH.values(),
    "docs/hepta-vnext/inference/"
    "HEPTA_INFERENCE_INF0C_SOURCE_RECEIPT_2026-08-28.json",
)
FORBIDDEN_TRANSIENT_PATHS = (
    ".github/workflows/hepta-inference-v4-one-shot-rustfmt.yml",
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
HIGHER_CLAIM_FIELDS = (
    "real_provider_executed",
    "real_native_model_executed",
    "hardware_qualified",
    "product_wired",
    "runtime_activated",
    "production_qualified",
    "operator_accepted",
    "promoted",
    "released",
)


def fail(message: str) -> None:
    raise SystemExit(f"FAIL_HEPTA_INFERENCE_V4_SOURCE_TRUTH: {message}")


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


def load_toml(relative: str) -> dict[str, Any]:
    try:
        value = tomllib.loads((ROOT / relative).read_text(encoding="utf-8"))
    except (OSError, tomllib.TOMLDecodeError) as error:
        fail(f"cannot parse {relative}: {error}")
    if not isinstance(value, dict):
        fail(f"{relative} must contain a TOML table")
    return value


def require_clean_exact_checkout() -> dict[str, str]:
    status = git("status", "--porcelain")
    if status:
        fail("checkout is not clean")
    head = git("rev-parse", "HEAD")
    tree = git("rev-parse", "HEAD^{tree}")
    parent = git("rev-parse", "HEAD^")
    if len(head) != 40 or len(tree) != 40 or len(parent) != 40:
        fail("invalid exact Git identity")
    return {"head": head, "tree": tree, "parent": parent}


def require_paths() -> dict[str, str]:
    missing = [relative for relative in REQUIRED_PATHS if not (ROOT / relative).is_file()]
    if missing:
        fail(f"missing required tracked files: {missing}")
    transient = [
        relative for relative in FORBIDDEN_TRANSIENT_PATHS if (ROOT / relative).exists()
    ]
    if transient:
        fail(f"transient mutation workflow remains tracked: {transient}")
    for relative in REQUIRED_PATHS:
        try:
            git("ls-files", "--error-unmatch", relative)
        except subprocess.CalledProcessError:
            fail(f"required path is not tracked: {relative}")
    return {relative: sha256_file(relative) for relative in REQUIRED_PATHS}


def require_closed_authority(document: dict[str, Any], relative: str) -> None:
    authority = document.get("authority")
    if not isinstance(authority, dict):
        fail(f"{relative} has no authority object")
    if authority.get("qualification_only") is not True:
        fail(f"{relative} qualification_only must be true")
    opened = [
        field for field in AUTHORITY_FALSE_FIELDS
        if authority.get(field) is not False
    ]
    if opened:
        fail(f"{relative} authority is not closed: {opened}")


def require_current_truth() -> dict[str, str]:
    pointer = load_json(CURRENT_POINTER)
    expected_pointer = {
        "active_plan_id": PLAN_ID,
        "active_plan_version": PLAN_VERSION,
        "active_plan_path": PLAN_PATH,
        "repository": REPOSITORY,
        "development_pr": DEVELOPMENT_PR,
        "development_branch": DEVELOPMENT_BRANCH,
    }
    for field, expected in expected_pointer.items():
        if pointer.get(field) != expected:
            fail(f"{CURRENT_POINTER} {field} drift")
    require_closed_authority(pointer, CURRENT_POINTER)

    links = pointer.get("current_truth")
    if not isinstance(links, dict):
        fail(f"{CURRENT_POINTER} current_truth must be an object")
    for key, relative in CURRENT_TRUTH.items():
        if links.get(key) != pathlib.Path(relative).name:
            fail(f"{CURRENT_POINTER} current_truth.{key} drift")

    digests = {CURRENT_POINTER: sha256_file(CURRENT_POINTER)}
    for relative in CURRENT_TRUTH.values():
        document = load_json(relative)
        for field, expected in (
            ("plan_id", PLAN_ID),
            ("plan_version", PLAN_VERSION),
            ("repository", REPOSITORY),
            ("development_pr", DEVELOPMENT_PR),
        ):
            if document.get(field) != expected:
                fail(f"{relative} {field} drift")
        branch = document.get("development_branch")
        if branch is not None and branch != DEVELOPMENT_BRANCH:
            fail(f"{relative} development_branch drift")
        require_closed_authority(document, relative)
        digests[relative] = sha256_file(relative)

    status = load_json(CURRENT_TRUTH["status"])
    claims = status.get("claims")
    if not isinstance(claims, dict):
        fail("current status has no claims object")
    if claims.get("source_candidate_present") is not True:
        fail("current status must acknowledge tracked source candidate")
    if claims.get("source_candidate_qualified") is not False:
        fail("tracked status cannot self-qualify the exact candidate")
    opened_claims = [
        field for field in HIGHER_CLAIM_FIELDS if claims.get(field) is not False
    ]
    if opened_claims:
        fail(f"tracked status opens unexecuted claims: {opened_claims}")

    implementation = load_json(CURRENT_TRUTH["implementation"])
    if implementation.get("status") != "SOURCE_PRESENT_NOT_QUALIFIED":
        fail("implementation status exceeds source-present posture")

    stage_matrix = load_json(CURRENT_TRUTH["stage_matrix"])
    global_claims = stage_matrix.get("global_claims")
    if not isinstance(global_claims, dict):
        fail("stage matrix has no global_claims")
    opened_global = [key for key, value in global_claims.items() if value is not False]
    if opened_global:
        fail(f"stage matrix opens global claims: {opened_global}")

    ledger = load_json(CURRENT_TRUTH["blocker_ledger"])
    summary = ledger.get("summary")
    if not isinstance(summary, dict) or summary.get("qualified") is not False:
        fail("blocker ledger must remain unqualified while blockers are open")

    contract = load_json(CURRENT_TRUTH["evidence_contract"])
    source_gate = contract.get("current_source_gate")
    if not isinstance(source_gate, dict):
        fail("evidence contract has no current_source_gate")
    if source_gate.get("toolchain") != "1.95.0":
        fail("evidence contract toolchain drift")
    if source_gate.get("skipped_is_pass") is not False:
        fail("evidence contract must reject skipped execution")
    if source_gate.get("non_empty_steps_required") is not True:
        fail("evidence contract must require non-empty steps")
    if source_gate.get("runner_id_nonzero_required") is not True:
        fail("evidence contract must require an assigned runner")

    plan = (ROOT / PLAN_PATH).read_text(encoding="utf-8")
    for marker in (
        PLAN_ID,
        REPOSITORY,
        "qualification_only: true",
        "production_listener: false",
        "real_provider_executed=false",
        "real_native_model_executed=false",
    ):
        if marker not in plan:
            fail(f"active plan missing marker {marker!r}")
    digests[PLAN_PATH] = sha256_file(PLAN_PATH)
    return digests


def require_workspace() -> dict[str, Any]:
    workspace = load_toml("codex-rs/Cargo.toml").get("workspace")
    if not isinstance(workspace, dict):
        fail("codex-rs/Cargo.toml has no workspace table")
    members = workspace.get("members")
    if not isinstance(members, list) or not all(isinstance(item, str) for item in members):
        fail("workspace.members must be a string array")

    package_names: dict[str, str] = {}
    for member, expected_name in INFERENCE_PACKAGES.items():
        if member not in members:
            fail(f"workspace missing inference member {member}")
        manifest_path = f"codex-rs/{member}/Cargo.toml"
        package = load_toml(manifest_path).get("package")
        if not isinstance(package, dict) or package.get("name") != expected_name:
            fail(f"{manifest_path} package.name drift")
        package_names[member] = expected_name

    if "hepta-infer-worker-host" in members:
        fail("worker-host may not enter workspace before its complete package lands")

    toolchain = load_toml("codex-rs/rust-toolchain.toml").get("toolchain")
    if not isinstance(toolchain, dict) or toolchain.get("channel") != "1.95.0":
        fail("codex-rs/rust-toolchain.toml must remain pinned to 1.95.0")

    return {
        "workspace_member_count": len(members),
        "inference_packages": package_names,
        "cargo_lock_sha256": sha256_file("codex-rs/Cargo.lock"),
        "rust_toolchain_sha256": sha256_file("codex-rs/rust-toolchain.toml"),
    }


def require_workflows() -> dict[str, str]:
    digests: dict[str, str] = {}
    for relative in WORKFLOWS[:2]:
        text = (ROOT / relative).read_text(encoding="utf-8")
        for marker in (
            "HEPTA-INFERENCE-RUNTIME-V4",
            "hepta-inference-v4-source-truth.py",
            "codex-hepta-infer-core",
            "codex-hepta-infer-client",
            "codex-hepta-inferd",
            "qualification-only",
            "real_provider_executed=false",
            "real_native_model_executed=false",
            "operator_accepted=false",
            "released=false",
        ):
            if marker not in text:
                fail(f"{relative} missing marker {marker!r}")
        digests[relative] = sha256_file(relative)

    stale = (
        "python3 scripts/hepta-inference-inf0c-evidence-v2-source-gate.py",
        "python3 scripts/hepta-inference-inf0-source-gate.py",
    )
    for relative in WORKFLOWS[2:]:
        historical = (ROOT / relative).read_text(encoding="utf-8")
        if "hepta-inference-inf0c-historical-receipt-gate.py" not in historical:
            fail(f"{relative} does not use immutable receipt gate")
        present_stale = [marker for marker in stale if marker in historical]
        if present_stale:
            fail(f"{relative} replays mutable current gates: {present_stale}")
        digests[relative] = sha256_file(relative)
    return digests


def require_hardening_source() -> dict[str, list[str]]:
    sources = {
        "controller": (
            ROOT / "codex-rs/hepta-infer-core/src/controller.rs"
        ).read_text(encoding="utf-8"),
        "security": (
            ROOT / "codex-rs/hepta-infer-core/src/security.rs"
        ).read_text(encoding="utf-8"),
        "client": (
            ROOT / "codex-rs/hepta-infer-client/src/lib.rs"
        ).read_text(encoding="utf-8"),
        "daemon": (
            ROOT / "codex-rs/hepta-inferd/src/lib.rs"
        ).read_text(encoding="utf-8"),
    }
    required = {
        "controller": [
            "inflight_requests",
            "running_requests",
            "accepted_token_count",
            "accepted_token_bytes",
            "token_chain_digest",
            "WorkerCancellationRequired",
            "forget_terminal",
        ],
        "security": [
            "PublicClient",
            "Worker",
            "Operator",
            "required_role",
        ],
        "client": [
            "is_public_client_operation",
            "RoleNotAuthorized",
        ],
        "daemon": [
            "Semaphore",
            "frame_read_timeout",
            "response_write_timeout",
            "ReceiptStore::open",
            "contains_request_id",
            "RoleNotAuthorized",
            "sync_directory",
        ],
    }
    for component, markers in required.items():
        missing = [marker for marker in markers if marker not in sources[component]]
        if missing:
            fail(f"{component} hardening source missing markers: {missing}")
    return required


def main() -> None:
    identity = require_clean_exact_checkout()
    path_digests = require_paths()
    truth_digests = require_current_truth()
    workspace = require_workspace()
    workflow_digests = require_workflows()
    hardening = require_hardening_source()

    receipt = {
        "schema": "hepta.inference.v4.exact_source_truth.v1",
        "plan_id": PLAN_ID,
        "plan_version": PLAN_VERSION,
        "repository": REPOSITORY,
        "pull_request_number": DEVELOPMENT_PR,
        "development_branch": DEVELOPMENT_BRANCH,
        **identity,
        "source_truth": "PASS",
        "path_digests": path_digests,
        "truth_digests": truth_digests,
        "workflow_digests": workflow_digests,
        "workspace": workspace,
        "hardening_markers": hardening,
        "current_plan_pointer_unique": True,
        "historical_current_gate_isolated": True,
        "public_worker_operator_role_source": "PRESENT",
        "inflight_running_deadline_source": "PRESENT",
        "token_integrity_source": "PRESENT",
        "connection_receipt_recovery_source": "PRESENT",
        "qualification_only": True,
        "real_provider_executed": False,
        "real_native_model_executed": False,
        "hardware_qualified": False,
        "product_wired": False,
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
