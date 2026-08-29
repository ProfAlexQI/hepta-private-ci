#!/usr/bin/env python3
from __future__ import annotations

import argparse
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import subprocess
import textwrap
from typing import Any

REPOSITORY = "ProfHepta/hepta-private-ci"
TARGET_BRANCH = "codex/hepta-intelligence-a0-authority-gap-closure-20260829"
TARGET_HEAD = "0b80caff91010f40a79c795c20487ff9d773d229"
TARGET_TREE = "9f67a4892a3474e7f424327ecc46d81a98421cc4"
Q0_HEAD = "c768bcbeb4c1168088d2499828c24da521a2a73a"
Q0_TREE = "ca455a9ef797cd95164c880c7b8faba80b305589"
SPEC_REL = "plans/hepta-intelligence/HEPTA_INTELLIGENCE_CONTROLLED_GAP_CLOSURE_EXECUTION_SPEC_V1.md"
READ_ORDER = [
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_CURRENT_PLAN.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRY_V1.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_EVIDENCE_INDEX_V1.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_CAPABILITY_REGISTRY_V1.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_PR_STACK_REGISTRY_V1.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_MASTER_PLAN.md",
    SPEC_REL,
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
CLASSIFICATIONS = [
    "PASS",
    "INTRODUCED_BY_CANDIDATE",
    "PRE_EXISTING_ON_BASE",
    "MERGE_INTERACTION",
    "RUNNER_OR_PLATFORM_INFRA",
    "CANCELLED_OR_SUPERSEDED",
    "NOT_REQUIRED_BY_SELECTED_POLICY",
    "UNKNOWN_FAIL_CLOSED",
]
OLD_CLASSIFICATIONS = [
    "FAIL_INTRODUCED_BY_CANDIDATE",
    "FAIL_PRESENT_IN_BASE",
    "INFRASTRUCTURE_BLOCKED",
    "CANCELLED_SUPERSEDED",
    "NOT_REQUIRED_FOR_SELECTED_TARGET",
]


def run(*args: str, cwd: Path, capture: bool = True, env: dict[str, str] | None = None) -> str:
    result = subprocess.run(
        list(args),
        cwd=cwd,
        env=env,
        check=True,
        text=True,
        capture_output=capture,
    )
    return result.stdout.strip() if capture else ""


def load(path: Path) -> dict[str, Any]:
    value = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(value, dict):
        raise RuntimeError(f"{path} is not an object")
    return value


def dump(path: Path, value: dict[str, Any]) -> None:
    path.write_text(
        json.dumps(value, indent=2, sort_keys=True, ensure_ascii=False) + "\n",
        encoding="utf-8",
    )


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def replace_once(text: str, old: str, new: str, label: str) -> str:
    count = text.count(old)
    if count != 1:
        raise RuntimeError(f"{label}: expected one match, found {count}")
    return text.replace(old, new, 1)


def deep_replace(value: Any, replacements: dict[str, str]) -> Any:
    if isinstance(value, dict):
        return {key: deep_replace(item, replacements) for key, item in value.items()}
    if isinstance(value, list):
        return [deep_replace(item, replacements) for item in value]
    if isinstance(value, str):
        for old, new in replacements.items():
            value = value.replace(old, new)
    return value


def patch_spec(spec_path: Path) -> str:
    text = spec_path.read_text(encoding="utf-8")
    text = replace_once(
        text,
        "Canonical parent plan: `HEPTA_INTELLIGENCE_MASTER_PLAN_V4` version `4.2.0`",
        "Canonical parent plan: `HEPTA_INTELLIGENCE_MASTER_PLAN_V4` version `4.3.0`",
        "spec parent version",
    )
    text = text.replace(
        "PASS_HEPTA_INTELLIGENCE_MASTER_PLAN_V4_2_SOURCE_ONLY",
        "PASS_HEPTA_INTELLIGENCE_MASTER_PLAN_V4_3_SOURCE_ONLY",
    )
    text = text.replace("Master Plan V4.2.0", "Master Plan V4.3.0")
    old_block = textwrap.dedent(
        """\
        Merge attribution uses `RepositoryCheckAttributionReceiptV1` and classifies each
        required check as PASS, FAIL_INTRODUCED_BY_CANDIDATE, FAIL_PRESENT_IN_BASE,
        INFRASTRUCTURE_BLOCKED, CANCELLED_SUPERSEDED or NOT_REQUIRED_FOR_SELECTED_TARGET.
        Only PASS or independently policy-approved target exclusion can yield merge
        eligibility."""
    )
    new_block = textwrap.dedent(
        """\
        Merge attribution uses `RepositoryCheckAttributionReceiptV1` and classifies each
        required check as PASS, INTRODUCED_BY_CANDIDATE, PRE_EXISTING_ON_BASE,
        MERGE_INTERACTION, RUNNER_OR_PLATFORM_INFRA, CANCELLED_OR_SUPERSEDED,
        NOT_REQUIRED_BY_SELECTED_POLICY or UNKNOWN_FAIL_CLOSED. UNKNOWN_FAIL_CLOSED
        blocks merge. Only PASS or an independently policy-approved target exclusion can
        yield merge eligibility."""
    )
    text = replace_once(text, old_block, new_block, "classification block")
    for stale in OLD_CLASSIFICATIONS:
        if stale in text:
            raise RuntimeError(f"stale spec classification: {stale}")
    for required in CLASSIFICATIONS:
        if required not in text:
            raise RuntimeError(f"missing spec classification: {required}")
    spec_path.write_text(text, encoding="utf-8")
    return sha(spec_path)


def patch_machine_files(root: Path, source_time: str, spec_digest: str) -> None:
    plan = root / "plans" / "hepta-intelligence"
    current_path = plan / "HEPTA_INTELLIGENCE_CURRENT_PLAN.json"
    document_path = plan / "HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRY_V1.json"
    capabilities_path = plan / "HEPTA_INTELLIGENCE_CAPABILITY_REGISTRY_V1.json"
    integration_path = plan / "HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json"
    evidence_path = plan / "HEPTA_INTELLIGENCE_EVIDENCE_INDEX_V1.json"
    pr_stack_path = plan / "HEPTA_INTELLIGENCE_PR_STACK_REGISTRY_V1.json"

    current = load(current_path)
    current["generated_at_utc"] = source_time
    current["session_bootstrap"]["read_order"] = READ_ORDER
    current["current_truth"]["selected"] = False
    current["source_snapshot_policy"]["future_dated_source_snapshot_allowed"] = False
    current["source_snapshot_provenance"] = {
        "basis": "SOURCE_PUBLISHER_WORKFLOW_OBSERVATION",
        "observed_at_utc": source_time,
        "predecessor_head": TARGET_HEAD,
        "future_timestamp_rejected": True,
    }
    current["operational_execution"]["execution_spec_sha256"] = spec_digest
    dump(current_path, current)

    document = load(document_path)
    document["generated_at_utc"] = source_time
    document["registered_operational_documents"][0]["content_sha256"] = spec_digest
    document["source_snapshot_policy"]["future_dated_source_snapshot_allowed"] = False
    document["source_snapshot_policy"]["source_snapshot_observed_at_utc"] = source_time
    dump(document_path, document)

    integration = deep_replace(
        load(integration_path),
        {
            "HEPTA_INTELLIGENCE_MASTER_PLAN.md version 4.2.0":
                "HEPTA_INTELLIGENCE_MASTER_PLAN.md version 4.3.0",
            "Master Plan V4.2.0": "Master Plan V4.3.0",
        },
    )
    integration["gap_closure_ledger"]["as_of_utc"] = source_time
    for item in integration.get("operational_documents", []):
        if item.get("path") == SPEC_REL:
            item["content_sha256"] = spec_digest
    integration["source_snapshot_provenance"] = {
        "basis": "SOURCE_PUBLISHER_WORKFLOW_OBSERVATION",
        "observed_at_utc": source_time,
        "predecessor_head": TARGET_HEAD,
        "future_timestamp_rejected": True,
    }
    dump(integration_path, integration)

    capabilities = load(capabilities_path)
    capabilities["as_of_utc"] = source_time
    capabilities["lifecycle"] = LIFECYCLE
    capabilities.setdefault("invariants", {})["selected_requires_candidate_qualified"] = True
    capabilities["invariants"]["wired_requires_selected"] = True
    for entry in capabilities["capabilities"]:
        entry["selected"] = False
    dump(capabilities_path, capabilities)

    old_spec_digest = "7e4caaf7ea096e826617e993a669a68a3f05629297dfa13a645bd4e4afa4df01"
    for path in (current_path, document_path, integration_path, evidence_path, pr_stack_path):
        value = deep_replace(load(path), {old_spec_digest: spec_digest})
        if path in (evidence_path, pr_stack_path) and "as_of_utc" in value:
            value["as_of_utc"] = source_time
        dump(path, value)


def patch_truth(root: Path) -> None:
    path = root / "scripts" / "hepta-intelligence-current-truth.py"
    text = path.read_text(encoding="utf-8")
    text = replace_once(
        text,
        "import argparse\nimport hashlib\nimport json\nfrom pathlib import Path\nimport sys",
        "import argparse\nfrom datetime import datetime\nimport hashlib\nimport json\nfrom pathlib import Path\nimport subprocess\nimport sys",
        "truth imports",
    )
    text = replace_once(
        text,
        'EXPECTED_A0_BRANCH = "codex/hepta-intelligence-a0-authority-gap-closure-20260829"\n',
        'EXPECTED_A0_BRANCH = "codex/hepta-intelligence-a0-authority-gap-closure-20260829"\n'
        f'EXPECTED_READ_ORDER = {READ_ORDER!r}\n'
        f'EXPECTED_LIFECYCLE = {LIFECYCLE!r}\n',
        "truth constants",
    )
    text = replace_once(
        text,
        "def canonical(value: Any) -> bytes:\n",
        "def parse_utc(value: Any, label: str) -> datetime:\n"
        "    require(isinstance(value, str) and value.endswith(\"Z\"), f\"{label} timestamp\")\n"
        "    try:\n"
        "        return datetime.fromisoformat(value.replace(\"Z\", \"+00:00\"))\n"
        "    except ValueError as exc:\n"
        "        fail(f\"{label} timestamp: {exc}\")\n\n\n"
        "def canonical(value: Any) -> bytes:\n",
        "truth timestamp helper",
    )
    text = replace_once(
        text,
        '    require(all_false(current.get("authority")), "current authority")\n',
        '    require(all_false(current.get("authority")), "current authority")\n'
        '    require(current.get("session_bootstrap", {}).get("read_order") == EXPECTED_READ_ORDER, "read order")\n'
        '    require(current.get("current_truth", {}).get("selected") is False, "selection fabricated")\n'
        '    snapshot_time = parse_utc(current.get("generated_at_utc"), "current")\n',
        "truth current checks",
    )
    text = replace_once(
        text,
        '    require(all_false(document.get("authority")), "document authority")\n',
        '    require(all_false(document.get("authority")), "document authority")\n'
        '    require(parse_utc(document.get("generated_at_utc"), "document") == snapshot_time, "document timestamp drift")\n',
        "truth document time",
    )
    text = replace_once(
        text,
        '    require(len(by_id) == len(entries), "duplicate capability")\n',
        '    require(len(by_id) == len(entries), "duplicate capability")\n'
        '    require(capabilities.get("lifecycle") == EXPECTED_LIFECYCLE, "capability lifecycle")\n'
        '    for capability_id, entry in by_id.items():\n'
        '        require(entry.get("selected") is False, f"{capability_id} selected")\n'
        '        if entry.get("wired") is True:\n'
        '            require(entry.get("selected") is True, f"{capability_id} wired without selection")\n',
        "truth lifecycle",
    )
    text = replace_once(
        text,
        '    require(all_false(integration.get("authority")), "integration authority")\n',
        '    require(all_false(integration.get("authority")), "integration authority")\n'
        '    require(parse_utc(integration.get("gap_closure_ledger", {}).get("as_of_utc"), "integration") == snapshot_time, "integration timestamp drift")\n'
        '    if (ROOT / ".git").exists():\n'
        '        commit_time = datetime.fromisoformat(subprocess.check_output(["git", "show", "-s", "--format=%cI", "HEAD"], cwd=ROOT, text=True).strip())\n'
        '        require(snapshot_time <= commit_time, "future-dated source snapshot")\n',
        "truth temporal check",
    )
    text = replace_once(
        text,
        '        "active_phase": current["active_phase"],\n',
        '        "active_phase": current["active_phase"],\n'
        '        "capability_lifecycle": capabilities["lifecycle"],\n'
        '        "selection": {"selected": False, "canonical_selection_receipt_present": False},\n',
        "truth selection output",
    )
    path.write_text(text, encoding="utf-8")


def patch_document_verifier(root: Path) -> None:
    path = root / "scripts" / "verify-hepta-intelligence-document-authority.py"
    text = path.read_text(encoding="utf-8")
    text = replace_once(
        text,
        "import hashlib\nimport json\nfrom pathlib import Path\nimport sys",
        "from datetime import datetime\nimport hashlib\nimport json\nfrom pathlib import Path\nimport subprocess\nimport sys",
        "document imports",
    )
    text = replace_once(
        text,
        'STATUS_V3 = PLAN / "HEPTA_INTELLIGENCE_EXECUTION_STATUS_V3.json"\n',
        'STATUS_V3 = PLAN / "HEPTA_INTELLIGENCE_EXECUTION_STATUS_V3.json"\n'
        f'EXPECTED_READ_ORDER = {READ_ORDER!r}\n'
        f'EXPECTED_LIFECYCLE = {LIFECYCLE!r}\n',
        "document constants",
    )
    text = replace_once(
        text,
        '    document = load(DOCUMENT)\n',
        '    document = load(DOCUMENT)\n    capabilities = load(CAPABILITIES)\n',
        "document capabilities",
    )
    old_read = textwrap.dedent(
        '''\
            require(current.get("session_bootstrap", {}).get("read_order") == [
                CURRENT.relative_to(ROOT).as_posix(),
                DOCUMENT.relative_to(ROOT).as_posix(),
                EVIDENCE.relative_to(ROOT).as_posix(),
                CAPABILITIES.relative_to(ROOT).as_posix(),
                PR_STACK.relative_to(ROOT).as_posix(),
                INTEGRATION.relative_to(ROOT).as_posix(),
                MASTER.relative_to(ROOT).as_posix(),
            ], "read order")
        '''
    )
    text = replace_once(
        text,
        old_read,
        '    require(current.get("session_bootstrap", {}).get("read_order") == EXPECTED_READ_ORDER, "read order")\n',
        "document read order",
    )
    text = replace_once(
        text,
        '    require(operational[0].get("content_sha256") == sha(SPEC), "spec digest")\n',
        '    require(operational[0].get("content_sha256") == sha(SPEC), "spec digest")\n'
        '    spec_text = SPEC.read_text(encoding="utf-8")\n'
        '    require("Canonical parent plan: `HEPTA_INTELLIGENCE_MASTER_PLAN_V4` version `4.3.0`" in spec_text, "spec parent version")\n'
        '    require("PASS_HEPTA_INTELLIGENCE_MASTER_PLAN_V4_3_SOURCE_ONLY" in spec_text, "spec marker")\n'
        f'    require(all(item in spec_text for item in {CLASSIFICATIONS!r}), "spec classifications")\n'
        f'    require(not any(item in spec_text for item in {OLD_CLASSIFICATIONS!r}), "stale spec classifications")\n'
        '    require(capabilities.get("lifecycle") == EXPECTED_LIFECYCLE, "capability lifecycle")\n'
        '    require(all(entry.get("selected") is False for entry in capabilities.get("capabilities", [])), "selected capability")\n'
        '    snapshot = datetime.fromisoformat(current["generated_at_utc"].replace("Z", "+00:00"))\n'
        '    require(document.get("generated_at_utc") == current.get("generated_at_utc"), "snapshot time drift")\n'
        '    if (ROOT / ".git").exists():\n'
        '        commit_time = datetime.fromisoformat(subprocess.check_output(["git", "show", "-s", "--format=%cI", "HEAD"], cwd=ROOT, text=True).strip())\n'
        '        require(snapshot <= commit_time, "future-dated source snapshot")\n',
        "document semantic checks",
    )
    path.write_text(text, encoding="utf-8")


def patch_master_verifier(root: Path) -> None:
    path = root / "scripts" / "verify-hepta-intelligence-master-plan.py"
    text = path.read_text(encoding="utf-8")
    text = replace_once(
        text,
        "import hashlib\nimport json\nfrom pathlib import Path\nimport sys",
        "from datetime import datetime\nimport hashlib\nimport json\nfrom pathlib import Path\nimport subprocess\nimport sys",
        "master imports",
    )
    text = replace_once(
        text,
        'INTEGRATION = PLAN / "HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json"\n',
        'INTEGRATION = PLAN / "HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json"\nCAPABILITIES = PLAN / "HEPTA_INTELLIGENCE_CAPABILITY_REGISTRY_V1.json"\n',
        "master capability constant",
    )
    text = replace_once(
        text,
        '    for path in [MASTER, SPEC, CURRENT, DOCUMENT, INTEGRATION, AGENTS, *HISTORICAL]:\n',
        '    for path in [MASTER, SPEC, CURRENT, DOCUMENT, INTEGRATION, CAPABILITIES, AGENTS, *HISTORICAL]:\n',
        "master paths",
    )
    text = replace_once(
        text,
        '    integration = load(INTEGRATION)\n',
        '    integration = load(INTEGRATION)\n    capabilities = load(CAPABILITIES)\n',
        "master capability load",
    )
    text = replace_once(
        text,
        '    require(operational.get("execution_spec_version") == "1.1.0", "spec version")\n',
        '    require(operational.get("execution_spec_version") == "1.1.0", "spec version")\n'
        f'    expected_read_order = {READ_ORDER!r}\n'
        f'    expected_lifecycle = {LIFECYCLE!r}\n'
        '    require(current.get("session_bootstrap", {}).get("read_order") == expected_read_order, "read order")\n'
        '    require(capabilities.get("lifecycle") == expected_lifecycle, "capability lifecycle")\n'
        '    require(all(entry.get("selected") is False for entry in capabilities.get("capabilities", [])), "selected capability")\n'
        '    require(current.get("current_truth", {}).get("selected") is False, "current selection")\n',
        "master contracts",
    )
    text = replace_once(
        text,
        '    text = MASTER.read_text(encoding="utf-8")\n',
        '    spec_text = SPEC.read_text(encoding="utf-8")\n'
        '    require("Canonical parent plan: `HEPTA_INTELLIGENCE_MASTER_PLAN_V4` version `4.3.0`" in spec_text, "spec parent version")\n'
        '    require("PASS_HEPTA_INTELLIGENCE_MASTER_PLAN_V4_3_SOURCE_ONLY" in spec_text, "spec marker")\n'
        f'    require(all(item in spec_text for item in {CLASSIFICATIONS!r}), "spec classifications")\n'
        f'    require(not any(item in spec_text for item in {OLD_CLASSIFICATIONS!r}), "stale spec classifications")\n'
        '    snapshot = datetime.fromisoformat(current["generated_at_utc"].replace("Z", "+00:00"))\n'
        '    require(document.get("generated_at_utc") == current.get("generated_at_utc"), "document timestamp drift")\n'
        '    require(integration.get("gap_closure_ledger", {}).get("as_of_utc") == current.get("generated_at_utc"), "integration timestamp drift")\n'
        '    if (ROOT / ".git").exists():\n'
        '        commit_time = datetime.fromisoformat(subprocess.check_output(["git", "show", "-s", "--format=%cI", "HEAD"], cwd=ROOT, text=True).strip())\n'
        '        require(snapshot <= commit_time, "future-dated source snapshot")\n'
        '    text = MASTER.read_text(encoding="utf-8")\n'
        '    ordered_names = [Path(item).name for item in expected_read_order]\n'
        '    positions = [text.find(item) for item in ordered_names]\n'
        '    require(all(position >= 0 for position in positions) and positions == sorted(positions), "master read order")\n',
        "master semantics",
    )
    path.write_text(text, encoding="utf-8")


def patch_a0_verifier(root: Path) -> None:
    path = root / "scripts" / "verify-hepta-intelligence-a0-authority.py"
    text = path.read_text(encoding="utf-8")
    text = replace_once(
        text,
        '    require(\n        current.get("current_truth", {}).get("wired") is False,\n        "runtime wiring unexpectedly enabled",\n    )\n',
        '    require(\n        current.get("current_truth", {}).get("selected") is False,\n        "canonical selection was fabricated",\n    )\n'
        '    require(\n        current.get("current_truth", {}).get("wired") is False,\n        "runtime wiring unexpectedly enabled",\n    )\n',
        "A0 selected state",
    )
    text = replace_once(
        text,
        '    require(len(by_id) == len(entries), "duplicate or malformed capability")\n',
        '    require(len(by_id) == len(entries), "duplicate or malformed capability")\n'
        f'    require(capabilities.get("lifecycle") == {LIFECYCLE!r}, "capability lifecycle drift")\n'
        '    for capability_id, entry in by_id.items():\n'
        '        require(entry.get("selected") is False, f"{capability_id} selected before canonical selection")\n',
        "A0 lifecycle",
    )
    text = replace_once(
        text,
        '        "a0_candidate_qualified": False,\n',
        '        "a0_candidate_qualified": False,\n        "selected": False,\n',
        "A0 source receipt selected",
    )
    path.write_text(text, encoding="utf-8")


def patch_execution_workflow(root: Path) -> None:
    path = root / ".github" / "workflows" / "hepta-intelligence-execution-spec.yml"
    text = path.read_text(encoding="utf-8")
    text = replace_once(
        text,
        '          integration = load("HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json")\n',
        '          integration = load("HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json")\n          capabilities = load("HEPTA_INTELLIGENCE_CAPABILITY_REGISTRY_V1.json")\n',
        "execution capabilities",
    )
    text = replace_once(
        text,
        '              ("integration", integration.get("authority")),\n',
        '              ("integration", integration.get("authority")),\n              ("capabilities", capabilities.get("authority")),\n',
        "execution capability authority",
    )
    text = replace_once(
        text,
        '          spec_text = spec.read_text(encoding="utf-8")\n',
        f'          require(current.get("session_bootstrap", {{}}).get("read_order") == {READ_ORDER!r}, "read order drift")\n'
        f'          require(capabilities.get("lifecycle") == {LIFECYCLE!r}, "capability lifecycle drift")\n'
        '          require(all(entry.get("selected") is False for entry in capabilities.get("capabilities", [])), "selected capability")\n'
        '          spec_text = spec.read_text(encoding="utf-8")\n'
        '          require("Canonical parent plan: `HEPTA_INTELLIGENCE_MASTER_PLAN_V4` version `4.3.0`" in spec_text, "spec parent version")\n'
        '          require("PASS_HEPTA_INTELLIGENCE_MASTER_PLAN_V4_3_SOURCE_ONLY" in spec_text, "spec marker")\n'
        f'          require(all(item in spec_text for item in {CLASSIFICATIONS!r}), "spec classifications")\n'
        f'          require(not any(item in spec_text for item in {OLD_CLASSIFICATIONS!r}), "stale spec classifications")\n',
        "execution semantics",
    )
    path.write_text(text, encoding="utf-8")


def patch_a0_workflow(root: Path) -> None:
    path = root / ".github" / "workflows" / "hepta-intelligence-a0-authority.yml"
    text = path.read_text(encoding="utf-8")
    old_extract = textwrap.dedent(
        '''\
                  curl -fsSL -L "${headers[@]}" "$artifact_url" \\
                    -o "$API_DIR/source-artifact.zip"
                  unzip -q "$API_DIR/source-artifact.zip" -d "$SOURCE_DIR"
        '''
    )
    new_extract = textwrap.dedent(
        '''\
                  curl -fsSL -L "${headers[@]}" "$artifact_url" \\
                    -o "$API_DIR/source-artifact.zip"
                  python3 - "$API_DIR/source-artifact.zip" "$SOURCE_DIR" "$API_DIR/artifacts.json" <<'PY'
                  from __future__ import annotations

                  import hashlib
                  import json
                  import os
                  from pathlib import Path, PurePosixPath
                  import re
                  import stat
                  import sys
                  import zipfile

                  archive = Path(sys.argv[1])
                  destination = Path(sys.argv[2])
                  artifacts = json.loads(Path(sys.argv[3]).read_text(encoding="utf-8")).get("artifacts", [])
                  matches = [item for item in artifacts if item.get("name") == os.environ["SOURCE_ARTIFACT_NAME"]]
                  if len(matches) != 1:
                      raise SystemExit("FAIL_HEPTA_INTELLIGENCE_A0_EXECUTABLE: source artifact metadata ambiguity")
                  expected = str(matches[0].get("digest", ""))
                  actual = "sha256:" + hashlib.sha256(archive.read_bytes()).hexdigest()
                  if expected != actual:
                      raise SystemExit("FAIL_HEPTA_INTELLIGENCE_A0_EXECUTABLE: source archive digest mismatch")

                  destination.mkdir(parents=True, exist_ok=True)
                  seen: set[str] = set()
                  with zipfile.ZipFile(archive) as bundle:
                      for info in bundle.infolist():
                          raw = info.filename.replace("\\\\", "/")
                          if not raw or raw.startswith("/") or re.match(r"^[A-Za-z]:", raw):
                              raise SystemExit("FAIL_HEPTA_INTELLIGENCE_A0_EXECUTABLE: unsafe archive path")
                          pure = PurePosixPath(raw)
                          if any(part in {"", ".", ".."} for part in pure.parts):
                              raise SystemExit("FAIL_HEPTA_INTELLIGENCE_A0_EXECUTABLE: unsafe archive component")
                          normalized = pure.as_posix().rstrip("/")
                          if normalized in seen:
                              raise SystemExit("FAIL_HEPTA_INTELLIGENCE_A0_EXECUTABLE: duplicate archive target")
                          seen.add(normalized)
                          if stat.S_ISLNK(info.external_attr >> 16):
                              raise SystemExit("FAIL_HEPTA_INTELLIGENCE_A0_EXECUTABLE: archive symlink rejected")
                          target = destination.joinpath(*pure.parts)
                          if info.is_dir():
                              target.mkdir(parents=True, exist_ok=True)
                              continue
                          target.parent.mkdir(parents=True, exist_ok=True)
                          with bundle.open(info) as source, target.open("xb") as output:
                              while True:
                                  chunk = source.read(1024 * 1024)
                                  if not chunk:
                                      break
                                  output.write(chunk)
                  PY
        '''
    )
    text = replace_once(text, old_extract, new_extract, "safe archive extraction")
    text = replace_once(
        text,
        '                  "artifacts_metadata_sha256": sha256(api_dir / "artifacts.json"),\n',
        '                  "artifacts_metadata_sha256": sha256(api_dir / "artifacts.json"),\n                  "source_artifact_archive_sha256": sha256(api_dir / "source-artifact.zip"),\n',
        "archive evidence",
    )
    text = replace_once(
        text,
        '          require(source_receipt.get("a0_candidate_qualified") is False, "source receipt self-qualified")\n',
        '          require(source_receipt.get("a0_candidate_qualified") is False, "source receipt self-qualified")\n          require(source_receipt.get("selected") is False, "source receipt fabricated selection")\n',
        "source selected check",
    )
    text = replace_once(
        text,
        '          require(truth.get("q0", {}).get("runtime_capability_qualified") is False, "current truth crossed runtime boundary")\n',
        '          require(truth.get("q0", {}).get("runtime_capability_qualified") is False, "current truth crossed runtime boundary")\n          require(truth.get("selection", {}).get("selected") is False, "current truth fabricated selection")\n',
        "truth selected check",
    )
    path.write_text(text, encoding="utf-8")


def build(root: Path, report_dir: Path) -> tuple[str, str, str]:
    source_time = datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")
    plan = root / "plans" / "hepta-intelligence"
    spec = plan / "HEPTA_INTELLIGENCE_CONTROLLED_GAP_CLOSURE_EXECUTION_SPEC_V1.md"
    master = plan / "HEPTA_INTELLIGENCE_MASTER_PLAN.md"

    if run("git", "rev-parse", "HEAD", cwd=root) != TARGET_HEAD:
        raise RuntimeError("target head drift")
    if run("git", "rev-parse", "HEAD^{tree}", cwd=root) != TARGET_TREE:
        raise RuntimeError("target tree drift")
    if run("git", "rev-parse", "HEAD^", cwd=root) != Q0_HEAD:
        raise RuntimeError("target parent drift")

    spec_digest = patch_spec(spec)
    patch_machine_files(root, source_time, spec_digest)
    patch_truth(root)
    patch_document_verifier(root)
    patch_master_verifier(root)
    patch_a0_verifier(root)
    patch_execution_workflow(root)
    patch_a0_workflow(root)

    # Re-propagate final spec digest.
    spec_digest = sha(spec)
    current_path = plan / "HEPTA_INTELLIGENCE_CURRENT_PLAN.json"
    document_path = plan / "HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRY_V1.json"
    integration_path = plan / "HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json"
    current = load(current_path)
    current["operational_execution"]["execution_spec_sha256"] = spec_digest
    dump(current_path, current)
    document = load(document_path)
    document["registered_operational_documents"][0]["content_sha256"] = spec_digest
    dump(document_path, document)
    integration = load(integration_path)
    for item in integration.get("operational_documents", []):
        if item.get("path") == SPEC_REL:
            item["content_sha256"] = spec_digest
    dump(integration_path, integration)

    capabilities = load(plan / "HEPTA_INTELLIGENCE_CAPABILITY_REGISTRY_V1.json")
    assert sha(master) == current["canonical"]["content_sha256"]
    assert sha(master) == document["current_plan_authority"]["human_plan_content_sha256"]
    assert current["session_bootstrap"]["read_order"] == READ_ORDER
    assert capabilities["lifecycle"] == LIFECYCLE
    assert all(entry["selected"] is False for entry in capabilities["capabilities"])
    assert current["generated_at_utc"] == source_time
    assert document["generated_at_utc"] == source_time
    assert integration["gap_closure_ledger"]["as_of_utc"] == source_time

    paths = [
        ".github/workflows/hepta-intelligence-a0-authority.yml",
        ".github/workflows/hepta-intelligence-execution-spec.yml",
        "plans/hepta-intelligence/HEPTA_INTELLIGENCE_CAPABILITY_REGISTRY_V1.json",
        "plans/hepta-intelligence/HEPTA_INTELLIGENCE_CURRENT_PLAN.json",
        "plans/hepta-intelligence/HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRY_V1.json",
        "plans/hepta-intelligence/HEPTA_INTELLIGENCE_EVIDENCE_INDEX_V1.json",
        "plans/hepta-intelligence/HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json",
        "plans/hepta-intelligence/HEPTA_INTELLIGENCE_PR_STACK_REGISTRY_V1.json",
        SPEC_REL,
        "scripts/hepta-intelligence-current-truth.py",
        "scripts/verify-hepta-intelligence-a0-authority.py",
        "scripts/verify-hepta-intelligence-document-authority.py",
        "scripts/verify-hepta-intelligence-master-plan.py",
    ]
    run("git", "add", *paths, cwd=root, capture=False)
    tree = run("git", "write-tree", cwd=root)
    message = (
        "fix(intelligence): close A0 v4.3 review findings\n\n"
        "Align the subordinate specification with V4.3, establish one eight-item "
        "bootstrap order, add the selected lifecycle state, reject future-dated "
        "source snapshots, and bind safe artifact archive verification/extraction. "
        "Preserve the exact 17-path A0 source surface and every negative authority."
    )
    commit = subprocess.run(
        ["git", "commit-tree", tree, "-p", Q0_HEAD],
        cwd=root,
        input=message + "\n",
        text=True,
        check=True,
        capture_output=True,
    ).stdout.strip()
    run("git", "reset", "--hard", commit, cwd=root, capture=False)

    run(
        "python3", "-m", "py_compile",
        "scripts/hepta-intelligence-current-truth.py",
        "scripts/verify-hepta-intelligence-a0-authority.py",
        "scripts/verify-hepta-intelligence-document-authority.py",
        "scripts/verify-hepta-intelligence-master-plan.py",
        cwd=root,
        capture=False,
    )
    run("python3", "scripts/verify-hepta-intelligence-master-plan.py", cwd=root, capture=False)
    run("python3", "scripts/verify-hepta-intelligence-document-authority.py", cwd=root, capture=False)
    run("python3", "scripts/hepta-intelligence-current-truth.py", "--verify", cwd=root, capture=False)
    env = os.environ.copy()
    env.update({
        "GITHUB_SHA": commit,
        "GITHUB_HEAD_REF": TARGET_BRANCH,
        "GITHUB_REF_NAME": TARGET_BRANCH,
        "GITHUB_REPOSITORY": REPOSITORY,
    })
    source_receipt = run(
        "python3", "scripts/verify-hepta-intelligence-a0-authority.py",
        cwd=root,
        env=env,
    )
    run("git", "diff", "--check", Q0_HEAD, commit, cwd=root, capture=False)
    changed = sorted(run("git", "diff", "--name-only", Q0_HEAD, commit, cwd=root).splitlines())
    expected = load(integration_path)["allowed_changed_paths"]
    if changed != expected or len(changed) != 17:
        raise RuntimeError(f"changed path drift: {changed}")

    report_dir.mkdir(parents=True, exist_ok=True)
    (report_dir / "a0-source-receipt.json").write_text(source_receipt + "\n", encoding="utf-8")
    receipt: dict[str, Any] = {
        "schema": "hepta_a0_v4_3_source_replacement_v1",
        "status": "PASS_SOURCE_PUBLISHER_STAGING",
        "repository": REPOSITORY,
        "candidate": {
            "branch": TARGET_BRANCH,
            "head": commit,
            "tree": tree,
            "parent": Q0_HEAD,
        },
        "source_snapshot_observed_at_utc": source_time,
        "master_sha256": sha(master),
        "execution_spec_sha256": sha(spec),
        "resolved_findings": [
            "A0-RV-002_V4_3_SPEC_ALIGNMENT",
            "A0-RV-003_EIGHT_ITEM_BOOTSTRAP_ORDER",
            "A0-RV-005_NON_FUTURE_SOURCE_SNAPSHOT",
            "A0-RV-006_SELECTED_LIFECYCLE",
            "A0-NB-ARCHIVE_DIGEST_AND_SAFE_EXTRACTION",
        ],
        "source_writeback_to_canonical_branch": False,
        "authority": {
            "runtime_wired": False,
            "production_authority": False,
            "operator_acceptance": False,
            "promotion": False,
            "release_authority": False,
            "callers_ratchet": False,
        },
    }
    receipt["receipt_binding_sha256"] = hashlib.sha256(
        json.dumps(receipt, sort_keys=True, separators=(",", ":")).encode()
    ).hexdigest()
    dump(report_dir / "candidate.json", receipt)
    (report_dir / "commit.txt").write_text(commit + "\n", encoding="utf-8")
    (report_dir / "tree.txt").write_text(tree + "\n", encoding="utf-8")
    return commit, tree, source_time


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, required=True)
    parser.add_argument("--report-dir", type=Path, required=True)
    args = parser.parse_args()
    commit, tree, source_time = build(args.root.resolve(), args.report_dir.resolve())
    print(f"PASS_HEPTA_A0_V4_3_SOURCE_PUBLISHER commit={commit} tree={tree} source_time={source_time}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
