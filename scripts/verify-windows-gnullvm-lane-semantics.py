#!/usr/bin/env python3
"""Q0.31 compatibility entry point; the direct-Bazel verifier owns the composed source contract."""
from pathlib import Path
import runpy

ROOT = Path(__file__).resolve().parents[1]
runpy.run_path(
    str(ROOT / "scripts" / "verify-windows-gnullvm-direct-bazel.py"),
    run_name="__main__",
)
print("PASS_WINDOWS_GNULLVM_Q0_31_LANE_SEMANTICS_SOURCE")
