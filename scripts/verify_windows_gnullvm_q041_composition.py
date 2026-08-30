#!/usr/bin/env python3
"""Q0.41 additive composition of startup order onto selected Q0.40."""

from __future__ import annotations

import hashlib
import stat
import subprocess
from pathlib import Path
from typing import Final

import verify_windows_gnullvm_q040_contract as q040


ROOT = Path(__file__).resolve().parents[1]
Q040_CONTRACT = ROOT / "scripts" / "verify_windows_gnullvm_q040_contract.py"
Q040_BLOB_CONTRACT = ROOT / "scripts" / "hepta_q040_blob_contract.py"

EXPECTED_Q040_CONTRACT_BLOB: Final = (
    "a8363f65f46fd0482034a107bf517bad9a6a6143"
)
EXPECTED_Q040_BLOB_CONTRACT_BLOB: Final = (
    "57c896107f8eab161a8ccde407e661bc2d9f4ac8"
)

DIRECT = """#!/usr/bin/env python3

from verify_windows_gnullvm_q041_composition import main


if __name__ == "__main__":
    main("direct-bazel")
"""
JOB = DIRECT.replace('main("direct-bazel")', 'main("job-executable")')

DELTA_BLOBS: Final = {
    ".github/scripts/run_bazel_q039_startup_order.py": (
        "e0923474a529b37ef416ab9af90cc0745079afe5"
    ),
    ".github/scripts/run_bazel_with_buildbuddy.py": (
        "cf5e7d990e1c649dac505ff98199cffa60def08d"
    ),
    ".github/scripts/test_run_bazel_startup_contract.py": (
        "20a37801e00df21fda03011102fde499963892bc"
    ),
    "scripts/verify-windows-gnullvm-startup-contract.py": (
        "de42d9bb3fbfef16ad880e0ab05345ef3731dfbd"
    ),
    "scripts/verify-windows-gnullvm-direct-bazel.py": (
        "d9115ba7ec71979c727fc979ef6b492166404bef"
    ),
    "scripts/verify-windows-gnullvm-job-executable.py": (
        "386838ea799ff00cb4628748ffc196bd273cb1ac"
    ),
}
DELTA_EXECUTABLE: Final = (
    ".github/scripts/test_run_bazel_startup_contract.py",
    "scripts/verify-windows-gnullvm-startup-contract.py",
    "scripts/verify-windows-gnullvm-direct-bazel.py",
    "scripts/verify-windows-gnullvm-job-executable.py",
    "scripts/verify_windows_gnullvm_q041_composition.py",
)

_composed = False


def fail(message: str) -> None:
    raise SystemExit(message)


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def read(relative: str) -> str:
    path = ROOT / relative
    require(path.is_file(), f"missing Q0.41 path: {relative}")
    return path.read_text(encoding="utf-8")


def blob(path: Path) -> str:
    data = path.read_bytes()
    framed = f"blob {len(data)}\0".encode("ascii") + data
    return hashlib.sha1(framed, usedforsecurity=False).hexdigest()


def require_git_executable(relative: str) -> None:
    path = ROOT / relative
    require(
        bool(path.stat().st_mode & stat.S_IXUSR),
        f"lost filesystem executable mode: {relative}",
    )
    result = subprocess.run(
        ["git", "ls-files", "--stage", "--", relative],
        cwd=ROOT,
        capture_output=True,
        text=True,
        check=False,
    )
    entries = result.stdout.splitlines()
    require(
        result.returncode == 0
        and len(entries) == 1
        and entries[0].split(maxsplit=1)[0] == "100755",
        f"lost Git executable mode: {relative}",
    )


def require_tokens(text: str, tokens: tuple[str, ...], owner: str) -> None:
    for token in tokens:
        require(token in text, f"{owner} lacks Q0.41 token: {token}")


def require_order(text: str, before: str, after: str, owner: str) -> None:
    require_tokens(text, (before, after), owner)
    require(
        text.index(before) < text.index(after),
        f"{owner} must place {before!r} before {after!r}",
    )


def _compose_q040_base() -> None:
    global _composed
    if _composed:
        return

    require(
        blob(Q040_CONTRACT) == EXPECTED_Q040_CONTRACT_BLOB,
        "immutable selected Q0.40 contract drifted",
    )
    require(
        blob(Q040_BLOB_CONTRACT) == EXPECTED_Q040_BLOB_CONTRACT_BLOB,
        "immutable selected Q0.40 blob contract drifted",
    )

    q040.BLOBS.update(DELTA_BLOBS)
    q040.EXECUTABLE = tuple(
        dict.fromkeys((*q040.EXECUTABLE, *DELTA_EXECUTABLE))
    )
    q040.DIRECT = DIRECT
    q040.JOB = JOB

    original_load_source = q040.load_source

    def load_source(relative: str, name: str):
        module = original_load_source(relative, name)
        if hasattr(module, "EXPECTED_WRAPPER_BLOB"):
            module.EXPECTED_WRAPPER_BLOB = DELTA_BLOBS[
                ".github/scripts/run_bazel_with_buildbuddy.py"
            ]
        return module

    q040.load_source = load_source
    _composed = True


def validate_startup_order() -> None:
    helper = read(".github/scripts/run_bazel_q039_startup_order.py")
    wrapper = read(".github/scripts/run_bazel_with_buildbuddy.py")
    test = read(".github/scripts/test_run_bazel_startup_contract.py")
    verifier = read("scripts/verify-windows-gnullvm-startup-contract.py")

    require_tokens(
        helper,
        (
            "def canonicalize_keyless_windows_gnullvm_base_startup",
            "exact startup vector requires exactly",
            "exact startup vector rejects unreviewed",
            "return [expected_output_root, DISABLED_REPO_CONTENTS_CACHE]",
        ),
        "startup-order helper",
    )
    require_tokens(
        wrapper,
        (
            "from run_bazel_q039_startup_order import (",
            "canonicalize_keyless_windows_gnullvm_base_startup",
            "startup = canonicalize_keyless_windows_gnullvm_base_startup(",
            "strict_rc = [",
            "validate_keyless_windows_gnullvm_final_args(command[1:], env)",
        ),
        "public Bazel wrapper",
    )
    require_order(
        wrapper,
        "startup = canonicalize_keyless_windows_gnullvm_base_startup(",
        "strict_rc = [",
        "public Bazel wrapper",
    )
    require_tokens(
        test,
        (
            "from run_bazel_q017_policy import _git_blob_sha1",
            "test_real_ci_explicit_output_user_root_is_canonicalized",
            "test_reversed_exact_base_startup_is_canonicalized",
            "test_duplicate_output_user_root_fails_closed",
            "test_startup_order_helper_rejects_unreviewed_options",
        ),
        "startup regression",
    )
    require_tokens(
        verifier,
        (
            "Q039_POLICY",
            "EXPECTED_Q039_POLICY_BLOB",
            "EXPECTED_WRAPPER_BLOB",
            "EXPECTED_STARTUP_TEST_BLOB",
            "canonicalize_keyless_windows_gnullvm_base_startup",
            "test_real_ci_explicit_output_user_root_is_canonicalized",
        ),
        "startup source verifier",
    )


def validate_q041_increment() -> None:
    for relative, expected in DELTA_BLOBS.items():
        path = ROOT / relative
        require(path.is_file(), f"missing Q0.41 delta blob: {relative}")
        require(blob(path) == expected, f"Q0.41 delta blob drift: {relative}")
    for relative in DELTA_EXECUTABLE:
        require_git_executable(relative)

    require(
        read("scripts/verify-windows-gnullvm-direct-bazel.py") == DIRECT,
        "Q0.41 direct-Bazel adapter drifted",
    )
    require(
        read("scripts/verify-windows-gnullvm-job-executable.py") == JOB,
        "Q0.41 job-executable adapter drifted",
    )
    validate_startup_order()


def main(owner: str = "q041") -> None:
    require(
        owner in {"q041", "direct-bazel", "job-executable"},
        f"unknown Q0.41 owner {owner!r}",
    )
    _compose_q040_base()
    if owner == "q041":
        q040.run_legacy("job-executable")
        q040.run_legacy("direct-bazel")
    else:
        q040.run_legacy(owner)
    q040.validate_increment()
    validate_q041_increment()
    suffix = owner.upper().replace("-", "_")
    print(f"PASS_WINDOWS_GNULLVM_Q0_41_{suffix}_SOURCE")


if __name__ == "__main__":
    main()
