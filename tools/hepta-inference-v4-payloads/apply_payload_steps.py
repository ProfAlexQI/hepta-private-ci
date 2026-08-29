#!/usr/bin/env python3
"""Apply only explicitly allow-listed source-generation steps from V4 payloads.

The payloads are immutable copies of package workflows. This program is deliberately
not a general GitHub Actions or YAML interpreter: it extracts exact named `run: |`
blocks, verifies their identities and rejects commands capable of committing,
pushing, downloading, escalating privileges, or escaping the source-build scope.
"""

from __future__ import annotations

import hashlib
import os
from pathlib import Path
import subprocess
import sys
from typing import NamedTuple

ROOT = Path(__file__).resolve().parents[2]
PAYLOAD_ROOT = ROOT / "tools" / "hepta-inference-v4-payloads"


class Step(NamedTuple):
    working_directory: str
    script: str


MANIFEST: tuple[tuple[str, tuple[str, ...]], ...] = (
    (
        "inf-s1.yml",
        (
            "Integrate capability primitives into the core API",
            "Add daemon-owned private worker and operator control plane",
        ),
    ),
    (
        "inf-s2.yml",
        (
            "Add explicit running-cancel lifecycle to the controller",
            "Add child-process cancel supervisor and fixture E2E",
        ),
    ),
    (
        "inf-s3.yml",
        (
            "Expose bounded SHA-256 receipt digests and compacted error",
            "Add crash-safe startup retention and tombstone journal",
        ),
    ),
    (
        "inf-s4.yml",
        ("Add deterministic EDF/WFQ scheduler and reservation journal",),
    ),
    (
        "inf-r1.yml",
        ("Add bounded loopback Ollama provider host",),
    ),
    (
        "inf-r2-chain.yml",
        (
            "Export the controller token-chain algorithm",
            "Bind worker submit and completion to the exact rolling chain",
            "Wire worker-host and missing workspace dependency",
        ),
    ),
    (
        "inf-r3.yml",
        ("Export the typed product shadow boundary",),
    ),
)

FORBIDDEN_SCRIPT_FRAGMENTS = (
    "git commit",
    "git push",
    "git reset",
    "git checkout",
    "git switch",
    "git clean",
    "gh ",
    "curl ",
    "wget ",
    "sudo ",
    "rm -rf",
    "GITHUB_TOKEN",
    "ACTIONS_RUNTIME_TOKEN",
)

ALLOWED_WORKDIRS = {".", "codex-rs"}


def fail(message: str) -> "NoReturn":
    raise SystemExit(message)


def parse_named_run_steps(text: str) -> dict[str, Step]:
    lines = text.splitlines()
    parsed: dict[str, Step] = {}
    index = 0
    while index < len(lines):
        line = lines[index]
        if not line.startswith("      - name: "):
            index += 1
            continue
        name = line[len("      - name: ") :]
        working_directory = "."
        run_start: int | None = None
        cursor = index + 1
        while cursor < len(lines) and not lines[cursor].startswith("      - "):
            current = lines[cursor]
            prefix = "        working-directory: "
            if current.startswith(prefix):
                working_directory = current[len(prefix) :].strip().strip('"\'')
            if current == "        run: |":
                run_start = cursor + 1
            cursor += 1
        if run_start is not None:
            block: list[str] = []
            for current in lines[run_start:cursor]:
                if current.startswith("          "):
                    block.append(current[10:])
                elif not current.strip():
                    block.append("")
                else:
                    fail(f"unexpected indentation in step {name!r}: {current!r}")
            if name in parsed:
                fail(f"duplicate named run step: {name}")
            parsed[name] = Step(working_directory, "\n".join(block).rstrip() + "\n")
        index = cursor
    return parsed


def validate_script(payload: str, name: str, step: Step) -> None:
    if step.working_directory not in ALLOWED_WORKDIRS:
        fail(
            f"payload {payload} step {name!r} uses forbidden working directory "
            f"{step.working_directory!r}"
        )
    lowered = step.script.lower()
    for fragment in FORBIDDEN_SCRIPT_FRAGMENTS:
        if fragment.lower() in lowered:
            fail(f"payload {payload} step {name!r} contains forbidden fragment {fragment!r}")
    if "set -euo pipefail" not in step.script:
        fail(f"payload {payload} step {name!r} is not fail-closed")


def execute(payload: str, name: str, step: Step) -> None:
    cwd = (ROOT / step.working_directory).resolve()
    if cwd != ROOT and cwd != ROOT / "codex-rs":
        fail(f"resolved working directory escaped repository: {cwd}")
    if not cwd.is_dir():
        fail(f"working directory does not exist: {cwd}")
    digest = hashlib.sha256(step.script.encode("utf-8")).hexdigest()
    print(
        f"APPLY payload={payload} step={name!r} cwd={step.working_directory} "
        f"sha256={digest}",
        flush=True,
    )
    environment = os.environ.copy()
    environment["HEPTA_V4_UNIFIED_SOURCE_MATERIALIZATION"] = "1"
    subprocess.run(
        ["bash", "-c", step.script],
        cwd=cwd,
        env=environment,
        check=True,
    )


def main() -> None:
    selected_total = 0
    for payload_name, selected_names in MANIFEST:
        path = PAYLOAD_ROOT / payload_name
        text = path.read_text(encoding="utf-8")
        parsed = parse_named_run_steps(text)
        missing = [name for name in selected_names if name not in parsed]
        if missing:
            fail(f"payload {payload_name} missing selected steps: {missing}")
        unselected_dangerous = {
            name
            for name in parsed
            if name.startswith("Commit ") or name.startswith("Enforce ")
        }
        for name in selected_names:
            step = parsed[name]
            validate_script(payload_name, name, step)
            execute(payload_name, name, step)
            selected_total += 1
        print(
            f"VERIFIED payload={payload_name} parsed_steps={len(parsed)} "
            f"selected_steps={len(selected_names)} "
            f"ignored_control_steps={len(unselected_dangerous)}",
            flush=True,
        )
    if selected_total != 12:
        fail(f"unexpected selected step count: {selected_total}")
    print("PASS_HEPTA_INFERENCE_V4_ALLOWLISTED_SOURCE_MATERIALIZATION", flush=True)


if __name__ == "__main__":
    main()
