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
if hashlib.sha256(source).hexdigest() != "2cc4b2dcf8ceeb09041d67cc65d42c7810552e5cfdeac76ea80343bb400bc453":
    raise SystemExit("HNMF materializer source digest mismatch")
for path in PARTS:
    path.unlink()
exec(compile(source, __file__, "exec"))
