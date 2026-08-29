#!/usr/bin/env python3
from __future__ import annotations

import hashlib
import json
import re
from datetime import datetime
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
PLAN = ROOT / "plans" / "hepta-intelligence"
SNAPSHOT_AT = "2026-08-29T19:33:44Z"
MASTER_VERSION = "4.3.0"
MASTER_ID = "HEPTA_INTELLIGENCE_MASTER_PLAN_V4"
SPEC_ID = "HEPTA_INTELLIGENCE_CONTROLLED_GAP_CLOSURE_EXECUTION_SPEC_V1"

CURRENT = PLAN / "HEPTA_INTELLIGENCE_CURRENT_PLAN.json"
DOCUMENT = PLAN / "HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRY_V1.json"
CAPABILITIES = PLAN / "HEPTA_INTELLIGENCE_CAPABILITY_REGISTRY_V1.json"
INTEGRATION = PLAN / "HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json"
MASTER = PLAN / "HEPTA_INTELLIGENCE_MASTER_PLAN.md"
SPEC = PLAN / "HEPTA_INTELLIGENCE_CONTROLLED_GAP_CLOSURE_EXECUTION_SPEC_V1.md"
TRUTH = ROOT / "scripts" / "hepta-intelligence-current-truth.py"
DOC_VERIFY = ROOT / "scripts" / "verify-hepta-intelligence-document-authority.py"
MASTER_VERIFY = ROOT / "scripts" / "verify-hepta-intelligence-master-plan.py"
A0_VERIFY = ROOT / "scripts" / "verify-hepta-intelligence-a0-authority.py"
A0_WORKFLOW = ROOT / ".github" / "workflows" / "hepta-intelligence-a0-authority.yml"
EXEC_WORKFLOW = ROOT / ".github" / "workflows" / "hepta-intelligence-execution-spec.yml"

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
    "implemented",
    "candidate_qualified",
    "selected",
    "wired",
    "runtime_qualified",
    "efficacy_proven",
    "operator_accepted",
    "promoted",
]
CHECK_CLASSIFICATIONS = [
    "PASS",
    "INTRODUCED_BY_CANDIDATE",
    "PRE_EXISTING_ON_BASE",
    "MERGE_INTERACTION",
    "RUNNER_OR_PLATFORM_INFRA",
    "CANCELLED_OR_SUPERSEDED",
    "NOT_REQUIRED_BY_SELECTED_POLICY",
    "UNKNOWN_FAIL_CLOSED",
]
OLD_TO_NEW = {
    "FAIL_INTRODUCED_BY_CANDIDATE": "INTRODUCED_BY_CANDIDATE",
    "FAIL_PRESENT_IN_BASE": "PRE_EXISTING_ON_BASE",
    "INFRASTRUCTURE_BLOCKED": "RUNNER_OR_PLATFORM_INFRA",
    "CANCELLED_SUPERSEDED": "CANCELLED_OR_SUPERSEDED",
    "NOT_REQUIRED_FOR_SELECTED_TARGET": "NOT_REQUIRED_BY_SELECTED_POLICY",
}


def fail(message: str) -> None:
    raise SystemExit(f"FAIL_A0_V4_3_SOURCE_REPAIR: {message}")


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def load(path: Path) -> dict[str, Any]:
    value = json.loads(path.read_text(encoding="utf-8"))
    require(isinstance(value, dict), f"{path} must contain an object")
    return value


def write_json(path: Path, value: dict[str, Any]) -> None:
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def replace_exact(text: str, old: str, new: str, *, count: int = 1) -> str:
    observed = text.count(old)
    require(observed == count, f"expected {count} occurrence(s) of {old!r}, found {observed}")
    return text.replace(old, new, count)


def recursive_replace(value: Any) -> Any:
    if isinstance(value, dict):
        return {key: recursive_replace(item) for key, item in value.items()}
    if isinstance(value, list):
        replaced = [recursive_replace(item) for item in value]
        if any(item in OLD_TO_NEW for item in value if isinstance(item, str)):
            for classification in CHECK_CLASSIFICATIONS:
                if classification not in replaced:
                    replaced.append(classification)
        return replaced
    if isinstance(value, str):
        result = value
        for old, new in OLD_TO_NEW.items():
            result = result.replace(old, new)
        return result
    return value


def patch_json() -> None:
    current = load(CURRENT)
    document = load(DOCUMENT)
    capabilities = load(CAPABILITIES)
    integration = recursive_replace(load(INTEGRATION))

    current["generated_at_utc"] = SNAPSHOT_AT
    current["session_bootstrap"]["read_order"] = READ_ORDER
    current["current_truth"]["selected"] = False
    current["operational_execution"]["canonical_parent_plan_id"] = MASTER_ID
    current["operational_execution"]["canonical_parent_plan_version"] = MASTER_VERSION

    document["generated_at_utc"] = SNAPSHOT_AT
    operational_docs = document["registered_operational_documents"]
    require(isinstance(operational_docs, list) and len(operational_docs) == 1, "document operational docs")
    operational_docs[0]["canonical_parent_plan_id"] = MASTER_ID
    operational_docs[0]["canonical_parent_plan_version"] = MASTER_VERSION

    capabilities["lifecycle"] = LIFECYCLE
    capabilities.setdefault("invariants", {})["selected_requires_candidate_qualified"] = True
    capabilities["invariants"]["wired_requires_selected"] = True
    entries = capabilities.get("capabilities")
    require(isinstance(entries, list) and entries, "capability entries")
    for entry in entries:
        require(isinstance(entry, dict), "malformed capability")
        entry["selected"] = False

    ledger = integration.get("gap_closure_ledger", {})
    require(isinstance(ledger, dict), "integration ledger")
    ledger["as_of_utc"] = SNAPSHOT_AT
    entries = ledger.get("entries")
    require(isinstance(entries, list), "integration ledger entries")
    for entry in entries:
        if entry.get("gap_id") == "A0-DOC-001":
            evidence = entry.get("closure_evidence")
            require(isinstance(evidence, list), "A0-DOC evidence")
            entry["closure_evidence"] = [
                item.replace("version 4.2.0", "version 4.3.0")
                for item in evidence
            ]
    integration["repository_check_classifications"] = CHECK_CLASSIFICATIONS
    integration_docs = integration.get("operational_documents")
    require(isinstance(integration_docs, list) and len(integration_docs) == 1, "integration operational docs")
    integration_docs[0]["canonical_parent_plan_id"] = MASTER_ID
    integration_docs[0]["canonical_parent_plan_version"] = MASTER_VERSION

    write_json(CURRENT, current)
    write_json(DOCUMENT, document)
    write_json(CAPABILITIES, capabilities)
    write_json(INTEGRATION, integration)


def patch_spec() -> str:
    text = SPEC.read_text(encoding="utf-8")
    text = replace_exact(
        text,
        "Canonical parent plan: `HEPTA_INTELLIGENCE_MASTER_PLAN_V4` version `4.2.0`",
        "Canonical parent plan: `HEPTA_INTELLIGENCE_MASTER_PLAN_V4` version `4.3.0`",
    )
    text = replace_exact(
        text,
        "PASS_HEPTA_INTELLIGENCE_MASTER_PLAN_V4_2_SOURCE_ONLY",
        "PASS_HEPTA_INTELLIGENCE_MASTER_PLAN_V4_3_SOURCE_ONLY",
    )
    old = (
        "Merge attribution uses `RepositoryCheckAttributionReceiptV1` and classifies each\n"
        "required check as PASS, FAIL_INTRODUCED_BY_CANDIDATE, FAIL_PRESENT_IN_BASE,\n"
        "INFRASTRUCTURE_BLOCKED, CANCELLED_SUPERSEDED or NOT_REQUIRED_FOR_SELECTED_TARGET.\n"
        "Only PASS or independently policy-approved target exclusion can yield merge\n"
        "eligibility."
    )
    new = (
        "Merge attribution uses `RepositoryCheckAttributionReceiptV1` and classifies each\n"
        "required check using this exact ordered vocabulary:\n\n"
        "```text\n"
        + "\n".join(CHECK_CLASSIFICATIONS)
        + "\n```\n\n"
        "`UNKNOWN_FAIL_CLOSED` blocks merge. Only `PASS` or an independently\n"
        "policy-approved `NOT_REQUIRED_BY_SELECTED_POLICY` target exclusion can yield\n"
        "merge eligibility."
    )
    text = replace_exact(text, old, new)
    require("version `4.2.0`" not in text, "stale canonical parent version")
    require("MASTER_PLAN_V4_2" not in text, "stale V4.2 marker")
    for old in OLD_TO_NEW:
        require(old not in text, f"stale classification {old}")
    SPEC.write_text(text, encoding="utf-8")
    return sha256(SPEC)


def bind_spec_digest(spec_digest: str) -> None:
    current = load(CURRENT)
    document = load(DOCUMENT)
    integration = load(INTEGRATION)
    current["operational_execution"]["execution_spec_sha256"] = spec_digest
    document["registered_operational_documents"][0]["content_sha256"] = spec_digest
    integration["operational_documents"][0]["content_sha256"] = spec_digest
    write_json(CURRENT, current)
    write_json(DOCUMENT, document)
    write_json(INTEGRATION, integration)


def patch_current_truth() -> None:
    text = TRUTH.read_text(encoding="utf-8")
    text = replace_exact(
        text,
        "import argparse\nimport hashlib\nimport json\nfrom pathlib import Path\nimport sys",
        "import argparse\nfrom datetime import datetime\nimport hashlib\nimport json\nfrom pathlib import Path\nimport subprocess\nimport sys",
    )
    anchor = 'EXPECTED_A0_BRANCH = "codex/hepta-intelligence-a0-authority-gap-closure-20260829"\n'
    constants = anchor + "EXPECTED_READ_ORDER = " + repr(READ_ORDER) + "\nEXPECTED_LIFECYCLE = " + repr(LIFECYCLE) + "\nEXPECTED_CHECK_CLASSIFICATIONS = " + repr(CHECK_CLASSIFICATIONS) + "\n"
    text = replace_exact(text, anchor, constants)
    anchor = "def canonical(value: Any) -> bytes:\n"
    helper = (
        "def parse_utc(value: Any, label: str) -> datetime:\n"
        "    require(isinstance(value, str) and value.endswith(\"Z\"), f\"{label} UTC timestamp\")\n"
        "    try:\n"
        "        return datetime.fromisoformat(value.removesuffix(\"Z\") + \"+00:00\")\n"
        "    except ValueError as exc:\n"
        "        fail(f\"{label} UTC timestamp: {exc}\")\n\n\n"
        + anchor
    )
    text = replace_exact(text, anchor, helper)
    anchor = '    require(current.get("schema") == "hepta_intelligence_current_plan_v2", "current schema")\n'
    insert = anchor + (
        "    snapshot_at = parse_utc(current.get(\"generated_at_utc\"), \"current snapshot\")\n"
        "    require(document.get(\"generated_at_utc\") == current.get(\"generated_at_utc\"), \"document snapshot drift\")\n"
        "    require(integration.get(\"gap_closure_ledger\", {}).get(\"as_of_utc\") == current.get(\"generated_at_utc\"), \"ledger snapshot drift\")\n"
        "    require(current.get(\"session_bootstrap\", {}).get(\"read_order\") == EXPECTED_READ_ORDER, \"read order\")\n"
    )
    text = replace_exact(text, anchor, insert)
    anchor = '    require(operational.get("execution_spec_version") == "1.1.0", "spec version")\n'
    insert = anchor + (
        "    require(operational.get(\"canonical_parent_plan_id\") == current.get(\"canonical\", {}).get(\"plan_id\"), \"spec parent id\")\n"
        "    require(operational.get(\"canonical_parent_plan_version\") == current.get(\"canonical\", {}).get(\"plan_version\"), \"spec parent version\")\n"
    )
    text = replace_exact(text, anchor, insert)
    anchor = '    require(capabilities.get("schema") == "hepta_intelligence_capability_registry_v1", "capability schema")\n'
    insert = anchor + (
        "    require(capabilities.get(\"lifecycle\") == EXPECTED_LIFECYCLE, \"capability lifecycle\")\n"
        "    require(integration.get(\"repository_check_classifications\") == EXPECTED_CHECK_CLASSIFICATIONS, \"check classifications\")\n"
    )
    text = replace_exact(text, anchor, insert)
    anchor = '    for name, entry in by_id.items():\n        require(all_false(entry.get("authority")), f"{name} authority")\n'
    insert = (
        '    for name, entry in by_id.items():\n'
        '        require(entry.get("selected") is False, f"{name} selected")\n'
        '        require(all_false(entry.get("authority")), f"{name} authority")\n'
    )
    text = replace_exact(text, anchor, insert)
    anchor = '    observation = q0.get("evidence_observation", {})\n'
    insert = (
        "    git_dir = ROOT / \".git\"\n"
        "    if git_dir.exists():\n"
        "        commit_time = subprocess.check_output(\n"
        "            [\"git\", \"show\", \"-s\", \"--format=%cI\", \"HEAD\"],\n"
        "            cwd=ROOT, text=True,\n"
        "        ).strip()\n"
        "        require(snapshot_at <= datetime.fromisoformat(commit_time), \"source snapshot later than commit\")\n\n"
        + anchor
    )
    text = replace_exact(text, anchor, insert)
    anchor = '        "claims": current.get("claim_levels"),\n'
    insert = anchor + (
        '        "capability_lifecycle": capabilities["lifecycle"],\n'
        '        "selection": {"selected": False, "selected_capability_ids": []},\n'
        '        "mandatory_read_order": current["session_bootstrap"]["read_order"],\n'
    )
    text = replace_exact(text, anchor, insert)
    TRUTH.write_text(text, encoding="utf-8")


def patch_document_verifier() -> None:
    text = DOC_VERIFY.read_text(encoding="utf-8")
    old = (
        '        INTEGRATION.relative_to(ROOT).as_posix(),\n'
        '        MASTER.relative_to(ROOT).as_posix(),\n'
        '    ], "read order")'
    )
    new = (
        '        INTEGRATION.relative_to(ROOT).as_posix(),\n'
        '        MASTER.relative_to(ROOT).as_posix(),\n'
        '        SPEC.relative_to(ROOT).as_posix(),\n'
        '    ], "read order")'
    )
    text = replace_exact(text, old, new)
    anchor = '    require(operational[0].get("content_sha256") == sha(SPEC), "spec digest")\n'
    insert = anchor + (
        '    require(operational[0].get("canonical_parent_plan_id") == "HEPTA_INTELLIGENCE_MASTER_PLAN_V4", "spec parent id")\n'
        '    require(operational[0].get("canonical_parent_plan_version") == "4.3.0", "spec parent version")\n'
    )
    text = replace_exact(text, anchor, insert)
    DOC_VERIFY.write_text(text, encoding="utf-8")


def patch_master_verifier() -> None:
    text = MASTER_VERIFY.read_text(encoding="utf-8")
    text = replace_exact(
        text,
        'INTEGRATION = PLAN / "HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json"\nAGENTS = PLAN / "AGENTS.md"',
        'INTEGRATION = PLAN / "HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json"\nCAPABILITIES = PLAN / "HEPTA_INTELLIGENCE_CAPABILITY_REGISTRY_V1.json"\nAGENTS = PLAN / "AGENTS.md"',
    )
    text = replace_exact(
        text,
        '    for path in [MASTER, SPEC, CURRENT, DOCUMENT, INTEGRATION, AGENTS, *HISTORICAL]:',
        '    for path in [MASTER, SPEC, CURRENT, DOCUMENT, INTEGRATION, CAPABILITIES, AGENTS, *HISTORICAL]:',
    )
    text = replace_exact(
        text,
        '    integration = load(INTEGRATION)\n    canonical = current.get("canonical", {})',
        '    integration = load(INTEGRATION)\n    capabilities = load(CAPABILITIES)\n    canonical = current.get("canonical", {})',
    )
    anchor = '    require(all_false(integration.get("authority")), "integration authority")\n'
    insert = anchor + (
        "    expected_read_order = " + repr(READ_ORDER) + "\n"
        "    expected_lifecycle = " + repr(LIFECYCLE) + "\n"
        "    expected_classifications = " + repr(CHECK_CLASSIFICATIONS) + "\n"
        '    require(current.get("session_bootstrap", {}).get("read_order") == expected_read_order, "read order")\n'
        '    require(capabilities.get("lifecycle") == expected_lifecycle, "capability lifecycle")\n'
        '    for entry in capabilities.get("capabilities", []):\n'
        '        require(entry.get("selected") is False, f"selected capability: {entry.get(\'capability_id\')}")\n'
        '    require(integration.get("repository_check_classifications") == expected_classifications, "classification registry")\n'
        '    spec_text = SPEC.read_text(encoding="utf-8")\n'
        '    require("Canonical parent plan: `HEPTA_INTELLIGENCE_MASTER_PLAN_V4` version `4.3.0`" in spec_text, "spec parent")\n'
        '    require("PASS_HEPTA_INTELLIGENCE_MASTER_PLAN_V4_3_SOURCE_ONLY" in spec_text, "spec PASS marker")\n'
        '    for item in expected_classifications:\n'
        '        require(item in spec_text, f"spec classification: {item}")\n'
        '    for stale in ' + repr(list(OLD_TO_NEW)) + ':\n'
        '        require(stale not in spec_text, f"stale spec classification: {stale}")\n'
    )
    text = replace_exact(text, anchor, insert)
    MASTER_VERIFY.write_text(text, encoding="utf-8")


def patch_a0_verifier() -> None:
    text = A0_VERIFY.read_text(encoding="utf-8")
    text = replace_exact(
        text,
        "import hashlib\nimport json\nimport os",
        "from datetime import datetime\nimport hashlib\nimport json\nimport os",
    )
    anchor = "def canonical(value: Any) -> bytes:\n"
    helper = (
        "def parse_utc(value: Any, label: str) -> datetime:\n"
        "    require(isinstance(value, str) and value.endswith(\"Z\"), f\"{label} UTC timestamp\")\n"
        "    try:\n"
        "        return datetime.fromisoformat(value.removesuffix(\"Z\") + \"+00:00\")\n"
        "    except ValueError as exc:\n"
        "        fail(f\"{label} UTC timestamp: {exc}\")\n\n\n"
        + anchor
    )
    text = replace_exact(text, anchor, helper)
    anchor = '    require(all_false(current.get("authority")), "current authority must remain false")\n'
    insert = anchor + (
        '    snapshot_at = parse_utc(current.get("generated_at_utc"), "current snapshot")\n'
        '    require(document.get("generated_at_utc") == current.get("generated_at_utc"), "document snapshot drift")\n'
        '    require(integration.get("gap_closure_ledger", {}).get("as_of_utc") == current.get("generated_at_utc"), "ledger snapshot drift")\n'
        '    require(current.get("session_bootstrap", {}).get("read_order")[-1] == "plans/hepta-intelligence/HEPTA_INTELLIGENCE_CONTROLLED_GAP_CLOSURE_EXECUTION_SPEC_V1.md", "spec missing from read order")\n'
    )
    text = replace_exact(text, anchor, insert)
    anchor = '    entries = capabilities.get("capabilities")\n'
    insert = (
        '    expected_lifecycle = ' + repr(LIFECYCLE) + '\n'
        '    require(capabilities.get("lifecycle") == expected_lifecycle, "capability lifecycle drift")\n'
        + anchor
    )
    text = replace_exact(text, anchor, insert)
    anchor = '    for capability_id, entry in by_id.items():\n        require(\n            all_false(entry.get("authority")),\n'
    insert = '    for capability_id, entry in by_id.items():\n        require(entry.get("selected") is False, f"{capability_id} selected before canonical selection")\n        require(\n            all_false(entry.get("authority")),\n'
    text = replace_exact(text, anchor, insert)
    anchor = '        env_sha = os.environ.get("GITHUB_SHA")\n'
    insert = (
        '        commit_time = datetime.fromisoformat(git("show", "-s", "--format=%cI", "HEAD"))\n'
        '        require(snapshot_at <= commit_time, "source snapshot is later than commit")\n'
        + anchor
    )
    text = replace_exact(text, anchor, insert)
    A0_VERIFY.write_text(text, encoding="utf-8")


def patch_a0_workflow() -> None:
    text = A0_WORKFLOW.read_text(encoding="utf-8")
    old = (
        '          curl -fsSL -L "${headers[@]}" "$artifact_url" \\\n'
        '            -o "$API_DIR/source-artifact.zip"\n'
        '          unzip -q "$API_DIR/source-artifact.zip" -d "$SOURCE_DIR"'
    )
    new = (
        '          curl -fsSL -L "${headers[@]}" "$artifact_url" \\\n'
        '            -o "$API_DIR/source-artifact.zip"\n'
        '          python3 - "$API_DIR/artifacts.json" "$API_DIR/source-artifact.zip" "$SOURCE_DIR" <<\'PY\'\n'
        '          from __future__ import annotations\n\n'
        '          import hashlib\n'
        '          import json\n'
        '          import os\n'
        '          from pathlib import Path, PurePosixPath\n'
        '          import stat\n'
        '          import sys\n'
        '          import zipfile\n\n'
        '          metadata = json.load(open(sys.argv[1], encoding="utf-8"))\n'
        '          archive = Path(sys.argv[2])\n'
        '          destination = Path(sys.argv[3])\n'
        '          matches = [item for item in metadata.get("artifacts", []) if item.get("name") == os.environ["SOURCE_ARTIFACT_NAME"]]\n'
        '          assert len(matches) == 1\n'
        '          expected = matches[0].get("digest")\n'
        '          actual = "sha256:" + hashlib.sha256(archive.read_bytes()).hexdigest()\n'
        '          assert expected == actual, (expected, actual)\n'
        '          seen: set[str] = set()\n'
        '          with zipfile.ZipFile(archive) as handle:\n'
        '              infos = handle.infolist()\n'
        '              assert infos\n'
        '              for info in infos:\n'
        '                  name = info.filename\n'
        '                  path = PurePosixPath(name)\n'
        '                  assert name and "\\\\" not in name\n'
        '                  assert not path.is_absolute()\n'
        '                  assert not any(part in {"", ".", ".."} for part in path.parts)\n'
        '                  normalized = path.as_posix()\n'
        '                  assert normalized not in seen\n'
        '                  seen.add(normalized)\n'
        '                  mode = (info.external_attr >> 16) & 0xFFFF\n'
        '                  assert not stat.S_ISLNK(mode)\n'
        '              handle.extractall(destination)\n'
        '          PY'
    )
    text = replace_exact(text, old, new)
    anchor = '                  "source_artifact_digest": source_artifact.get("digest"),\n'
    insert = anchor + '                  "source_artifact_archive_sha256": sha256(api_dir / "source-artifact.zip"),\n'
    text = replace_exact(text, anchor, insert)
    A0_WORKFLOW.write_text(text, encoding="utf-8")


def patch_execution_workflow() -> None:
    text = EXEC_WORKFLOW.read_text(encoding="utf-8")
    anchor = '          spec_text = spec.read_text(encoding="utf-8")\n'
    insert = anchor + (
        '          require("Canonical parent plan: `HEPTA_INTELLIGENCE_MASTER_PLAN_V4` version `4.3.0`" in spec_text, "spec parent drift")\n'
        '          classifications = ' + repr(CHECK_CLASSIFICATIONS) + '\n'
        '          require(integration.get("repository_check_classifications") == classifications, "classification registry drift")\n'
        '          for item in classifications:\n'
        '              require(item in spec_text, f"spec classification missing: {item}")\n'
        '          for stale in ' + repr(list(OLD_TO_NEW)) + ':\n'
        '              require(stale not in spec_text, f"stale spec classification: {stale}")\n'
        '          require(current.get("session_bootstrap", {}).get("read_order")[-1] == spec_rel, "spec missing from read order")\n'
    )
    text = replace_exact(text, anchor, insert)
    EXEC_WORKFLOW.write_text(text, encoding="utf-8")


def final_assertions() -> None:
    current = load(CURRENT)
    document = load(DOCUMENT)
    capabilities = load(CAPABILITIES)
    integration = load(INTEGRATION)
    require(current["session_bootstrap"]["read_order"] == READ_ORDER, "final read order")
    require(current["generated_at_utc"] == SNAPSHOT_AT, "final current timestamp")
    require(document["generated_at_utc"] == SNAPSHOT_AT, "final document timestamp")
    require(integration["gap_closure_ledger"]["as_of_utc"] == SNAPSHOT_AT, "final ledger timestamp")
    require(capabilities["lifecycle"] == LIFECYCLE, "final lifecycle")
    require(all(entry.get("selected") is False for entry in capabilities["capabilities"]), "selected capability")
    require(integration["repository_check_classifications"] == CHECK_CLASSIFICATIONS, "final classifications")
    spec_digest = sha256(SPEC)
    require(current["operational_execution"]["execution_spec_sha256"] == spec_digest, "current spec digest")
    require(document["registered_operational_documents"][0]["content_sha256"] == spec_digest, "document spec digest")
    require(integration["operational_documents"][0]["content_sha256"] == spec_digest, "integration spec digest")
    require(current["canonical"]["content_sha256"] == sha256(MASTER), "master digest")
    datetime.fromisoformat(SNAPSHOT_AT.removesuffix("Z") + "+00:00")


def main() -> int:
    patch_json()
    spec_digest = patch_spec()
    bind_spec_digest(spec_digest)
    patch_current_truth()
    patch_document_verifier()
    patch_master_verifier()
    patch_a0_verifier()
    patch_a0_workflow()
    patch_execution_workflow()
    final_assertions()
    print("PASS_A0_V4_3_SOURCE_REPAIR")
    print(f"spec_sha256={spec_digest}")
    print(f"master_sha256={sha256(MASTER)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
