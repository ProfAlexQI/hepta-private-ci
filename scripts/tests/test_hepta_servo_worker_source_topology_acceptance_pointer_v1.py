#!/usr/bin/env python3
"""Load governed WEB-C1 Worker topology acceptance regression tests."""
from __future__ import annotations

from pathlib import Path

_PART_ROOT = Path(__file__).with_name("hepta_servo_worker_source_topology_acceptance_v1")
_PARTS = tuple(_PART_ROOT / f"part{index:02d}.pyinc" for index in range(1, 4))
_source = "".join(path.read_text(encoding="utf-8") for path in _PARTS)
exec(compile(_source, str(_PART_ROOT / "combined_test.py"), "exec"), globals(), globals())
