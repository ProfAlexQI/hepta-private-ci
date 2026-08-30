#!/usr/bin/env python3
"""Load the audited publisher and supply an explicit immutable commit message."""

from __future__ import annotations

import importlib.util
from pathlib import Path
import sys


SOURCE = Path(__file__).with_name("run-hepta-a0-v451-operational-drift-publisher.py")
SPEC = importlib.util.spec_from_file_location("hepta_a0_drift_publisher", SOURCE)
if SPEC is None or SPEC.loader is None:
    raise SystemExit("cannot load A0 drift publisher")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)
ORIGINAL_RUN = MODULE.run


def run_with_commit_message(*args: str, cwd: Path, capture: bool = False) -> str:
    if len(args) >= 2 and args[0] == "git" and args[1] == "commit-tree" and "-m" not in args:
        args = (*args, "-m", "fix(intelligence): close V4.5.1 operational-document drift")
    return ORIGINAL_RUN(*args, cwd=cwd, capture=capture)


MODULE.run = run_with_commit_message
raise SystemExit(MODULE.main())
