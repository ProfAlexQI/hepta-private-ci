#!/usr/bin/env python3
import base64, bz2, hashlib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
PARTS = [
    ROOT / "scripts/.hnmf-materializer.part0",
    ROOT / "scripts/.hnmf-materializer.part1",
    ROOT / "scripts/.hnmf-materializer.part2",
]
encoded = "".join(path.read_text(encoding="ascii").strip() for path in PARTS)
source = bz2.decompress(base64.b64decode(encoded, validate=True))
if hashlib.sha256(source).hexdigest() != "127a2d54f79d89ace2a9f4f525fe75abde3c0e68642d37a66ac6d3e5c9da40a3":
    raise SystemExit("HNMF materializer source digest mismatch")
for path in PARTS:
    path.unlink()
exec(compile(source, __file__, "exec"))
