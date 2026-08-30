#!/usr/bin/env python3

from __future__ import annotations

import hashlib
import stat
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SCRIPTS = ROOT / ".github" / "scripts"
WRAPPER = SCRIPTS / "run_bazel_with_buildbuddy.py"
Q029_CONTRACT = SCRIPTS / "run_bazel_q029_execution_context.py"
Q030_CONTRACT = SCRIPTS / "run_bazel_q030_cached_bazel.py"
Q030_TEST = SCRIPTS / "test_run_bazel_cached_bazel.py"
FIXTURE = SCRIPTS / "test_run_bazel_qualification_boundary.sh"
BOUNDARY = ROOT / ".github" / "workflows" / "windows-gnullvm-qualification-boundary.yml"
REPO_CHECKS = ROOT / ".github" / "workflows" / "repo-checks.yml"
SETUP_CI = ROOT / ".github" / "actions" / "setup-ci" / "action.yml"
SETUP_BAZEL = ROOT / ".github" / "actions" / "setup-bazel-ci" / "action.yml"

EXPECTED_BLOBS = {
    SCRIPTS / "run_bazel_q027_lane_semantics.py": (
        "a507da0da4ac370a73d79eb305b227f0a080170a"
    ),
    SCRIPTS / "run_bazel_q028_startup_contract.py": (
        "6711a2d5cdb63466f1895d539391af44e48b793f"
    ),
    Q029_CONTRACT: "282cc29e5615b94a616991e8acfd844def7b7031",
    SCRIPTS / "test_run_bazel_execution_context.py": (
        "5df0ea1c54f880d6d2da11168702d5e40d42839f"
    ),
    ROOT / "scripts" / "verify-windows-gnullvm-execution-context.py": (
        "cfbe2a2bc460b6831cf4f1d3a5cd9a5e1b973c1c"
    ),
    SCRIPTS / "run_bazel_with_buildbuddy_base.py": (
        "913708d5651678c1623faac2b18656c2b86300bb"
    ),
    SCRIPTS / "run-bazel-ci-impl.sh": (
        "2fe7cf37a0fddc1bb2f42f3e8a1e3b5a9e30f96b"
    ),
    SETUP_CI: "8abd2dbd5f09585734f8213011a9ed540a2ee88e",
    SETUP_BAZEL: "ac4f5aa97c7556f6049bd1d0a33220759d9d13d1",
}
EXPECTED_BAZELISK_SHA256 = (
    "b9d65a1f7c2d7af885a96a4fd5aa36b40fb41816d30944390569eef908bdc954"
)
EXPECTED_BAZEL_SHA256 = (
    "463faee497df2913854d80776784137cb47f42960b4ef4e4f85068c8da4849a8"
)


def fail(message: str) -> None:
    raise SystemExit(message)


def read(path: Path) -> str:
    if not path.is_file():
        fail(f"missing Q0.30 source path: {path.relative_to(ROOT)}")
    return path.read_text(encoding="utf-8")


def git_blob_sha(path: Path) -> str:
    content = path.read_bytes()
    framed = f"blob {len(content)}\0".encode("ascii") + content
    return hashlib.sha1(framed, usedforsecurity=False).hexdigest()


def require(text: str, token: str, owner: str) -> None:
    if token not in text:
        fail(f"{owner} lacks Q0.30 contract text: {token}")


def reject(text: str, token: str, owner: str) -> None:
    if token in text:
        fail(f"{owner} contains forbidden Q0.30 contract text: {token}")


def require_order(text: str, before: str, after: str, owner: str) -> None:
    require(text, before, owner)
    require(text, after, owner)
    if text.index(before) >= text.index(after):
        fail(f"{owner} must place {before!r} before {after!r}")


def require_executable(path: Path) -> None:
    if not path.stat().st_mode & stat.S_IXUSR:
        fail(f"required Q0.30 launcher lost executable mode: {path.relative_to(ROOT)}")


def main() -> None:
    for path, expected in EXPECTED_BLOBS.items():
        read(path)
        observed = git_blob_sha(path)
        if observed != expected:
            fail(
                "immutable Q0.29 compatibility input drifted: "
                f"{path.relative_to(ROOT)} expected {expected}, observed {observed}"
            )

    wrapper = read(WRAPPER)
    q029 = read(Q029_CONTRACT)
    q030 = read(Q030_CONTRACT)
    test = read(Q030_TEST)
    fixture = read(FIXTURE)
    boundary = read(BOUNDARY)
    repo_checks = read(REPO_CHECKS)
    setup_ci = read(SETUP_CI)
    setup_bazel = read(SETUP_BAZEL)

    for path in (WRAPPER, Q030_TEST, FIXTURE):
        require_executable(path)

    for token in (
        "Q0.17-Q0.29 qualification ratchets and Q0.30 closure",
        "from run_bazel_q029_execution_context import bind_verified_bazelisk",
        "from run_bazel_q030_cached_bazel import bind_output_base_startup",
        "from run_bazel_q030_cached_bazel import clear_setup_bazel_transport_token",
        "from run_bazel_q030_cached_bazel import resolve_verified_cached_bazel",
        "validate_keyless_windows_gnullvm_cached_bazel_context",
        'if env.get("GITHUB_ACTIONS") == "true":',
        "clear_setup_bazel_transport_token(env)",
        "prepare_bazelisk_environment(env)",
        "command = bind_verified_bazelisk(command, env)",
        "validate_keyless_windows_gnullvm_execution_context(command, env)",
        "command = bind_output_base_startup(command, env)",
        "command = resolve_verified_cached_bazel(command, env)",
        "validate_keyless_windows_gnullvm_cached_bazel_context(command, env)",
    ):
        require(wrapper, token, "BuildBuddy wrapper")

    require_order(
        wrapper,
        "clear_setup_bazel_transport_token(env)",
        "prepare_bazelisk_environment(env)",
        "BuildBuddy wrapper",
    )
    require_order(
        wrapper,
        "prepare_bazelisk_environment(env)",
        "command = bind_verified_bazelisk(command, env)",
        "BuildBuddy wrapper",
    )
    require_order(
        wrapper,
        "command = bind_verified_bazelisk(command, env)",
        "validate_keyless_windows_gnullvm_execution_context(command, env)",
        "BuildBuddy wrapper",
    )
    require_order(
        wrapper,
        "validate_keyless_windows_gnullvm_execution_context(command, env)",
        "command = bind_output_base_startup(command, env)",
        "BuildBuddy wrapper",
    )
    require_order(
        wrapper,
        "command = bind_output_base_startup(command, env)",
        "command = resolve_verified_cached_bazel(command, env)",
        "BuildBuddy wrapper",
    )
    require_order(
        wrapper,
        "command = resolve_verified_cached_bazel(command, env)",
        "validate_keyless_windows_gnullvm_cached_bazel_context(command, env)",
        "BuildBuddy wrapper",
    )

    require(
        q029,
        '"BAZELISK_GITHUB_TOKEN"',
        "Q0.29 execution-context policy",
    )
    require(
        q029,
        EXPECTED_BAZELISK_SHA256,
        "Q0.29 execution-context policy",
    )
    require(q029, EXPECTED_BAZEL_SHA256, "Q0.29 execution-context policy")

    for token in (
        'SETUP_BAZEL_TRANSPORT_TOKEN = "BAZELISK_GITHUB_TOKEN"',
        'OUTPUT_BASE_PREFIX = "--output_base="',
        EXPECTED_BAZEL_SHA256,
        "def clear_setup_bazel_transport_token(",
        "env.pop(SETUP_BAZEL_TRANSPORT_TOKEN, None)",
        "def bind_output_base_startup(",
        "Q0.30 requires output_base to be absent before final startup binding",
        "def _parse_print_env(",
        "must emit exactly one PATH binding",
        "retained the setup-only transport token",
        "def _validate_bazel_cas_path(",
        '"downloads"',
        '"sha256"',
        '"bin"',
        '"bazel.exe"',
        "def resolve_verified_cached_bazel(",
        '[str(bazelisk), "--print_env"]',
        "capture_output=True",
        "timeout=180",
        "cached Bazel executable SHA-256 drifted",
        'env["PATH"] = child_path',
        'env.pop("BAZEL_REAL", None)',
        'env.pop("BAZELISK", None)',
        "def _validate_exact_startup(",
        "Q0.30 final startup arguments are not exact",
        "def _validate_output_base(",
        '_require_path(env, "BAZEL_OUTPUT_BASE", build_root / "o")',
        "def validate_keyless_windows_gnullvm_cached_bazel_context(",
        "_validate_runner_and_job(command_name, options, env)",
        "_validate_paths(command_name, options, env, job)",
        "cached Bazel executable SHA-256 drifted before launch",
        "_validate_child_path(executable, env)",
    ):
        require(q030, token, "Q0.30 cached-Bazel policy")
    reject(q030, "result.stderr.strip()", "Q0.30 cached-Bazel policy")
    reject(q030, "result.stdout.strip()", "Q0.30 cached-Bazel policy")

    required_tests = (
        "test_setup_bazel_transport_token_is_consumed",
        "test_output_base_is_appended_after_q029_startup",
        "test_preexisting_output_base_fails_closed",
        "test_cached_bazel_is_resolved_rehashed_and_bypasses_bazelisk",
        "test_cached_bazel_tamper_fails_even_after_bazelisk_success",
        "test_cached_bazel_outside_cas_layout_fails_closed",
        "test_duplicate_print_env_path_fails_closed",
        "test_print_env_transport_token_fails_closed",
        "test_nonzero_print_env_does_not_echo_transport_output",
        "test_final_cached_bazel_context_passes",
        "test_final_output_base_drift_fails_closed",
        "test_final_cached_bazel_is_rehashed_immediately_before_launch",
        "test_wrapper_orders_q030_after_q029_and_before_return",
    )
    for test_name in required_tests:
        require(test, test_name, "Q0.30 regression suite")

    for token in (
        "python3 .github/scripts/test_run_bazel_execution_context.py",
        "python3 .github/scripts/test_run_bazel_cached_bazel.py",
        "python3 scripts/verify-windows-gnullvm-execution-context.py",
        "python3 scripts/verify-windows-gnullvm-cached-bazel.py",
    ):
        require(fixture, token, "qualification fixture")

    for token in (
        "python3 scripts/verify-windows-gnullvm-execution-context.py",
        "python3 scripts/verify-windows-gnullvm-cached-bazel.py",
        '"cached_bazel_source_contract":',
        '"explicit_output_base_source_bound": True',
        '"setup_bazel_transport_token_reaches_final_launch": False',
        '"windows_cached_bazel_digest_executed": False',
        '"a0_candidate_qualified": False',
    ):
        require(boundary, token, "qualification workflow")

    require(
        repo_checks,
        "python3 -m unittest discover -s .github/scripts "
        "-p 'test_run_bazel*.py'",
        "ordinary repo-checks",
    )
    require(setup_ci, 'bazel_output_base="$CI_BUILD_ROOT/o"', "setup-ci action")
    require(
        setup_bazel,
        "bazel-contrib/setup-bazel@c5acdfb288317d0b5c0bbd7a396a3dc868bb0f86",
        "setup-bazel-ci action",
    )
    require(setup_bazel, "bazelisk-version: 1.28.1", "setup-bazel-ci action")
    require(
        setup_bazel,
        "output-base: ${{ steps.setup_ci.outputs.bazel-output-base }}",
        "setup-bazel-ci action",
    )

    print("PASS_WINDOWS_GNULLVM_Q0_30_CACHED_BAZEL_SOURCE")


if __name__ == "__main__":
    main()
