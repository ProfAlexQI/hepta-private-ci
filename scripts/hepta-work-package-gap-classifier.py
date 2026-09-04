#!/usr/bin/env python3
"""Classify every non-complete V8 work package without widening authority."""

from __future__ import annotations

import argparse
import json
import sys
from collections import Counter
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
REGISTRY = ROOT / "docs" / "delivery" / "WORK_PACKAGES.json"
OUTPUT = ROOT / "qualification" / "gap-closure" / "WORK_PACKAGE_GAPS.json"

COMPLETE_STATES = {"source_implemented"}
KNOWN_INCOMPLETE_STATES = {
    "blocked_external",
    "planned",
    "semantic_review_pending",
    "source_implemented_execution_pending",
}

EXTERNAL_KEYWORDS = {
    "future_time_window",
    "longitudinal",
    "physical",
    "real_provider",
    "real_credential",
    "real_matrix",
    "operator_acceptance",
    "independent_acceptance",
    "canary",
    "hardware",
    "robot",
    "fleet",
    "production_traffic",
    "distribution_shift",
    "delayed_outcome",
    "retention",
    "forgetting",
    "unlearning",
}

ACTIVATION_KEYWORDS = {
    "activation",
    "production_caller",
    "product_caller",
    "runtime_install",
    "install",
    "rollout",
    "cutover",
    "retirement",
    "migration",
    "reload",
    "rollback",
    "release",
    "promotion",
    "selection",
    "composition",
    "routing",
    "supervisor",
    "daemon",
    "service",
}


def load_registry() -> list[dict[str, Any]]:
    try:
        document = json.loads(REGISTRY.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise RuntimeError(f"cannot parse work-package registry: {error}") from error
    packages = document.get("packages")
    if not isinstance(packages, list) or not all(isinstance(item, dict) for item in packages):
        raise RuntimeError("work-package registry packages must be a list of objects")
    return packages


def string_set(package: dict[str, Any], *keys: str) -> set[str]:
    values: set[str] = set()
    for key in keys:
        value = package.get(key)
        if isinstance(value, str):
            values.add(value)
        elif isinstance(value, list):
            values.update(str(item) for item in value)
        elif isinstance(value, dict):
            for nested in value.values():
                if isinstance(nested, str):
                    values.add(nested)
                elif isinstance(nested, list):
                    values.update(str(item) for item in nested)
    return values


def contains_keyword(values: set[str], keywords: set[str]) -> bool:
    lowered = {value.lower() for value in values}
    return any(keyword in value for value in lowered for keyword in keywords)


def classify(package: dict[str, Any]) -> tuple[str, list[str]]:
    state = package.get("state")
    if state == "blocked_external":
        return "external_governance_gate", ["canonical state is blocked_external"]
    if state == "semantic_review_pending":
        return "independent_semantic_review", ["canonical state requires semantic review"]
    if state == "source_implemented_execution_pending":
        return "execution_evidence_pending", ["source exists but declared execution evidence is pending"]
    if state != "planned":
        raise RuntimeError(f"unclassified work-package state: {state!r}")

    values = string_set(
        package,
        "parallelClass",
        "qualificationProfile",
        "deliverables",
        "acceptanceCriteria",
        "claimImpact",
    )
    reasons: list[str] = []
    if package.get("parallelClass") == "external_evidence_coordinated":
        reasons.append("parallelClass requires external evidence coordination")
    if contains_keyword(values, EXTERNAL_KEYWORDS):
        reasons.append("deliverables or acceptance criteria require external/longitudinal/physical evidence")
    if reasons:
        return "external_empirical_evidence", reasons

    source_mutation_allowed = package.get("sourceMutationAllowed")
    allowed_write_paths = package.get("allowedWritePaths")
    if source_mutation_allowed is False or allowed_write_paths == []:
        return "decision_or_governance_gate", ["package cannot mutate source"]

    if contains_keyword(values, ACTIVATION_KEYWORDS):
        return "activation_or_composition", ["deliverables include activation/composition lifecycle work"]

    return "internal_source_implementation", ["bounded source mutation is allowed and no external evidence keyword is present"]


def dependency_state(
    package: dict[str, Any],
    by_id: dict[str, dict[str, Any]],
    key: str,
) -> tuple[bool, list[str]]:
    value = package.get(key)
    if not isinstance(value, list) or not all(isinstance(item, str) for item in value):
        raise RuntimeError(f"{package.get('id')}.{key} must be a string list")
    unresolved = [
        package_id
        for package_id in value
        if package_id not in by_id or by_id[package_id].get("state") not in COMPLETE_STATES
    ]
    return not unresolved, unresolved


def build() -> dict[str, Any]:
    packages = load_registry()
    by_id: dict[str, dict[str, Any]] = {}
    for package in packages:
        package_id = package.get("id")
        if not isinstance(package_id, str) or not package_id:
            raise RuntimeError("work package has no stable id")
        if package_id in by_id:
            raise RuntimeError(f"duplicate work package: {package_id}")
        by_id[package_id] = package

    entries: list[dict[str, Any]] = []
    for package in packages:
        state = package.get("state")
        if state in COMPLETE_STATES:
            continue
        if state not in KNOWN_INCOMPLETE_STATES:
            raise RuntimeError(f"unknown incomplete state: {state!r}")
        category, reasons = classify(package)
        development_ready, development_blockers = dependency_state(
            package, by_id, "developmentAfter"
        )
        activation_ready, activation_blockers = dependency_state(
            package, by_id, "activationAfter"
        )
        evidence_ready, evidence_blockers = dependency_state(
            package, by_id, "evidenceAfter"
        )
        entries.append(
            {
                "id": package["id"],
                "module": package.get("module"),
                "priority": package.get("priority"),
                "state": state,
                "category": category,
                "classificationReasons": reasons,
                "qualificationProfile": package.get("qualificationProfile"),
                "parallelClass": package.get("parallelClass"),
                "sourceMutationAllowed": package.get("sourceMutationAllowed"),
                "allowedWritePaths": package.get("allowedWritePaths"),
                "deliverables": package.get("deliverables"),
                "acceptanceCriteria": package.get("acceptanceCriteria"),
                "developmentReady": development_ready,
                "developmentBlockers": development_blockers,
                "activationReady": activation_ready,
                "activationBlockers": activation_blockers,
                "evidenceReady": evidence_ready,
                "evidenceBlockers": evidence_blockers,
                "authorityDelta": package.get("authorityDelta"),
            }
        )
    entries.sort(key=lambda item: (item.get("priority", 999), item["id"]))

    categories = Counter(entry["category"] for entry in entries)
    states = Counter(entry["state"] for entry in entries)
    ready_internal = [
        entry["id"]
        for entry in entries
        if entry["developmentReady"]
        and entry["category"]
        in {
            "execution_evidence_pending",
            "internal_source_implementation",
            "activation_or_composition",
        }
    ]
    blocked_external = [
        entry["id"]
        for entry in entries
        if entry["category"]
        in {
            "external_empirical_evidence",
            "external_governance_gate",
            "independent_semantic_review",
        }
    ]

    return {
        "schema": "hepta.v8-work-package-gap-classification.v1",
        "schemaVersion": 1,
        "planId": "HEPTA-GLOBAL-MODULAR-DEVELOPMENT-PLAN",
        "planVersion": "8.0.0",
        "candidateBranch": "codex/hepta-v8-gap-closure-20260905",
        "completePackageCount": len(packages) - len(entries),
        "remainingPackageCount": len(entries),
        "stateCounts": dict(sorted(states.items())),
        "categoryCounts": dict(sorted(categories.items())),
        "developmentReadyInternalPackageIds": ready_internal,
        "externallyBlockedPackageIds": blocked_external,
        "packages": entries,
        "authorityInterpretation": (
            "classification and source implementation do not grant activation, effect, "
            "independent acceptance, selection, promotion, merge or release authority"
        ),
    }


def render(document: dict[str, Any]) -> str:
    return json.dumps(document, indent=2, ensure_ascii=False) + "\n"


def normalize() -> bool:
    document = build()
    text = render(document)
    current = OUTPUT.read_text(encoding="utf-8") if OUTPUT.exists() else ""
    if current == text:
        return False
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text, encoding="utf-8")
    return True


def verify() -> list[str]:
    expected = build()
    if not OUTPUT.is_file():
        return ["WORK_PACKAGE_GAPS.json is missing"]
    try:
        actual = json.loads(OUTPUT.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        return [f"cannot parse WORK_PACKAGE_GAPS.json: {error}"]
    failures: list[str] = []
    if actual != expected:
        failures.append("WORK_PACKAGE_GAPS.json is stale")
    for entry in expected["packages"]:
        if entry.get("authorityDelta") != "none":
            failures.append(f"work package declares authority delta: {entry['id']}")
    if expected["remainingPackageCount"] != len(expected["packages"]):
        failures.append("remaining package count is inconsistent")
    return failures


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("command", choices=("normalize", "verify"))
    args = parser.parse_args()
    try:
        changed = normalize() if args.command == "normalize" else None
        failures = verify()
    except (OSError, RuntimeError) as error:
        print(error, file=sys.stderr)
        return 1
    if failures:
        for failure in failures:
            print(f"- {failure}", file=sys.stderr)
        return 1
    if changed is not None:
        print(f"classification_changed={str(changed).lower()}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
