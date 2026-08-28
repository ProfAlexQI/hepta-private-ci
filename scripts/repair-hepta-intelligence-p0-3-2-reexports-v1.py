#!/usr/bin/env python3
"""Repair the final P0.3.2 durable-grounding re-export visibility seam."""

from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent


def replace_once(relative: str, old: str, new: str) -> None:
    path = ROOT / relative
    text = path.read_text(encoding="utf-8")
    count = text.count(old)
    if count != 1:
        raise SystemExit(
            f"{relative}: expected exactly one replacement target, observed {count}: {old!r}"
        )
    path.write_text(text.replace(old, new, 1), encoding="utf-8")


def main() -> None:
    ledger = (
        "codex-rs/hepta-memory/src/fact_grounding/"
        "durable/grounding/ledger.rs"
    )
    replace_once(
        ledger,
        "pub(super) use insert::insert_tx;",
        "pub(in super::super) use insert::insert_tx;",
    )
    replace_once(
        ledger,
        "pub(super) use verify::verify_receipts;",
        "pub(in super::super) use verify::verify_receipts;",
    )

    verifier = "scripts/verify-hepta-intelligence-shared-projection-planner-v5.py"
    replace_once(
        verifier,
        '''            "pub(super) use support::durable_receipt_digest;",
            "pub(super) use verify::verify_receipts;",''',
        '''            "pub(super) use support::durable_receipt_digest;",
            "pub(in super::super) use insert::insert_tx;",
            "pub(in super::super) use verify::verify_receipts;",''',
    )


if __name__ == "__main__":
    main()
