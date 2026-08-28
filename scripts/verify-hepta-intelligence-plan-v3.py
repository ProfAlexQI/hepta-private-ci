#!/usr/bin/env python3
"""Fail-closed source verifier for Hepta Intelligence Development Plan v3."""

from __future__ import annotations

import hashlib
import json
import re
import sys
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
PLAN = ROOT / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_DEVELOPMENT_PLAN_V3_2026-08-28.md"
POINTER = ROOT / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_CURRENT_PLAN.json"
STATUS = ROOT / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_EXECUTION_STATUS_V3.json"
CLAIMS = ROOT / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_CLAIM_LADDER_V1.json"

EXPECTED_BASE_HEAD = "7691978b786dd00c69477d1a3355be13db2c4d67"
EXPECTED_BASE_TREE = "bc2342443fe28d2b803cf1c8273c5d3cd4171ced"
EXPECTED_SOURCE_CANDIDATE = "7bb26ec016c2e2c83084756485ea324e79bcddbe"
EXPECTED_PLAN_ID = "HEPTA_INTELLIGENCE_DEVELOPMENT_PLAN_V3_2026-08-28"

REQUIRED_PLAN_MARKERS = (
    "# Hepta Intelligence Development Plan v3",
    "# 1. 审计结论与必须修复的架构问题",
    "# 3. Capability、Learning 与 Bio-inspired Claim Ladder",
    "# 5. Canonical Contracts",
    "LearningEpisodeV1",
    "LearningEventV1",
    "recall_eligible",
    "training_eligible",
    "evaluation_eligible",
    "promotion_eligible",
    "# 7. H5 Neuron 深化计划",
    "# 8. H6 Intuition 深化计划",
    "# 9. H7 Learning 深化计划",
    "# 10. 双速长期学习与抗遗忘",
    "Unqualified Stack Budget",
    "N0_METAPHORICAL_TYPED_PROPOSAL",
    "I0_DETERMINISTIC_SELECTIVE_POLICY",
    "L0_STATIC_SHADOW",
    "STOP unqualified implementation stacking",
)

FALSE_AUTHORITY_KEYS = (
    "runtime_wired",
    "default_open_wired",
    "app_runtime_attached",
    "tool_registered",
    "memory_write_authority",
    "projection_write_authority",
    "learning_write_authority",
    "outbox_dispatch_authority",
    "production_authority",
    "external_effects",
    "operator_acceptance",
    "promotion",
    "callers_ratchet",
)


class VerificationError(RuntimeError):
    pass


def load_json(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except FileNotFoundError as exc:
        raise VerificationError(f"missing required file: {path.relative_to(ROOT)}") from exc
    except json.JSONDecodeError as exc:
        raise VerificationError(f"invalid JSON in {path.relative_to(ROOT)}: {exc}") from exc
    if not isinstance(value, dict):
        raise VerificationError(f"{path.relative_to(ROOT)} must contain a JSON object")
    return value


def require(condition: bool, message: str) -> None:
    if not condition:
        raise VerificationError(message)


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def verify_authority_false(authority: dict[str, Any], label: str) -> None:
    for key in FALSE_AUTHORITY_KEYS:
        if key in authority:
            require(authority[key] is False, f"{label}.{key} must remain false")


def verify_commit(value: Any, label: str, allow_null: bool = False) -> None:
    if value is None and allow_null:
        return
    require(isinstance(value, str), f"{label} must be a string")
    require(re.fullmatch(r"[0-9a-f]{40}", value) is not None, f"{label} must be a lowercase 40-hex Git SHA")


def main() -> int:
    try:
        plan_bytes = PLAN.read_bytes()
        plan_text = plan_bytes.decode("utf-8")
        plan_sha256 = sha256_bytes(plan_bytes)
        for marker in REQUIRED_PLAN_MARKERS:
            require(marker in plan_text, f"plan is missing required marker: {marker}")

        pointer = load_json(POINTER)
        status = load_json(STATUS)
        claims = load_json(CLAIMS)

        require(pointer.get("schema") == "hepta_intelligence_current_plan_v1", "unexpected current-plan schema")
        require(status.get("schema") == "hepta_intelligence_execution_status_v3", "unexpected status schema")
        require(claims.get("schema") == "hepta_intelligence_claim_ladder_v1", "unexpected claim-ladder schema")

        current = pointer.get("current")
        require(isinstance(current, dict), "current-plan.current must be an object")
        require(current.get("plan_id") == EXPECTED_PLAN_ID, "current plan id mismatch")
        require(current.get("plan_file") == str(PLAN.relative_to(ROOT)), "current plan path mismatch")
        require(current.get("plan_content_sha256") == plan_sha256, "current plan SHA-256 mismatch")
        verify_commit(current.get("plan_commit"), "current.plan_commit", allow_null=True)

        exact_base = pointer.get("exact_base")
        require(isinstance(exact_base, dict), "current-plan.exact_base must be an object")
        require(exact_base.get("head") == EXPECTED_BASE_HEAD, "current-plan base head mismatch")
        require(exact_base.get("tree") == EXPECTED_BASE_TREE, "current-plan base tree mismatch")
        require(
            exact_base.get("p0_4c_hardened_source_candidate") == EXPECTED_SOURCE_CANDIDATE,
            "current-plan P0.4c candidate mismatch",
        )

        status_base = status.get("exact_base")
        require(isinstance(status_base, dict), "status.exact_base must be an object")
        require(status_base.get("head") == EXPECTED_BASE_HEAD, "status base head mismatch")
        require(status_base.get("tree") == EXPECTED_BASE_TREE, "status base tree mismatch")
        require(status.get("plan_id") == EXPECTED_PLAN_ID, "status plan id mismatch")
        require(status.get("plan_content_sha256") == plan_sha256, "status plan SHA-256 mismatch")
        verify_commit(status.get("plan_commit"), "status.plan_commit", allow_null=True)
        require(status.get("plan_commit") == current.get("plan_commit"), "pointer/status plan commit mismatch")
        require(status.get("plan_blob_sha") == current.get("plan_blob_sha"), "pointer/status plan blob mismatch")

        verify_authority_false(pointer.get("authority", {}), "current-plan.authority")
        verify_authority_false(status.get("authority", {}), "status.authority")
        verify_authority_false(status.get("current_tranche", {}).get("claims", {}), "status.current_tranche.claims")
        verify_authority_false(claims.get("authority", {}), "claims.authority")

        require(status.get("current_tranche", {}).get("qualified") is False, "plan tranche qualified must remain false")
        require(status.get("stack_budget", {}).get("runtime_source_freeze") is True, "runtime source freeze must be true")
        require(status.get("next_phase", {}).get("id") == "Q0", "next phase must be Q0 qualification debt closure")

        current_claims = claims.get("current_claims")
        require(isinstance(current_claims, dict), "claim ladder current_claims must be an object")
        require(current_claims.get("h5_level") == "N0_METAPHORICAL_TYPED_PROPOSAL", "H5 claim level mismatch")
        require(current_claims.get("h6_level") == "I0_DETERMINISTIC_SELECTIVE_POLICY", "H6 claim level mismatch")
        for key in (
            "self_evolution",
            "longitudinal_learning_efficacy",
            "closed_loop_learning",
            "structural_plasticity",
            "neuromorphic_mechanism",
            "biological_mechanism_replication",
        ):
            require(current_claims.get(key) is False, f"current claim {key} must remain false")

        receipt_rel = current.get("review_receipt_file")
        if current.get("plan_commit") is not None:
            require(isinstance(receipt_rel, str) and receipt_rel, "receipted plan must name a review receipt")
            receipt_path = ROOT / receipt_rel
            receipt = load_json(receipt_path)
            require(receipt.get("schema") == "hepta_intelligence_plan_review_receipt_v1", "unexpected receipt schema")
            require(receipt.get("plan_commit") == current.get("plan_commit"), "receipt plan commit mismatch")
            require(receipt.get("plan_content_sha256") == plan_sha256, "receipt plan SHA-256 mismatch")
            require(receipt.get("plan_blob_sha") == current.get("plan_blob_sha"), "receipt plan blob mismatch")
            verify_authority_false(receipt.get("authority", {}), "receipt.authority")

        workflow = (ROOT / ".github/workflows/hepta-intelligence-plan-v3.yml").read_text(encoding="utf-8")
        require("scripts/verify-hepta-intelligence-plan-v3.py" in workflow, "workflow must run the v3 verifier")
        require("contents: read" in workflow, "workflow must use read-only contents permission")
        for forbidden in ("deploy", "publish", "release"):
            require(f"{forbidden}:" not in workflow.lower(), f"workflow must not define a {forbidden} job")

        print(
            json.dumps(
                {
                    "result": "PASS_HEPTA_INTELLIGENCE_PLAN_V3_SOURCE_ONLY",
                    "plan_sha256": plan_sha256,
                    "plan_commit": current.get("plan_commit"),
                    "runtime_wired": False,
                    "production_authority": False,
                    "external_effects": False,
                    "operator_acceptance": False,
                    "promotion": False,
                    "callers_ratchet": False,
                },
                sort_keys=True,
            )
        )
        return 0
    except (VerificationError, UnicodeDecodeError, OSError) as exc:
        print(f"FAIL_HEPTA_INTELLIGENCE_PLAN_V3: {exc}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
