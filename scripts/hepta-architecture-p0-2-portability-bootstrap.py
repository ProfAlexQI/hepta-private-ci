#!/usr/bin/env python3
"""Apply the bounded P0.2 cross-platform journal durability hardening."""

from __future__ import annotations

import pathlib

ROOT = pathlib.Path(__file__).resolve().parents[1]
TARGET = ROOT / "codex-rs/hepta-contracts/tests/cross_owner_fault_matrix.rs"

OLD = '''    let parent = File::open(
        final_path
            .parent()
            .ok_or_else(|| io::Error::other("journal path has no parent"))?,
    )?;
    parent.sync_all()?;
'''

NEW = '''    if cfg!(unix) {
        let parent = File::open(
            final_path
                .parent()
                .ok_or_else(|| io::Error::other("journal path has no parent"))?,
        )?;
        parent.sync_all()?;
    }
'''


def main() -> int:
    source = TARGET.read_text(encoding="utf-8")
    if NEW in source:
        print("PORTABILITY_HARDENING_ALREADY_APPLIED")
        return 0
    if source.count(OLD) != 1:
        raise SystemExit("journal parent-sync source anchor drifted")
    TARGET.write_text(source.replace(OLD, NEW), encoding="utf-8")
    print("PORTABILITY_HARDENING_APPLIED")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
