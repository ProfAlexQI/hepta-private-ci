#!/usr/bin/env python3

from __future__ import annotations

import hashlib
import stat
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCRIPTS = ROOT / ".github" / "scripts"
Q026_POLICY = SCRIPTS / "run_bazel_q022_negative_targets.py"
Q028_POLICY = SCRIPTS / "run_bazel_q028_startup_contract.py"
Q029_POLICY = SCRIPTS / "run_bazel_q029_job_executable.py"
Q030_POLICY = SCRIPTS / "run_bazel_q030_direct_bazel.py"
WRAPPER = SCRIPTS / "run_bazel_with_buildbuddy.py"
TEST = SCRIPTS / "test_run_bazel_direct_bazel.py"
TOKEN_TEST = SCRIPTS / "test_run_bazel_transport_token.py"
STARTUP_TEST = SCRIPTS / "test_run_bazel_startup_contract.py"
BOUNDARY = SCRIPTS / "test_run_bazel_qualification_boundary.sh"
WORKFLOW = ROOT / ".github" / "workflows" / "windows-gnullvm-qualification-boundary.yml"
BAZELVERSION = ROOT / ".bazelversion"

EXPECTED_Q026_BLOB = "e0729bd796b342568c624d15faf3638a1372d01d"
EXPECTED_Q028_BLOB = "86225acd9158132df8cd5ae9dc6720205a7c47a6"
EXPECTED_Q029_BLOB = "2d57d5e222b87a89b2f8b1c93c476f450b03e646"
EXPECTED_Q030_BLOB = "cadc4ffd91e16171e92091a092f020a10ae7cfb0"
EXPECTED_WRAPPER_BLOB = "233d98f151b897caa42f4d762d119645cb13e641"
EXPECTED_BAZELVERSION_BLOB = "f7ee06693c17a06e2a0f51ef7eb2a61866e77b8e"


def fail(message: str) -> None:
    raise SystemExit(message)


def read(path: Path) -> str:
    if not path.is_file():
        fail(f"missing Q0.30/Q0.32 contract path: {path.relative_to(ROOT)}")
    return path.read_text(encoding="utf-8")


def git_blob_sha(path: Path) -> str:
    content = path.read_bytes()
    framed = f"blob {len(content)}\0".encode("ascii") + content
    return hashlib.sha1(framed, usedforsecurity=False).hexdigest()


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def require_token(text: str, token: str, owner: str) -> None:
    require(token in text, f"{owner} lacks Q0.30/Q0.32 token: {token}")


def require_executable(path: Path) -> None:
    require(
        bool(path.stat().st_mode & stat.S_IXUSR),
        f"required executable lost mode: {path.relative_to(ROOT)}",
    )


def main() -> None:
    q026 = read(Q026_POLICY)
    q028 = read(Q028_POLICY)
    q029 = read(Q029_POLICY)
    q030 = read(Q030_POLICY)
    wrapper = read(WRAPPER)
    test = read(TEST)
    token_test = read(TOKEN_TEST)
    startup_test = read(STARTUP_TEST)
    boundary = read(BOUNDARY)
    workflow = read(WORKFLOW)

    for path in (WRAPPER, TEST, TOKEN_TEST, STARTUP_TEST, BOUNDARY):
        require_executable(path)

    for path, expected, owner in (
        (Q026_POLICY, EXPECTED_Q026_BLOB, "Q0.26 compatibility policy"),
        (Q028_POLICY, EXPECTED_Q028_BLOB, "Q0.28 startup policy"),
        (Q029_POLICY, EXPECTED_Q029_BLOB, "Q0.29 job policy"),
        (Q030_POLICY, EXPECTED_Q030_BLOB, "Q0.30/Q0.32 direct Bazel policy"),
        (WRAPPER, EXPECTED_WRAPPER_BLOB, "public Bazel wrapper"),
    ):
        require(git_blob_sha(path) == expected, f"{owner} drifted")

    for token in (
        'CANONICAL_CLIPPY_NEGATIVE_TARGET = "-//codex-rs/v8-poc:all"',
        "outside the canonical V8 exclusion",
    ):
        require_token(q026, token, "Q0.26 compatibility policy")

    for token in (
        "OUTPUT_BASE_PREFIX",
        "requires BAZEL_OUTPUT_BASE",
        "requires BAZEL_OUTPUT_USER_ROOT",
        "def _validate_exact_startup",
    ):
        require_token(q028, token, "Q0.28 startup policy")

    for token in (
        "prepare_bazelisk_environment",
        "_validate_bazelisk_inputs",
        "_validate_runner_identity",
        "_validate_paths",
        "_validate_job_binding",
    ):
        require_token(q029, token, "Q0.29 job policy")

    for token in (
        "Q0.30/Q0.32 direct Bazel CAS, token, and launch authority.",
        'BAZELISK_BARE_OVERRIDE = "BAZELISK"',
        'SETUP_BAZEL_TRANSPORT_TOKEN = "BAZELISK_GITHUB_TOKEN"',
        "resolver_env = dict(env)",
        "env.pop(SETUP_BAZEL_TRANSPORT_TOKEN, None)",
        "resolver_env.pop(SETUP_BAZEL_TRANSPORT_TOKEN, None)",
        "Bazelisk --print_env leaked the setup-only transport token",
        "setup-bazel transport token reached direct Bazel launch",
        "resolve_verified_bazel_command",
        'run(\n            [str(bazelisk), "--print_env"]',
        "Bazelisk executable changed during child resolution",
        "cached Bazel executable SHA-256 drifted",
        "content-addressed store",
        'env["PATH"] = child_path',
        "validate_keyless_windows_gnullvm_command",
        "_validate_q028(command[1:], env)",
        "verified direct Bazel executable changed before launch",
    ):
        require_token(q030, token, "Q0.30/Q0.32 direct Bazel policy")

    for token in (
        "Q0.17-Q0.30 qualification ratchets",
        "from run_bazel_q030_direct_bazel import prepare_bazelisk_environment",
        "from run_bazel_q030_direct_bazel import resolve_verified_bazel_command",
        "command = resolve_verified_bazel_command(command, os.environ)",
        "validate_keyless_windows_gnullvm_command(command, os.environ)",
        'f"--output_base={output_base}"',
        "os.execvpe(command[0], command, os.environ)",
    ):
        require_token(wrapper, token, "public Bazel wrapper")

    for token in (
        "test_resolver_verifies_bazelisk_and_cached_bazel",
        "test_cached_bazel_is_rehashed_even_when_bazelisk_succeeds",
        "test_cached_bazel_must_use_content_addressed_path",
        "test_print_env_failure_fails_closed",
        "test_missing_or_duplicate_path_binding_fails_closed",
        "test_bare_bazelisk_override_fails_closed",
        "test_unverified_initial_argv0_fails_closed",
        "test_direct_bazel_is_rehashed_immediately_before_launch",
        "test_q026_canonical_clippy_negative_target_passes",
        "test_q026_arbitrary_clippy_negative_target_fails_closed",
    ):
        require_token(test, token, "Q0.30 regression")

    for token in (
        "test_transport_token_is_resolver_only",
        "test_print_env_transport_token_leak_fails_closed",
        "test_resolution_failure_still_scrubs_transport_token",
        "test_direct_launch_rejects_reintroduced_transport_token",
    ):
        require_token(token_test, token, "Q0.32 token regression")

    for token in (
        "test_canonical_startup_vector_passes",
        "test_missing_output_user_root_fails_closed",
        "test_missing_output_base_fails_closed",
        "test_output_base_drift_fails_closed",
    ):
        require_token(startup_test, token, "Q0.30 startup regression")

    for token in (
        "python3 .github/scripts/test_run_bazel_direct_bazel.py",
        "python3 .github/scripts/test_run_bazel_transport_token.py",
        "python3 scripts/verify-windows-gnullvm-direct-bazel.py",
    ):
        require_token(boundary, token, "qualification boundary fixture")
    require_token(
        workflow,
        "python3 scripts/verify-windows-gnullvm-direct-bazel.py",
        "qualification workflow",
    )

    require(
        BAZELVERSION.read_bytes() == b"9.0.0\n",
        ".bazelversion bytes drifted",
    )
    require(
        git_blob_sha(BAZELVERSION) == EXPECTED_BAZELVERSION_BLOB,
        ".bazelversion Git blob drifted",
    )

    print("PASS_WINDOWS_GNULLVM_Q0_32_TRANSPORT_TOKEN_SOURCE")


if __name__ == "__main__":
    main()
