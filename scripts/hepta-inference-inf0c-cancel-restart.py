#!/usr/bin/env python3
"""Loader for the bounded INF-0C cancellation/restart evidence source parts."""
from pathlib import Path

_ROOT = Path(__file__).resolve().parent
_PARTS = tuple(_ROOT / f"hepta-inference-inf0c-cancel-restart.part{index}.inc.py" for index in range(1, 6))
_SOURCE = "".join(part.read_text(encoding="utf-8") for part in _PARTS)
exec(compile(_SOURCE, str(Path(__file__).resolve()), "exec"), globals(), globals())
