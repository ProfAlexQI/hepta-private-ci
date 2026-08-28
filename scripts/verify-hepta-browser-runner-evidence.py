#!/usr/bin/env python3
"""Verify and classify exact-head GitHub Actions runner evidence."""

from __future__ import annotations

import pathlib
import sys

SCRIPT_DIR = pathlib.Path(__file__).resolve().parent
if str(SCRIPT_DIR) not in sys.path:
    sys.path.insert(0, str(SCRIPT_DIR))

from hepta_browser_runner_evidence import *  # noqa: E402,F401,F403


if __name__ == "__main__":
    raise SystemExit(main())
