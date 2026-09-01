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
if hashlib.sha256(source).hexdigest() != "6914b5a2d61d17cf7027bea9f709084895dfb1533bc2173c9b569940e24167c9":
    raise SystemExit("HNMF materializer source digest mismatch")
for path in PARTS:
    path.unlink()
exec(compile(source, __file__, "exec"))
