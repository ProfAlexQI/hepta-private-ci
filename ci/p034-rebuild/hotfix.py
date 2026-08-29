#!/usr/bin/env python3
"""Fail-closed validator for the exact P0.3.4 reconstruction payload.

This helper never edits the extracted payload. The archive digest is checked by
its workflow before extraction; this script then binds every extracted regular
file, validates the negative-authority status, and byte-compiles the Python
entrypoints before the publisher may touch the target branch.
"""

from __future__ import annotations

import hashlib
import json
import py_compile
import sys
from pathlib import Path

EXPECTED: dict[str, tuple[int, str]] = {
    "0012_legacy_grounding_governance.sql": (11095, "e31c69906fd4051ec5a12da310d35d460bff04c057aff5bdf604362cab238da4"),
    "P034_PLAN.md": (3188, "4027d75bbf430fcebd4e1ce63954cfecb7adbe6dc6eddaac66f1047085de64fe"),
    "P034_STATUS.json": (3107, "a7b410ed5640a1e00cb5eda578220bd54ade6f74c8572523f4a035c6e5bfec01"),
    "P034_TRANCHE.md": (1070, "379faa581645389bb0da19a9c8b8e64763fc27b3a0ac003211d70a1f28d85c6a"),
    "apply.py": (14425, "72e006bb01f50819820b9bdb65657affc87c48e053822a785602f6e68d9863fc"),
    "backfill.rs": (13956, "ee03f42fdb3c533372e8cd3d69c373bce9ae68244c7262a88a9e8d64caf7f35f"),
    "candidate-workflow.yml": (4111, "4bf939c40ac29038e8aeea44e12e2d45f8e5279553bbf3945aec2d749ab8bc4c"),
    "legacy_governance.rs": (70311, "a205514af4ae75d6a3565fa0bed1bb0d291e21948936d089f84f34ee8c66b324"),
    "run.py": (8118, "300b4b8854085d87e96bfeafe9c5db9f7652702ec0812cb2f811245a9f28b3e7"),
    "verify.py": (12185, "99fa58ca3f16024dde06a38160d5f5b90cae95aaee2b96f92c603bbe926bbdbd"),
}

FALSE_AUTHORITY = (
    "wired",
    "default_projection_pointer_changed",
    "default_recall_query_changed",
    "production_projection_gate",
    "production_authority",
    "external_effects",
    "operator_accepted",
    "promoted",
    "callers_ratchet",
)


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> int:
    if len(sys.argv) != 2:
        raise SystemExit("usage: p034-payload-validator.py EXTRACTED_PAYLOAD_DIR")
    root = Path(sys.argv[1]).resolve()
    if not root.is_dir():
        raise SystemExit(f"P0.3.4 extracted payload is not a directory: {root}")

    entries = list(root.iterdir())
    unsafe = [path.name for path in entries if path.is_symlink() or not path.is_file()]
    if unsafe:
        raise SystemExit(f"P0.3.4 payload contains non-regular entries: {sorted(unsafe)}")
    actual = {path.name for path in entries}
    expected = set(EXPECTED)
    if actual != expected:
        raise SystemExit(
            "P0.3.4 payload inventory mismatch: "
            f"missing={sorted(expected - actual)} extra={sorted(actual - expected)}"
        )

    observed: dict[str, dict[str, object]] = {}
    for name, (expected_size, expected_sha256) in sorted(EXPECTED.items()):
        path = root / name
        size = path.stat().st_size
        digest = sha256(path)
        if size != expected_size or digest != expected_sha256:
            raise SystemExit(
                f"P0.3.4 payload file drifted: {name} "
                f"size={size}/{expected_size} sha256={digest}/{expected_sha256}"
            )
        observed[name] = {"size": size, "sha256": digest}

    for name in ("apply.py", "run.py", "verify.py"):
        py_compile.compile(str(root / name), doraise=True)
    cache = root / "__pycache__"
    if cache.exists():
        for path in cache.iterdir():
            path.unlink()
        cache.rmdir()

    status = json.loads((root / "P034_STATUS.json").read_text(encoding="utf-8"))
    current = status.get("current_tranche")
    authority = status.get("authority")
    if not isinstance(current, dict) or not isinstance(authority, dict):
        raise SystemExit("P0.3.4 status contract structure drifted")
    if (
        current.get("id") != "P0.3.4"
        or current.get("implemented") is not True
        or current.get("qualified") is not False
        or current.get("wired") is not False
        or current.get("historical_payload_digest_reused") is not False
        or current.get("inherited_h7_expiry_test_race_hardened") is not True
        or current.get("strict_clippy_private_dead_fields_removed") is not True
    ):
        raise SystemExit("P0.3.4 tranche status is not the expected fail-closed candidate")
    drift = [key for key in FALSE_AUTHORITY if authority.get(key) is not False]
    if drift:
        raise SystemExit(f"P0.3.4 payload authority drifted: {drift}")

    receipt = {
        "schema": "hepta_intelligence_p0_3_4_payload_validation_v1",
        "archive_sha256": "f66d0ccfe03fd2dac66786e02077110ca7c5c1d251c445737fe528ca9061e621",
        "files": observed,
        "python_compile": True,
        "authority_all_false": True,
        "status": "PASS_P0_3_4_EXACT_RECONSTRUCTION_PAYLOAD",
    }
    print(json.dumps(receipt, indent=2, sort_keys=True), flush=True)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
