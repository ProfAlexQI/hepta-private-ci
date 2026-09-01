#!/usr/bin/env python3
import base64
import zlib
from pathlib import Path

parts_dir = Path(__file__).with_name("hepta-doc3-materialize.parts")
payload = "".join(
    path.read_text(encoding="ascii").strip()
    for path in sorted(parts_dir.glob("*.part"))
)
if not payload:
    raise SystemExit("DOC-3 materializer payload is missing")
source = zlib.decompress(base64.b85decode(payload)).decode("utf-8")
exec(compile(source, __file__, "exec"))
