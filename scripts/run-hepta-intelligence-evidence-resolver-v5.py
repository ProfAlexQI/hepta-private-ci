#!/usr/bin/env python3
"""Execute all P0.3.3 exact-head gates and persist one fail-closed receipt."""
from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path
from typing import Sequence

ROOT = Path(__file__).resolve().parent.parent
WORKSPACE = ROOT / "codex-rs"
OUT = ROOT / "artifacts" / "hepta-intelligence-evidence-resolver-v5"
LOGS = OUT / "logs"
RUST_FILES = [
    "ext/hepta-memory/src/framing.rs",
    "ext/hepta-memory/src/cognitive/evidence_resolver_v4.rs",
    "ext/hepta-memory/src/cognitive/evidence_resolver_v4/receipt.rs",
    "ext/hepta-memory/src/cognitive/evidence_resolver_v4/resolver.rs",
    "ext/hepta-memory/src/cognitive/evidence_resolver_v4/schema.rs",
    "ext/hepta-memory/src/cognitive/evidence_resolver_v4/support.rs",
    "ext/hepta-memory/src/cognitive/evidence_resolver_v4/tests.rs",
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
    head = subprocess.check_output(
        ["git", "rev-parse", "HEAD"], cwd=ROOT, text=True
    ).strip()
    tree = subprocess.check_output(
        ["git", "rev-parse", "HEAD^{tree}"], cwd=ROOT, text=True
    ).strip()

    dependency_check = (
        "import json; from pathlib import Path; "
        "s=json.loads(Path('plans/hepta-intelligence/HEPTA_INTELLIGENCE_KG_EXECUTION_STATUS_V3_2.json').read_text()); "
        "d=s['dependency']; b=s['stack_base']; "
        "assert d.get('id')=='P0.3.2'; assert d.get('qualified') is True; "
        "assert d.get('implemented_in_repository') is True; "
        "assert d.get('activation_blocking') is False; "
        "assert d.get('ledger_verified_in_snapshot') is True; "
        "assert d.get('repository_branch')=='codex/hepta-intelligence-shared-projection-planner-v5-20260828'; "
        "assert b.get('branch')=='codex/hepta-intelligence-shared-projection-planner-v5-20260828'; "
        "assert isinstance(b.get('head'),str) and len(b['head'])==40 and all(c in '0123456789abcdef' for c in b['head'])"
    )

    checks = [
        run(
            "python_compile",
            [
                sys.executable,
                "-m",
                "py_compile",
                "scripts/verify-hepta-intelligence-evidence-resolver-v4.py",
                "scripts/check-hepta-intelligence-p0-3-3-clippy.py",
                "scripts/run-hepta-intelligence-evidence-resolver-v5.py",
            ],
        ),
        run(
            "source_contract",
            [sys.executable, "scripts/verify-hepta-intelligence-evidence-resolver-v4.py"],
        ),
        run("dependency_qualified", [sys.executable, "-c", dependency_check]),
        run(
            "candidate_rustfmt",
            ["rustfmt", "--edition", "2024", "--config", "skip_children=true", "--check", *RUST_FILES],
            WORKSPACE,
        ),
        run(
            "extension_all_targets_check",
            ["cargo", "check", "-p", "codex-hepta-memory-extension", "--all-targets"],
            WORKSPACE,
        ),
        run(
            "core_all_targets_check",
            ["cargo", "check", "-p", "codex-hepta-memory", "--all-targets"],
            WORKSPACE,
        ),
        run(
            "resolver_tests",
            [
                "cargo",
                "test",
                "-p",
                "codex-hepta-memory-extension",
                "evidence_resolver_v4",
                "--",
                "--nocapture",
            ],
            WORKSPACE,
        ),
        run(
            "core_grounding_tests",
            [
                "cargo",
                "test",
                "-p",
                "codex-hepta-memory",
                "fact_grounding",
                "--",
                "--nocapture",
            ],
            WORKSPACE,
        ),
        run(
            "extension_full_tests",
            ["cargo", "test", "-p", "codex-hepta-memory-extension"],
            WORKSPACE,
        ),
        run(
            "core_full_tests",
            ["cargo", "test", "-p", "codex-hepta-memory"],
            WORKSPACE,
        ),
        run(
            "scoped_clippy",
            [sys.executable, "scripts/check-hepta-intelligence-p0-3-3-clippy.py"],
        ),
        run(
            "source_tree_clean",
            ["git", "diff", "--quiet", "--", ".", ":(exclude)artifacts"],
        ),
    ]

    source_receipt = None
    source_log = LOGS / "source_contract.log"
    if source_log.exists():
        try:
            source_receipt = json.loads(source_log.read_text(encoding="utf-8"))
        except json.JSONDecodeError:
            source_receipt = {"invalid_json": True}
    dependency_qualified = bool(
        isinstance(source_receipt, dict)
        and source_receipt.get("p0_3_2_dependency_qualified") is True
    )
    qualified = dependency_qualified and all(bool(check["passed"]) for check in checks)
    status = json.loads(Path(
        "plans/hepta-intelligence/HEPTA_INTELLIGENCE_KG_EXECUTION_STATUS_V3_2.json"
    ).read_text(encoding="utf-8"))
    receipt = {
        "schema": "hepta_intelligence_p0_3_3_exact_head_qualification_v5",
        "repository": "ProfAlexQI/hepta-private-ci",
        "branch": "codex/hepta-intelligence-evidence-resolver-v4-20260828",
        "head": head,
        "tree": tree,
        "qualification_platform": "linux_exact_head",
        "source_receipt": source_receipt,
        "checks": checks,
        "p0_3_2_dependency_qualified": dependency_qualified,
        "p0_3_2_qualified_head": status.get("stack_base", {}).get("head"),
        "qualified": qualified,
        "implemented": True,
        "wired": False,
        "host_resolves_offsets": True,
        "host_computes_digests": True,
        "tool_v4_registered": False,
        "default_projection_pointer_changed": False,
        "default_recall_query_changed": False,
        "production_projection_gate": False,
        "production_authority": False,
        "external_effects": False,
        "operator_accepted": False,
        "promoted": False,
        "callers_ratchet": False,
    }
    (OUT / "qualification-receipt.json").write_text(
        json.dumps(receipt, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )
    # Workflow enforcement owns the job conclusion so the receipt is always uploaded.
    return 0


if __name__ == "__main__":
    sys.exit(main())
