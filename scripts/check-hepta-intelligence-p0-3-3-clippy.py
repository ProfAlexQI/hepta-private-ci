#!/usr/bin/env python3
"""Fail on P0.3.3 Clippy diagnostics in the governed extension surface."""
from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
WORKSPACE = ROOT / "codex-rs"
GOVERNED = {
    "ext/hepta-memory/src/framing.rs",
    "ext/hepta-memory/src/cognitive/evidence_resolver_v4.rs",
    "ext/hepta-memory/src/cognitive/evidence_resolver_v4/receipt.rs",
    "ext/hepta-memory/src/cognitive/evidence_resolver_v4/resolver.rs",
    "ext/hepta-memory/src/cognitive/evidence_resolver_v4/schema.rs",
    "ext/hepta-memory/src/cognitive/evidence_resolver_v4/support.rs",
    "ext/hepta-memory/src/cognitive/evidence_resolver_v4/tests.rs",
}


def main() -> int:
    command = [
        "cargo",
        "clippy",
        "-p",
        "codex-hepta-memory-extension",
        "--all-targets",
        "--no-deps",
        "--message-format=json",
        "--",
        "-W",
        "warnings",
    ]
    process = subprocess.run(
        command,
        cwd=WORKSPACE,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    governed_diagnostics: list[dict[str, object]] = []
    parsed_events = 0
    for line in process.stdout.splitlines():
        try:
            event = json.loads(line)
        except json.JSONDecodeError:
            continue
        parsed_events += 1
        if event.get("reason") != "compiler-message":
            continue
        message = event.get("message") or {}
        if message.get("level") not in {"warning", "error"}:
            continue
        primary_files: set[str] = set()
        for span in message.get("spans") or []:
            if not span.get("is_primary") or not span.get("file_name"):
                continue
            candidate = Path(str(span["file_name"]))
            if candidate.is_absolute():
                try:
                    candidate = candidate.relative_to(WORKSPACE)
                except ValueError:
                    pass
            primary_files.add(candidate.as_posix())
        governed = sorted(primary_files & GOVERNED)
        if governed:
            governed_diagnostics.append(
                {
                    "level": message.get("level"),
                    "code": (message.get("code") or {}).get("code"),
                    "message": message.get("message"),
                    "files": governed,
                    "rendered": message.get("rendered"),
                }
            )

    passed = process.returncode == 0 and not governed_diagnostics
    receipt = {
        "schema": "hepta_intelligence_p0_3_3_scoped_clippy_v1",
        "command": command,
        "cargo_exit_code": process.returncode,
        "json_events_parsed": parsed_events,
        "governed_files": sorted(GOVERNED),
        "governed_diagnostics": governed_diagnostics,
        "passed": passed,
        "stderr_tail": "\n".join(process.stderr.splitlines()[-100:])[-8000:],
    }
    print(json.dumps(receipt, indent=2, sort_keys=True))
    return 0 if passed else 1


if __name__ == "__main__":
    sys.exit(main())
