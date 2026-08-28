#!/usr/bin/env python3
"""Load the governed WEB-C1 Worker source/API topology acceptance implementation."""
from __future__ import annotations

from pathlib import Path

_PART_ROOT = Path(__file__).with_name("hepta-servo-worker-source-topology-acceptance-v1")
_PARTS = tuple(_PART_ROOT / f"part{index:02d}.pyinc" for index in range(1, 6))
_source = "".join(path.read_text(encoding="utf-8") for path in _PARTS)
exec(compile(_source, str(_PART_ROOT / "combined.py"), "exec"), globals(), globals())
