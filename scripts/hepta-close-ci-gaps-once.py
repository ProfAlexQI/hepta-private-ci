#!/usr/bin/env python3
"""One-shot, fail-closed patch for architecture qualification CI gaps.

This script is branch-local bootstrap material. The invoking workflow removes
both this file and itself before committing the repaired source.
"""

from __future__ import annotations

import pathlib

ROOT = pathlib.Path(__file__).resolve().parents[1]


def read(relative: str) -> str:
    return (ROOT / relative).read_text(encoding="utf-8")


def write(relative: str, content: str) -> None:
    (ROOT / relative).write_text(content, encoding="utf-8")


def replace_exact(relative: str, old: str, new: str, expected: int = 1) -> None:
    source = read(relative)
    actual = source.count(old)
    if actual != expected:
        raise SystemExit(
            f"{relative}: expected {expected} occurrence(s) of {old!r}, found {actual}"
        )
    write(relative, source.replace(old, new))


def patch_canonical_workflow() -> None:
    relative = ".github/workflows/hepta-architecture-convergence-p0-2.yml"
    replace_exact(
        relative,
        "          cargo test --locked -p codex-hepta-automation --lib -- --nocapture\n",
        "          cargo test --locked -p codex-hepta-automation --lib -- --nocapture\n"
        "          cargo test --locked -p codex-hepta-automation --test automation -- --nocapture\n",
        expected=2,
    )
    replace_exact(
        relative,
        "    if: ${{ always() }}\n    needs:\n      - exact-source-head\n",
        "    if: ${{ always() && !cancelled() }}\n    needs:\n      - exact-source-head\n",
    )


def patch_blocking_workflow() -> None:
    relative = ".github/workflows/blocking-ci.yml"
    replace_exact(
        relative,
        "    if: ${{ always() }}\n    needs:\n      - bazel\n",
        "    if: ${{ always() && !cancelled() }}\n    needs:\n      - bazel\n",
    )


def patch_gap_verifier() -> None:
    relative = "scripts/verify-hepta-architecture-gap-ledger.py"
    replace_exact(
        relative,
        '        "Hepta architecture convergence required",\n'
        '        "python3 scripts/verify-hepta-architecture-gap-ledger.py",\n',
        '        "Hepta architecture convergence required",\n'
        '        "if: ${{ always() && !cancelled() }}",\n'
        '        "cargo test --locked -p codex-hepta-automation --test automation -- --nocapture",\n'
        '        "python3 scripts/verify-hepta-architecture-gap-ledger.py",\n',
    )
    replace_exact(
        relative,
        '    if "- hepta-architecture-convergence" not in blocking:\n'
        '        fail("blocking-ci required aggregator omits architecture convergence")\n',
        '    if "- hepta-architecture-convergence" not in blocking:\n'
        '        fail("blocking-ci required aggregator omits architecture convergence")\n'
        '    if "if: ${{ always() && !cancelled() }}" not in blocking:\n'
        '        fail("blocking-ci required aggregator can survive cancellation and starve the latest head")\n',
    )


def patch_cross_owner_verifier() -> None:
    relative = "scripts/verify-hepta-cross-owner-operation-wiring.py"
    replace_exact(
        relative,
        '        "cargo test --locked -p codex-hepta-contracts provider_operation::tests",\n'
        '        "cargo test --locked -p codex-hepta-matrix-store --features qualification-fault-injection --test sqlite_full",\n',
        '        "cargo test --locked -p codex-hepta-contracts provider_operation::tests",\n'
        '        "cargo test --locked -p codex-hepta-automation --test automation -- --nocapture",\n'
        '        "cargo test --locked -p codex-hepta-matrix-store --features qualification-fault-injection --test sqlite_full",\n',
    )
    replace_exact(
        relative,
        '        "Hepta architecture convergence required",\n',
        '        "Hepta architecture convergence required",\n'
        '        "if: ${{ always() && !cancelled() }}",\n',
    )


def main() -> int:
    patch_canonical_workflow()
    patch_blocking_workflow()
    patch_gap_verifier()
    patch_cross_owner_verifier()
    print("PASS_HEPTA_CLOSE_CI_GAPS_ONCE")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
