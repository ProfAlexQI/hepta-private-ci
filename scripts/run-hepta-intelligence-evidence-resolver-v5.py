#!/usr/bin/env python3
"""Run every P0.3.3 exact-head gate and emit one fail-closed receipt."""

from __future__ import annotations

import json
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Sequence

ROOT = Path(__file__).resolve().parent.parent
CODEX = ROOT / "codex-rs"
OUT = ROOT / "artifacts/hepta-intelligence-evidence-resolver-v5"
LOGS = OUT / "logs"
STATUS = (
    ROOT
    / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_KG_EXECUTION_STATUS_V3_2.json"
)

CANDIDATE_RUST_FILES = (
    "ext/hepta-memory/src/framing.rs",
    "ext/hepta-memory/src/cognitive/evidence_resolver_v4.rs",
    "ext/hepta-memory/src/cognitive/evidence_resolver_v4/receipt.rs",
    "ext/hepta-memory/src/cognitive/evidence_resolver_v4/resolver.rs",
    "ext/hepta-memory/src/cognitive/evidence_resolver_v4/schema.rs",
    "ext/hepta-memory/src/cognitive/evidence_resolver_v4/support.rs",
    "ext/hepta-memory/src/cognitive/evidence_resolver_v4/tests.rs",
)


@dataclass(frozen=True)
class Gate:
    gate_id: str
    command_name: str
    command: Sequence[str]
    cwd: Path
    log_name: str


def text_output(command: Sequence[str], cwd: Path = ROOT) -> str:
    return subprocess.check_output(command, cwd=cwd, text=True).strip()


def run_gate(gate: Gate) -> int:
    path = LOGS / gate.log_name
    try:
        with path.open("wb") as log:
            result = subprocess.run(
                list(gate.command),
                cwd=gate.cwd,
                stdout=log,
                stderr=subprocess.STDOUT,
                check=False,
            )
        return int(result.returncode)
    except OSError as error:
        path.write_text(f"failed to execute gate: {error}\n", encoding="utf-8")
        return 127


def run_source_gate() -> int:
    stdout_path = OUT / "source-gate.json"
    stderr_path = LOGS / "source-gate.stderr.log"
    try:
        with stdout_path.open("wb") as stdout, stderr_path.open("wb") as stderr:
            result = subprocess.run(
                [
                    "python3",
                    "scripts/verify-hepta-intelligence-evidence-resolver-v4.py",
                ],
                cwd=ROOT,
                stdout=stdout,
                stderr=stderr,
                check=False,
            )
        return int(result.returncode)
    except OSError as error:
        stderr_path.write_text(
            f"failed to execute source gate: {error}\n", encoding="utf-8"
        )
        return 127


def dependency_state() -> tuple[bool, dict[str, object]]:
    status = json.loads(STATUS.read_text(encoding="utf-8"))
    dependency = status.get("dependency")
    if not isinstance(dependency, dict):
        return False, {}
    return dependency.get("qualified") is True, dependency


def log_tail(path: Path) -> str:
    if not path.exists():
        return ""
    text = path.read_text(encoding="utf-8", errors="replace")
    return "\n".join(text.splitlines()[-100:])[-8000:]


def parse_source_receipt() -> object | None:
    path = OUT / "source-gate.json"
    if not path.exists():
        return None
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError:
        return {"invalid_json": True}


def main() -> int:
    LOGS.mkdir(parents=True, exist_ok=True)
    head = text_output(["git", "rev-parse", "HEAD"])
    tree = text_output(["git", "rev-parse", "HEAD^{tree}"])
    (OUT / "head.txt").write_text(head + "\n", encoding="utf-8")
    (OUT / "tree.txt").write_text(tree + "\n", encoding="utf-8")
    (OUT / "rustc.txt").write_text(
        text_output(["rustc", "--version"]) + "\n", encoding="utf-8"
    )
    (OUT / "cargo.txt").write_text(
        text_output(["cargo", "--version"]) + "\n", encoding="utf-8"
    )

    dependency_qualified, dependency = dependency_state()
    dependency_log = LOGS / "dependency.log"
    dependency_log.write_text(
        json.dumps(
            {
                "dependency": dependency,
                "qualified": dependency_qualified,
            },
            indent=2,
            sort_keys=True,
        )
        + "\n",
        encoding="utf-8",
    )

    gates = [
        Gate(
            "python_compile",
            "python3 -m py_compile P0.3.3 source and runner",
            [
                "python3",
                "-m",
                "py_compile",
                "scripts/verify-hepta-intelligence-evidence-resolver-v4.py",
                "scripts/run-hepta-intelligence-evidence-resolver-v5.py",
            ],
            ROOT,
            "python-compile.log",
        ),
        Gate(
            "fmt",
            "rustfmt --edition 2024 --check P0.3.3 candidate files",
            ["rustfmt", "--edition", "2024", "--check", *CANDIDATE_RUST_FILES],
            CODEX,
            "fmt.log",
        ),
        Gate(
            "resolver_tests",
            "cargo test -p codex-hepta-memory-extension evidence_resolver_v4 -- --nocapture",
            [
                "cargo",
                "test",
                "-p",
                "codex-hepta-memory-extension",
                "evidence_resolver_v4",
                "--",
                "--nocapture",
            ],
            CODEX,
            "resolver-tests.log",
        ),
        Gate(
            "core_grounding_tests",
            "cargo test -p codex-hepta-memory fact_grounding -- --nocapture",
            [
                "cargo",
                "test",
                "-p",
                "codex-hepta-memory",
                "fact_grounding",
                "--",
                "--nocapture",
            ],
            CODEX,
            "core-grounding-tests.log",
        ),
        Gate(
            "extension_full_tests",
            "cargo test -p codex-hepta-memory-extension",
            ["cargo", "test", "-p", "codex-hepta-memory-extension"],
            CODEX,
            "extension-full-tests.log",
        ),
        Gate(
            "core_full_tests",
            "cargo test -p codex-hepta-memory",
            ["cargo", "test", "-p", "codex-hepta-memory"],
            CODEX,
            "core-full-tests.log",
        ),
        Gate(
            "extension_clippy",
            "cargo clippy -p codex-hepta-memory-extension --all-targets -- -D warnings",
            [
                "cargo",
                "clippy",
                "-p",
                "codex-hepta-memory-extension",
                "--all-targets",
                "--",
                "-D",
                "warnings",
            ],
            CODEX,
            "extension-clippy.log",
        ),
        Gate(
            "core_clippy",
            "cargo clippy -p codex-hepta-memory --all-targets -- -D warnings",
            [
                "cargo",
                "clippy",
                "-p",
                "codex-hepta-memory",
                "--all-targets",
                "--",
                "-D",
                "warnings",
            ],
            CODEX,
            "core-clippy.log",
        ),
        Gate(
            "dirty",
            "git diff --quiet after qualification",
            ["git", "diff", "--quiet", "--", ".", ":(exclude)artifacts"],
            ROOT,
            "dirty-check.log",
        ),
    ]

    exits: dict[str, int] = {}
    exits["dependency"] = 0 if dependency_qualified else 1
    for gate in gates[:1]:
        exits[gate.gate_id] = run_gate(gate)
    exits["source"] = run_source_gate()
    for gate in gates[1:]:
        exits[gate.gate_id] = run_gate(gate)

    command_names = {gate.gate_id: gate.command_name for gate in gates}
    command_names["dependency"] = "require independently qualified P0.3.2 receipt"
    command_names["source"] = (
        "python3 scripts/verify-hepta-intelligence-evidence-resolver-v4.py"
    )
    log_names = {gate.gate_id: gate.log_name for gate in gates}
    log_names["dependency"] = "dependency.log"
    log_names["source"] = "source-gate.stderr.log"
    order = (
        "dependency",
        "python_compile",
        "source",
        "fmt",
        "resolver_tests",
        "core_grounding_tests",
        "extension_full_tests",
        "core_full_tests",
        "extension_clippy",
        "core_clippy",
        "dirty",
    )
    checks = []
    for gate_id in order:
        exit_code = exits.get(gate_id, 255)
        log_name = log_names[gate_id]
        checks.append(
            {
                "id": gate_id,
                "command": command_names[gate_id],
                "exit_code": exit_code,
                "passed": exit_code == 0,
                "log": f"logs/{log_name}",
                "log_tail": log_tail(LOGS / log_name),
            }
        )

    qualified = all(check["passed"] for check in checks)
    receipt = {
        "schema": "hepta_intelligence_p0_3_3_exact_head_qualification_v1",
        "repository": "ProfAlexQI/hepta-private-ci",
        "branch": "codex/hepta-intelligence-evidence-resolver-v4-20260828",
        "head": head,
        "tree": tree,
        "rustc": (OUT / "rustc.txt").read_text(encoding="utf-8").strip(),
        "cargo": (OUT / "cargo.txt").read_text(encoding="utf-8").strip(),
        "dependency": dependency,
        "source_receipt": parse_source_receipt(),
        "checks": checks,
        "qualified": qualified,
        "implemented": True,
        "wired": False,
        "efficacy_proven": False,
        "operator_accepted": False,
        "promoted": False,
        "p0_3_2_dependency_qualified": dependency_qualified,
        "model_supplies_offsets": False,
        "model_supplies_digests": False,
        "host_resolves_offsets": True,
        "host_computes_digests": True,
        "tool_v4_registered": False,
        "default_projection_pointer_changed": False,
        "default_recall_query_changed": False,
        "production_projection_gate": False,
        "production_authority": False,
        "external_effects": False,
        "callers_ratchet": False,
    }
    (OUT / "qualification-receipt.json").write_text(
        json.dumps(receipt, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    print(
        json.dumps(
            {
                "head": head,
                "qualified": qualified,
                "failed": [
                    check["id"] for check in checks if not check["passed"]
                ],
            },
            sort_keys=True,
        )
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
