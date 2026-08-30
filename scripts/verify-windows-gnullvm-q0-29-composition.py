#!/usr/bin/env python3

"""Fail-closed source contract for the Q0.29 startup/execution composition."""

from __future__ import annotations

import hashlib
import stat
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCRIPTS = ROOT / ".github" / "scripts"
WRAPPER = SCRIPTS / "run_bazel_with_buildbuddy.py"
STARTUP_POLICY = SCRIPTS / "run_bazel_q028_startup_contract.py"
EXECUTION_POLICY = SCRIPTS / "run_bazel_q028_execution_context.py"
STARTUP_TEST = SCRIPTS / "test_run_bazel_startup_contract.py"
EXECUTION_TEST = SCRIPTS / "test_run_bazel_execution_context.py"
BOUNDARY = SCRIPTS / "test_run_bazel_qualification_boundary.sh"
FINAL_VERIFIER = ROOT / "scripts" / "verify-windows-gnullvm-final-command.py"
STARTUP_VERIFIER = ROOT / "scripts" / "verify-windows-gnullvm-startup-contract.py"
EXECUTION_VERIFIER = ROOT / "scripts" / "verify-windows-gnullvm-execution-context.py"
WORKFLOW = ROOT / ".github" / "workflows" / "windows-gnullvm-qualification-boundary.yml"

EXPECTED_BLOBS = {
    STARTUP_POLICY: "6711a2d5cdb63466f1895d539391af44e48b793f",
    EXECUTION_POLICY: "530f043516fc8f5db239b59daa9e10180e552482",
    STARTUP_TEST: "db2968ba6da135a1edb023be3d518e6e1768e0f5",
    EXECUTION_TEST: "2224aaddf42149da5fb50ecadd27268941c9363d",
    FINAL_VERIFIER: "234283066fc97a9d76ff40f043f790ef53bee29e",
    STARTUP_VERIFIER: "6d795d7764f489baf63189ee979a39e3b5408f99",
    EXECUTION_VERIFIER: "0d2829e2b7782739b18f9942dbfb9dcb2f9e9754",
}


def fail(message: str) -> None:
    raise SystemExit(message)


def read(path: Path) -> str:
    if not path.is_file():
        fail(f"missing Q0.29 composition path: {path.relative_to(ROOT)}")
    return path.read_text(encoding="utf-8")


def git_blob_sha(path: Path) -> str:
    content = path.read_bytes()
    framed = f"blob {len(content)}\0".encode("ascii") + content
    return hashlib.sha1(framed, usedforsecurity=False).hexdigest()


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def require_token(text: str, token: str, owner: str) -> None:
    require(token in text, f"{owner} lacks Q0.29 contract token: {token}")


def require_order(text: str, before: str, after: str, owner: str) -> None:
    require_token(text, before, owner)
    require_token(text, after, owner)
    require(
        text.index(before) < text.index(after),
        f"{owner} must place {before!r} before {after!r}",
    )


def main() -> None:
    for path, expected in EXPECTED_BLOBS.items():
        read(path)
        observed = git_blob_sha(path)
        require(
            observed == expected,
            f"Q0.29 immutable sibling input drifted: {path.relative_to(ROOT)} "
            f"expected {expected}, observed {observed}",
        )

    wrapper = read(WRAPPER)
    boundary = read(BOUNDARY)
    workflow = read(WORKFLOW)

    for path in (
        WRAPPER,
        STARTUP_TEST,
        EXECUTION_TEST,
        BOUNDARY,
        FINAL_VERIFIER,
        STARTUP_VERIFIER,
        EXECUTION_VERIFIER,
    ):
        require(
            bool(path.stat().st_mode & stat.S_IXUSR),
            f"Q0.29 executable mode lost: {path.relative_to(ROOT)}",
        )

    for token in (
        "composed at Q0.29",
        "_validate_q026_compatibility_base",
        "_validate_q027_compatibility_base",
        "from run_bazel_q028_startup_contract import (",
        "from run_bazel_q028_execution_context import bind_verified_bazelisk",
        "from run_bazel_q028_execution_context import prepare_bazelisk_environment",
        "validate_keyless_windows_gnullvm_execution_context",
        "def bazel_command(",
        "validate_keyless_windows_gnullvm_final_args(command[1:], env)",
        "def executable_command(",
        "prepare_bazelisk_environment(env)",
        "command = bind_verified_bazelisk(command, env)",
        "validate_keyless_windows_gnullvm_execution_context(command, env)",
        "command = executable_command(*sys.argv[1:])",
    ):
        require_token(wrapper, token, "run_bazel_with_buildbuddy.py")

    require_order(
        wrapper,
        "validate_keyless_windows_gnullvm_final_args(command[1:], env)",
        "prepare_bazelisk_environment(env)",
        "run_bazel_with_buildbuddy.py",
    )
    require_order(
        wrapper,
        "prepare_bazelisk_environment(env)",
        "command = bind_verified_bazelisk(command, env)",
        "run_bazel_with_buildbuddy.py",
    )
    require_order(
        wrapper,
        "command = bind_verified_bazelisk(command, env)",
        "validate_keyless_windows_gnullvm_execution_context(command, env)",
        "run_bazel_with_buildbuddy.py",
    )

    boundary_tokens = (
        "python3 .github/scripts/test_run_bazel_startup_contract.py",
        "python3 .github/scripts/test_run_bazel_execution_context.py",
        "python3 scripts/verify-windows-gnullvm-final-command.py",
        "python3 scripts/verify-windows-gnullvm-startup-contract.py",
        "python3 scripts/verify-windows-gnullvm-execution-context.py",
        "python3 scripts/verify-windows-gnullvm-q0-29-composition.py",
    )
    for token in boundary_tokens:
        require_token(boundary, token, "qualification boundary fixture")

    workflow_tokens = (
        "python3 scripts/verify-windows-gnullvm-final-command.py",
        "python3 scripts/verify-windows-gnullvm-startup-contract.py",
        "python3 scripts/verify-windows-gnullvm-execution-context.py",
        "python3 scripts/verify-windows-gnullvm-q0-29-composition.py",
        '"final_command_source_contract": "PASS_WINDOWS_GNULLVM_FINAL_COMMAND_SOURCE"',
        '"startup_vector_source_contract": "PASS_WINDOWS_GNULLVM_STARTUP_CONTRACT_SOURCE"',
        '"execution_context_source_contract": "PASS_WINDOWS_GNULLVM_Q0_28_EXECUTION_CONTEXT_SOURCE"',
        '"q0_29_composition_source_contract": "PASS_WINDOWS_GNULLVM_Q0_29_COMPOSITION_SOURCE"',
        '"windows_executable_digest_executed": False',
        '"a0_candidate_qualified": False',
        '"runtime_authority": False',
        '"production_authority": False',
        '"operator_acceptance": False',
        '"promotion": False',
        '"release_authority": False',
        '"callers_ratchet": False',
    )
    for token in workflow_tokens:
        require_token(workflow, token, "Windows boundary workflow")

    print("PASS_WINDOWS_GNULLVM_Q0_29_COMPOSITION_SOURCE")


if __name__ == "__main__":
    main()
