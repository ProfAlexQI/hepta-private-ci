#!/usr/bin/env python3
"""Q0.41 linear composition of Q0.40 and startup-order authority."""

from __future__ import annotations

import hashlib
import importlib.util
import stat
import subprocess
import sys
from pathlib import Path
from typing import Final

import verify_windows_gnullvm_q040_contract as q040
from hepta_q041_blob_contract import BLOBS, DIRECT, EXECUTABLE, JOB


ROOT = Path(__file__).resolve().parents[1]
GITHUB_SCRIPTS = ROOT / ".github" / "scripts"
BLOB_CONTRACT = ROOT / "scripts" / "hepta_q041_blob_contract.py"
BLOB_CONTRACT_SHA: Final = "b32370e9de9d63fbe7787b5547c3c4e3058254a1"
WRAPPER = ".github/scripts/run_bazel_with_buildbuddy.py"
DIRECT_WRAPPER = "scripts/verify-windows-gnullvm-direct-bazel.py"
JOB_WRAPPER = "scripts/verify-windows-gnullvm-job-executable.py"
Q040_DIRECT_BASE = "scripts/verify_windows_gnullvm_q038_direct_bazel_base.py"
Q040_JOB_BASE = "scripts/verify_windows_gnullvm_q038_job_executable_base.py"


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
        require(token in text, f"{owner} lacks Q0.41 token: {token}")


def load_source(relative: str, name: str):
    spec = importlib.util.spec_from_file_location(name, ROOT / relative)
    require(spec is not None and spec.loader is not None, f"cannot load {relative}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


def run_q040(owner: str) -> None:
    original_load = q040.load_source
    original_blobs = dict(q040.BLOBS)
    original_direct = q040.DIRECT
    original_job = q040.JOB

    def patched_load(relative: str, name: str):
        module = original_load(relative, name)
        if relative in {Q040_DIRECT_BASE, Q040_JOB_BASE}:
            require(
                hasattr(module, "EXPECTED_WRAPPER_BLOB"),
                f"legacy verifier lacks wrapper binding: {relative}",
            )
            module.EXPECTED_WRAPPER_BLOB = BLOBS[WRAPPER]
        return module

    q040.load_source = patched_load
    q040.BLOBS[DIRECT_WRAPPER] = BLOBS[DIRECT_WRAPPER]
    q040.BLOBS[JOB_WRAPPER] = BLOBS[JOB_WRAPPER]
    q040.DIRECT = DIRECT
    q040.JOB = JOB
    try:
        q040.main(owner)
    finally:
        q040.load_source = original_load
        q040.BLOBS.clear()
        q040.BLOBS.update(original_blobs)
        q040.DIRECT = original_direct
        q040.JOB = original_job


def validate_helper_semantics() -> None:
    if str(GITHUB_SCRIPTS) not in sys.path:
        sys.path.insert(0, str(GITHUB_SCRIPTS))
    helper = load_source(
        ".github/scripts/run_bazel_q039_startup_order.py",
        "_hepta_q041_startup_order",
    )
    cache = "--noexperimental_remote_repo_contents_cache"
    root = "--output_user_root=D:/b"
    env = {"BAZEL_OUTPUT_USER_ROOT": "D:/b"}
    require(
        helper.canonicalize_keyless_windows_gnullvm_base_startup(
            [cache, root],
            env,
        )
        == [root, cache],
        "Q0.41 did not normalize the real caller startup vector",
    )
    for candidate in (
        [root, root, cache],
        [root, cache, "--host_jvm_args=-Xmx4g"],
        ["--output_user_root=C:/attacker", cache],
    ):
        try:
            helper.canonicalize_keyless_windows_gnullvm_base_startup(
                candidate,
                env,
            )
        except ValueError:
            continue
        fail(f"Q0.41 accepted invalid startup vector: {candidate!r}")


def validate_increment() -> None:
    require(
        blob(BLOB_CONTRACT) == BLOB_CONTRACT_SHA,
        "Q0.41 blob contract drifted",
    )
    for relative, expected in BLOBS.items():
        path = ROOT / relative
        require(path.is_file(), f"missing Q0.41 blob: {relative}")
        require(blob(path) == expected, f"Q0.41 blob drift: {relative}")
    for relative in EXECUTABLE:
        require_executable(relative)

    require(
        read(DIRECT_WRAPPER) == DIRECT,
        "Q0.41 direct verifier compatibility wrapper drifted",
    )
    require(
        read(JOB_WRAPPER) == JOB,
        "Q0.41 job verifier compatibility wrapper drifted",
    )

    wrapper = read(WRAPPER)
    require_tokens(
        wrapper,
        (
            "Q0.34/Q0.39 execution binding",
            "from run_bazel_q039_startup_order import (",
            "canonicalize_keyless_windows_gnullvm_base_startup",
            "startup = canonicalize_keyless_windows_gnullvm_base_startup(",
            "validate_keyless_windows_gnullvm_final_args(command[1:], env)",
        ),
        "public Bazel wrapper",
    )
    require(
        wrapper.index("startup = canonicalize_keyless_windows_gnullvm_base_startup")
        < wrapper.index("strict_rc = ["),
        "startup canonicalization must precede strict rc construction",
    )

    startup_test = read(
        ".github/scripts/test_run_bazel_startup_contract.py"
    )
    require_tokens(
        startup_test,
        (
            "from run_bazel_q017_policy import _git_blob_sha1",
            "test_real_ci_explicit_output_user_root_is_canonicalized",
            "test_reversed_exact_base_startup_is_canonicalized",
            "test_duplicate_output_user_root_fails_closed",
            "test_startup_order_helper_rejects_unreviewed_options",
        ),
        "startup regression",
    )

    startup_verifier = read(
        "scripts/verify-windows-gnullvm-startup-contract.py"
    )
    require_tokens(
        startup_verifier,
        (
            'EXPECTED_Q039_POLICY_BLOB = "e0923474a529b37ef416ab9af90cc0745079afe5"',
            'EXPECTED_WRAPPER_BLOB = "cf5e7d990e1c649dac505ff98199cffa60def08d"',
            'EXPECTED_STARTUP_TEST_BLOB = "20a37801e00df21fda03011102fde499963892bc"',
            "Q0.39 canonicalization must precede strict rc construction",
            "PASS_WINDOWS_GNULLVM_STARTUP_CONTRACT_SOURCE",
        ),
        "startup source verifier",
    )

    validate_helper_semantics()
    verifier = load_source(
        "scripts/verify-windows-gnullvm-startup-contract.py",
        "_hepta_q041_startup_verifier",
    )
    verifier.main()


def main(owner: str = "q041") -> None:
    require(
        owner in {"q041", "direct-bazel", "job-executable"},
        f"unknown Q0.41 owner {owner!r}",
    )
    q040_owner = "q040" if owner == "q041" else owner
    run_q040(q040_owner)
    validate_increment()
    suffix = owner.upper().replace("-", "_")
    print(f"PASS_WINDOWS_GNULLVM_Q0_41_{suffix}_SOURCE")


if __name__ == "__main__":
    main()
