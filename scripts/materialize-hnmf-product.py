#!/usr/bin/env python3
import base64, bz2, hashlib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
PARTS = [
    ROOT / "scripts/.hnmf-materializer.part0",
    ROOT / "scripts/.hnmf-materializer.part1",
    ROOT / "scripts/.hnmf-materializer.part2",
]
compressed = b"".join(
    base64.b64decode(path.read_text(encoding="ascii").strip(), validate=True)
    for path in PARTS
)
source = bz2.decompress(compressed)
if hashlib.sha256(source).hexdigest() != "2cc4b2dcf8ceeb09041d67cc65d42c7810552e5cfdeac76ea80343bb400bc453":
    raise SystemExit("HNMF materializer source digest mismatch")
for path in PARTS:
    path.unlink()
exec(compile(source, __file__, "exec"))
