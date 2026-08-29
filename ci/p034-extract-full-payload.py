#!/usr/bin/env python3
"""Validate the canonical P0.3.4 payload with a legacy recovery fallback."""

from __future__ import annotations

import base64
import binascii
import hashlib
import sys
import tarfile
import tempfile
from pathlib import Path, PurePosixPath

EXPECTED_SHA256 = "bd21ef335c14fa2c78f6a2bb87b4c39d1a8937361c62ca80a92b5907a43617cd"
START_MARKERS = (
    "cat > /tmp/p034-payload-full.tgz.b64 <<'EOF'\n",
    "cat > /tmp/p034-payload.b64 <<'EOF'\n",
)
END = "\nEOF\n"


def compact_base64(text: str) -> str:
    return "".join(text.split())


def diagnose_invalid_base64(encoded: str) -> str:
    alphabet = frozenset(
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/="
    )
    for index, character in enumerate(encoded):
        if character not in alphabet:
            return f"invalid character U+{ord(character):04X} at compact offset {index}"
    return "invalid padding or length"


def validate_payload(encoded_text: str, *, source: Path) -> tuple[str, bytes, str]:
    encoded = compact_base64(encoded_text)
    if not encoded:
        raise RuntimeError(f"{source}: complete P0.3.4 payload is empty")
    try:
        payload = base64.b64decode(encoded, validate=True)
    except (binascii.Error, ValueError) as error:
        detail = diagnose_invalid_base64(encoded)
        raise RuntimeError(
            f"{source}: complete P0.3.4 payload is not canonical Base64 "
            f"({detail}): {error}"
        ) from error

    actual = hashlib.sha256(payload).hexdigest()
    if actual != EXPECTED_SHA256:
        raise RuntimeError(
            f"{source}: complete P0.3.4 payload digest mismatch: expected "
            f"{EXPECTED_SHA256}, got {actual}"
        )

    with tempfile.TemporaryDirectory(prefix="p034-full-payload-") as temporary:
        archive = Path(temporary) / "payload.tar.gz"
        archive.write_bytes(payload)
        with tarfile.open(archive, mode="r:gz") as tar:
            members = tar.getmembers()
            if not members:
                raise RuntimeError("complete P0.3.4 payload archive is empty")
            names = {member.name for member in members}
            if "p034_apply_patch.py" not in names:
                raise RuntimeError("complete payload lacks p034_apply_patch.py")
            for member in members:
                path = PurePosixPath(member.name)
                if (
                    path.is_absolute()
                    or ".." in path.parts
                    or member.issym()
                    or member.islnk()
                    or member.isdev()
                ):
                    raise RuntimeError(
                        f"unsafe complete P0.3.4 payload entry: {member.name}"
                    )
    return encoded, payload, actual


def extract_workflow_payload(workflow: Path) -> str:
    text = workflow.read_text(encoding="utf-8").replace("\r\n", "\n")
    matches = [marker for marker in START_MARKERS if marker in text]
    if len(matches) != 1:
        raise RuntimeError(
            f"{workflow}: expected one complete P0.3.4 payload start marker, "
            f"observed {len(matches)}"
        )
    tail = text.split(matches[0], 1)[1]
    if END not in tail:
        raise RuntimeError(
            f"{workflow}: complete P0.3.4 payload end marker is missing"
        )
    return tail.split(END, 1)[0]


def validate_and_write(encoded_text: str, output: Path, *, source: Path) -> None:
    encoded, payload, actual = validate_payload(encoded_text, source=source)
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(encoded + "\n", encoding="ascii")
    print(
        f"validated complete P0.3.4 payload from {source}: "
        f"bytes={len(payload)} sha256={actual}"
    )


def main() -> int:
    if len(sys.argv) == 2:
        payload = Path(sys.argv[1])
        validate_and_write(payload.read_text(encoding="ascii"), payload, source=payload)
        return 0

    if len(sys.argv) != 3:
        raise SystemExit(
            "usage: p034-extract-full-payload.py PAYLOAD_B64 | "
            "IMMUTABLE_WORKFLOW OUTPUT_B64"
        )

    workflow = Path(sys.argv[1])
    output = Path(sys.argv[2])
    if output.is_file():
        try:
            validate_and_write(
                output.read_text(encoding="ascii"), output, source=output
            )
            return 0
        except (OSError, UnicodeError, RuntimeError) as canonical_error:
            print(
                f"canonical payload validation failed; attempting immutable "
                f"workflow recovery: {canonical_error}",
                file=sys.stderr,
            )

    try:
        recovered = extract_workflow_payload(workflow)
        validate_and_write(recovered, output, source=workflow)
    except (OSError, UnicodeError, RuntimeError) as error:
        raise SystemExit(str(error)) from error
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
