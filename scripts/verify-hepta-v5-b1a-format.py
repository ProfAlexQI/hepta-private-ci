#!/usr/bin/env python3
from __future__ import annotations

import argparse
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
PATHS = [
    Path("codex-rs/hepta-contracts/src/lib.rs"),
    Path("codex-rs/hepta-contracts/src/checked_provider_operation.rs"),
    *sorted(Path("codex-rs/hepta-contracts/src/checked_provider_operation_parts").glob("*.rs")),
    Path("codex-rs/hepta-contracts/tests/provider_verified_use_boundary.rs"),
    *sorted(Path("codex-rs/hepta-contracts/tests/provider_verified_use_boundary_parts").glob("*.rs")),
    Path("docs/architecture/HEPTA_P0_7B_B1A_PROVIDER_BOUNDARY_CONTRACT_V1.md"),
    Path("docs/architecture/HEPTA_P0_7B_B1A_STATUS.json"),
    Path("docs/architecture/HEPTA_V5_B1A_PROVIDER_CALLSITE_INVENTORY.json"),
    Path("scripts/verify-hepta-v5-b1a-provider-boundary.py"),
    Path(".github/workflows/hepta-architecture-v5-b1a-provider-boundary.yml"),
]


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    parser.parse_args()
    for path in PATHS:
        data = (ROOT / path).read_bytes()
        if b"\r" in data or not data.endswith(b"\n"):
            raise SystemExit(f"FAIL_HEPTA_V5_B1A_FORMAT: newline policy: {path}")
        for line_no, line in enumerate(data.decode("utf-8").splitlines(), 1):
            if line.rstrip(" \t") != line:
                raise SystemExit(f"FAIL_HEPTA_V5_B1A_FORMAT: trailing whitespace {path}:{line_no}")
    print("PASS_HEPTA_V5_B1A_FORMAT")
    return 0


if __name__ == "__main__":
    sys.exit(main())
