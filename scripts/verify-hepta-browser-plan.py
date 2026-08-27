#!/usr/bin/env python3
"""Compatibility entrypoint for the canonical Hepta browser plan verifier."""

from __future__ import annotations

import runpy
from pathlib import Path
from typing import Any


CANONICAL_VERIFIER = Path(__file__).with_name("verify-hepta-browser-plan-v2.py")


def main() -> int:
    namespace: dict[str, Any] = runpy.run_path(str(CANONICAL_VERIFIER))
    verifier = namespace.get("main")
    if not callable(verifier):
        raise RuntimeError("canonical Hepta browser verifier has no callable main")
    result = verifier()
    if not isinstance(result, int):
        raise RuntimeError("canonical Hepta browser verifier returned a non-integer result")
    return result


if __name__ == "__main__":
    raise SystemExit(main())
