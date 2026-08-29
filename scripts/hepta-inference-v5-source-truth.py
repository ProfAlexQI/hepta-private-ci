#!/usr/bin/env python3
"""Exact-head source and authority gate for HEPTA-INFERENCE-RUNTIME-V5.

The gate distinguishes staged component payloads from an integrated runtime and from
real software/device/operator evidence. It never upgrades a state merely because a
file, fixture, queued workflow, or historical receipt exists.
"""

from __future__ import annotations

import hashlib
import json
import pathlib
import subprocess
import sys
from typing import Any

ROOT = pathlib.Path(__file__).resolve().parents[1]
DOCS = ROOT / "docs/hepta-vnext/inference"
PLAN_ID = "HEPTA-INFERENCE-RUNTIME-V5"
PLAN_VERSION = "5.0.0"
REPOSITORY = "ProfHepta/hepta-private-ci"

AUTHORITY = {
    "qualification_only": True,
    "production_listener": False,
    "production_writer": False,
    "provider_effect": False,
    "external_effect": False,
    "shared_kg_write": False,
    "memory_write": False,
    "route_write": False,
    "fleet_write": False,
    "model_npu": False,
    "remote_inference": False,
    "automatic_model_install": False,
    "operator_acceptance": False,
    "promotion": False,
    "release": False,
}

MACHINE_FILES = {
    "current_plan": DOCS / "HEPTA_INFERENCE_CURRENT_PLAN_V2.json",
    "current_status": DOCS / "HEPTA_INFERENCE_CURRENT_STATUS_V5.json",
    "implementation": DOCS / "HEPTA_INFERENCE_IMPLEMENTATION_STATUS_V4.json",
    "stage_matrix": DOCS / "HEPTA_INFERENCE_STAGE_MATRIX_V6.json",
    "blocker_ledger": DOCS / "HEPTA_INFERENCE_BLOCKER_LEDGER_V3.json",
    "evidence_contract": DOCS / "HEPTA_INFERENCE_V5_CLOSURE_EVIDENCE_CONTRACT_V2.json",
}

STAGING_PAYLOADS = (
    ROOT / "tools/hepta-inference-v4-payloads/inf-s1.yml",
    ROOT / "tools/hepta-inference-v4-payloads/inf-s2.yml",
    ROOT / "tools/hepta-inference-v4-payloads/inf-s3.yml",
    ROOT / "tools/hepta-inference-v4-payloads/inf-s4.yml",
    ROOT / "tools/hepta-inference-v4-payloads/inf-r1.yml",
    ROOT / "tools/hepta-inference-v4-payloads/inf-r2-chain.yml",
    ROOT / "tools/hepta-inference-v4-payloads/inf-r3.yml",
    ROOT / "tools/hepta-inference-v4-payloads/apply_payload_steps.py",
    ROOT / "tools/hepta-inference-v4-payloads/post_materialize_hardening.py",
)

BASE_COMPONENTS = (
    ROOT / "codex-rs/hepta-infer-core/Cargo.toml",
    ROOT / "codex-rs/hepta-infer-client/Cargo.toml",
    ROOT / "codex-rs/hepta-inferd/Cargo.toml",
    ROOT / "codex-rs/hepta-infer-worker-host/Cargo.toml",
    ROOT / "codex-rs/hepta-infer-client/src/shadow_bridge.rs",
)

INTEGRATION_MARKERS = {
    ROOT / "codex-rs/hepta-inferd/src/runtime_coordinator.rs": (
        "PromptInputLease",
        "DeterministicScheduler",
        "ProviderRuntimeHost",
    ),
    ROOT / "codex-rs/hepta-inferd/src/prompt_input_lease.rs": (
        "PromptInputLease",
        "prompt_digest",
        "consume",
    ),
    ROOT / "codex-rs/hepta-agentd/src/inference_shadow.rs": (
        "ProductShadowBridge",
        "ShadowCompareOnly",
        "kill",
    ),
}

FORBIDDEN_HIGHER_TRUE_FIELDS = (
    "source_runtime_integrated",
    "source_candidate_qualified",
    "real_provider_executed",
    "real_native_model_executed",
    "hardware_qualified",
    "product_shadow_wired",
    "privacy_accepted",
    "operator_accepted",
    "promoted",
    "released",
)


def fail(message: str) -> "NoReturn":
    raise SystemExit(message)


def git(*args: str) -> str:
    return subprocess.check_output(
        ["git", "-C", str(ROOT), *args], text=True
    ).strip()


def load_json(path: pathlib.Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        fail(f"invalid JSON {path.relative_to(ROOT)}: {error}")
    if not isinstance(value, dict):
        fail(f"top-level JSON object required: {path.relative_to(ROOT)}")
    return value


def require_file(path: pathlib.Path) -> None:
    if not path.is_file():
        fail(f"required file missing: {path.relative_to(ROOT)}")


def require_plan_identity(label: str, document: dict[str, Any]) -> None:
    if document.get("plan_id") != PLAN_ID:
        fail(f"{label}: plan_id drift")
    if document.get("plan_version") != PLAN_VERSION:
        fail(f"{label}: plan_version drift")
    if document.get("repository") != REPOSITORY:
        fail(f"{label}: repository drift")
    if document.get("authority") != AUTHORITY:
        fail(f"{label}: authority drift")


def sha256_file(path: pathlib.Path) -> str:
    return "sha256:" + hashlib.sha256(path.read_bytes()).hexdigest()


def verify_current_pointer(pointer: dict[str, Any]) -> None:
    expected = {
        "schema": "hepta.inference.current_plan.v2",
        "active_plan_id": PLAN_ID,
        "active_plan_version": PLAN_VERSION,
        "active_plan_path": (
            "docs/hepta-vnext/inference/"
            "HEPTA_INFERENCE_RUNTIME_DEVELOPMENT_PLAN_V5.md"
        ),
        "repository": REPOSITORY,
    }
    for key, value in expected.items():
        if pointer.get(key) != value:
            fail(f"current pointer drift: {key}")
    if pointer.get("authority") != AUTHORITY:
        fail("current pointer authority drift")


def verify_higher_claims(status: dict[str, Any]) -> None:
    conclusion = status.get("current_legal_conclusion")
    if not isinstance(conclusion, dict):
        fail("current status legal conclusion missing")
    if conclusion.get("source_components_present") is not True:
        fail("source component presence must be explicit")
    for field in FORBIDDEN_HIGHER_TRUE_FIELDS:
        if conclusion.get(field) is not False:
            fail(f"unproved higher claim must remain false: {field}")


def verify_payloads() -> dict[str, str]:
    digests: dict[str, str] = {}
    for path in STAGING_PAYLOADS:
        require_file(path)
        digests[str(path.relative_to(ROOT))] = sha256_file(path)
    return digests


def verify_base_components() -> dict[str, str]:
    digests: dict[str, str] = {}
    for path in BASE_COMPONENTS:
        require_file(path)
        digests[str(path.relative_to(ROOT))] = sha256_file(path)
    return digests


def integration_state() -> tuple[bool, list[str]]:
    missing: list[str] = []
    for path, markers in INTEGRATION_MARKERS.items():
        if not path.is_file():
            missing.append(str(path.relative_to(ROOT)))
            continue
        text = path.read_text(encoding="utf-8")
        for marker in markers:
            if marker not in text:
                missing.append(f"{path.relative_to(ROOT)}::{marker}")
    return (not missing, missing)


def verify_no_secret_or_raw_prompt_evidence() -> None:
    forbidden_literals = (
        '"raw_prompt":',
        '"prompt_text":',
        '"capability_secret":',
        '"session_secret":',
        '"bootstrap_secret":',
    )
    for path in MACHINE_FILES.values():
        text = path.read_text(encoding="utf-8")
        for literal in forbidden_literals:
            if literal in text:
                fail(f"secret/raw prompt evidence field forbidden: {path.name}: {literal}")


def main() -> None:
    require_file(DOCS / "HEPTA_INFERENCE_RUNTIME_DEVELOPMENT_PLAN_V5.md")
    require_file(ROOT / "scripts/hepta-inference-v5-source-truth.py")
    for path in MACHINE_FILES.values():
        require_file(path)

    documents = {name: load_json(path) for name, path in MACHINE_FILES.items()}
    verify_current_pointer(documents["current_plan"])
    for name in (
        "current_status",
        "implementation",
        "stage_matrix",
        "blocker_ledger",
        "evidence_contract",
    ):
        require_plan_identity(name, documents[name])
    verify_higher_claims(documents["current_status"])
    verify_no_secret_or_raw_prompt_evidence()

    payload_digests = verify_payloads()
    component_digests = verify_base_components()
    integrated, missing_integration = integration_state()
    if integrated:
        fail(
            "V5 tracked status still declares source_runtime_integrated=false; "
            "publish a descendant exact-head status/evidence update before qualification"
        )

    receipt = {
        "schema": "hepta.inference.v5.source_truth_receipt.v1",
        "plan_id": PLAN_ID,
        "plan_version": PLAN_VERSION,
        "repository": REPOSITORY,
        "head": git("rev-parse", "HEAD"),
        "tree": git("rev-parse", "HEAD^{tree}"),
        "parent": git("rev-parse", "HEAD^"),
        "plan_digest": sha256_file(
            DOCS / "HEPTA_INFERENCE_RUNTIME_DEVELOPMENT_PLAN_V5.md"
        ),
        "machine_file_digests": {
            name: sha256_file(path) for name, path in MACHINE_FILES.items()
        },
        "staging_payload_digests": payload_digests,
        "base_component_digests": component_digests,
        "source_components_present": True,
        "source_runtime_integrated": False,
        "missing_integration_markers": missing_integration,
        "source_candidate_qualified": False,
        "real_provider_executed": False,
        "real_native_model_executed": False,
        "hardware_qualified": False,
        "product_shadow_wired": False,
        "operator_accepted": False,
        "promoted": False,
        "released": False,
        "authority": AUTHORITY,
    }
    print(json.dumps(receipt, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
