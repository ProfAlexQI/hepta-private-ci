#!/usr/bin/env python3
from __future__ import annotations

import base64
import hashlib
import io
import tarfile
import zlib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
ARCHIVE_SHA256 = "8462cdf4fdf05ae042e8e2b3c66ed5b2056512cd26ebf0f06c250afb367ccb67"
CHUNKS = tuple(sorted(Path(__file__).resolve().parent.glob("hepta-inference-inf1-payload-*.b85")))


def main() -> None:
    if not CHUNKS:
        raise SystemExit("missing INF-1 payload chunks")
    encoded = "".join(path.read_text(encoding="ascii") for path in CHUNKS)
    archive = zlib.decompress(base64.b85decode(encoded))
    if hashlib.sha256(archive).hexdigest() != ARCHIVE_SHA256:
        raise SystemExit("INF-1 archive digest mismatch")
    with tarfile.open(fileobj=io.BytesIO(archive), mode="r:") as bundle:
        for member in bundle.getmembers():
            member_path = Path(member.name)
            if not member.isfile() or member_path.is_absolute() or ".." in member_path.parts:
                raise SystemExit(f"unsafe INF-1 archive member: {member.name}")
            source = bundle.extractfile(member)
            if source is None:
                raise SystemExit(f"missing INF-1 archive payload: {member.name}")
            payload = source.read()
            target = ROOT / member_path
            target.parent.mkdir(parents=True, exist_ok=True)
            if target.exists() and target.read_bytes() != payload:
                raise SystemExit(f"refusing divergent INF-1 path: {member.name}")
            target.write_bytes(payload)
            target.chmod(member.mode)
    print("PASS_HEPTA_INFERENCE_INF1_MATERIALIZE")


if __name__ == "__main__":
    main()
