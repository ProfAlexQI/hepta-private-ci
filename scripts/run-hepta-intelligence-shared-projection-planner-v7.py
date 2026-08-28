#!/usr/bin/env python3
"""Execute every P0.3.2 exact-head qualification gate and persist one receipt."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path
from typing import Sequence

ROOT = Path(__file__).resolve().parent.parent
WORKSPACE = ROOT / "codex-rs"
OUT = ROOT / "artifacts" / "hepta-intelligence-shared-projection-planner-v7"
LOGS = OUT / "logs"

RUST_FILES = [
    "hepta-memory/src/cognitive_store.rs",
    "hepta-memory/src/cognitive_kg_store.rs",
    "hepta-memory/src/cognitive_projection_planner.rs",
    "hepta-memory/src/cognitive_test_support.rs",
    "hepta-memory/src/fact_grounding.rs",
    "hepta-memory/src/fact_grounding/durable.rs",
    "hepta-memory/src/fact_grounding/durable/grounding.rs",
    "hepta-memory/src/fact_grounding/durable/grounding/prepare.rs",
    "hepta-memory/src/fact_grounding/durable/grounding/ledger.rs",
    "hepta-memory/src/fact_grounding/durable/grounding/ledger/insert.rs",
    "hepta-memory/src/fact_grounding/durable/grounding/ledger/support.rs",
    "hepta-memory/src/fact_grounding/durable/grounding/ledger/verify.rs",
    "hepta-memory/src/fact_grounding/durable/schema.rs",
    "hepta-memory/src/fact_grounding/durable/tests.rs",
    "hepta-memory/src/fact_grounding/shadow_projection_gate.rs",
    "hepta-memory/src/framing.rs",
]


def run(check_id: str, command: Sequence[str], cwd: Path = ROOT) -> dict[str, object]:
    result = subprocess.run(
        list(command),
        cwd=cwd,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    LOGS.mkdir(parents=True, exist_ok=True)
    log = LOGS / f"{check_id}.log"
    text = result.stdout + result.stderr
    log.write_text(text, encoding="utf-8")
    return {
        "id": check_id,
        "command": list(command),
        "exit_code": result.returncode,
        "passed": result.returncode == 0,
        "log": str(log.relative_to(OUT)),
        "log_tail": "\n".join(text.splitlines()[-100:])[-8000:],
    }


def main() -> int:
    OUT.mkdir(parents=True, exist_ok=True)
    LOGS.mkdir(parents=True, exist_ok=True)

    head = subprocess.run(
        ["git", "rev-parse", "HEAD"],
        cwd=ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=True,
    ).stdout.strip()
    tree = subprocess.run(
        ["git", "rev-parse", "HEAD^{tree}"],
        cwd=ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=True,
    ).stdout.strip()

    checks = [
        run(
            "python_compile",
            [
                sys.executable,
                "-m",
                "py_compile",
                "scripts/verify-hepta-intelligence-shared-projection-planner-v5.py",
                "scripts/check-hepta-intelligence-p0-3-2-clippy.py",
                "scripts/run-hepta-intelligence-shared-projection-planner-v7.py",
            ],
        ),
        run(
            "source_contract",
            [
                sys.executable,
                "scripts/verify-hepta-intelligence-shared-projection-planner-v5.py",
            ],
        ),
        run(
            "test_root_contract",
            [
                sys.executable,
                "-c",
                (
                    "from pathlib import Path; "
                    "helper=Path('codex-rs/hepta-memory/src/cognitive_test_support.rs')"
                    ".read_text(encoding='utf-8'); "
                    "store=Path('codex-rs/hepta-memory/src/cognitive_store.rs')"
                    ".read_text(encoding='utf-8'); "
                    "assert '.canonicalize()' in helper; "
                    "assert 'canonicalize fleet root for platform-stable tests' in helper; "
                    "assert 'per-agent cognitive root must be canonical and must not traverse a symlink' in store"
                ),
            ],
        ),
        run(
            "candidate_rustfmt",
            ["rustfmt", "--edition", "2024", "--check", *RUST_FILES],
            WORKSPACE,
        ),
        run(
            "planner_tests",
            [
                "cargo",
                "test",
                "-p",
                "codex-hepta-memory",
                "cognitive_projection_planner",
                "--",
                "--nocapture",
            ],
            WORKSPACE,
        ),
        run(
            "shadow_tests",
            [
                "cargo",
                "test",
                "-p",
                "codex-hepta-memory",
                "shadow_",
                "--",
                "--nocapture",
            ],
            WORKSPACE,
        ),
        run(
            "durable_grounding_tests",
            [
                "cargo",
                "test",
                "-p",
                "codex-hepta-memory",
                "durable_grounding",
                "--",
                "--nocapture",
            ],
            WORKSPACE,
        ),
        run(
            "full_crate_tests",
            ["cargo", "test", "-p", "codex-hepta-memory"],
            WORKSPACE,
        ),
        run(
            "scoped_clippy",
            [
                sys.executable,
                "scripts/check-hepta-intelligence-p0-3-2-clippy.py",
            ],
        ),
        run(
            "source_tree_clean",
            [
                "git",
                "diff",
                "--quiet",
                "--",
                ".",
                ":(exclude)artifacts",
            ],
        ),
    ]

    source_receipt = None
    source_log = OUT / "logs" / "source_contract.log"
    if source_log.exists():
        try:
            source_receipt = json.loads(source_log.read_text(encoding="utf-8"))
        except json.JSONDecodeError:
            source_receipt = {"invalid_json": True}

    qualified = all(bool(check["passed"]) for check in checks)
    check_by_id = {str(check["id"]): check for check in checks}
    receipt = {
        "schema": "hepta_intelligence_p0_3_2_exact_head_qualification_v7",
        "repository": "ProfAlexQI/hepta-private-ci",
        "branch": "codex/hepta-intelligence-shared-projection-planner-v5-20260828",
        "head": head,
        "tree": tree,
        "source_receipt": source_receipt,
        "checks": checks,
        "qualified": qualified,
        "implemented": True,
        "wired": False,
        "efficacy_proven": False,
        "operator_accepted": False,
        "promoted": False,
        "shared_projection_planner": True,
        "current_projection_replanned": True,
        "ledger_verified_in_snapshot": True,
        "test_root_canonicalized": bool(
            check_by_id["test_root_contract"]["passed"]
        ),
        "clippy_scope": "governed_p0_3_2_library_files",
        "tool_v3_registered": False,
        "tool_v4_registered": False,
        "default_projection_pointer_changed": False,
        "default_recall_query_changed": False,
        "production_projection_gate": False,
        "production_authority": False,
        "external_effects": False,
        "callers_ratchet": False,
    }
    (OUT / "qualification-receipt.json").write_text(
        json.dumps(receipt, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )
    # The workflow enforcement step owns the final job conclusion so the
    # receipt is uploaded even when one or more gates fail.
    return 0


if __name__ == "__main__":
    sys.exit(main())
