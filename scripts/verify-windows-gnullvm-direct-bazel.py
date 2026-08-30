#!/usr/bin/env python3

from __future__ import annotations

import hashlib
import stat
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCRIPTS = ROOT / ".github" / "scripts"
Q026_POLICY = SCRIPTS / "run_bazel_q022_negative_targets.py"
Q028_POLICY = SCRIPTS / "run_bazel_q028_startup_contract.py"
Q029_POLICY = SCRIPTS / "run_bazel_q029_execution_context.py"
Q031_POLICY = SCRIPTS / "run_bazel_q031_direct_bazel.py"
WRAPPER = SCRIPTS / "run_bazel_with_buildbuddy.py"
TEST = SCRIPTS / "test_run_bazel_direct_bazel.py"
STARTUP_TEST = SCRIPTS / "test_run_bazel_startup_contract.py"
EXECUTION_TEST = SCRIPTS / "test_run_bazel_execution_context.py"
BOUNDARY = SCRIPTS / "test_run_bazel_qualification_boundary.sh"
WORKFLOW = ROOT / ".github" / "workflows" / "windows-gnullvm-qualification-boundary.yml"
BAZELVERSION = ROOT / ".bazelversion"
PLAN = ROOT / "plans" / "hepta-intelligence" / "HEPTA_INTELLIGENCE_Q0_31_DIRECT_BAZEL_PLAN_2026-08-30.md"
STATUS = ROOT / "plans" / "hepta-intelligence" / "HEPTA_INTELLIGENCE_Q0_31_STATUS.json"

EXPECTED_Q026_BLOB = "e0729bd796b342568c624d15faf3638a1372d01d"
EXPECTED_Q028_BLOB = "86225acd9158132df8cd5ae9dc6720205a7c47a6"
EXPECTED_Q029_BLOB = "282cc29e5615b94a616991e8acfd844def7b7031"
EXPECTED_Q031_BLOB = "91b6d76885650cf72df02c8c44b895374e4c8606"
EXPECTED_WRAPPER_BLOB = "38575f96e16bb7c3d26c8de401717bb9b69fd4d7"
EXPECTED_TEST_BLOB = "5a06121763ae6ffc38d6bb138fdbe9cfa1ea4a74"
EXPECTED_STARTUP_TEST_BLOB = "81779fc628e7a1c2cf310b29ee22a84af254ab79"
EXPECTED_EXECUTION_TEST_BLOB = "5df0ea1c54f880d6d2da11168702d5e40d42839f"
EXPECTED_BAZELVERSION_BLOB = "f7ee06693c17a06e2a0f51ef7eb2a61866e77b8e"


def fail(message: str) -> None:
    raise SystemExit(message)


def read(path: Path) -> str:
    if not path.is_file():
        fail(f"missing Q0.31 contract path: {path.relative_to(ROOT)}")
    return path.read_text(encoding="utf-8")


def git_blob_sha(path: Path) -> str:
    content = path.read_bytes()
    framed = f"blob {len(content)}\0".encode("ascii") + content
    return hashlib.sha1(framed, usedforsecurity=False).hexdigest()


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def require_token(text: str, token: str, owner: str) -> None:
    require(token in text, f"{owner} lacks Q0.31 token: {token}")


def require_executable(path: Path) -> None:
    require(
        bool(path.stat().st_mode & stat.S_IXUSR),
        f"required executable lost mode: {path.relative_to(ROOT)}",
    )


def main() -> None:
    q026 = read(Q026_POLICY)
    q028 = read(Q028_POLICY)
    q029 = read(Q029_POLICY)
    q031 = read(Q031_POLICY)
    wrapper = read(WRAPPER)
    test = read(TEST)
    startup_test = read(STARTUP_TEST)
    execution_test = read(EXECUTION_TEST)
    boundary = read(BOUNDARY)
    workflow = read(WORKFLOW)
    plan = read(PLAN)
    status = read(STATUS)

    for path in (
        WRAPPER,
        TEST,
        STARTUP_TEST,
        EXECUTION_TEST,
        BOUNDARY,
        PLAN,
    ):
        if path.suffix != ".md":
            require_executable(path)

    for path, expected, owner in (
        (Q026_POLICY, EXPECTED_Q026_BLOB, "Q0.26 compatibility policy"),
        (Q028_POLICY, EXPECTED_Q028_BLOB, "Q0.28/Q0.31 startup policy"),
        (Q029_POLICY, EXPECTED_Q029_BLOB, "canonical Q0.29 execution-context policy"),
        (Q031_POLICY, EXPECTED_Q031_BLOB, "Q0.31 direct Bazel policy"),
        (WRAPPER, EXPECTED_WRAPPER_BLOB, "public Bazel wrapper"),
        (TEST, EXPECTED_TEST_BLOB, "Q0.31 direct Bazel regression"),
        (STARTUP_TEST, EXPECTED_STARTUP_TEST_BLOB, "Q0.31 startup regression"),
        (EXECUTION_TEST, EXPECTED_EXECUTION_TEST_BLOB, "Q0.29 execution regression"),
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
        require_token(q028, token, "Q0.28/Q0.31 startup policy")

    for token in (
        "prepare_bazelisk_environment",
        "_validate_bazelisk_inputs",
        "_validate_runner_and_job",
        "_validate_paths",
        "validate_keyless_windows_gnullvm_execution_context",
        "if path.drive and not path.root and len(path.parts) == 1",
    ):
        require_token(q029, token, "canonical Q0.29 execution-context policy")

    for token in (
        "Q0.31 direct Bazel CAS and pre-launch authority contract",
        'BAZELISK_BARE_OVERRIDE = "BAZELISK"',
        "from run_bazel_q029_execution_context import (",
        "resolve_verified_bazel_command",
        '[str(bazelisk), "--print_env"]',
        "Bazelisk executable changed during child resolution",
        "cached Bazel executable SHA-256 drifted",
        "content-addressed store",
        'env["PATH"] = child_path',
        "validate_keyless_windows_gnullvm_command",
        "_validate_q028(command[1:], env)",
        "_validate_runner_and_job(command_name, options, env)",
        "_validate_paths(command_name, options, env, job)",
        "verified direct Bazel executable changed before launch",
    ):
        require_token(q031, token, "Q0.31 direct Bazel policy")

    for token in (
        "Q0.17-Q0.31 qualification ratchets",
        "from run_bazel_q031_direct_bazel import prepare_bazelisk_environment",
        "from run_bazel_q031_direct_bazel import resolve_verified_bazel_command",
        "from run_bazel_q031_direct_bazel import validate_keyless_windows_gnullvm_command",
        "return resolve_verified_bazel_command(command, env)",
        "validate_keyless_windows_gnullvm_command(command, env)",
        'f"--output_base={output_base}"',
        "command = executable_command(*sys.argv[1:])",
        "os.execvpe(command[0], command, os.environ)",
    ):
        require_token(wrapper, token, "public Bazel wrapper")

    required_tests = (
        "test_resolver_verifies_bazelisk_and_cached_bazel",
        "test_cached_bazel_is_rehashed_even_when_bazelisk_succeeds",
        "test_cached_bazel_must_use_content_addressed_path",
        "test_print_env_failure_fails_closed",
        "test_missing_or_duplicate_path_binding_fails_closed",
        "test_bare_bazelisk_override_fails_closed",
        "test_unverified_initial_argv0_fails_closed",
        "test_bazelisk_is_rehashed_after_child_resolution",
        "test_direct_bazel_is_rehashed_immediately_before_launch",
        "test_q026_canonical_clippy_negative_target_passes",
        "test_q026_arbitrary_clippy_negative_target_fails_closed",
    )
    for token in required_tests:
        require_token(test, token, "Q0.31 direct Bazel regression")

    for token in (
        "test_canonical_startup_vector_passes",
        "test_missing_output_user_root_fails_closed",
        "test_missing_output_base_fails_closed",
        "test_output_base_drift_fails_closed",
    ):
        require_token(startup_test, token, "Q0.31 startup regression")

    for token in (
        "test_wrapper_launch_order_binds_context_after_q027",
        "test_all_three_canonical_jobs_pass",
        "test_bare_dev_drive_root_is_canonicalized",
        "test_bazelisk_digest_drift_fails_closed",
    ):
        require_token(execution_test, token, "canonical Q0.29 execution regression")

    for token in (
        "python3 .github/scripts/test_run_bazel_execution_context.py",
        "python3 .github/scripts/test_run_bazel_direct_bazel.py",
        "python3 scripts/verify-windows-gnullvm-execution-context.py",
        "python3 scripts/verify-windows-gnullvm-direct-bazel.py",
    ):
        require_token(boundary, token, "qualification boundary fixture")

    for token in (
        "python3 scripts/verify-windows-gnullvm-execution-context.py",
        "python3 scripts/verify-windows-gnullvm-direct-bazel.py",
        '"direct_bazel_source_contract":',
        '"explicit_output_base_source_bound": True',
        '"direct_cached_bazel_rehash_source_bound": True',
        '"windows_executable_digest_executed": False',
        '"a0_candidate_qualified": False',
    ):
        require_token(workflow, token, "qualification workflow")

    require_token(plan, "Q0.31", "Q0.31 plan")
    for token in (
        '"qualified": false',
        '"full_repository_merge_green": false',
        '"runtime_authority": false',
        '"production_authority": false',
        '"operator_acceptance": false',
        '"promotion": false',
        '"release_authority": false',
        '"callers_ratchet": false',
    ):
        require_token(status, token, "Q0.31 status")

    require(BAZELVERSION.read_bytes() == b"9.0.0\n", ".bazelversion bytes drifted")
    require(
        git_blob_sha(BAZELVERSION) == EXPECTED_BAZELVERSION_BLOB,
        ".bazelversion Git blob drifted",
    )

    print("PASS_WINDOWS_GNULLVM_Q0_31_DIRECT_BAZEL_SOURCE")


if __name__ == "__main__":
    main()
