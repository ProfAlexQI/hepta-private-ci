#!/usr/bin/env python3
from __future__ import annotations

import importlib.util
from pathlib import Path

LEGACY = Path(__file__).with_name(
    "materialize-hepta-intelligence-p1c2-final-trust-closure.py"
)


def load_legacy():
    spec = importlib.util.spec_from_file_location("p1c2_final_trust_legacy", LEGACY)
    if spec is None or spec.loader is None:
        raise SystemExit(f"unable to load {LEGACY}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def patch_verifier() -> None:
    path = Path("scripts/verify-hepta-intelligence-p1-1c2-reviewed-efficacy.py")
    text = path.read_text(encoding="utf-8")
    anchor = "    failures = sorted(name for name, passed in checks.items() if not passed)\n"
    if text.count(anchor) != 1:
        raise SystemExit(f"{path}: verifier failure-summary anchor is not unique")
    block = r'''    checks["evaluation.acceptance_policy_and_canonicality"] = contains_all(
        evaluation,
        (
            "acceptance_policy_matches",
            "acceptance.reference_policy_mismatch",
            "AcceptancePolicy::default()",
            "blocked reasons must be strictly sorted and unique",
            "P1.1c.2 receipt status is not canonical",
            "evaluated lanes are not in canonical ablation order",
            "ContradictionLabel::Potential",
            "acceptance.privacy_blocked",
        ),
    )
    checks["tests.final_trust_regressions"] = contains_all(
        tests,
        (
            "alternate_valid_acceptance_policy_is_blocked",
            "potential_contradiction_labels_are_preserved",
            "privacy_block_uses_a_distinct_fail_closed_blocker",
            "receipt_rejects_noncanonical_status_lane_order_and_blockers",
        ),
    )
    checks["workflow.final_trust_matrix"] = contains_all(
        workflow,
        (
            "expected 25 P1.1c.2 tests",
            'receipt["acceptance_policy_matches"] is True',
        ),
    )
    checks["status.final_trust_contract"] = (
        status.get("implementation", {}).get("caller_reference_substitution_blocked") is True
        and status.get("implementation", {}).get("reference_acceptance_policy_binding") is True
        and status.get("implementation", {}).get("potential_contradiction_preserved") is True
        and status.get("implementation", {}).get("privacy_block_distinguished") is True
        and status.get("implementation", {}).get("receipt_canonicality_enforced") is True
    )
    checks["implementation_receipt.final_trust_contract"] = (
        receipt.get("claims", {}).get("caller_reference_substitution_blocked") is True
        and receipt.get("claims", {}).get("reference_acceptance_policy_binding") is True
        and receipt.get("claims", {}).get("potential_contradiction_preserved") is True
        and receipt.get("claims", {}).get("privacy_block_distinguished") is True
        and receipt.get("claims", {}).get("receipt_canonicality_enforced") is True
    )
'''
    text = text.replace(anchor, block + anchor, 1)
    path.write_text(text, encoding="utf-8")


def main() -> None:
    legacy = load_legacy()
    legacy.patch_verifier = patch_verifier
    legacy.main()


if __name__ == "__main__":
    main()
