#!/usr/bin/env python3
"""Q0.44 replace every inherited host-mode proof before Q0.43 replay."""

from __future__ import annotations

import hashlib
import re
import subprocess
from pathlib import Path
from typing import Final

import verify_windows_gnullvm_q041_contract as q041
import verify_windows_gnullvm_q043_contract as q043
from hepta_q044_blob_contract import BLOBS, DIRECT, EXECUTABLE, JOB


ROOT = Path(__file__).resolve().parents[1]
BLOB_CONTRACT = ROOT / "scripts" / "hepta_q044_blob_contract.py"
BLOB_CONTRACT_SHA: Final = "3d156a5275cc406a65219116fe077ac159a7ccf2"
Q043_BLOB_CONTRACT = "scripts/hepta_q043_blob_contract.py"
Q043_SOURCE = "scripts/verify_windows_gnullvm_q043_contract.py"
Q041_SOURCE = "scripts/verify_windows_gnullvm_q041_contract.py"
Q040_SOURCE = "scripts/verify_windows_gnullvm_q040_contract.py"
STARTUP_VERIFIER = "scripts/verify-windows-gnullvm-startup-contract.py"
RECEIPT_VERIFIER = (
    "scripts/verify-windows-gnullvm-setup-token-receipt-truth.py"
)
DIRECT_ADAPTER = "scripts/verify-windows-gnullvm-direct-bazel.py"
JOB_ADAPTER = "scripts/verify-windows-gnullvm-job-executable.py"
_ORIGINAL_LOAD_SOURCE = q041.q040.load_source


def fail(message: str) -> None:
    raise SystemExit(message)


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def read(relative: str) -> str:
    path = ROOT / relative
    require(path.is_file(), f"missing Q0.44 path: {relative}")
    return path.read_text(encoding="utf-8")


def blob(path: Path) -> str:
    data = path.read_bytes()
    framed = f"blob {len(data)}\0".encode("ascii") + data
    return hashlib.sha1(framed, usedforsecurity=False).hexdigest()


def _git_relative(path_or_relative: str | Path) -> str:
    candidate = Path(path_or_relative)
    if candidate.is_absolute():
        try:
            candidate = candidate.resolve(strict=False).relative_to(
                ROOT.resolve(strict=True)
            )
        except (OSError, ValueError) as error:
            fail(
                "executable proof path escaped repository: "
                f"{path_or_relative}: {error}"
            )
    normalized = candidate.as_posix()
    require(
        normalized not in {"", "."}
        and not normalized.startswith("../")
        and "/../" not in f"/{normalized}",
        f"non-canonical executable proof path: {normalized!r}",
    )
    return normalized


def _validate_git_index_output(relative: str, output: str) -> None:
    entries = output.splitlines()
    require(
        len(entries) == 1,
        f"expected exactly one Git index entry for {relative}; "
        f"observed {len(entries)}",
    )
    fields = entries[0].split(maxsplit=3)
    require(len(fields) == 4, f"malformed Git index entry for {relative}")
    mode, object_id, stage, indexed_path = fields
    require(mode == "100755", f"lost Git executable mode: {relative}")
    require(stage == "0", f"unmerged Git index stage for {relative}: {stage}")
    require(
        re.fullmatch(r"[0-9a-f]{40}", object_id) is not None,
        f"invalid Git object ID for {relative}: {object_id!r}",
    )
    require(
        indexed_path == relative,
        f"Git index path drift for {relative}: {indexed_path!r}",
    )


def require_git_executable(path_or_relative: str | Path) -> None:
    """Prove executable authority from one canonical stage-0 Git index entry."""

    relative = _git_relative(path_or_relative)
    result = subprocess.run(
        ["git", "ls-files", "--stage", "--", relative],
        cwd=ROOT,
        capture_output=True,
        text=True,
        check=False,
    )
    require(result.returncode == 0, f"Git index lookup failed: {relative}")
    _validate_git_index_output(relative, result.stdout)


def prove_git_index_parser_fail_closed() -> None:
    relative = "scripts/example.py"
    object_id = "a" * 40
    valid = f"100755 {object_id} 0\t{relative}\n"
    _validate_git_index_output(relative, valid)

    invalid = (
        "",
        valid + valid,
        f"100644 {object_id} 0\t{relative}\n",
        f"100755 {object_id} 1\t{relative}\n",
        f"100755 not-an-object 0\t{relative}\n",
        f"100755 {object_id} 0\tscripts/other.py\n",
        f"100755 {object_id}\n",
    )
    for output in invalid:
        try:
            _validate_git_index_output(relative, output)
        except SystemExit:
            continue
        fail(f"Git-index adversarial fixture unexpectedly passed: {output!r}")

    try:
        _git_relative("../escape.py")
    except SystemExit:
        pass
    else:
        fail("repository-escape executable path unexpectedly passed")


def strict_load_source(relative: str, name: str):
    """Load an archived verifier and replace host-mode helpers before execution."""

    module = _ORIGINAL_LOAD_SOURCE(relative, name)
    for attribute in (
        "require_executable",
        "require_git_executable",
        "require_git_mode",
    ):
        if hasattr(module, attribute):
            setattr(module, attribute, require_git_executable)
    return module


def patch_q043_compatibility() -> None:
    require(
        blob(ROOT / Q043_BLOB_CONTRACT) == BLOBS[Q043_BLOB_CONTRACT],
        "immutable Q0.43 blob contract drifted",
    )
    require(
        blob(ROOT / Q043_SOURCE) == BLOBS[Q043_SOURCE],
        "immutable Q0.43 source contract drifted",
    )
    require(
        blob(ROOT / Q041_SOURCE) == BLOBS[Q041_SOURCE],
        "corrected Q0.41 source contract drifted",
    )
    require(
        blob(ROOT / Q040_SOURCE) == BLOBS[Q040_SOURCE],
        "immutable Q0.40 source contract drifted",
    )

    q043.BLOBS = dict(q043.BLOBS)
    q043.BLOBS[DIRECT_ADAPTER] = BLOBS[DIRECT_ADAPTER]
    q043.BLOBS[JOB_ADAPTER] = BLOBS[JOB_ADAPTER]
    q043.DIRECT = DIRECT
    q043.JOB = JOB

    # Patch every known static and dynamically loaded executable-mode proof
    # before Q0.43 invokes Q0.41, Q0.40, startup, receipt, or archived verifiers.
    q041.require_executable = require_git_executable
    q041.q040.require_executable = require_git_executable
    q043.require_git_mode = require_git_executable
    q041.q040.load_source = strict_load_source


def validate_increment() -> None:
    require(
        blob(BLOB_CONTRACT) == BLOB_CONTRACT_SHA,
        "Q0.44 blob contract drifted",
    )
    for relative, expected in BLOBS.items():
        path = ROOT / relative
        require(path.is_file(), f"missing Q0.44 blob: {relative}")
        require(blob(path) == expected, f"Q0.44 blob drift: {relative}")
    for relative in EXECUTABLE:
        require_git_executable(relative)
    prove_git_index_parser_fail_closed()

    require(read(DIRECT_ADAPTER) == DIRECT, "Q0.44 direct adapter drifted")
    require(read(JOB_ADAPTER) == JOB, "Q0.44 job adapter drifted")

    q041_source = read(Q041_SOURCE)
    for token in (
        'output_user_root = env.get("BAZEL_OUTPUT_USER_ROOT")',
        "if not output_user_root:",
    ):
        require(token in q041_source, f"corrected Q0.41 token missing: {token}")

    for owner, observed in (
        ("Q0.41", q041.require_executable),
        ("Q0.40", q041.q040.require_executable),
        ("Q0.43", q043.require_git_mode),
    ):
        require(
            observed is require_git_executable,
            f"{owner} retained a host-filesystem executable-mode proof",
        )
    require(
        q041.q040.load_source is strict_load_source,
        "dynamic verifier loading bypassed Q0.44 Git-mode proof",
    )

    receipt = read(RECEIPT_VERIFIER)
    for token in (
        '"git", "ls-files", "--stage", "--", relative',
        'entries[0].split(maxsplit=1)[0] == "100755"',
        "PASS_WINDOWS_GNULLVM_Q0_42_RECEIPT_GIT_MODE_TRUTH_SOURCE",
    ):
        require(token in receipt, f"receipt Git-mode proof missing: {token}")

    startup_module = strict_load_source(
        STARTUP_VERIFIER,
        "_hepta_q044_startup_mode_probe",
    )
    require(
        startup_module.require_executable is require_git_executable,
        "startup verifier retained host executable-bit inference",
    )

    print("PASS_WINDOWS_GNULLVM_Q0_44_CROSS_PLATFORM_MODE_PROOF_SOURCE")


def main(owner: str = "q044") -> None:
    require(
        owner in {"q044", "direct-bazel", "job-executable"},
        f"unknown Q0.44 owner {owner!r}",
    )
    patch_q043_compatibility()
    q043.main("q043" if owner == "q044" else owner)
    validate_increment()
    suffix = owner.upper().replace("-", "_")
    print(f"PASS_WINDOWS_GNULLVM_Q0_44_{suffix}_SOURCE")


if __name__ == "__main__":
    main()
