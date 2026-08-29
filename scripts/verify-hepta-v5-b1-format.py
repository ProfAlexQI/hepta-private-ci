#!/usr/bin/env python3
"""Format or verify only the source-owned V5/B1 provider-boundary Rust files."""

from __future__ import annotations

import pathlib
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
FILES = (
    "codex-rs/hepta-contracts/src/checked_provider_operation.rs",
    "codex-rs/hepta-contracts/tests/provider_verified_use_boundary.rs",
)


def main() -> int:
    check = sys.argv[1:] == ["--check"]
    if sys.argv[1:] not in ([], ["--check"]):
        raise SystemExit("usage: verify-hepta-v5-b1-format.py [--check]")
    missing = [path for path in FILES if not (ROOT / path).is_file()]
    if missing:
        raise SystemExit(f"missing V5/B1 Rust source files: {missing}")
    command = [
        "rustfmt",
        "--edition",
        "2024",
        "--config-path",
        str(ROOT / "codex-rs/rustfmt.toml"),
    ]
    if check:
        command.append("--check")
    command.extend(str(ROOT / path) for path in FILES)
    return subprocess.run(command, cwd=ROOT, check=False).returncode


if __name__ == "__main__":
    raise SystemExit(main())
