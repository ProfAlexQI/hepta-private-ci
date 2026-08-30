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
STARTUP_TEST = SCRIPTS / "test_run_bazel_startup_contract.py"
BOUNDARY = SCRIPTS / "test_run_bazel_qualification_boundary.sh"
WORKFLOW = (
    ROOT / ".github" / "workflows" / "windows-gnullvm-qualification-boundary.yml"
)
BAZELVERSION = ROOT / ".bazelversion"

EXPECTED_Q026_BLOB = "e0729bd796b342568c624d15faf3638a1372d01d"
EXPECTED_Q028_BLOB = "86225acd9158132df8cd5ae9dc6720205a7c47a6"
EXPECTED_Q029_BLOB = "2d57d5e222b87a89b2f8b1c93c476f450b03e646"
EXPECTED_Q030_BLOB = "1614f53c9572cdda3b1d7cf227f3a730e27b2adb"
EXPECTED_WRAPPER_BLOB = "233d98f151b897caa42f4d762d119645cb13e641"
EXPECTED_TEST_BLOB = "f03bb5d31ce5bca1c82f9a6349b506387e43b8e7"
EXPECTED_WORKFLOW_BLOB = "854acc3f53c86ada494f83e7212b1cc55448cc86"
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


def reject_token(text: str, token: str, owner: str) -> None:
    require(token not in text, f"{owner} contains forbidden token: {token}")


def require_order(text: str, before: str, after: str, owner: str) -> None:
    require_token(text, before, owner)
    require_token(text, after, owner)
    require(
        text.index(before) < text.index(after),
        f"{owner} must place {before!r} before {after!r}",
    )


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
    startup_test = read(STARTUP_TEST)
    boundary = read(BOUNDARY)
    workflow = read(WORKFLOW)

    for path in (WRAPPER, TEST, STARTUP_TEST, BOUNDARY):
        require_executable(path)

    for path, expected, owner in (
        (Q026_POLICY, EXPECTED_Q026_BLOB, "Q0.26 compatibility policy"),
        (Q028_POLICY, EXPECTED_Q028_BLOB, "Q0.28 startup policy"),
        (Q029_POLICY, EXPECTED_Q029_BLOB, "Q0.29 job policy"),
        (Q030_POLICY, EXPECTED_Q030_BLOB, "Q0.32 direct Bazel policy"),
        (WRAPPER, EXPECTED_WRAPPER_BLOB, "public Bazel wrapper"),
        (TEST, EXPECTED_TEST_BLOB, "Q0.32 direct Bazel regression"),
        (WORKFLOW, EXPECTED_WORKFLOW_BLOB, "qualification workflow"),
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
        "Q0.32 direct Bazel CAS, transport-token, and pre-launch authority",
        'SETUP_BAZEL_TRANSPORT_TOKEN = "BAZELISK_GITHUB_TOKEN"',
        "def _matching_env_names(",
        "def consume_setup_bazel_transport_token(",
        "def _require_transport_token_absent(",
        "consume_setup_bazel_transport_token(env)",
        "Q0.32 Bazelisk preparation",
        "resolve_verified_bazel_command",
        'run(\n        [str(bazelisk), "--print_env"]',
        "Q0.32 Bazelisk resolution",
        "retained the setup-only transport token",
        "Bazelisk executable changed during child resolution",
        "cached Bazel executable SHA-256 drifted",
        "content-addressed store",
        'env["PATH"] = child_path',
        'env.pop("BAZEL_REAL", None)',
        'env.pop("BAZELISK", None)',
        "def _validate_child_path(",
        "final PATH head is not the verified cached Bazel directory",
        "validate_keyless_windows_gnullvm_command",
        "Q0.32 final direct Bazel launch",
        "_validate_child_path(real_bazel, env)",
        "_validate_q028(command[1:], env)",
        "verified direct Bazel executable changed before launch",
    ):
        require_token(q030, token, "Q0.32 direct Bazel policy")

    require_order(
        q030,
        "consume_setup_bazel_transport_token(env)",
        "_prepare_q029(env)",
        "Q0.32 direct Bazel policy",
    )
    require_order(
        q030,
        "_validate_child_path(real_bazel, env)",
        "_validate_q028(command[1:], env)",
        "Q0.32 direct Bazel policy",
    )
    reject_token(
        q030,
        "stderr={result.stderr.strip()!r}",
        "Q0.32 direct Bazel policy",
    )
    reject_token(
        q030,
        "result.stdout.strip()",
        "Q0.32 direct Bazel policy",
    )

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
        "test_prepare_consumes_setup_bazel_transport_token",
        "test_resolver_rejects_unconsumed_transport_token",
        "test_resolver_verifies_bazelisk_and_cached_bazel",
        "test_cached_bazel_is_rehashed_even_when_bazelisk_succeeds",
        "test_cached_bazel_must_use_content_addressed_path",
        "test_print_env_failure_fails_closed",
        "test_print_env_failure_does_not_echo_transport_output",
        "test_print_env_transport_token_fails_closed",
        "test_missing_or_duplicate_path_binding_fails_closed",
        "test_bare_bazelisk_override_fails_closed",
        "test_unverified_initial_argv0_fails_closed",
        "test_direct_bazel_is_rehashed_immediately_before_launch",
        "test_final_command_rejects_transport_token",
        "test_final_path_head_drift_fails_closed",
        "test_q026_canonical_clippy_negative_target_passes",
        "test_q026_arbitrary_clippy_negative_target_fails_closed",
    ):
        require_token(test, token, "Q0.32 regression")

    for token in (
        "test_canonical_startup_vector_passes",
        "test_missing_output_user_root_fails_closed",
        "test_missing_output_base_fails_closed",
        "test_output_base_drift_fails_closed",
    ):
        require_token(startup_test, token, "Q0.30 startup regression")

    for token in (
        "python3 .github/scripts/test_run_bazel_direct_bazel.py",
        "python3 scripts/verify-windows-gnullvm-direct-bazel.py",
    ):
        require_token(boundary, token, "qualification boundary fixture")

    for token in (
        "python3 scripts/verify-windows-gnullvm-direct-bazel.py",
        '"setup_bazel_transport_token_consumed_before_resolution": True',
        '"setup_bazel_transport_token_reaches_bazelisk": False',
        '"setup_bazel_transport_token_reaches_direct_bazel": False',
        '"bazelisk_failure_output_echoed": False',
        '"final_child_path_head_source_bound": True',
        '"cached_bazel_executed_on_this_linux_source_job": False',
    ):
        require_token(workflow, token, "qualification workflow")

    require(
        BAZELVERSION.read_bytes() == b"9.0.0\n",
        ".bazelversion bytes drifted",
    )
    require(
        git_blob_sha(BAZELVERSION) == EXPECTED_BAZELVERSION_BLOB,
        ".bazelversion Git blob drifted",
    )

    print("PASS_WINDOWS_GNULLVM_Q0_30_DIRECT_BAZEL_SOURCE")
    print("PASS_WINDOWS_GNULLVM_Q0_32_TRANSPORT_AND_PATH_SOURCE")


if __name__ == "__main__":
    main()
