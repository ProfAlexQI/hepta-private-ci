#!/usr/bin/env python3
"""Build a reviewed A0 V4.3 replacement tree without writing candidate source.

This script is support-branch tooling only. It rewrites the checked-out working tree,
runs fail-closed source verification, and exports the exact modified A0 files as an
artifact. It never invokes git commit, push, update-ref, or any GitHub write API.
"""

from __future__ import annotations

import hashlib
import json
import os
from pathlib import Path
import py_compile
import re
import shutil
import subprocess
import sys
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
PLAN = ROOT / "plans" / "hepta-intelligence"
ARTIFACT = ROOT / "artifacts" / "a0-v43-reviewed-repair"
Q0_HEAD = "c768bcbeb4c1168088d2499828c24da521a2a73a"
SNAPSHOT_UTC = "2026-08-29T20:08:24Z"
MASTER_VERSION = "4.3.0"
SPEC_VERSION = "1.1.0"

READ_ORDER = [
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_CURRENT_PLAN.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRY_V1.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_EVIDENCE_INDEX_V1.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_CAPABILITY_REGISTRY_V1.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_PR_STACK_REGISTRY_V1.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_MASTER_PLAN.md",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_CONTROLLED_GAP_CLOSURE_EXECUTION_SPEC_V1.md",
]

CAPABILITY_LIFECYCLE = [
    "implemented",
    "candidate_qualified",
    "selected",
    "wired",
    "runtime_qualified",
    "efficacy_proven",
    "operator_accepted",
    "promoted",
]

GAP_IDS = [
    "A0-DOC-001",
    "A0-TRIGGER-001",
    "A0-EVIDENCE-001",
    "A0-REVIEW-001",
    "A0-MERGE-001",
    "B0-BOUNDARY-001",
    "C0-LEDGER-001",
    "M0-COORDINATOR-001",
    "J0-LIFECYCLE-001",
    "R1-RETRIEVAL-001",
    "R1-CORPUS-001",
    "N1-NEURON-001",
    "I1-INTUITION-001",
    "L1-LEARNING-001",
    "C1-CLOSED-LOOP-001",
    "EXT-HARDWARE-001",
    "EXT-OPERATOR-001",
]

GAP_CLASSES = [
    "CLOSED_SOURCE_CONTROLLED",
    "OPEN_SOURCE_CONTROLLED",
    "BLOCKED_EXTERNAL_EVIDENCE",
    "BLOCKED_UPSTREAM",
    "STOP_CONDITION",
]

CHECK_CLASSES = [
    "PASS",
    "INTRODUCED_BY_CANDIDATE",
    "PRE_EXISTING_ON_BASE",
    "MERGE_INTERACTION",
    "RUNNER_OR_PLATFORM_INFRA",
    "CANCELLED_OR_SUPERSEDED",
    "NOT_REQUIRED_BY_SELECTED_POLICY",
    "UNKNOWN_FAIL_CLOSED",
]

ALLOWED_PATHS = [
    ".github/workflows/hepta-intelligence-a0-authority.yml",
    ".github/workflows/hepta-intelligence-execution-spec.yml",
    ".github/workflows/hepta-intelligence-master-plan.yml",
    "plans/hepta-intelligence/AGENTS.md",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_CAPABILITY_REGISTRY_V1.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_CONTROLLED_GAP_CLOSURE_EXECUTION_SPEC_V1.md",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_CURRENT_PLAN.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRY_V1.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_EVIDENCE_INDEX_V1.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_MASTER_PLAN.md",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_PR_STACK_REGISTRY_V1.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_Q0_QUALIFICATION_RECEIPT_2026-08-29.json",
    "scripts/hepta-intelligence-current-truth.py",
    "scripts/verify-hepta-intelligence-a0-authority.py",
    "scripts/verify-hepta-intelligence-document-authority.py",
    "scripts/verify-hepta-intelligence-master-plan.py",
]

AUTHORITY_KEYS = [
    "callers_ratchet",
    "default_open_wired",
    "external_effects",
    "learning_write_authority",
    "memory_write_authority",
    "model_runtime_authority",
    "operator_acceptance",
    "outbox_dispatch_authority",
    "product_module_registered",
    "production_authority",
    "projection_write_authority",
    "promotion",
    "provider_dispatch_authority",
    "release_authority",
    "runtime_wired",
    "tool_registered",
]


def authority_false() -> dict[str, bool]:
    return {key: False for key in AUTHORITY_KEYS}


def load(path: Path) -> dict[str, Any]:
    value = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(value, dict):
        raise SystemExit(f"{path} must contain a JSON object")
    return value


def dump(path: Path, value: Any) -> None:
    path.write_text(
        json.dumps(value, indent=2, sort_keys=True, ensure_ascii=False) + "\n",
        encoding="utf-8",
    )


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def replace_required(text: str, old: str, new: str, label: str) -> str:
    if old not in text:
        raise SystemExit(f"missing required replacement marker {label}: {old!r}")
    return text.replace(old, new)


def scan_consumers(file_name: str) -> list[str]:
    consumers: list[str] = []
    roots = [ROOT / "scripts", ROOT / ".github" / "workflows", PLAN]
    excluded = {
        PLAN / "HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRY_V1.json",
    }
    for search_root in roots:
        if not search_root.exists():
            continue
        for path in search_root.rglob("*"):
            if not path.is_file() or path in excluded:
                continue
            if path.suffix not in {".py", ".yml", ".yaml", ".json", ".md"}:
                continue
            try:
                body = path.read_text(encoding="utf-8")
            except UnicodeDecodeError:
                continue
            if file_name in body:
                consumers.append(path.relative_to(ROOT).as_posix())
    return sorted(set(consumers))


def patch_master_and_spec() -> None:
    master_path = PLAN / "HEPTA_INTELLIGENCE_MASTER_PLAN.md"
    master = master_path.read_text(encoding="utf-8")
    marker = "## 20. V4.3 exact-head review-defect repair invariants"
    if marker not in master:
        master += f"""

---

{marker}

The following rules are normative for the A0 replacement that closes
`A0-RV-002` through `A0-RV-009`:

1. The mandatory bootstrap order is the exact eight-item list in section 1,
   including the subordinate execution specification as the final item. Its
   inclusion grants no current-plan or promotion authority.
2. Capability lifecycle is exactly `implemented → candidate_qualified →
   selected → wired → runtime_qualified → efficacy_proven → operator_accepted
   → promoted`; every current capability carries `selected=false`.
3. Repository-check attribution uses exactly: `{', '.join(CHECK_CLASSES)}`.
4. Checked-in source snapshot time must be RFC 3339 UTC and must not be later
   than the commit containing it. Live workflow observations never rewrite it.
5. Candidate-produced workflows may emit source and executable-mechanics
   evidence, but must keep `a0_candidate_qualified=false`. Candidate
   qualification requires a separate completed-run attestation that binds all
   required contexts, followed by a distinct reviewer and canonical selector.
6. Artifact consumers verify archive bytes, reject absolute paths, `..`,
   symlinks, duplicate extraction targets and non-regular entries before
   extraction.
7. Compatibility contracts and immutable tranche snapshots are content-sealed;
   registered consumers, schemas, digests and migration policy are validated.
8. Gap IDs, lifecycle fields, dependency lists, resume predicates, evidence
   bindings and all authority-negative invariants are exact, not prose-only.
"""
    master_path.write_text(master, encoding="utf-8")

    spec_path = PLAN / "HEPTA_INTELLIGENCE_CONTROLLED_GAP_CLOSURE_EXECUTION_SPEC_V1.md"
    spec = spec_path.read_text(encoding="utf-8")
    spec = replace_required(
        spec,
        "Canonical parent plan: `HEPTA_INTELLIGENCE_MASTER_PLAN_V4` version `4.2.0`",
        "Canonical parent plan: `HEPTA_INTELLIGENCE_MASTER_PLAN_V4` version `4.3.0`",
        "spec parent plan",
    )
    spec = spec.replace(
        "PASS_HEPTA_INTELLIGENCE_MASTER_PLAN_V4_2_SOURCE_ONLY",
        "PASS_HEPTA_INTELLIGENCE_MASTER_PLAN_V4_3_SOURCE_ONLY",
    )
    replacements = {
        "FAIL_INTRODUCED_BY_CANDIDATE": "INTRODUCED_BY_CANDIDATE",
        "FAIL_PRESENT_IN_BASE": "PRE_EXISTING_ON_BASE",
        "INFRASTRUCTURE_BLOCKED": "RUNNER_OR_PLATFORM_INFRA",
        "CANCELLED_SUPERSEDED": "CANCELLED_OR_SUPERSEDED",
        "NOT_REQUIRED_FOR_SELECTED_TARGET": "NOT_REQUIRED_BY_SELECTED_POLICY",
    }
    for old, new in replacements.items():
        spec = spec.replace(old, new)
    repair_marker = "## 16. V4.3 exact contract alignment"
    if repair_marker not in spec:
        spec += f"""

---

{repair_marker}

Mandatory bootstrap order:

```text
{chr(10).join(READ_ORDER)}
```

Capability lifecycle:

```text
{' → '.join(CAPABILITY_LIFECYCLE)}
```

Exact repository-check classifications:

```text
{chr(10).join(CHECK_CLASSES)}
```

A local candidate workflow is source/mechanics evidence only and must emit
`a0_candidate_qualified=false`. A separate completed-run attestor must observe
all required exact-head contexts and artifacts after their producing jobs are
terminal. Only that attestation, a distinct signed reviewer receipt and an
independent canonical-selection receipt can advance selection.

Before reading any artifact ZIP, consumers must hash the archive bytes against
its API digest, reject absolute or parent-traversal paths, symlinks, duplicate
normalized targets, unsupported file types and extraction outside the target
root. The attested artifact is the completed upstream artifact observed by the
sealing job; the sealing envelope is transport, not self-attestation.

Checked-in `SOURCE_SNAPSHOT` timestamps are deterministic RFC 3339 UTC and may
never postdate the commit that contains them. `LIVE_EVIDENCE` is separate and
never mutates source directly.
"""
    spec_path.write_text(spec, encoding="utf-8")


def write_current_truth_script() -> None:
    path = ROOT / "scripts" / "hepta-intelligence-current-truth.py"
    path.write_text(
        r'''#!/usr/bin/env python3
"""Emit and fail-closed verify deterministic Hepta Intelligence current truth."""

from __future__ import annotations

import argparse
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import re
import subprocess
import sys
from typing import Any, NoReturn

ROOT = Path(__file__).resolve().parents[1]
PLAN = ROOT / "plans" / "hepta-intelligence"
PATHS = {
    "current": PLAN / "HEPTA_INTELLIGENCE_CURRENT_PLAN.json",
    "document": PLAN / "HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRY_V1.json",
    "evidence": PLAN / "HEPTA_INTELLIGENCE_EVIDENCE_INDEX_V1.json",
    "capabilities": PLAN / "HEPTA_INTELLIGENCE_CAPABILITY_REGISTRY_V1.json",
    "pr_stack": PLAN / "HEPTA_INTELLIGENCE_PR_STACK_REGISTRY_V1.json",
    "integration": PLAN / "HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json",
    "master": PLAN / "HEPTA_INTELLIGENCE_MASTER_PLAN.md",
    "spec": PLAN / "HEPTA_INTELLIGENCE_CONTROLLED_GAP_CLOSURE_EXECUTION_SPEC_V1.md",
    "q0": PLAN / "HEPTA_INTELLIGENCE_Q0_QUALIFICATION_RECEIPT_2026-08-29.json",
}
READ_ORDER = [
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_CURRENT_PLAN.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRY_V1.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_EVIDENCE_INDEX_V1.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_CAPABILITY_REGISTRY_V1.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_PR_STACK_REGISTRY_V1.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_MASTER_PLAN.md",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_CONTROLLED_GAP_CLOSURE_EXECUTION_SPEC_V1.md",
]
LIFECYCLE = [
    "implemented", "candidate_qualified", "selected", "wired",
    "runtime_qualified", "efficacy_proven", "operator_accepted", "promoted",
]
GAP_IDS = [
    "A0-DOC-001", "A0-TRIGGER-001", "A0-EVIDENCE-001", "A0-REVIEW-001",
    "A0-MERGE-001", "B0-BOUNDARY-001", "C0-LEDGER-001",
    "M0-COORDINATOR-001", "J0-LIFECYCLE-001", "R1-RETRIEVAL-001",
    "R1-CORPUS-001", "N1-NEURON-001", "I1-INTUITION-001",
    "L1-LEARNING-001", "C1-CLOSED-LOOP-001", "EXT-HARDWARE-001",
    "EXT-OPERATOR-001",
]
GAP_CLASSES = {
    "CLOSED_SOURCE_CONTROLLED", "OPEN_SOURCE_CONTROLLED",
    "BLOCKED_EXTERNAL_EVIDENCE", "BLOCKED_UPSTREAM", "STOP_CONDITION",
}
CHECK_CLASSES = [
    "PASS", "INTRODUCED_BY_CANDIDATE", "PRE_EXISTING_ON_BASE",
    "MERGE_INTERACTION", "RUNNER_OR_PLATFORM_INFRA",
    "CANCELLED_OR_SUPERSEDED", "NOT_REQUIRED_BY_SELECTED_POLICY",
    "UNKNOWN_FAIL_CLOSED",
]
SCHEMAS = {
    "current": "hepta_intelligence_current_plan_v2",
    "document": "hepta_intelligence_document_authority_registry_v1",
    "evidence": "hepta_intelligence_evidence_index_v1",
    "capabilities": "hepta_intelligence_capability_registry_v1",
    "pr_stack": "hepta_intelligence_pr_stack_registry_v1",
    "integration": "hepta_intelligence_integration_candidate_v1",
    "q0": "hepta_intelligence_q0_external_evidence_summary_v1",
}
HEX64 = re.compile(r"[0-9a-f]{64}")


def fail(message: str) -> NoReturn:
    raise SystemExit(f"FAIL_HEPTA_INTELLIGENCE_CURRENT_TRUTH: {message}")


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def load(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except Exception as exc:
        fail(f"cannot parse {path.relative_to(ROOT)}: {exc}")
    require(isinstance(value, dict), f"{path.relative_to(ROOT)} must contain object")
    return value


def canonical(value: Any) -> bytes:
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode()


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def all_false(value: Any) -> bool:
    return isinstance(value, dict) and bool(value) and all(item is False for item in value.values())


def parse_utc(value: Any, label: str) -> datetime:
    require(isinstance(value, str) and value.endswith("Z"), f"{label} must be UTC Z time")
    try:
        parsed = datetime.fromisoformat(value[:-1] + "+00:00")
    except ValueError as exc:
        fail(f"{label} invalid: {exc}")
    require(parsed.tzinfo is not None, f"{label} lacks timezone")
    return parsed.astimezone(timezone.utc)


def validate_timestamp_not_future(snapshot: datetime) -> None:
    git_dir = ROOT / ".git"
    if not git_dir.exists():
        return
    raw = subprocess.check_output(
        ["git", "show", "-s", "--format=%cI", "HEAD"], cwd=ROOT, text=True
    ).strip()
    commit_time = datetime.fromisoformat(raw.replace("Z", "+00:00")).astimezone(timezone.utc)
    require(snapshot <= commit_time, "source snapshot postdates containing commit")


def validate_capability(entry: Any) -> None:
    require(isinstance(entry, dict), "capability entry must be object")
    cid = entry.get("capability_id")
    require(isinstance(cid, str) and cid, "capability_id missing")
    for field in LIFECYCLE:
        require(type(entry.get(field)) is bool, f"{cid}.{field} must be bool")
    require(not entry["candidate_qualified"] or entry["implemented"], f"{cid}: candidate without implementation")
    require(not entry["selected"] or entry["candidate_qualified"], f"{cid}: selected without qualification")
    require(not entry["wired"] or entry["selected"], f"{cid}: wired without selection")
    require(not entry["runtime_qualified"] or entry["wired"], f"{cid}: runtime-qualified without wiring")
    require(not entry["efficacy_proven"] or entry["runtime_qualified"], f"{cid}: efficacy without runtime")
    require(not entry["operator_accepted"] or entry["efficacy_proven"], f"{cid}: operator acceptance without efficacy")
    require(not entry["promoted"] or entry["operator_accepted"], f"{cid}: promotion without acceptance")
    require(all_false(entry.get("authority")), f"{cid}: authority must remain false")


def validate_and_build() -> dict[str, Any]:
    for path in PATHS.values():
        require(path.is_file(), f"missing {path.relative_to(ROOT)}")
    docs = {key: load(path) for key, path in PATHS.items() if path.suffix == ".json"}
    for key, schema in SCHEMAS.items():
        require(docs[key].get("schema") == schema, f"{key} schema mismatch")

    current = docs["current"]
    document = docs["document"]
    evidence = docs["evidence"]
    capabilities = docs["capabilities"]
    pr_stack = docs["pr_stack"]
    integration = docs["integration"]
    q0 = docs["q0"]
    repository = "ProfHepta/hepta-private-ci"
    for label, value in docs.items():
        if label == "q0":
            continue
        require(value.get("repository") == repository, f"{label} repository mismatch")
    require(q0.get("evidence_observation", {}).get("repository", {}).get("full_name") == repository, "Q0 repository mismatch")

    canonical_plan = current.get("canonical", {})
    require(canonical_plan.get("plan_id") == "HEPTA_INTELLIGENCE_MASTER_PLAN_V4", "plan id mismatch")
    require(canonical_plan.get("plan_version") == "4.3.0", "plan version mismatch")
    require(canonical_plan.get("content_sha256") == sha256(PATHS["master"]), "master digest mismatch")
    operational = current.get("operational_execution", {})
    require(operational.get("execution_spec_version") == "1.1.0", "spec version mismatch")
    require(operational.get("execution_spec_sha256") == sha256(PATHS["spec"]), "spec digest mismatch")
    require(current.get("session_bootstrap", {}).get("read_order") == READ_ORDER, "mandatory read order mismatch")

    snapshot = parse_utc(current.get("generated_at_utc"), "current.generated_at_utc")
    require(parse_utc(document.get("as_of_utc"), "document.as_of_utc") == snapshot, "document timestamp drift")
    require(parse_utc(integration.get("gap_closure_ledger", {}).get("as_of_utc"), "ledger.as_of_utc") == snapshot, "ledger timestamp drift")
    validate_timestamp_not_future(snapshot)

    q0_copy = dict(q0)
    observed_binding = q0_copy.pop("receipt_binding_sha256", None)
    require(isinstance(observed_binding, str) and HEX64.fullmatch(observed_binding) is not None, "Q0 binding shape invalid")
    require(observed_binding == hashlib.sha256(canonical(q0_copy)).hexdigest(), "Q0 receipt binding mismatch")
    q0_obs = q0.get("evidence_observation", {})
    require(q0_obs.get("head") == "c768bcbeb4c1168088d2499828c24da521a2a73a", "Q0 head mismatch")
    require(q0_obs.get("tree") == "ca455a9ef797cd95164c880c7b8faba80b305589", "Q0 tree mismatch")
    require(q0.get("conclusion", {}).get("qualified_candidate") is True, "Q0 not qualified")
    require(q0.get("conclusion", {}).get("runtime_capability_qualified") is False, "Q0 crossed runtime boundary")
    require(all_false(q0.get("authority")), "Q0 authority must remain false")
    indexed_q0 = evidence.get("q0_current_evidence", {})
    require(indexed_q0.get("summary_file_sha256") == sha256(PATHS["q0"]), "evidence/Q0 cross-hash mismatch")
    require(indexed_q0.get("q0_executable_qualified") is True, "evidence lost Q0 qualification")

    require(capabilities.get("lifecycle") == LIFECYCLE, "capability lifecycle mismatch")
    entries = capabilities.get("capabilities")
    require(isinstance(entries, list) and entries, "capability registry empty")
    for entry in entries:
        validate_capability(entry)
    ids = [entry["capability_id"] for entry in entries]
    require(len(ids) == len(set(ids)), "duplicate capability ID")
    capability_map = {entry["capability_id"]: entry for entry in entries}
    a0 = capability_map.get("A0_CANONICAL_CAPABILITY_AND_EVIDENCE_AUTHORITY", {})
    require(a0.get("candidate_qualified") is False and a0.get("selected") is False, "A0 self-qualified or self-selected")

    registered_inputs = document.get("registered_canonical_inputs")
    require(isinstance(registered_inputs, list) and len(registered_inputs) == 4, "registered canonical inputs incomplete")
    expected_input_paths = {
        "plans/hepta-intelligence/HEPTA_INTELLIGENCE_EVIDENCE_INDEX_V1.json": PATHS["evidence"],
        "plans/hepta-intelligence/HEPTA_INTELLIGENCE_CAPABILITY_REGISTRY_V1.json": PATHS["capabilities"],
        "plans/hepta-intelligence/HEPTA_INTELLIGENCE_PR_STACK_REGISTRY_V1.json": PATHS["pr_stack"],
        "plans/hepta-intelligence/HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json": PATHS["integration"],
    }
    observed_paths = {item.get("path") for item in registered_inputs if isinstance(item, dict)}
    require(observed_paths == set(expected_input_paths), "registered canonical input path drift")
    for item in registered_inputs:
        rel = item["path"]
        require(item.get("content_sha256") == sha256(expected_input_paths[rel]), f"registered input digest drift: {rel}")
        require(item.get("current_plan_authority") is False, f"registered input gained authority: {rel}")

    require(all_false(current.get("authority")), "current authority must remain false")
    require(all_false(document.get("authority")), "document authority must remain false")
    require(all_false(evidence.get("authority")), "evidence authority must remain false")
    require(all_false(capabilities.get("authority")), "capability authority must remain false")
    require(all_false(pr_stack.get("authority")), "PR stack authority must remain false")
    require(all_false(integration.get("authority")), "integration authority must remain false")

    ledger = integration.get("gap_closure_ledger", {})
    require(ledger.get("schema") == "hepta_intelligence_gap_closure_ledger_v1", "gap ledger schema mismatch")
    ledger_entries = ledger.get("entries")
    require(isinstance(ledger_entries, list) and len(ledger_entries) == len(GAP_IDS), "gap ledger cardinality mismatch")
    observed_gap_ids = [entry.get("gap_id") for entry in ledger_entries if isinstance(entry, dict)]
    require(observed_gap_ids == GAP_IDS, "gap IDs/order mismatch")
    require(len(observed_gap_ids) == len(set(observed_gap_ids)), "duplicate gap ID")
    for entry in ledger_entries:
        gid = entry["gap_id"]
        require(entry.get("classification") in GAP_CLASSES, f"{gid}: illegal classification")
        for field in ("title", "owner_class", "status", "closure_evidence", "next_action", "authority_effect", "dependencies", "resume_predicate"):
            require(field in entry, f"{gid}: missing {field}")
        require(isinstance(entry["dependencies"], list), f"{gid}: dependencies must be list")
        require(all(dep in GAP_IDS for dep in entry["dependencies"]), f"{gid}: unknown dependency")
        require(gid not in entry["dependencies"], f"{gid}: self dependency")
        require(isinstance(entry["closure_evidence"], list) and entry["closure_evidence"], f"{gid}: empty closure evidence")
    invariants = ledger.get("invariants", {})
    require(invariants.get("external_evidence_may_be_synthesized") is False, "external evidence synthesis enabled")
    require(invariants.get("unknown_gap_fails_closed") is True, "unknown gaps do not fail closed")

    check_contract = integration.get("repository_check_attribution_contract", {})
    require(check_contract.get("classifications") == CHECK_CLASSES, "repository-check classification drift")
    require(check_contract.get("unknown_fails_closed") is True, "unknown check does not fail closed")
    require(check_contract.get("candidate_workflow_may_self_qualify") is False, "candidate workflow can self-qualify")

    source_truth = current.get("current_truth", {})
    require(source_truth.get("a0_candidate_qualified") is False, "source snapshot self-qualified A0")
    require(source_truth.get("selected") is False, "source snapshot self-selected A0")
    require(source_truth.get("full_repository_merge_green") is False, "source fabricated merge-green")
    claims = current.get("claim_levels", {})
    for key in ("self_evolution", "closed_loop_learning", "structural_plasticity", "neuromorphic_mechanism", "local_small_model_used_by_h5", "local_small_model_used_by_h6"):
        require(claims.get(key) is False, f"positive unsupported claim: {key}")

    return {
        "schema": "hepta_intelligence_current_truth_v1",
        "repository": repository,
        "source_snapshot": {
            "as_of_utc": current.get("generated_at_utc"),
            "classification": "SOURCE_SNAPSHOT_NOT_LIVE_CI",
            "future_dated": False,
        },
        "plan": {
            "id": canonical_plan.get("plan_id"),
            "version": canonical_plan.get("plan_version"),
            "content_sha256": canonical_plan.get("content_sha256"),
            "spec_version": operational.get("execution_spec_version"),
            "spec_sha256": operational.get("execution_spec_sha256"),
            "read_order": READ_ORDER,
        },
        "active_phase": current.get("active_phase"),
        "q0": {
            "candidate": {key: q0_obs.get(key) for key in ("branch", "head", "tree", "parent")},
            "run_id": q0_obs.get("run_id"),
            "qualified_candidate": True,
            "runtime_capability_qualified": False,
        },
        "capability_lifecycle": LIFECYCLE,
        "capabilities": capability_map,
        "integration_candidate": {
            "candidate_id": integration.get("candidate_id"),
            "branch": integration.get("branch"),
            "expected_parent": integration.get("expected_parent"),
            "allowed_changed_paths": integration.get("allowed_changed_paths"),
            "gap_ids": observed_gap_ids,
            "repository_check_classifications": CHECK_CLASSES,
        },
        "claims": claims,
        "current_truth": source_truth,
        "input_sha256": {key: sha256(path) for key, path in PATHS.items()},
        "authority": current.get("authority"),
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--verify", action="store_true")
    parser.add_argument("--compact", action="store_true")
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    truth = validate_and_build()
    if args.verify:
        print("PASS_HEPTA_INTELLIGENCE_CURRENT_TRUTH_V1")
        return 0
    encoded = (
        canonical(truth) + b"\n"
        if args.compact
        else (json.dumps(truth, indent=2, sort_keys=True, ensure_ascii=False) + "\n").encode()
    )
    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_bytes(encoded)
    else:
        sys.stdout.buffer.write(encoded)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
''',
        encoding="utf-8",
    )
    path.chmod(0o755)


def write_master_verifier() -> None:
    path = ROOT / "scripts" / "verify-hepta-intelligence-master-plan.py"
    path.write_text(
        r'''#!/usr/bin/env python3
"""Verify exact V4.3 master/spec/read-order/lifecycle alignment."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path
import re
import sys
from typing import Any, NoReturn

ROOT = Path(__file__).resolve().parents[1]
PLAN = ROOT / "plans" / "hepta-intelligence"
MASTER = PLAN / "HEPTA_INTELLIGENCE_MASTER_PLAN.md"
SPEC = PLAN / "HEPTA_INTELLIGENCE_CONTROLLED_GAP_CLOSURE_EXECUTION_SPEC_V1.md"
CURRENT = PLAN / "HEPTA_INTELLIGENCE_CURRENT_PLAN.json"
AGENTS = PLAN / "AGENTS.md"
READ_ORDER = [
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_CURRENT_PLAN.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRY_V1.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_EVIDENCE_INDEX_V1.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_CAPABILITY_REGISTRY_V1.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_PR_STACK_REGISTRY_V1.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_MASTER_PLAN.md",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_CONTROLLED_GAP_CLOSURE_EXECUTION_SPEC_V1.md",
]
LIFECYCLE = [
    "implemented", "candidate_qualified", "selected", "wired",
    "runtime_qualified", "efficacy_proven", "operator_accepted", "promoted",
]
CHECK_CLASSES = [
    "PASS", "INTRODUCED_BY_CANDIDATE", "PRE_EXISTING_ON_BASE",
    "MERGE_INTERACTION", "RUNNER_OR_PLATFORM_INFRA",
    "CANCELLED_OR_SUPERSEDED", "NOT_REQUIRED_BY_SELECTED_POLICY",
    "UNKNOWN_FAIL_CLOSED",
]
OLD_CHECK_CLASSES = [
    "FAIL_INTRODUCED_BY_CANDIDATE", "FAIL_PRESENT_IN_BASE",
    "INFRASTRUCTURE_BLOCKED", "CANCELLED_SUPERSEDED",
    "NOT_REQUIRED_FOR_SELECTED_TARGET",
]


def fail(message: str) -> NoReturn:
    raise SystemExit(f"FAIL_HEPTA_INTELLIGENCE_MASTER_PLAN_V4_3: {message}")


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def load(path: Path) -> dict[str, Any]:
    value = json.loads(path.read_text(encoding="utf-8"))
    require(isinstance(value, dict), f"{path.name} must contain object")
    return value


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def ordered_contains(text: str, values: list[str], label: str) -> None:
    cursor = -1
    for value in values:
        index = text.find(value, cursor + 1)
        require(index >= 0, f"{label} missing {value}")
        require(index > cursor, f"{label} order drift at {value}")
        cursor = index


def main() -> int:
    for path in (MASTER, SPEC, CURRENT, AGENTS):
        require(path.is_file(), f"missing {path.relative_to(ROOT)}")
    current = load(CURRENT)
    master = MASTER.read_text(encoding="utf-8")
    spec = SPEC.read_text(encoding="utf-8")
    agents = AGENTS.read_text(encoding="utf-8")
    canonical = current.get("canonical", {})
    require(canonical.get("plan_id") == "HEPTA_INTELLIGENCE_MASTER_PLAN_V4", "plan id drift")
    require(canonical.get("plan_version") == "4.3.0", "plan version drift")
    require(canonical.get("content_sha256") == sha256(MASTER), "master digest mismatch")
    operational = current.get("operational_execution", {})
    require(operational.get("execution_spec_version") == "1.1.0", "spec version drift")
    require(operational.get("execution_spec_sha256") == sha256(SPEC), "spec digest mismatch")
    require(current.get("session_bootstrap", {}).get("read_order") == READ_ORDER, "machine read order drift")
    ordered_contains(master, [Path(item).name for item in READ_ORDER], "master read order")
    ordered_contains(agents, [Path(item).name for item in READ_ORDER], "AGENTS read order")
    ordered_contains(spec, READ_ORDER, "spec read order")
    require("Version: `4.3.0`" in master, "master header version drift")
    require("version `4.3.0`" in spec, "spec parent version drift")
    require("PASS_HEPTA_INTELLIGENCE_MASTER_PLAN_V4_3_SOURCE_ONLY" in spec, "spec PASS marker drift")
    require("PASS_HEPTA_INTELLIGENCE_MASTER_PLAN_V4_2_SOURCE_ONLY" not in spec, "stale V4.2 PASS marker")
    for value in CHECK_CLASSES:
        require(value in master and value in spec, f"check class missing: {value}")
    for value in OLD_CHECK_CLASSES:
        require(value not in spec, f"stale check class remains: {value}")
    lifecycle_line = " → ".join(LIFECYCLE)
    require(lifecycle_line in master, "master lifecycle drift")
    require(lifecycle_line in spec, "spec lifecycle drift")
    required_markers = [
        "CANONICAL_CURRENT / PLAN_ONLY / FAIL_CLOSED",
        "A0IndependentReviewReceiptV1",
        "CanonicalSelectionReceiptV1",
        "RepositoryCheckAttributionReceiptV1",
        "SOURCE_SNAPSHOT",
        "LIVE_EVIDENCE",
        "candidate_workflow_may_self_qualify",
        "a0_candidate_qualified=false",
        "archive bytes",
        "UNKNOWN_FAIL_CLOSED",
        "selected",
    ]
    combined = master + "\n" + spec
    for marker in required_markers:
        require(marker in combined, f"required marker missing: {marker}")
    for marker in (
        "self_evolution=true", "closed_loop_learning=true",
        "structural_plasticity=true", "neuromorphic_mechanism=true",
        "local_small_model_used_by_h5=true", "local_small_model_used_by_h6=true",
    ):
        require(marker not in combined, f"unsupported positive claim: {marker}")
    candidates = sorted(PLAN.glob("HEPTA_INTELLIGENCE_*PLAN*.md"))
    canonical_docs = [p for p in candidates if "CANONICAL_CURRENT" in p.read_text(encoding="utf-8")]
    require(canonical_docs == [MASTER], "canonical human plan is not unique")
    print("PASS_HEPTA_INTELLIGENCE_MASTER_PLAN_V4_3_SOURCE_ONLY")
    return 0


if __name__ == "__main__":
    sys.exit(main())
''',
        encoding="utf-8",
    )
    path.chmod(0o755)


def write_document_verifier() -> None:
    path = ROOT / "scripts" / "verify-hepta-intelligence-document-authority.py"
    path.write_text(
        r'''#!/usr/bin/env python3
"""Verify document authority, compatibility seals, snapshots and consumers."""

from __future__ import annotations

from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import subprocess
import sys
from typing import Any, NoReturn

ROOT = Path(__file__).resolve().parents[1]
PLAN = ROOT / "plans" / "hepta-intelligence"
CURRENT = PLAN / "HEPTA_INTELLIGENCE_CURRENT_PLAN.json"
MASTER = PLAN / "HEPTA_INTELLIGENCE_MASTER_PLAN.md"
SPEC = PLAN / "HEPTA_INTELLIGENCE_CONTROLLED_GAP_CLOSURE_EXECUTION_SPEC_V1.md"
REGISTRY = PLAN / "HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRY_V1.json"
EVIDENCE = PLAN / "HEPTA_INTELLIGENCE_EVIDENCE_INDEX_V1.json"
CAPABILITIES = PLAN / "HEPTA_INTELLIGENCE_CAPABILITY_REGISTRY_V1.json"
PR_STACK = PLAN / "HEPTA_INTELLIGENCE_PR_STACK_REGISTRY_V1.json"
INTEGRATION = PLAN / "HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json"
Q0 = PLAN / "HEPTA_INTELLIGENCE_Q0_QUALIFICATION_RECEIPT_2026-08-29.json"
CLAIMS = PLAN / "HEPTA_INTELLIGENCE_CLAIM_LADDER_V1.json"
LEGACY = {
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_EXECUTION_STATUS_V2.json": "hepta_intelligence_execution_status_v2",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_EXECUTION_STATUS_V3.json": "hepta_intelligence_execution_status_v3",
}
READ_ORDER = [
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_CURRENT_PLAN.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRY_V1.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_EVIDENCE_INDEX_V1.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_CAPABILITY_REGISTRY_V1.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_PR_STACK_REGISTRY_V1.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_MASTER_PLAN.md",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_CONTROLLED_GAP_CLOSURE_EXECUTION_SPEC_V1.md",
]


def fail(message: str) -> NoReturn:
    raise SystemExit(f"FAIL_HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY: {message}")


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def load(path: Path) -> dict[str, Any]:
    value = json.loads(path.read_text(encoding="utf-8"))
    require(isinstance(value, dict), f"{path.name} must contain object")
    return value


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def all_false(value: Any) -> bool:
    return isinstance(value, dict) and bool(value) and all(item is False for item in value.values())


def utc(value: Any, label: str) -> datetime:
    require(isinstance(value, str) and value.endswith("Z"), f"{label} must be UTC")
    return datetime.fromisoformat(value[:-1] + "+00:00").astimezone(timezone.utc)


def main() -> int:
    for path in (CURRENT, MASTER, SPEC, REGISTRY, EVIDENCE, CAPABILITIES, PR_STACK, INTEGRATION, Q0, CLAIMS):
        require(path.is_file(), f"missing {path.relative_to(ROOT)}")
    current = load(CURRENT)
    registry = load(REGISTRY)
    require(registry.get("schema") == "hepta_intelligence_document_authority_registry_v1", "registry schema drift")
    require(registry.get("repository") == "ProfHepta/hepta-private-ci", "repository drift")
    require(all_false(registry.get("authority")), "registry authority must remain false")
    require(current.get("session_bootstrap", {}).get("read_order") == READ_ORDER, "read order drift")
    human = registry.get("current_authority", {}).get("human", {})
    require(human.get("path") == MASTER.relative_to(ROOT).as_posix(), "human authority path drift")
    require(human.get("plan_version") == "4.3.0", "human authority version drift")
    require(human.get("content_sha256") == sha256(MASTER), "human authority digest drift")
    machine = registry.get("current_authority", {}).get("machine", {})
    require(machine.get("path") == CURRENT.relative_to(ROOT).as_posix(), "machine authority path drift")
    operational = registry.get("registered_operational_documents")
    require(isinstance(operational, list) and len(operational) == 1, "operational document registry drift")
    op = operational[0]
    require(op.get("path") == SPEC.relative_to(ROOT).as_posix(), "spec path drift")
    require(op.get("version") == "1.1.0", "spec version drift")
    require(op.get("content_sha256") == sha256(SPEC), "spec digest drift")
    require(op.get("current_plan_authority") is False and op.get("promotion_authority") is False, "spec gained authority")

    expected_inputs = {
        EVIDENCE.relative_to(ROOT).as_posix(): EVIDENCE,
        CAPABILITIES.relative_to(ROOT).as_posix(): CAPABILITIES,
        PR_STACK.relative_to(ROOT).as_posix(): PR_STACK,
        INTEGRATION.relative_to(ROOT).as_posix(): INTEGRATION,
    }
    registered_inputs = registry.get("registered_canonical_inputs")
    require(isinstance(registered_inputs, list) and len(registered_inputs) == 4, "registered input surface incomplete")
    require({item.get("path") for item in registered_inputs} == set(expected_inputs), "registered input paths drift")
    for item in registered_inputs:
        rel = item["path"]
        require(item.get("content_sha256") == sha256(expected_inputs[rel]), f"registered input digest drift: {rel}")
        require(item.get("current_plan_authority") is False, f"registered input gained authority: {rel}")

    contracts = registry.get("compatibility_contracts")
    require(isinstance(contracts, list) and len(contracts) == 2, "compatibility contracts incomplete")
    by_path = {item.get("path"): item for item in contracts if isinstance(item, dict)}
    require(set(by_path) == set(LEGACY), "compatibility path set drift")
    for rel, schema in LEGACY.items():
        path = ROOT / rel
        item = by_path[rel]
        value = load(path)
        require(value.get("schema") == schema, f"legacy schema drift: {rel}")
        require(all_false(value.get("authority")), f"legacy authority drift: {rel}")
        require(item.get("schema") == schema, f"registered schema drift: {rel}")
        require(item.get("content_sha256") == sha256(path), f"compatibility digest drift: {rel}")
        require(item.get("current_authority") is False and item.get("schema_preserved") is True, f"compatibility authority/state drift: {rel}")
        consumers = item.get("registered_consumers")
        require(isinstance(consumers, list) and consumers, f"compatibility consumers missing: {rel}")
        for consumer in consumers:
            require((ROOT / consumer).is_file(), f"registered consumer missing: {consumer}")
            require(Path(rel).name in (ROOT / consumer).read_text(encoding="utf-8"), f"consumer no longer references contract: {consumer}")

    snapshots = registry.get("compatibility_snapshots")
    disk_snapshots = sorted((PLAN / "status-snapshots").glob("*.json"))
    require(isinstance(snapshots, list) and len(snapshots) == len(disk_snapshots), "snapshot registry cardinality drift")
    snapshot_map = {item.get("path"): item for item in snapshots if isinstance(item, dict)}
    require(set(snapshot_map) == {path.relative_to(ROOT).as_posix() for path in disk_snapshots}, "snapshot path set drift")
    for path in disk_snapshots:
        rel = path.relative_to(ROOT).as_posix()
        value = load(path)
        item = snapshot_map[rel]
        require(item.get("content_sha256") == sha256(path), f"snapshot digest drift: {rel}")
        require(item.get("schema") == value.get("schema"), f"snapshot schema drift: {rel}")
        require(item.get("snapshot_id") == value.get("snapshot_id"), f"snapshot id drift: {rel}")
        require(value.get("current_authority") is False, f"snapshot gained authority: {rel}")
        require(all_false(value.get("authority")), f"snapshot authority drift: {rel}")

    claims = load(CLAIMS)
    require(claims.get("schema") == "hepta_intelligence_claim_ladder_v1", "claim ladder schema drift")
    require(claims.get("current_state_authority") is False, "claim ladder gained current authority")
    require(all_false(claims.get("authority")), "claim ladder authority drift")
    for consumer in registry.get("registered_consumers", []):
        require((ROOT / consumer).is_file(), f"registered current consumer missing: {consumer}")

    source_time = utc(current.get("generated_at_utc"), "current generated time")
    require(utc(registry.get("as_of_utc"), "registry time") == source_time, "registry/current time drift")
    if (ROOT / ".git").exists():
        raw = subprocess.check_output(["git", "show", "-s", "--format=%cI", "HEAD"], cwd=ROOT, text=True).strip()
        commit_time = datetime.fromisoformat(raw.replace("Z", "+00:00")).astimezone(timezone.utc)
        require(source_time <= commit_time, "source snapshot postdates containing commit")
    print("PASS_HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRIES_AND_COMPATIBILITY")
    return 0


if __name__ == "__main__":
    sys.exit(main())
''',
        encoding="utf-8",
    )
    path.chmod(0o755)


def write_a0_verifier() -> None:
    path = ROOT / "scripts" / "verify-hepta-intelligence-a0-authority.py"
    path.write_text(
        r'''#!/usr/bin/env python3
"""Fail-closed A0 source verifier and adversarial self-test."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
from typing import Any, NoReturn

ROOT = Path(__file__).resolve().parents[1]
PLAN = ROOT / "plans" / "hepta-intelligence"
CURRENT = PLAN / "HEPTA_INTELLIGENCE_CURRENT_PLAN.json"
INTEGRATION = PLAN / "HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json"
TRUTH = ROOT / "scripts" / "hepta-intelligence-current-truth.py"
MASTER_VERIFY = ROOT / "scripts" / "verify-hepta-intelligence-master-plan.py"
DOC_VERIFY = ROOT / "scripts" / "verify-hepta-intelligence-document-authority.py"
EXPECTED_REPOSITORY = "ProfHepta/hepta-private-ci"
EXPECTED_BRANCH = "codex/hepta-intelligence-a0-authority-gap-closure-20260829"
EXPECTED_PARENT = "c768bcbeb4c1168088d2499828c24da521a2a73a"


def fail(message: str) -> NoReturn:
    raise SystemExit(f"FAIL_HEPTA_INTELLIGENCE_A0_AUTHORITY: {message}")


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def load(path: Path) -> dict[str, Any]:
    value = json.loads(path.read_text(encoding="utf-8"))
    require(isinstance(value, dict), f"{path.name} must contain object")
    return value


def all_false(value: Any) -> bool:
    return isinstance(value, dict) and bool(value) and all(item is False for item in value.values())


def git(*args: str) -> str:
    return subprocess.check_output(["git", *args], cwd=ROOT, text=True).strip()


def expect_failure(command: list[str], cwd: Path, label: str) -> None:
    result = subprocess.run(command, cwd=cwd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
    if result.returncode == 0:
        raise SystemExit(f"FAIL_A0_SELF_TEST_ACCEPTED_DRIFT: {label}\n{result.stdout}")


def self_test() -> int:
    cases: list[tuple[str, str]] = [
        ("read_order", "current"),
        ("selected", "capabilities"),
        ("gap_id", "integration"),
        ("q0_binding", "q0"),
        ("compat_digest", "document"),
        ("spec_parent", "spec"),
    ]
    for label, target in cases:
        with tempfile.TemporaryDirectory(prefix=f"a0-negative-{label}-") as tmp_raw:
            tmp = Path(tmp_raw)
            shutil.copytree(ROOT / "plans", tmp / "plans")
            shutil.copytree(ROOT / "scripts", tmp / "scripts")
            if target == "current":
                path = tmp / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_CURRENT_PLAN.json"
                value = load(path)
                value["session_bootstrap"]["read_order"] = value["session_bootstrap"]["read_order"][:-1]
                path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")
                command = [sys.executable, str(tmp / "scripts/hepta-intelligence-current-truth.py"), "--verify"]
            elif target == "capabilities":
                path = tmp / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_CAPABILITY_REGISTRY_V1.json"
                value = load(path)
                value["capabilities"][0].pop("selected")
                path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")
                command = [sys.executable, str(tmp / "scripts/hepta-intelligence-current-truth.py"), "--verify"]
            elif target == "integration":
                path = tmp / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json"
                value = load(path)
                value["gap_closure_ledger"]["entries"][1]["gap_id"] = value["gap_closure_ledger"]["entries"][0]["gap_id"]
                path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")
                command = [sys.executable, str(tmp / "scripts/hepta-intelligence-current-truth.py"), "--verify"]
            elif target == "q0":
                path = tmp / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_Q0_QUALIFICATION_RECEIPT_2026-08-29.json"
                value = load(path)
                value["receipt_binding_sha256"] = "0" * 64
                path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")
                command = [sys.executable, str(tmp / "scripts/hepta-intelligence-current-truth.py"), "--verify"]
            elif target == "document":
                path = tmp / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRY_V1.json"
                value = load(path)
                value["compatibility_contracts"][0]["content_sha256"] = "0" * 64
                path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")
                command = [sys.executable, str(tmp / "scripts/verify-hepta-intelligence-document-authority.py")]
            else:
                path = tmp / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_CONTROLLED_GAP_CLOSURE_EXECUTION_SPEC_V1.md"
                body = path.read_text(encoding="utf-8").replace("version `4.3.0`", "version `4.2.0`", 1)
                path.write_text(body, encoding="utf-8")
                command = [sys.executable, str(tmp / "scripts/verify-hepta-intelligence-master-plan.py")]
            expect_failure(command, tmp, label)
    print("PASS_HEPTA_INTELLIGENCE_A0_NEGATIVE_SELF_TESTS")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if args.self_test:
        return self_test()
    for command in (
        [sys.executable, str(MASTER_VERIFY)],
        [sys.executable, str(DOC_VERIFY)],
        [sys.executable, str(TRUTH), "--verify"],
    ):
        subprocess.run(command, cwd=ROOT, check=True)
    current = load(CURRENT)
    integration = load(INTEGRATION)
    require(current.get("active_phase", {}).get("id") == "A0", "A0 is not active")
    require(all_false(current.get("authority")), "current authority must remain false")
    require(integration.get("expected_parent") == EXPECTED_PARENT, "expected parent drift")
    allowed = integration.get("allowed_changed_paths")
    require(isinstance(allowed, list) and allowed == sorted(set(allowed)) and len(allowed) == 17, "allowlist invalid")
    require(integration.get("repository_check_attribution_contract", {}).get("candidate_workflow_may_self_qualify") is False, "candidate workflow self-qualification enabled")
    changed: list[str] = []
    head = tree = None
    if (ROOT / ".git").exists() and os.environ.get("HEPTA_A0_FACTORY") != "1":
        head = git("rev-parse", "HEAD")
        tree = git("rev-parse", "HEAD^{tree}")
        parent = git("rev-parse", "HEAD^")
        require(parent == EXPECTED_PARENT, f"exact parent mismatch: {parent}")
        changed = sorted(filter(None, git("diff", "--name-only", "HEAD^", "HEAD").splitlines()))
        require(changed == allowed, f"changed-path surface mismatch: {changed}")
        branch = os.environ.get("GITHUB_HEAD_REF") or os.environ.get("GITHUB_REF_NAME")
        if branch:
            require(branch == EXPECTED_BRANCH, f"branch mismatch: {branch}")
        repo = os.environ.get("GITHUB_REPOSITORY")
        if repo:
            require(repo == EXPECTED_REPOSITORY, f"repository mismatch: {repo}")
        env_sha = os.environ.get("GITHUB_SHA")
        if env_sha:
            require(env_sha == head, "GITHUB_SHA mismatch")
        subprocess.run(["git", "diff", "--check", "HEAD^", "HEAD"], cwd=ROOT, check=True)
    compact_a = subprocess.check_output([sys.executable, str(TRUTH), "--compact"], cwd=ROOT)
    compact_b = subprocess.check_output([sys.executable, str(TRUTH), "--compact"], cwd=ROOT)
    require(compact_a == compact_b, "current truth is not byte deterministic")
    receipt = {
        "schema": "hepta_intelligence_a0_source_gate_receipt_v2",
        "status": "PASS_HEPTA_INTELLIGENCE_A0_SOURCE_ONLY",
        "repository": EXPECTED_REPOSITORY,
        "candidate": {"branch": EXPECTED_BRANCH, "head": head, "tree": tree, "parent": EXPECTED_PARENT},
        "changed_files": changed or allowed,
        "current_truth_sha256": hashlib.sha256(compact_a).hexdigest(),
        "source_writeback": False,
        "a0_source_validated": True,
        "a0_candidate_qualified": False,
        "selected": False,
        "full_repository_merge_green": False,
        "required_sibling_contexts_bound": False,
        "authority": current.get("authority"),
    }
    print(json.dumps(receipt, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    sys.exit(main())
''',
        encoding="utf-8",
    )
    path.chmod(0o755)


def write_workflows() -> None:
    a0 = ROOT / ".github" / "workflows" / "hepta-intelligence-a0-authority.yml"
    a0.write_text(
        r'''name: Hepta Intelligence A0 source execution

on:
  push:
    branches:
      - "codex/hepta-intelligence-a0-authority-gap-closure-*"
  pull_request:
    paths:
      - ".github/workflows/hepta-intelligence-a0-authority.yml"
      - ".github/workflows/hepta-intelligence-execution-spec.yml"
      - ".github/workflows/hepta-intelligence-master-plan.yml"
      - "plans/hepta-intelligence/**"
      - "scripts/hepta-intelligence-current-truth.py"
      - "scripts/hepta-intelligence-status-compat.py"
      - "scripts/verify-hepta-intelligence-a0-authority.py"
      - "scripts/verify-hepta-intelligence-document-authority.py"
      - "scripts/verify-hepta-intelligence-master-plan.py"
  workflow_dispatch:

permissions:
  actions: read
  contents: read

concurrency:
  group: hepta-intelligence-a0-source-${{ github.event.pull_request.head.sha || github.sha }}-${{ github.event_name }}
  cancel-in-progress: true

env:
  EXPECTED_REPOSITORY: ProfHepta/hepta-private-ci
  EXPECTED_BRANCH: codex/hepta-intelligence-a0-authority-gap-closure-20260829
  EXPECTED_PARENT: c768bcbeb4c1168088d2499828c24da521a2a73a
  EXPECTED_HEAD_SHA: ${{ github.event.pull_request.head.sha || github.sha }}
  SOURCE_ARTIFACT_NAME: hepta-intelligence-a0-source-${{ github.event.pull_request.head.sha || github.sha }}

jobs:
  source-authority:
    name: A0 canonical authority source gate
    runs-on: ubuntu-24.04
    timeout-minutes: 30
    env:
      ARTIFACT_DIR: artifacts/hepta-intelligence-a0
    steps:
      - name: Checkout exact source candidate
        uses: actions/checkout@11d5960a326750d5838078e36cf38b85af677262
        with:
          ref: ${{ env.EXPECTED_HEAD_SHA }}
          fetch-depth: 2
          persist-credentials: false
      - name: Verify exact repository branch head tree and parent
        shell: bash
        run: |
          set -euo pipefail
          test "$GITHUB_REPOSITORY" = "$EXPECTED_REPOSITORY"
          test "${GITHUB_HEAD_REF:-${GITHUB_REF_NAME}}" = "$EXPECTED_BRANCH"
          test "$(git rev-parse HEAD)" = "$EXPECTED_HEAD_SHA"
          test "$(git rev-parse HEAD^)" = "$EXPECTED_PARENT"
          test -n "$(git rev-parse HEAD^{tree})"
      - name: Compile A0 tools
        run: |
          python3 -m py_compile \
            scripts/hepta-intelligence-current-truth.py \
            scripts/verify-hepta-intelligence-a0-authority.py \
            scripts/verify-hepta-intelligence-document-authority.py \
            scripts/verify-hepta-intelligence-master-plan.py
      - name: Parse registered machine inputs
        shell: bash
        run: |
          set -euo pipefail
          for path in \
            plans/hepta-intelligence/HEPTA_INTELLIGENCE_CURRENT_PLAN.json \
            plans/hepta-intelligence/HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRY_V1.json \
            plans/hepta-intelligence/HEPTA_INTELLIGENCE_EVIDENCE_INDEX_V1.json \
            plans/hepta-intelligence/HEPTA_INTELLIGENCE_CAPABILITY_REGISTRY_V1.json \
            plans/hepta-intelligence/HEPTA_INTELLIGENCE_PR_STACK_REGISTRY_V1.json \
            plans/hepta-intelligence/HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json \
            plans/hepta-intelligence/HEPTA_INTELLIGENCE_Q0_QUALIFICATION_RECEIPT_2026-08-29.json; do
            python3 -m json.tool "$path" >/dev/null
          done
      - name: Verify master spec document and deterministic truth
        shell: bash
        run: |
          set -euo pipefail
          python3 scripts/verify-hepta-intelligence-master-plan.py
          python3 scripts/verify-hepta-intelligence-document-authority.py
          python3 scripts/hepta-intelligence-current-truth.py --verify
          for tranche in P0.2 P0.3 P0.4a P0.4b P0.4c; do
            python3 scripts/hepta-intelligence-status-compat.py "$tranche" --check-only
          done
      - name: Run adversarial negative matrix
        run: python3 scripts/verify-hepta-intelligence-a0-authority.py --self-test
      - name: Build source-only receipt
        shell: bash
        run: |
          set -euo pipefail
          mkdir -p "$ARTIFACT_DIR"
          GITHUB_SHA="$EXPECTED_HEAD_SHA" \
            python3 scripts/verify-hepta-intelligence-a0-authority.py \
            | tee "$ARTIFACT_DIR/a0-source-gate-receipt.json"
          python3 scripts/hepta-intelligence-current-truth.py --compact \
            >"$ARTIFACT_DIR/current-truth-1.json"
          python3 scripts/hepta-intelligence-current-truth.py --compact \
            >"$ARTIFACT_DIR/current-truth-2.json"
          cmp "$ARTIFACT_DIR/current-truth-1.json" "$ARTIFACT_DIR/current-truth-2.json"
          cp "$ARTIFACT_DIR/current-truth-1.json" "$ARTIFACT_DIR/current-truth.json"
          git ls-tree HEAD .github/workflows/hepta-intelligence-a0-authority.yml \
            >"$ARTIFACT_DIR/workflow-blob.txt"
      - name: Enforce non-qualification boundary and clean source
        shell: bash
        run: |
          set -euo pipefail
          python3 - <<'PY'
          import json
          from pathlib import Path
          value = json.loads(Path("artifacts/hepta-intelligence-a0/a0-source-gate-receipt.json").read_text())
          assert value["status"] == "PASS_HEPTA_INTELLIGENCE_A0_SOURCE_ONLY"
          assert value["a0_candidate_qualified"] is False
          assert value["selected"] is False
          assert value["full_repository_merge_green"] is False
          assert value["required_sibling_contexts_bound"] is False
          assert value["authority"] and all(item is False for item in value["authority"].values())
          PY
          git diff --check HEAD^ HEAD
          test -z "$(git status --porcelain --untracked-files=no)"
      - name: Upload exact-head source evidence
        uses: actions/upload-artifact@ea165f8d65b6e75b540449e92b4886f43607fa02
        with:
          name: ${{ env.SOURCE_ARTIFACT_NAME }}
          path: artifacts/hepta-intelligence-a0/**
          if-no-files-found: error
          retention-days: 30

  source-execution-receipt:
    name: A0 source executable mechanics receipt
    needs: source-authority
    if: ${{ always() && needs.source-authority.result == 'success' }}
    runs-on: ubuntu-24.04
    timeout-minutes: 20
    env:
      API_DIR: artifacts/hepta-intelligence-a0-api
      SOURCE_DIR: artifacts/hepta-intelligence-a0-source
      FINAL_DIR: artifacts/hepta-intelligence-a0-final
    steps:
      - name: Checkout exact candidate
        uses: actions/checkout@11d5960a326750d5838078e36cf38b85af677262
        with:
          ref: ${{ env.EXPECTED_HEAD_SHA }}
          fetch-depth: 2
          persist-credentials: false
      - name: Fetch completed source metadata and archive
        shell: bash
        env:
          GH_TOKEN: ${{ github.token }}
        run: |
          set -euo pipefail
          mkdir -p "$API_DIR" "$SOURCE_DIR" "$FINAL_DIR"
          api="https://api.github.com/repos/${GITHUB_REPOSITORY}"
          headers=(-H "Authorization: Bearer ${GH_TOKEN}" -H "Accept: application/vnd.github+json" -H "X-GitHub-Api-Version: 2022-11-28")
          curl -fsSL "${headers[@]}" "${api}/actions/runs/${GITHUB_RUN_ID}" >"$API_DIR/run.json"
          curl -fsSL "${headers[@]}" "${api}/actions/runs/${GITHUB_RUN_ID}/jobs?filter=latest&per_page=100" >"$API_DIR/jobs.json"
          ready=false
          for _ in $(seq 1 30); do
            curl -fsSL "${headers[@]}" "${api}/actions/runs/${GITHUB_RUN_ID}/artifacts?per_page=100" >"$API_DIR/artifacts.json"
            if python3 - "$API_DIR/jobs.json" "$API_DIR/artifacts.json" <<'PY'
          import json, os, sys
          jobs = json.load(open(sys.argv[1]))["jobs"]
          artifacts = json.load(open(sys.argv[2]))["artifacts"]
          source = [j for j in jobs if j.get("name") == "A0 canonical authority source gate"]
          item = [a for a in artifacts if a.get("name") == os.environ["SOURCE_ARTIFACT_NAME"]]
          ok = len(source) == 1 and source[0].get("conclusion") == "success" and int(source[0].get("runner_id") or 0) > 0 and bool(source[0].get("steps")) and len(item) == 1 and item[0].get("expired") is False and str(item[0].get("digest", "")).startswith("sha256:") and bool(item[0].get("expires_at"))
          raise SystemExit(0 if ok else 1)
          PY
            then ready=true; break; fi
            sleep 2
          done
          test "$ready" = true
          artifact_url="$(python3 - "$API_DIR/artifacts.json" <<'PY'
          import json, os, sys
          items=[a for a in json.load(open(sys.argv[1]))["artifacts"] if a.get("name")==os.environ["SOURCE_ARTIFACT_NAME"]]
          assert len(items)==1
          print(items[0]["archive_download_url"])
          PY
          )"
          curl -fsSL -L "${headers[@]}" "$artifact_url" -o "$API_DIR/source.zip"
      - name: Verify archive digest and safe entry manifest
        shell: bash
        run: |
          set -euo pipefail
          python3 - "$API_DIR/artifacts.json" "$API_DIR/source.zip" "$SOURCE_DIR" <<'PY'
          import hashlib, json, os, pathlib, stat, sys, zipfile
          metadata=json.load(open(sys.argv[1]))["artifacts"]
          matches=[a for a in metadata if a.get("name")==os.environ["SOURCE_ARTIFACT_NAME"]]
          assert len(matches)==1
          expected=matches[0]["digest"].removeprefix("sha256:")
          archive=pathlib.Path(sys.argv[2])
          observed=hashlib.sha256(archive.read_bytes()).hexdigest()
          assert observed==expected, (observed, expected)
          target=pathlib.Path(sys.argv[3]).resolve()
          seen=set()
          with zipfile.ZipFile(archive) as zf:
            for info in zf.infolist():
              name=info.filename.replace("\\", "/")
              pure=pathlib.PurePosixPath(name)
              assert not pure.is_absolute() and ".." not in pure.parts
              normalized=pure.as_posix().rstrip("/")
              if not normalized: continue
              assert normalized not in seen
              seen.add(normalized)
              mode=(info.external_attr >> 16) & 0xFFFF
              assert not stat.S_ISLNK(mode)
              assert info.is_dir() or mode == 0 or stat.S_ISREG(mode)
              destination=(target / pathlib.Path(*pure.parts)).resolve()
              assert target == destination or target in destination.parents
            zf.extractall(target)
          PY
      - name: Build source-execution-only receipt
        shell: bash
        run: |
          set -euo pipefail
          python3 - "$API_DIR" "$SOURCE_DIR" "$FINAL_DIR/a0-source-execution-receipt.json" <<'PY'
          import hashlib, json, os, pathlib, subprocess, sys
          api=pathlib.Path(sys.argv[1]); source=pathlib.Path(sys.argv[2]); output=pathlib.Path(sys.argv[3])
          run=json.load(open(api/"run.json")); jobs=json.load(open(api/"jobs.json"))["jobs"]; artifacts=json.load(open(api/"artifacts.json"))["artifacts"]
          sj=[j for j in jobs if j.get("name")=="A0 canonical authority source gate"]; assert len(sj)==1; sj=sj[0]
          sa=[a for a in artifacts if a.get("name")==os.environ["SOURCE_ARTIFACT_NAME"]]; assert len(sa)==1; sa=sa[0]
          source_receipts=list(source.rglob("a0-source-gate-receipt.json")); truths=list(source.rglob("current-truth.json")); workflow_blobs=list(source.rglob("workflow-blob.txt"))
          assert len(source_receipts)==len(truths)==len(workflow_blobs)==1
          source_receipt=json.load(open(source_receipts[0])); assert source_receipt["a0_candidate_qualified"] is False
          tree=subprocess.check_output(["git","rev-parse","HEAD^{tree}"],text=True).strip()
          workflow_blob=subprocess.check_output(["git","rev-parse","HEAD:.github/workflows/hepta-intelligence-a0-authority.yml"],text=True).strip()
          receipt={
            "schema":"hepta_intelligence_a0_source_execution_receipt_v1",
            "status":"PASS_HEPTA_INTELLIGENCE_A0_SOURCE_EXECUTION_ONLY",
            "candidate":{"repository":os.environ["EXPECTED_REPOSITORY"],"branch":os.environ["EXPECTED_BRANCH"],"head":os.environ["EXPECTED_HEAD_SHA"],"tree":tree,"parent":os.environ["EXPECTED_PARENT"]},
            "workflow":{"path":".github/workflows/hepta-intelligence-a0-authority.yml","blob_sha":workflow_blob,"run_id":run["id"],"run_attempt":run["run_attempt"],"event":run["event"],"source_job_id":sj["id"],"source_runner_id":sj["runner_id"],"source_runner_name":sj["runner_name"],"source_runner_labels":sj["labels"],"source_steps":[{"name":s["name"],"conclusion":s["conclusion"]} for s in sj["steps"]]},
            "source_artifact":{"id":sa["id"],"name":sa["name"],"digest":sa["digest"],"expires_at":sa["expires_at"],"expired":sa["expired"]},
            "evidence":{"source_receipt_sha256":hashlib.sha256(source_receipts[0].read_bytes()).hexdigest(),"current_truth_sha256":hashlib.sha256(truths[0].read_bytes()).hexdigest(),"downloaded_archive_sha256":hashlib.sha256((api/"source.zip").read_bytes()).hexdigest()},
            "a0_candidate_qualified":False,"selected":False,"full_repository_merge_green":False,"required_sibling_contexts_bound":False,"requires_completed_run_attestor":True,"authority":source_receipt["authority"],
          }
          receipt["receipt_binding_sha256"]=hashlib.sha256(json.dumps(receipt,sort_keys=True,separators=(",",":"),ensure_ascii=False).encode()).hexdigest()
          output.write_text(json.dumps(receipt,indent=2,sort_keys=True)+"\n")
          PY
      - name: Enforce downgraded boundary and upload
        shell: bash
        run: |
          set -euo pipefail
          python3 - "$FINAL_DIR/a0-source-execution-receipt.json" <<'PY'
          import hashlib,json,sys
          value=json.load(open(sys.argv[1])); binding=value.pop("receipt_binding_sha256")
          assert binding==hashlib.sha256(json.dumps(value,sort_keys=True,separators=(",",":"),ensure_ascii=False).encode()).hexdigest()
          assert value["status"]=="PASS_HEPTA_INTELLIGENCE_A0_SOURCE_EXECUTION_ONLY"
          assert value["a0_candidate_qualified"] is False and value["selected"] is False and value["full_repository_merge_green"] is False
          assert value["requires_completed_run_attestor"] is True
          assert value["authority"] and all(item is False for item in value["authority"].values())
          PY
          git diff --check HEAD^ HEAD
          test -z "$(git status --porcelain --untracked-files=no)"
      - name: Upload exact-head source execution receipt
        uses: actions/upload-artifact@ea165f8d65b6e75b540449e92b4886f43607fa02
        with:
          name: hepta-intelligence-a0-source-execution-${{ env.EXPECTED_HEAD_SHA }}
          path: |
            artifacts/hepta-intelligence-a0-final/**
            artifacts/hepta-intelligence-a0-api/run.json
            artifacts/hepta-intelligence-a0-api/jobs.json
            artifacts/hepta-intelligence-a0-api/artifacts.json
          if-no-files-found: error
          retention-days: 30
''',
        encoding="utf-8",
    )

    execution = ROOT / ".github" / "workflows" / "hepta-intelligence-execution-spec.yml"
    execution.write_text(
        r'''name: Hepta Intelligence controlled gap-closure specification

on:
  push:
    branches:
      - "codex/hepta-intelligence-a0-authority-gap-closure-*"
  pull_request:
    paths:
      - ".github/workflows/hepta-intelligence-execution-spec.yml"
      - "plans/hepta-intelligence/**"
      - "scripts/hepta-intelligence-current-truth.py"
      - "scripts/verify-hepta-intelligence-a0-authority.py"
      - "scripts/verify-hepta-intelligence-document-authority.py"
      - "scripts/verify-hepta-intelligence-master-plan.py"
  workflow_dispatch:

permissions:
  contents: read

concurrency:
  group: hepta-intelligence-execution-spec-${{ github.event.pull_request.head.sha || github.sha }}-${{ github.event_name }}
  cancel-in-progress: true

env:
  EXPECTED_REPOSITORY: ProfHepta/hepta-private-ci
  EXPECTED_BRANCH: codex/hepta-intelligence-a0-authority-gap-closure-20260829
  EXPECTED_PARENT: c768bcbeb4c1168088d2499828c24da521a2a73a
  EXPECTED_HEAD_SHA: ${{ github.event.pull_request.head.sha || github.sha }}
  ARTIFACT_DIR: artifacts/hepta-intelligence-execution-spec

jobs:
  verify:
    name: A0 controlled specification source gate
    runs-on: ubuntu-24.04
    timeout-minutes: 30
    steps:
      - name: Checkout exact source candidate
        uses: actions/checkout@11d5960a326750d5838078e36cf38b85af677262
        with:
          ref: ${{ env.EXPECTED_HEAD_SHA }}
          fetch-depth: 2
          persist-credentials: false
      - name: Verify exact identity
        shell: bash
        run: |
          set -euo pipefail
          test "$GITHUB_REPOSITORY" = "$EXPECTED_REPOSITORY"
          test "${GITHUB_HEAD_REF:-${GITHUB_REF_NAME}}" = "$EXPECTED_BRANCH"
          test "$(git rev-parse HEAD)" = "$EXPECTED_HEAD_SHA"
          test "$(git rev-parse HEAD^)" = "$EXPECTED_PARENT"
      - name: Verify aligned source contracts
        shell: bash
        run: |
          set -euo pipefail
          python3 scripts/verify-hepta-intelligence-master-plan.py
          python3 scripts/verify-hepta-intelligence-document-authority.py
          python3 scripts/hepta-intelligence-current-truth.py --verify
          GITHUB_SHA="$EXPECTED_HEAD_SHA" python3 scripts/verify-hepta-intelligence-a0-authority.py >/dev/null
          python3 scripts/verify-hepta-intelligence-a0-authority.py --self-test
      - name: Emit non-qualifying specification receipt
        shell: bash
        run: |
          set -euo pipefail
          mkdir -p "$ARTIFACT_DIR"
          python3 - "$ARTIFACT_DIR/receipt.json" <<'PY'
          import hashlib,json,pathlib,subprocess,sys
          root=pathlib.Path.cwd(); output=pathlib.Path(sys.argv[1])
          spec=root/"plans/hepta-intelligence/HEPTA_INTELLIGENCE_CONTROLLED_GAP_CLOSURE_EXECUTION_SPEC_V1.md"
          value={"schema":"hepta_intelligence_execution_spec_source_receipt_v2","status":"PASS_HEPTA_INTELLIGENCE_EXECUTION_SPEC_SOURCE_ONLY","head":subprocess.check_output(["git","rev-parse","HEAD"],text=True).strip(),"tree":subprocess.check_output(["git","rev-parse","HEAD^{tree}"],text=True).strip(),"parent":subprocess.check_output(["git","rev-parse","HEAD^"],text=True).strip(),"spec_sha256":hashlib.sha256(spec.read_bytes()).hexdigest(),"a0_candidate_qualified":False,"selected":False,"source_writeback":False}
          output.write_text(json.dumps(value,indent=2,sort_keys=True)+"\n")
          PY
          git diff --check HEAD^ HEAD
          test -z "$(git status --porcelain --untracked-files=no)"
      - name: Upload exact-head specification receipt
        uses: actions/upload-artifact@ea165f8d65b6e75b540449e92b4886f43607fa02
        with:
          name: hepta-intelligence-execution-spec-${{ env.EXPECTED_HEAD_SHA }}
          path: artifacts/hepta-intelligence-execution-spec/**
          if-no-files-found: error
          retention-days: 30
''',
        encoding="utf-8",
    )

    master = ROOT / ".github" / "workflows" / "hepta-intelligence-master-plan.yml"
    master.write_text(
        r'''name: Hepta Intelligence canonical master plan

on:
  pull_request:
    paths:
      - ".github/workflows/hepta-intelligence-a0-authority.yml"
      - ".github/workflows/hepta-intelligence-execution-spec.yml"
      - ".github/workflows/hepta-intelligence-master-plan.yml"
      - "plans/hepta-intelligence/**"
      - "scripts/hepta-intelligence-current-truth.py"
      - "scripts/hepta-intelligence-status-compat.py"
      - "scripts/verify-hepta-intelligence-a0-authority.py"
      - "scripts/verify-hepta-intelligence-document-authority.py"
      - "scripts/verify-hepta-intelligence-master-plan.py"
  workflow_dispatch:

permissions:
  contents: read

concurrency:
  group: hepta-intelligence-master-plan-${{ github.event.pull_request.head.sha || github.sha }}-${{ github.event_name }}
  cancel-in-progress: true

jobs:
  canonical-plan-source:
    name: Canonical plan document compatibility source gate
    runs-on: ubuntu-24.04
    timeout-minutes: 30
    env:
      EXPECTED_REPOSITORY: ProfHepta/hepta-private-ci
      EXPECTED_BRANCH: codex/hepta-intelligence-a0-authority-gap-closure-20260829
      EXPECTED_PARENT: c768bcbeb4c1168088d2499828c24da521a2a73a
      EXPECTED_SHA: ${{ github.event.pull_request.head.sha || github.sha }}
    steps:
      - name: Checkout explicit candidate identity
        uses: actions/checkout@11d5960a326750d5838078e36cf38b85af677262
        with:
          ref: ${{ env.EXPECTED_SHA }}
          fetch-depth: 2
          persist-credentials: false
      - name: Prove exact candidate
        shell: bash
        run: |
          set -euo pipefail
          test "$GITHUB_REPOSITORY" = "$EXPECTED_REPOSITORY"
          test "${GITHUB_HEAD_REF:-${GITHUB_REF_NAME}}" = "$EXPECTED_BRANCH"
          test "$(git rev-parse HEAD)" = "$EXPECTED_SHA"
          test "$(git rev-parse HEAD^)" = "$EXPECTED_PARENT"
      - name: Verify canonical plan specification authority and compatibility
        shell: bash
        run: |
          set -euo pipefail
          python3 -m py_compile \
            scripts/hepta-intelligence-current-truth.py \
            scripts/hepta-intelligence-status-compat.py \
            scripts/verify-hepta-intelligence-a0-authority.py \
            scripts/verify-hepta-intelligence-document-authority.py \
            scripts/verify-hepta-intelligence-master-plan.py
          python3 scripts/verify-hepta-intelligence-master-plan.py
          python3 scripts/verify-hepta-intelligence-document-authority.py
          python3 scripts/hepta-intelligence-current-truth.py --verify
          for tranche in P0.2 P0.3 P0.4a P0.4b P0.4c; do
            python3 scripts/hepta-intelligence-status-compat.py "$tranche" --check-only
          done
          python3 scripts/verify-hepta-intelligence-a0-authority.py --self-test
      - name: Enforce source-only boundary
        shell: bash
        run: |
          set -euo pipefail
          python3 - <<'PY'
          import json,pathlib
          current=json.loads(pathlib.Path("plans/hepta-intelligence/HEPTA_INTELLIGENCE_CURRENT_PLAN.json").read_text())
          assert current["current_truth"]["a0_candidate_qualified"] is False
          assert current["current_truth"]["selected"] is False
          assert current["authority"] and all(item is False for item in current["authority"].values())
          PY
          git diff --check HEAD^ HEAD
          test -z "$(git status --porcelain --untracked-files=no)"
''',
        encoding="utf-8",
    )


def patch_json_documents() -> None:
    current_path = PLAN / "HEPTA_INTELLIGENCE_CURRENT_PLAN.json"
    capability_path = PLAN / "HEPTA_INTELLIGENCE_CAPABILITY_REGISTRY_V1.json"
    integration_path = PLAN / "HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json"
    evidence_path = PLAN / "HEPTA_INTELLIGENCE_EVIDENCE_INDEX_V1.json"
    pr_stack_path = PLAN / "HEPTA_INTELLIGENCE_PR_STACK_REGISTRY_V1.json"
    doc_path = PLAN / "HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRY_V1.json"
    master_path = PLAN / "HEPTA_INTELLIGENCE_MASTER_PLAN.md"
    spec_path = PLAN / "HEPTA_INTELLIGENCE_CONTROLLED_GAP_CLOSURE_EXECUTION_SPEC_V1.md"

    capabilities = load(capability_path)
    capabilities["as_of_utc"] = SNAPSHOT_UTC
    capabilities["lifecycle"] = CAPABILITY_LIFECYCLE
    capabilities["authority"] = authority_false()
    for entry in capabilities.get("capabilities", []):
        entry["selected"] = False
        entry["authority"] = authority_false()
    dump(capability_path, capabilities)

    evidence = load(evidence_path)
    evidence["as_of_utc"] = SNAPSHOT_UTC
    evidence["authority"] = authority_false()
    dump(evidence_path, evidence)

    pr_stack = load(pr_stack_path)
    pr_stack["as_of_utc"] = SNAPSHOT_UTC
    pr_stack["authority"] = authority_false()
    dump(pr_stack_path, pr_stack)

    integration = load(integration_path)
    integration["authority"] = authority_false()
    integration["allowed_changed_paths"] = ALLOWED_PATHS
    integration["expected_changed_path_count"] = len(ALLOWED_PATHS)
    ledger = integration["gap_closure_ledger"]
    ledger["as_of_utc"] = SNAPSHOT_UTC
    ledger["schema"] = "hepta_intelligence_gap_closure_ledger_v1"
    entries = ledger.get("entries", [])
    by_id = {entry.get("gap_id"): entry for entry in entries}
    if set(by_id) != set(GAP_IDS):
        raise SystemExit(f"unexpected gap IDs: {sorted(by_id)}")
    normalized = []
    for gap_id in GAP_IDS:
        entry = by_id[gap_id]
        dependencies = entry.pop("blocked_by", entry.get("dependencies", []))
        entry["dependencies"] = list(dependencies or [])
        entry.setdefault("resume_predicate", "all declared dependencies and authority-negative invariants remain valid")
        if gap_id == "A0-DOC-001":
            entry["closure_evidence"] = [
                "HEPTA_INTELLIGENCE_MASTER_PLAN.md version 4.3.0",
                "HEPTA_INTELLIGENCE_CONTROLLED_GAP_CLOSURE_EXECUTION_SPEC_V1.md version 1.1.0",
                "exact read order, selected lifecycle, compatibility seals, negative tests and non-self-qualification boundary",
            ]
            entry["status"] = "CLOSED_IN_REVIEWED_REPLACEMENT_SOURCE_AWAITING_FRESH_EXECUTION"
        normalized.append(entry)
    ledger["entries"] = normalized
    ledger["invariants"] = {
        "external_evidence_may_be_synthesized": False,
        "fixture_may_close_external_gap": False,
        "positive_authority_allowed": False,
        "source_closed_requires_exact_head_executable_evidence": True,
        "unknown_gap_fails_closed": True,
    }
    integration["repository_check_attribution_contract"] = {
        "schema": "RepositoryCheckAttributionReceiptV1",
        "classifications": CHECK_CLASSES,
        "unknown_fails_closed": True,
        "candidate_workflow_may_self_qualify": False,
        "selection_required_before_merge_attribution": True,
        "target_exclusion_requires_independent_policy_authority": True,
    }
    integration["source_snapshot"] = {
        "as_of_utc": SNAPSHOT_UTC,
        "classification": "SOURCE_SNAPSHOT_NOT_LIVE_EVIDENCE",
        "must_not_postdate_containing_commit": True,
    }
    dump(integration_path, integration)

    current = load(current_path)
    current["generated_at_utc"] = SNAPSHOT_UTC
    current["canonical"]["plan_version"] = MASTER_VERSION
    current["canonical"]["content_sha256"] = sha256(master_path)
    current["operational_execution"]["execution_spec_version"] = SPEC_VERSION
    current["operational_execution"]["execution_spec_sha256"] = sha256(spec_path)
    current["session_bootstrap"]["read_order"] = READ_ORDER
    current["active_phase"]["active_task"] = "A0.3_REVIEW_DEFECT_REPAIR_AND_FRESH_SOURCE_EXECUTION"
    current["active_phase"]["current_work_unit"] = "A0.3B_OBTAIN_FRESH_SOURCE_EXECUTION_THEN_DISTINCT_REVIEW"
    current["authority"] = authority_false()
    truth = current["current_truth"]
    truth["a0_source_execution_complete"] = False
    truth["a0_candidate_qualified"] = False
    truth["selected"] = False
    truth["qualified"] = False
    truth["wired"] = False
    truth["runtime_qualified"] = False
    truth["efficacy_proven"] = False
    truth["operator_accepted"] = False
    truth["promoted"] = False
    truth["full_repository_merge_green"] = False
    current["next_actions"] = [
        {"id": "A0.3b", "order": 1, "action": "Obtain fresh exact-head source and source-execution receipts", "blocked": False},
        {"id": "A0.4", "order": 2, "action": "Obtain distinct signed independent review", "blocked": True, "blocked_by": "fresh exact-head source execution"},
        {"id": "A0.4b", "order": 3, "action": "Issue CanonicalSelectionReceiptV1", "blocked": True, "blocked_by": "distinct independent approval"},
        {"id": "A0.5", "order": 4, "action": "Attest selected merge candidate and classify every required check", "blocked": True, "blocked_by": "canonical selection"},
        {"id": "B0", "order": 5, "action": "Begin B0.1 contracts extraction", "blocked": True, "blocked_by": "A0 selected merge-candidate admission"},
    ]
    current["source_snapshot_policy"] = {
        "classification": "SOURCE_SNAPSHOT_NOT_LIVE_CI",
        "as_of_utc": SNAPSHOT_UTC,
        "live_evidence_may_directly_mutate_source": False,
        "live_observation_requires_exact_receipt": True,
        "queued_or_incomplete_is_pass": False,
        "superseded_head_evidence_reusable": False,
        "source_timestamp_must_not_exceed_commit_time": True,
        "candidate_workflow_may_self_qualify": False,
    }
    previous = current.pop("a0_previous_exact_head_provenance", None)
    history = current.get("a0_previous_exact_head_provenance_chain", [])
    if previous and previous not in history:
        history.append(previous)
    history.append({
        "head": "0b80caff91010f40a79c795c20487ff9d773d229",
        "tree": "9f67a4892a3474e7f424327ecc46d81a98421cc4",
        "classification": "SUPERSEDED_BY_REVIEW_DEFECT_REPLACEMENT",
        "reusable_for_replacement_head": False,
        "blocking_findings": ["A0-RV-002", "A0-RV-003", "A0-RV-004", "A0-RV-005", "A0-RV-006", "A0-RV-007", "A0-RV-008", "A0-RV-009"],
    })
    current["a0_previous_exact_head_provenance_chain"] = history[-8:]
    current["stack_budget"]["reason"] = "A0 V4.3 review defects repaired in source; fresh source execution, distinct review, selection and merge attribution remain required"
    dump(current_path, current)

    compatibility: list[dict[str, Any]] = []
    for rel, schema in (
        ("plans/hepta-intelligence/HEPTA_INTELLIGENCE_EXECUTION_STATUS_V2.json", "hepta_intelligence_execution_status_v2"),
        ("plans/hepta-intelligence/HEPTA_INTELLIGENCE_EXECUTION_STATUS_V3.json", "hepta_intelligence_execution_status_v3"),
    ):
        file_path = ROOT / rel
        compatibility.append({
            "path": rel,
            "schema": schema,
            "classification": "LEGACY_MACHINE_COMPATIBILITY_CONTRACT",
            "current_authority": False,
            "schema_preserved": True,
            "mutation_policy": "BACKWARD_COMPATIBLE_ONLY_UNTIL_ALL_REGISTERED_CONSUMERS_MIGRATE_WITH_RECEIPT",
            "content_sha256": sha256(file_path),
            "registered_consumers": scan_consumers(file_path.name),
        })
    snapshots = []
    for snapshot_path in sorted((PLAN / "status-snapshots").glob("*.json")):
        value = load(snapshot_path)
        snapshots.append({
            "path": snapshot_path.relative_to(ROOT).as_posix(),
            "schema": value.get("schema"),
            "snapshot_id": value.get("snapshot_id"),
            "content_sha256": sha256(snapshot_path),
            "current_authority": False,
        })

    doc = load(doc_path)
    doc["as_of_utc"] = SNAPSHOT_UTC
    doc["authority"] = authority_false()
    doc["current_authority"]["human"] = {
        "path": master_path.relative_to(ROOT).as_posix(),
        "plan_id": "HEPTA_INTELLIGENCE_MASTER_PLAN_V4",
        "plan_version": MASTER_VERSION,
        "content_sha256": sha256(master_path),
    }
    doc["current_authority"]["machine"] = {
        "path": current_path.relative_to(ROOT).as_posix(),
        "schema": "hepta_intelligence_current_plan_v2",
    }
    doc["registered_canonical_inputs"] = [
        {"path": evidence_path.relative_to(ROOT).as_posix(), "schema": "hepta_intelligence_evidence_index_v1", "role": "TIME_BOUNDED_EVIDENCE_INDEX", "content_sha256": sha256(evidence_path), "current_plan_authority": False},
        {"path": capability_path.relative_to(ROOT).as_posix(), "schema": "hepta_intelligence_capability_registry_v1", "role": "CAPABILITY_STATE_INPUT", "content_sha256": sha256(capability_path), "current_plan_authority": False},
        {"path": pr_stack_path.relative_to(ROOT).as_posix(), "schema": "hepta_intelligence_pr_stack_registry_v1", "role": "PR_STACK_INPUT", "content_sha256": sha256(pr_stack_path), "current_plan_authority": False},
        {"path": integration_path.relative_to(ROOT).as_posix(), "schema": "hepta_intelligence_integration_candidate_v1", "role": "INTEGRATION_CANDIDATE_INPUT", "content_sha256": sha256(integration_path), "current_plan_authority": False},
    ]
    doc["compatibility_contracts"] = compatibility
    doc["compatibility_snapshots"] = snapshots
    doc["registered_operational_documents"] = [{
        "path": spec_path.relative_to(ROOT).as_posix(),
        "specification_id": "HEPTA_INTELLIGENCE_CONTROLLED_GAP_CLOSURE_EXECUTION_SPEC_V1",
        "version": SPEC_VERSION,
        "classification": "SUBORDINATE_EXECUTION_SPEC_NO_CURRENT_PLAN_OR_PROMOTION_AUTHORITY",
        "content_sha256": sha256(spec_path),
        "current_plan_authority": False,
        "promotion_authority": False,
        "role": "PACKAGE_LEVEL_GAP_CLOSURE_RUNBOOK",
    }]
    doc["rules"].update({
        "single_current_machine_authority": True,
        "single_current_human_authority": True,
        "compatibility_contracts_are_current_authority": False,
        "breaking_a_registered_consumer_requires_migration_receipt": True,
        "registered_inputs_may_grant_production_authority": False,
        "subordinate_operational_document_may_grant_authority": False,
        "unknown_document_fails_closed": True,
    })
    dump(doc_path, doc)


def build_artifact() -> None:
    if ARTIFACT.exists():
        shutil.rmtree(ARTIFACT)
    files_root = ARTIFACT / "files"
    files_root.mkdir(parents=True)
    manifest: list[dict[str, Any]] = []
    for rel in ALLOWED_PATHS:
        source = ROOT / rel
        if not source.is_file():
            raise SystemExit(f"missing allowed path after repair: {rel}")
        destination = files_root / rel
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(source, destination)
        manifest.append({
            "path": rel,
            "sha256": sha256(source),
            "bytes": source.stat().st_size,
            "mode": "100755" if os.access(source, os.X_OK) else "100644",
        })
    receipt = {
        "schema": "hepta_intelligence_a0_review_fix_factory_receipt_v1",
        "status": "PASS_A0_RV_002_THROUGH_009_REPAIR_FACTORY",
        "source_candidate": "0b80caff91010f40a79c795c20487ff9d773d229",
        "required_parent": Q0_HEAD,
        "replacement_tree_parent_policy": "SOLE_PARENT_Q0",
        "review_findings_closed_in_source": [f"A0-RV-{number:03d}" for number in range(2, 10)],
        "candidate_workflow_may_self_qualify": False,
        "a0_candidate_qualified": False,
        "selected": False,
        "source_writeback": False,
        "files": manifest,
        "authority": authority_false(),
    }
    dump(ARTIFACT / "factory-receipt.json", receipt)
    (ARTIFACT / "SHA256SUMS").write_text(
        "".join(f"{item['sha256']}  files/{item['path']}\n" for item in manifest),
        encoding="utf-8",
    )


def validate() -> None:
    scripts = [
        ROOT / "scripts/hepta-intelligence-current-truth.py",
        ROOT / "scripts/verify-hepta-intelligence-master-plan.py",
        ROOT / "scripts/verify-hepta-intelligence-document-authority.py",
        ROOT / "scripts/verify-hepta-intelligence-a0-authority.py",
    ]
    for path in scripts:
        py_compile.compile(str(path), doraise=True)
    env = dict(os.environ)
    env["HEPTA_A0_FACTORY"] = "1"
    commands = [
        [sys.executable, "scripts/verify-hepta-intelligence-master-plan.py"],
        [sys.executable, "scripts/verify-hepta-intelligence-document-authority.py"],
        [sys.executable, "scripts/hepta-intelligence-current-truth.py", "--verify"],
        [sys.executable, "scripts/verify-hepta-intelligence-a0-authority.py"],
        [sys.executable, "scripts/verify-hepta-intelligence-a0-authority.py", "--self-test"],
    ]
    for command in commands:
        subprocess.run(command, cwd=ROOT, env=env, check=True)
    for tranche in ("P0.2", "P0.3", "P0.4a", "P0.4b", "P0.4c"):
        subprocess.run(
            [sys.executable, "scripts/hepta-intelligence-status-compat.py", tranche, "--check-only"],
            cwd=ROOT,
            env=env,
            check=True,
        )
    subprocess.run(["git", "diff", "--check"], cwd=ROOT, check=True)


def main() -> int:
    patch_master_and_spec()
    write_current_truth_script()
    write_master_verifier()
    write_document_verifier()
    write_a0_verifier()
    write_workflows()
    patch_json_documents()
    validate()
    build_artifact()
    print("PASS_A0_V43_REVIEW_FIX_FACTORY")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
