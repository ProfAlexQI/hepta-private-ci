#!/usr/bin/env python3
"""Fail-closed verifier for the default-branch Hepta discovery baseline."""

from __future__ import annotations

from datetime import datetime
from pathlib import Path
import json
import re
import sys
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
CURRENT = ROOT / "docs/CURRENT.json"
README = ROOT / "README.md"
STATUS = ROOT / "docs/STATUS.md"
ARCHITECTURE_README = ROOT / "docs/architecture/README.md"
SHA1 = re.compile(r"^[0-9a-f]{40}$")


class DuplicateKeyError(ValueError):
    pass


def reject_duplicate_keys(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise DuplicateKeyError(f"duplicate JSON key: {key}")
        result[key] = value
    return result


def fail(message: str) -> None:
    raise SystemExit(f"FAIL_HEPTA_CURRENT_BASELINE: {message}")


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def require_sha(value: Any, label: str) -> None:
    require(isinstance(value, str) and SHA1.fullmatch(value) is not None, f"invalid {label}")


def require_false_tree(value: Any, path: str) -> None:
    if isinstance(value, dict):
        for key, child in value.items():
            require_false_tree(child, f"{path}.{key}")
    elif isinstance(value, bool):
        require(not value, f"positive authority/evidence rule at {path}")


def main() -> int:
    try:
        data = json.loads(
            CURRENT.read_text(encoding="utf-8"),
            object_pairs_hook=reject_duplicate_keys,
        )
    except (OSError, json.JSONDecodeError, DuplicateKeyError) as exc:
        fail(f"cannot load CURRENT.json: {exc}")

    require(data.get("schema") == "hepta.repository-current.v1", "wrong schema")
    require(data.get("schemaVersion") == 1, "wrong schema version")
    require(data.get("documentClass") == "candidate_discovery_index", "wrong document class")
    require(data.get("authority") == "discovery_only", "discovery index claims authority")
    require(data.get("repository") == "TrillionniumFoundation/hepta-private-ci", "repository mismatch")
    require(isinstance(data.get("staleAfterSeconds"), int), "missing staleAfterSeconds")
    require(1 <= data["staleAfterSeconds"] <= 604800, "unbounded stale interval")
    try:
        datetime.fromisoformat(data["observedAt"].replace("Z", "+00:00"))
    except (KeyError, AttributeError, ValueError) as exc:
        fail(f"invalid observedAt: {exc}")

    baseline = data.get("defaultBaseline")
    require(isinstance(baseline, dict), "default baseline missing")
    require(baseline.get("branch") == "integration/vnext-main-20260811", "default branch mismatch")
    require_sha(baseline.get("commit"), "default commit")
    require_sha(baseline.get("tree"), "default tree")
    require(baseline.get("productionAuthority") is False, "default baseline claims production authority")

    plan = data.get("normativePlanCandidate")
    require(isinstance(plan, dict), "plan candidate missing")
    require(plan.get("planId") == "HEPTA-ARCHITECTURE-CONVERGENCE-V5", "unexpected plan id")
    require(plan.get("version") == "5.0.1", "unexpected plan version")
    require(plan.get("selectionState") == "candidate_not_default_branch", "candidate silently selected")
    require(plan.get("qualificationState") == "not_proven", "candidate overclaims qualification")
    require_sha(plan.get("commit"), "plan commit")
    require_sha(plan.get("tree"), "plan tree")

    allowed_execution_states = data.get("executionStateVocabulary")
    require(isinstance(allowed_execution_states, list), "execution vocabulary missing")
    require(len(allowed_execution_states) == len(set(allowed_execution_states)), "duplicate execution state")
    required_states = {
        "not_run",
        "queued",
        "running",
        "passed",
        "failed",
        "blocked",
        "cancelled",
        "timed_out",
        "skipped",
        "neutral",
        "action_required",
        "stale",
        "superseded",
    }
    require(set(allowed_execution_states) == required_states, "incomplete GitHub execution vocabulary")

    stack = data.get("activeStack")
    require(isinstance(stack, list) and stack, "active stack missing")
    ids: set[str] = set()
    for entry in stack:
        require(isinstance(entry, dict), "active-stack entry is not an object")
        identifier = entry.get("id")
        require(isinstance(identifier, str) and identifier, "active-stack id missing")
        require(identifier not in ids, f"duplicate active-stack id: {identifier}")
        ids.add(identifier)
        require_sha(entry.get("commit"), f"{identifier} commit")
        require_sha(entry.get("tree"), f"{identifier} tree")
        require(entry.get("executionState") in required_states, f"unknown execution state for {identifier}")
        require(entry.get("activationAllowed") is False, f"activation opened for {identifier}")
        require(entry.get("qualificationClaim") in {"not_qualified", "not_evidence"}, f"qualification overclaim for {identifier}")

    required_stack_ids = {
        "P0.7a_signed_runtime_bootstrap_closure",
        "P0.7b/B0_verified_use_kernel",
        "P0.7b/B1a_provider_verified_use_boundary",
        "P0.7b/B1b_model_invocation_verified_use_boundary",
        "P0.7b/B1b_read_only_recovery",
        "AUTHBUS_P1_3_CANONICAL_QUOTA",
    }
    require(ids == required_stack_ids, "active stack is incomplete or contains an unreviewed entry")

    by_id = {entry["id"]: entry for entry in stack}
    b1b = by_id["P0.7b/B1b_model_invocation_verified_use_boundary"]
    require(b1b.get("executionState") == "failed", "B1b payload failure hidden")
    require(b1b.get("failureClass") == "payload_checksum_failure", "B1b failure class drift")

    authbus = by_id["AUTHBUS_P1_3_CANONICAL_QUOTA"]
    require(authbus.get("executionState") == "passed", "AuthBus executed-test fact missing")
    require(authbus.get("semanticQualification") == "blocked", "AuthBus semantic blocker hidden")
    expected_authbus_gaps = {
        "window_keyed_rpm_tpm_day_budget_accounting",
        "per_request_context_non_accumulation",
        "reservation_conservation_and_state_rules",
        "canonical_window_and_digest_chain_verification",
        "adversarial_multi_window_and_multi_request_tests",
    }
    require(set(authbus.get("openSemanticGaps", [])) == expected_authbus_gaps, "AuthBus gap ledger drift")

    evidence_rules = data.get("evidenceRules")
    require(isinstance(evidence_rules, dict) and evidence_rules, "evidence rules missing")
    require_false_tree(evidence_rules, "evidenceRules")

    authority_flags = data.get("authorityFlags")
    require(isinstance(authority_flags, dict) and authority_flags, "authority flags missing")
    require_false_tree(authority_flags, "authorityFlags")

    external_gates = data.get("externalGates")
    require(isinstance(external_gates, dict) and external_gates, "external gates missing")
    require(all(value == "not_issued" for value in external_gates.values()), "external decision self-issued")

    texts: dict[str, str] = {}
    for path in [README, STATUS, ARCHITECTURE_README]:
        try:
            text = path.read_text(encoding="utf-8")
        except OSError as exc:
            fail(f"cannot read {path.relative_to(ROOT)}: {exc}")
        require(len(text.strip()) >= 200, f"{path.relative_to(ROOT)} is empty or incomplete")
        texts[str(path.relative_to(ROOT))] = text

    require("docs/CURRENT.json" in texts["README.md"], "README does not point to CURRENT.json")
    require("HEPTA-ARCHITECTURE-CONVERGENCE-V5" in texts["README.md"], "README omits current plan candidate")
    require("CURRENT.json" in texts["docs/STATUS.md"], "STATUS does not name its machine source")

    joined = "\n".join(texts.values()) + "\n" + CURRENT.read_text(encoding="utf-8")
    for forbidden in ["/Users/", "/Volumes/", "C:\\Users\\"]:
        require(forbidden not in joined, f"host-local absolute path leaked: {forbidden}")

    for entry in stack:
        require(entry["commit"] in joined, f"human status omits exact commit for {entry['id']}")

    print("PASS_HEPTA_CURRENT_BASELINE_V1")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except BrokenPipeError:
        sys.exit(1)
