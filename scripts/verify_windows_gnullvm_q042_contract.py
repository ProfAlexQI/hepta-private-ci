#!/usr/bin/env python3

"""Q0.42 bounded successor for Q0.41 startup and Q0.40 qualification."""

from __future__ import annotations

import hashlib
import stat
import subprocess
from pathlib import Path
from typing import Final

import verify_windows_gnullvm_q040_contract as q040
from hepta_q042_blob_contract import BLOBS, DIRECT, EXECUTABLE, JOB


ROOT = Path(__file__).resolve().parents[1]
BLOB_CONTRACT = ROOT / "scripts" / "hepta_q042_blob_contract.py"
BLOB_CONTRACT_SHA: Final = "26857f4d8e63ffac33e02dee58f6ee11ccc132ac"
WRAPPER = ".github/scripts/run_bazel_with_buildbuddy.py"
STARTUP_POLICY = ".github/scripts/run_bazel_q039_startup_order.py"
STARTUP_TEST = ".github/scripts/test_run_bazel_startup_contract.py"
STARTUP_VERIFIER = "scripts/verify-windows-gnullvm-startup-contract.py"
Q041_PREDECESSOR = "scripts/verify_windows_gnullvm_q041_composition.py"
DIRECT_ADAPTER = "scripts/verify-windows-gnullvm-direct-bazel.py"
JOB_ADAPTER = "scripts/verify-windows-gnullvm-job-executable.py"


def fail(message: str) -> None:
    raise SystemExit(message)


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def read(relative: str) -> str:
    path = ROOT / relative
    require(path.is_file(), f"missing Q0.42 path: {relative}")
    return path.read_text(encoding="utf-8")


def blob(path: Path) -> str:
    data = path.read_bytes()
    framed = f"blob {len(data)}\0".encode("ascii") + data
    return hashlib.sha1(framed, usedforsecurity=False).hexdigest()


def require_executable(relative: str) -> None:
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
        require(token in text, f"{owner} lacks Q0.42 token: {token}")


def run_legacy(owner: str) -> None:
    if owner == "direct-bazel":
        module = q040.load_source(
            "scripts/verify_windows_gnullvm_q038_direct_bazel_base.py",
            "_hepta_q042_direct_base",
        )
        patches = {
            "EXPECTED_Q034_BLOB": q040.BLOBS[
                ".github/scripts/run_bazel_q034_execution_manifest.py"
            ],
            "EXPECTED_WRAPPER_BLOB": BLOBS[WRAPPER],
            "EXPECTED_EXECUTION_TEST_BLOB": q040.BLOBS[
                ".github/scripts/test_run_bazel_execution_manifest.py"
            ],
            "EXPECTED_BOUNDARY_BLOB": q040.BLOBS[
                ".github/scripts/test_run_bazel_qualification_boundary.sh"
            ],
            "EXPECTED_QUALIFICATION_WORKFLOW_BLOB": q040.BLOBS[
                ".github/workflows/windows-gnullvm-qualification-boundary.yml"
            ],
        }
    else:
        module = q040.load_source(
            "scripts/verify_windows_gnullvm_q038_job_executable_base.py",
            "_hepta_q042_job_base",
        )
        patches = {
            "EXPECTED_Q034_BLOB": q040.BLOBS[
                ".github/scripts/run_bazel_q034_execution_manifest.py"
            ],
            "EXPECTED_WRAPPER_BLOB": BLOBS[WRAPPER],
            "EXPECTED_EXECUTION_TEST_BLOB": q040.BLOBS[
                ".github/scripts/test_run_bazel_execution_manifest.py"
            ],
            "EXPECTED_FIXTURE_BLOB": q040.BLOBS[
                ".github/scripts/test_run_bazel_qualification_boundary.sh"
            ],
            "EXPECTED_BOUNDARY_BLOB": q040.BLOBS[
                ".github/workflows/windows-gnullvm-qualification-boundary.yml"
            ],
        }
    for key, value in patches.items():
        require(hasattr(module, key), f"legacy {owner} verifier lacks {key}")
        setattr(module, key, value)
    module.main()


def patch_q040_compatibility() -> None:
    q040.BLOBS[DIRECT_ADAPTER] = BLOBS[DIRECT_ADAPTER]
    q040.BLOBS[JOB_ADAPTER] = BLOBS[JOB_ADAPTER]
    q040.DIRECT = DIRECT
    q040.JOB = JOB
    q040.run_legacy = run_legacy


def validate_increment() -> None:
    require(
        blob(BLOB_CONTRACT) == BLOB_CONTRACT_SHA,
        "Q0.42 blob contract drifted",
    )
    for relative, expected in BLOBS.items():
        path = ROOT / relative
        require(path.is_file(), f"missing Q0.42 blob: {relative}")
        require(blob(path) == expected, f"Q0.42 blob drift: {relative}")
    for relative in EXECUTABLE:
        require_executable(relative)

    require(read(DIRECT_ADAPTER) == DIRECT, "Q0.42 direct adapter drifted")
    require(read(JOB_ADAPTER) == JOB, "Q0.42 job adapter drifted")

    predecessor = read(Q041_PREDECESSOR)
    require_tokens(
        predecessor,
        (
            "EXPECTED_Q040_CONTRACT_BLOB",
            "EXPECTED_Q040_BLOB_CONTRACT_BLOB",
            "DELTA_BLOBS",
            "_compose_q040_base",
            "validate_startup_order",
            "PASS_WINDOWS_GNULLVM_Q0_41_",
        ),
        "immutable Q0.41 predecessor",
    )

    wrapper = read(WRAPPER)
    policy = read(STARTUP_POLICY)
    test = read(STARTUP_TEST)
    verifier = read(STARTUP_VERIFIER)

    require_tokens(
        wrapper,
        (
            "from run_bazel_q039_startup_order import (",
            "canonicalize_keyless_windows_gnullvm_base_startup",
            "startup = canonicalize_keyless_windows_gnullvm_base_startup(",
            "validate_keyless_windows_gnullvm_final_args(command[1:], env)",
            "validate_keyless_windows_gnullvm_execution",
        ),
        "public Bazel wrapper",
    )
    require(
        wrapper.index(
            "startup = canonicalize_keyless_windows_gnullvm_base_startup("
        )
        < wrapper.index("strict_rc = ["),
        "startup canonicalization must precede strict rc construction",
    )
    require_tokens(
        policy,
        (
            "def canonicalize_keyless_windows_gnullvm_base_startup",
            "requires BAZEL_OUTPUT_USER_ROOT",
            "exact startup vector requires exactly",
            "exact startup vector rejects unreviewed",
            "return [expected_output_root, DISABLED_REPO_CONTENTS_CACHE]",
        ),
        "startup-order policy",
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
            f'EXPECTED_Q039_POLICY_BLOB = "{BLOBS[STARTUP_POLICY]}"',
            f'EXPECTED_WRAPPER_BLOB = "{BLOBS[WRAPPER]}"',
            f'EXPECTED_STARTUP_TEST_BLOB = "{BLOBS[STARTUP_TEST]}"',
            "canonicalization must precede strict rc construction",
            "PASS_WINDOWS_GNULLVM_STARTUP_CONTRACT_SOURCE",
        ),
        "startup source verifier",
    )

    boundary = read(
        ".github/scripts/test_run_bazel_qualification_boundary.sh"
    )
    require_tokens(
        boundary,
        (
            "python3 .github/scripts/test_run_bazel_startup_contract.py",
            "python3 scripts/verify-windows-gnullvm-startup-contract.py",
            "python3 scripts/verify-windows-gnullvm-direct-bazel.py",
            "python3 scripts/verify-windows-gnullvm-job-executable.py",
        ),
        "qualification boundary",
    )

    startup_module = q040.load_source(
        STARTUP_VERIFIER,
        "_hepta_q042_startup_contract",
    )
    startup_module.main()


def main(owner: str = "q042") -> None:
    require(
        owner in {"q042", "direct-bazel", "job-executable"},
        f"unknown Q0.42 owner {owner!r}",
    )
    patch_q040_compatibility()
    q040.main("q040" if owner == "q042" else owner)
    validate_increment()
    suffix = owner.upper().replace("-", "_")
    print(f"PASS_WINDOWS_GNULLVM_Q0_42_{suffix}_SOURCE")


if __name__ == "__main__":
    main()
