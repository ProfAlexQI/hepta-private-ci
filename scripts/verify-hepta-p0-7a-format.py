#!/usr/bin/env python3
"""Format or verify only the source-owned P0.7a runtime-bootstrap package."""
from __future__ import annotations

import pathlib
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
FILES = (
    "codex-rs/hepta-contracts/src/runtime_bootstrap.rs",
    "codex-rs/hepta-fleet/src/runtime_bootstrap_registry.rs",
    "codex-rs/hepta-fleet/src/runtime_bootstrap_registry_tests.rs",
    "codex-rs/hepta-supervisor/src/runtime_bootstrap.rs",
    "codex-rs/hepta-supervisor/src/runtime_bootstrap_tests.rs",
    "codex-rs/hepta-agentd/src/runtime_bootstrap.rs",
    "codex-rs/hepta-agentd/src/runtime_bootstrap_tests.rs",
)


def main() -> int:
    check = sys.argv[1:] == ["--check"]
    if sys.argv[1:] not in ([], ["--check"]):
        raise SystemExit("usage: verify-hepta-p0-7a-format.py [--check]")
    missing = [path for path in FILES if not (ROOT / path).is_file()]
    if missing:
        raise SystemExit(f"missing P0.7a source files: {missing}")
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
