#!/usr/bin/env python3
"""Finalize P1.1c source qualification with explicit governance state flow."""

from __future__ import annotations

import hashlib
import importlib.util
import json
import os
from pathlib import Path

ROOT = Path.cwd()
LEGACY_PATH = ROOT / "scripts/qualification/finalize-hepta-intelligence-p1-1c-source.py"


def load_helpers():
    spec = importlib.util.spec_from_file_location("p1c_finalize_helpers", LEGACY_PATH)
    if spec is None or spec.loader is None:
        raise SystemExit("unable to load P1.1c finalization helpers")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def replace_once(path: Path, old: str, new: str, label: str) -> None:
    text = path.read_text(encoding="utf-8")
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"{label}: expected one anchor in {path}, found {count}")
    path.write_text(text.replace(old, new, 1), encoding="utf-8")


def required_env(name: str) -> str:
    value = os.environ.get(name, "").strip()
    if not value:
        raise SystemExit(f"missing required environment variable {name}")
    return value


def correct_verifier_state_flow(verifier_path: Path) -> None:
    replace_once(
        verifier_path,
        "def verify_governance() -> None:\n",
        "def verify_governance() -> bool:\n",
        "governance-return-type",
    )
    replace_once(
        verifier_path,
        '''    else:
        if status["status"] != "IMPLEMENTED_PENDING_EXECUTABLE_QUALIFICATION":
            fail("pending P1.1c source must keep pending execution status")
        if EVIDENCE.exists():
            fail("pending P1.1c source must not carry qualification evidence")


def git_head() -> str:
''',
        '''    else:
        if status["status"] != "IMPLEMENTED_PENDING_EXECUTABLE_QUALIFICATION":
            fail("pending P1.1c source must keep pending execution status")
        if EVIDENCE.exists():
            fail("pending P1.1c source must not carry qualification evidence")
    return source_qualified


def git_head() -> str:
''',
        "governance-return-value",
    )
    replace_once(
        verifier_path,
        "    verify_governance()\n    receipt = {\n",
        "    source_qualified = verify_governance()\n    receipt = {\n",
        "governance-main-binding",
    )
    replace_once(
        verifier_path,
        '        "source_qualified": status["qualification"]["source_qualified"],\n',
        '        "source_qualified": source_qualified,\n',
        "source-qualified-main-receipt",
    )


def main() -> None:
    helpers = load_helpers()
    source_parent = required_env("SOURCE_PARENT")
    run_id = int(required_env("RUN_ID"))
    run_attempt = int(required_env("RUN_ATTEMPT"))
    receipt_path = Path(required_env("EVALUATION_RECEIPT_PATH"))
    receipt_bytes = receipt_path.read_bytes()
    receipt_sha256 = hashlib.sha256(receipt_bytes).hexdigest()
    if receipt_sha256 != required_env("EVALUATION_RECEIPT_SHA256"):
        raise SystemExit("evaluation receipt digest changed before finalization")

    helpers.update_status(source_parent, run_id, receipt_sha256)
    helpers.write_evidence(source_parent, run_id, run_attempt, receipt_sha256)
    helpers.patch_verifier()
    correct_verifier_state_flow(helpers.VERIFIER_PATH)
    helpers.patch_workflow_allowlist()

    print(
        json.dumps(
            {
                "status": "PASS_P1_1C_SOURCE_FINALIZATION_PATCH_V2",
                "source_parent": source_parent,
                "qualification_run_id": run_id,
                "evaluation_receipt_sha256": receipt_sha256,
                "governance_paths": [
                    str(helpers.STATUS_PATH.relative_to(ROOT)),
                    str(helpers.EVIDENCE_PATH.relative_to(ROOT)),
                    str(helpers.VERIFIER_PATH.relative_to(ROOT)),
                    str(helpers.WORKFLOW_PATH.relative_to(ROOT)),
                ],
            },
            indent=2,
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
