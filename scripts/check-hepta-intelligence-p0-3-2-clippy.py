#!/usr/bin/env python3
"""Run Clippy and fail only on diagnostics in the governed P0.3.2 library surface."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
WORKSPACE = ROOT / "codex-rs"
GOVERNED = {
    "hepta-memory/src/cognitive_store.rs",
    "hepta-memory/src/cognitive_kg_store.rs",
    "hepta-memory/src/cognitive_projection_planner.rs",
    "hepta-memory/src/fact_grounding.rs",
    "hepta-memory/src/fact_grounding/durable.rs",
    "hepta-memory/src/fact_grounding/durable/grounding.rs",
    "hepta-memory/src/fact_grounding/durable/grounding/prepare.rs",
    "hepta-memory/src/fact_grounding/durable/grounding/ledger.rs",
    "hepta-memory/src/fact_grounding/durable/grounding/ledger/insert.rs",
    "hepta-memory/src/fact_grounding/durable/grounding/ledger/support.rs",
    "hepta-memory/src/fact_grounding/durable/grounding/ledger/verify.rs",
    "hepta-memory/src/fact_grounding/durable/schema.rs",
    "hepta-memory/src/fact_grounding/shadow_projection_gate.rs",
    "hepta-memory/src/framing.rs",
}


def main() -> int:
    command = [
        "cargo",
        "clippy",
        "-p",
        "codex-hepta-memory",
        "--lib",
        "--no-deps",
        "--message-format=json",
        "--",
        "-W",
        "warnings",
        "--cap-lints=warn",
    ]
    process = subprocess.run(
        command,
        cwd=WORKSPACE,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    diagnostics: list[dict[str, object]] = []
    for line in process.stdout.splitlines():
        try:
            event = json.loads(line)
        except json.JSONDecodeError:
            continue
        if event.get("reason") != "compiler-message":
            continue
        message = event.get("message") or {}
        if message.get("level") not in {"warning", "error"}:
            continue
        spans = message.get("spans") or []
        files = set()
        for span in spans:
            if not span.get("is_primary") or not span.get("file_name"):
                continue
            candidate = Path(str(span["file_name"]))
            if candidate.is_absolute():
                try:
                    candidate = candidate.relative_to(WORKSPACE)
                except ValueError:
                    pass
            files.add(candidate.as_posix())
        governed = sorted(files & GOVERNED)
        if governed:
            diagnostics.append(
                {
                    "level": message.get("level"),
                    "message": message.get("message"),
                    "code": (message.get("code") or {}).get("code"),
                    "files": governed,
                    "rendered": message.get("rendered"),
                }
            )

    passed = process.returncode == 0 and not diagnostics
    receipt = {
        "schema": "hepta_intelligence_p0_3_2_scoped_clippy_v1",
        "command": command,
        "cargo_exit_code": process.returncode,
        "governed_files": sorted(GOVERNED),
        "governed_diagnostics": diagnostics,
        "passed": passed,
        "stderr_tail": "\n".join(process.stderr.splitlines()[-80:])[-6000:],
    }
    print(json.dumps(receipt, indent=2, sort_keys=True))
    return 0 if passed else 1


if __name__ == "__main__":
    sys.exit(main())
