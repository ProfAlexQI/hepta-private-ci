#!/usr/bin/env python3
"""Compatibility guard for a retired local-profile generator."""
from __future__ import annotations

import sys

NOTICE = """This local-profile generator has been retired.

Canonical human development authority: docs/DEVELOPMENT.md
Canonical machine pointer: docs/CURRENT.json
Generated project status: docs/STATUS.md
Verification command: python3 scripts/hepta-docs.py verify

This compatibility entry point creates no profile, receipt, capability, runtime
state, operator acceptance, promotion, production authority, or release authority.
"""


def main() -> int:
    print(NOTICE, file=sys.stdout)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
