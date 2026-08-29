#!/usr/bin/env python3
"""Thin wrapper over the strict canonical current-truth validator."""
from __future__ import annotations
import importlib.util
from pathlib import Path
import sys


def main() -> int:
    root = Path(__file__).resolve().parents[1]
    path = root / "scripts" / "hepta-intelligence-current-truth.py"
    spec = importlib.util.spec_from_file_location("hepta_current_truth", path)
    if spec is None or spec.loader is None:
        raise SystemExit("cannot load current-truth validator")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    truth = module.validate_and_build()

    print('PASS_HEPTA_INTELLIGENCE_MASTER_PLAN_V4_5_SOURCE_ONLY')
    return 0



if __name__ == "__main__":
    raise SystemExit(main())
