#!/usr/bin/env python3
"""Attribute P0.3.3 Clippy diagnostics without masking compiler failures."""
from __future__ import annotations

import json
import re
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
RUST_ERROR_CODE = re.compile(r"E[0-9]{4}")
MAX_RECORDED_DIAGNOSTICS = 100


def primary_files(message: dict[str, object]) -> set[str]:
    files: set[str] = set()
    for raw_span in message.get("spans") or []:
        if not isinstance(raw_span, dict):
            continue
        if not raw_span.get("is_primary") or not raw_span.get("file_name"):
            continue
        candidate = Path(str(raw_span["file_name"]))
        if candidate.is_absolute():
            try:
                candidate = candidate.relative_to(WORKSPACE)
            except ValueError:
                pass
        files.add(candidate.as_posix())
    return files


def record(message: dict[str, object], files: set[str]) -> dict[str, object]:
    raw_code = message.get("code") or {}
    code = raw_code.get("code") if isinstance(raw_code, dict) else None
    return {
        "level": message.get("level"),
        "code": code,
        "message": message.get("message"),
        "files": sorted(files),
        "rendered": message.get("rendered"),
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
    compiler_errors: list[dict[str, object]] = []
    non_governed_lint_debt: list[dict[str, object]] = []
    parsed_events = 0
    build_finished_seen = False
    build_finished_success: bool | None = None

    for line in process.stdout.splitlines():
        try:
            event = json.loads(line)
        except json.JSONDecodeError:
            continue
        parsed_events += 1
        if event.get("reason") == "build-finished":
            build_finished_seen = True
            build_finished_success = bool(event.get("success"))
            continue
        if event.get("reason") != "compiler-message":
            continue
        message = event.get("message") or {}
        if not isinstance(message, dict) or message.get("level") not in {"warning", "error"}:
            continue

        files = primary_files(message)
        item = record(message, files)
        governed = files & GOVERNED
        if governed:
            item["files"] = sorted(governed)
            if len(governed_diagnostics) < MAX_RECORDED_DIAGNOSTICS:
                governed_diagnostics.append(item)
            continue

        code = item.get("code")
        true_compiler_error = (
            message.get("level") == "error"
            and (
                not files
                or code is None
                or bool(RUST_ERROR_CODE.fullmatch(str(code)))
            )
        )
        if true_compiler_error:
            if len(compiler_errors) < MAX_RECORDED_DIAGNOSTICS:
                compiler_errors.append(item)
        elif len(non_governed_lint_debt) < MAX_RECORDED_DIAGNOSTICS:
            non_governed_lint_debt.append(item)

    # A non-zero Clippy process is admissible only when Cargo produced parsed
    # compiler events and every failure is attributable to lint debt outside
    # the governed P0.3.3 files. Independent all-target cargo-check and full-test
    # gates in the exact-head runner still have to pass, so compile failures
    # cannot be converted into a scoped-Clippy PASS.
    attributable_non_governed_failure = (
        process.returncode != 0
        and parsed_events > 0
        and bool(non_governed_lint_debt)
        and not governed_diagnostics
        and not compiler_errors
    )
    infrastructure_failure = (
        process.returncode != 0
        and not attributable_non_governed_failure
        and not governed_diagnostics
        and not compiler_errors
    )
    passed = (
        not governed_diagnostics
        and not compiler_errors
        and not infrastructure_failure
        and (process.returncode == 0 or attributable_non_governed_failure)
    )

    receipt = {
        "schema": "hepta_intelligence_p0_3_3_scoped_clippy_v2",
        "command": command,
        "cargo_exit_code": process.returncode,
        "json_events_parsed": parsed_events,
        "build_finished_seen": build_finished_seen,
        "build_finished_success": build_finished_success,
        "governed_files": sorted(GOVERNED),
        "governed_diagnostics": governed_diagnostics,
        "compiler_errors": compiler_errors,
        "non_governed_lint_debt": non_governed_lint_debt,
        "attributable_non_governed_failure": attributable_non_governed_failure,
        "infrastructure_failure": infrastructure_failure,
        "passed": passed,
        "stderr_tail": "\n".join(process.stderr.splitlines()[-100:])[-8000:],
    }
    print(json.dumps(receipt, indent=2, sort_keys=True))
    return 0 if passed else 1


if __name__ == "__main__":
    sys.exit(main())
