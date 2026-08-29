#!/usr/bin/env python3
"""Recover and validate the complete P0.3.4 payload from its immutable workflow."""

from __future__ import annotations

import base64
import hashlib
import sys
import tarfile
import tempfile
from pathlib import Path, PurePosixPath

EXPECTED_SHA256 = "bd21ef335c14fa2c78f6a2bb87b4c39d1a8937361c62ca80a92b5907a43617cd"
START = "cat > /tmp/p034-payload.b64 <<'EOF'\n"
END = "\nEOF\n"


def main() -> int:
    if len(sys.argv) != 3:
        raise SystemExit(
            "usage: p034-extract-full-payload.py IMMUTABLE_WORKFLOW OUTPUT_B64"
        )
    workflow = Path(sys.argv[1])
    output = Path(sys.argv[2])
    text = workflow.read_text(encoding="utf-8")
    if text.count(START) != 1:
        raise SystemExit("complete P0.3.4 payload start marker drifted")
    tail = text.split(START, 1)[1]
    if END not in tail:
        raise SystemExit("complete P0.3.4 payload end marker is missing")
    encoded = "".join(tail.split(END, 1)[0].split())
    payload = base64.b64decode(encoded, validate=True)
    actual = hashlib.sha256(payload).hexdigest()
    if actual != EXPECTED_SHA256:
        raise SystemExit(
            f"complete P0.3.4 payload digest mismatch: expected "
            f"{EXPECTED_SHA256}, got {actual}"
        )

    with tempfile.TemporaryDirectory(prefix="p034-full-payload-") as temporary:
        archive = Path(temporary) / "payload.tar.gz"
        archive.write_bytes(payload)
        with tarfile.open(archive, mode="r:gz") as tar:
            members = tar.getmembers()
            if not members:
                raise SystemExit("complete P0.3.4 payload archive is empty")
            names = {member.name for member in members}
            if "p034_apply_patch.py" not in names:
                raise SystemExit("complete payload lacks p034_apply_patch.py")
            for member in members:
                path = PurePosixPath(member.name)
                if (
                    path.is_absolute()
                    or ".." in path.parts
                    or member.issym()
                    or member.islnk()
                    or member.isdev()
                ):
                    raise SystemExit(
                        f"unsafe complete P0.3.4 payload entry: {member.name}"
                    )

    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(encoded + "\n", encoding="ascii")
    print(
        f"recovered complete P0.3.4 payload: bytes={len(payload)} "
        f"sha256={actual}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
