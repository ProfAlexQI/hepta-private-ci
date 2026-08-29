#!/usr/bin/env python3
"""Apply exact, digest-bound P0.3.4 reconstruction corrections."""

from __future__ import annotations

import hashlib
import sys
from pathlib import Path

BEFORE_SHA256 = "89516216e15fde9a573300aab53dace604114308bf71a6fce6d62c360b56fe66"
AFTER_SHA256 = "8ec425fec8c97f36eccff50d6e24f2438987339fb40bfe2734511a0b1b20304e"

OLD = """## Reconstruction decision

The historical payload was truncated: the archived gzip stream had no EOF and
contained only the first 6,024 bytes of a declared 79,891-byte Rust module.
That artifact is not source evidence. P0.3.4 is therefore independently
reconstructed from the recovered written contract, the complete migration, and
the qualified P0.2/P0.3.3 implementation. The new candidate must not reuse the
historical payload digest or claim byte identity with the lost source.
"""

NEW = """## Reconstruction decision

historical payload was truncated: the archived gzip stream had no EOF and contained only the first 6,024 bytes of a declared 79,891-byte Rust module. This candidate is independently reconstructed from the recovered written contract, the complete migration, and the qualified P0.2/P0.3.3 implementation. It must not reuse the historical payload digest or claim byte identity with the lost source.
"""


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> int:
    if len(sys.argv) != 2:
        raise SystemExit("usage: p034-rebuild-hotfix.py EXTRACTED_PAYLOAD_DIR")
    root = Path(sys.argv[1]).resolve()
    plan = root / "P034_PLAN.md"
    actual_before = digest(plan)
    if actual_before != BEFORE_SHA256:
        raise SystemExit(
            f"P0.3.4 plan input drifted: expected {BEFORE_SHA256}, got {actual_before}"
        )
    text = plan.read_text(encoding="utf-8")
    if text.count(OLD) != 1:
        raise SystemExit("P0.3.4 reconstruction-decision block drifted")
    plan.write_text(text.replace(OLD, NEW, 1), encoding="utf-8")
    actual_after = digest(plan)
    if actual_after != AFTER_SHA256:
        raise SystemExit(
            f"P0.3.4 repaired plan digest mismatch: expected {AFTER_SHA256}, got {actual_after}"
        )
    print(f"P0.3.4 exact plan repair PASS sha256={actual_after}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
