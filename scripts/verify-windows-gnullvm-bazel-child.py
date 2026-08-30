#!/usr/bin/env python3

from __future__ import annotations

import hashlib
import stat
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SCRIPTS = ROOT / ".github" / "scripts"
WRAPPER = SCRIPTS / "run_bazel_with_buildbuddy.py"
Q029_CONTRACT = SCRIPTS / "run_bazel_q029_execution_context.py"
Q030_CONTRACT = SCRIPTS / "run_bazel_q030_bazel_child.py"
Q030_TEST = SCRIPTS / "test_run_bazel_q030_bazel_child.py"
FIXTURE = SCRIPTS / "test_run_bazel_qualification_boundary.sh"
BOUNDARY = ROOT / ".github" / "workflows" / "windows-gnullvm-qualification-boundary.yml"
SETUP_BAZEL = ROOT / ".github" / "actions" / "setup-bazel-ci" / "action.yml"

EXPECTED_Q029_CONTRACT_BLOB = "282cc29e5615b94a616991e8acfd844def7b7031"
EXPECTED_SETUP_BAZEL_BLOB = "ac4f5aa97c7556f6049bd1d0a33220759d9d13d1"
SETUP_BAZEL_ACTION_COMMIT = "c5acdfb288317d0b5c0bbd7a396a3dc868bb0f86"
SETUP_BAZEL_CONFIG_JS_BLOB = "92bb7cd0077d8958b1bbca368a25169971d7a8d3"
BAZELISK_SOURCE_COMMIT = "1e6aaf11d51e83ec8d18e66b461f49d4b7877321"
BAZELISK_CORE_GO_BLOB = "15b131a22fc28377d3cc3d70ac602123d1530c08"


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


def require(text: str, expected: str, owner: str) -> None:
    if expected not in text:
        fail(f"{owner} lacks Q0.30 contract text: {expected}")


def require_order(text: str, before: str, after: str, owner: str) -> None:
    require(text, before, owner)
    require(text, after, owner)
    if text.index(before) >= text.index(after):
        fail(f"{owner} must place {before!r} before {after!r}")


def main() -> None:
    if git_blob_sha(Q029_CONTRACT) != EXPECTED_Q029_CONTRACT_BLOB:
        fail("selected Q0.29 execution-context contract drifted")
    if git_blob_sha(SETUP_BAZEL) != EXPECTED_SETUP_BAZEL_BLOB:
        fail("setup-bazel-ci action drifted from the reviewed input")

    wrapper = read(WRAPPER)
    q029 = read(Q029_CONTRACT)
    q030 = read(Q030_CONTRACT)
    test = read(Q030_TEST)
    fixture = read(FIXTURE)
    boundary = read(BOUNDARY)
    setup_bazel = read(SETUP_BAZEL)

    for path in (WRAPPER, Q030_TEST, FIXTURE):
        if not path.stat().st_mode & stat.S_IXUSR:
            fail(
                "required Q0.30 launcher lost executable mode: "
                f"{path.relative_to(ROOT)}"
            )

    require(q029, '"BAZELISK_GITHUB_TOKEN"', "Q0.29 compatibility contract")
    for expected in (
        'BAZELISK_TRANSPORT_TOKEN_ENV = "BAZELISK_GITHUB_TOKEN"',
        SETUP_BAZEL_ACTION_COMMIT,
        SETUP_BAZEL_CONFIG_JS_BLOB,
        BAZELISK_SOURCE_COMMIT,
        BAZELISK_CORE_GO_BLOB,
        "token = env.pop(BAZELISK_TRANSPORT_TOKEN_ENV, missing)",
        "_prepare_q029(env)",
        "env[BAZELISK_TRANSPORT_TOKEN_ENV] = token",
        '[str(bazelisk), "--print_env"]',
        "Bazelisk --print_env must emit exactly one PATH binding",
        "cached Bazel executable SHA-256 drifted",
        "outside the reviewed Bazelisk CAS",
        'env["PATH"] = child_path',
        "env.pop(BAZELISK_TRANSPORT_TOKEN_ENV, None)",
        "Bazelisk transport token survived into the Bazel launch",
        "def validate_verified_bazel_prelaunch(",
        "verified Bazel child PATH does not lead",
    ):
        require(q030, expected, "Q0.30 Bazel child contract")

    for expected in (
        "prepare_bazelisk_environment as _prepare_q029_compatibility_base",
        "from run_bazel_q030_bazel_child import prepare_bazelisk_environment",
        "from run_bazel_q030_bazel_child import resolve_verified_bazel_command",
        "from run_bazel_q030_bazel_child import validate_verified_bazel_prelaunch",
        "assert _prepare_q029_compatibility_base is not None",
        'if os.name == "nt":',
        "command = resolve_verified_bazel_command(command, env)",
        "validate_verified_bazel_prelaunch(command, env)",
    ):
        require(wrapper, expected, "BuildBuddy wrapper")
    require_order(
        wrapper,
        "command = bind_verified_bazelisk(command, env)",
        "validate_keyless_windows_gnullvm_execution_context(command, env)",
        "BuildBuddy wrapper",
    )
    require_order(
        wrapper,
        "validate_keyless_windows_gnullvm_execution_context(command, env)",
        "command = resolve_verified_bazel_command(command, env)",
        "BuildBuddy wrapper",
    )
    require_order(
        wrapper,
        "command = resolve_verified_bazel_command(command, env)",
        "validate_verified_bazel_prelaunch(command, env)",
        "BuildBuddy wrapper",
    )

    for expected in (
        "test_setup_bazel_transport_token_is_preserved",
        "test_missing_transport_token_remains_absent",
        "test_transport_token_is_restored_when_q029_rejects",
        "test_nontransport_bazelisk_override_still_fails_closed",
        "test_cached_bazel_is_rehashed_and_launched_directly",
        "test_cached_bazel_digest_drift_fails_closed",
        "test_cached_bazel_outside_cas_fails_closed",
        "test_bazelisk_print_env_failure_fails_closed",
        "test_duplicate_print_env_path_fails_closed",
        "test_prelaunch_digest_replacement_fails_closed",
        "test_prelaunch_path_mismatch_fails_closed",
    ):
        require(test, expected, "Q0.30 regression suite")

    for expected in (
        "python3 .github/scripts/test_run_bazel_q030_bazel_child.py",
        "python3 scripts/verify-windows-gnullvm-bazel-child.py",
    ):
        require(fixture, expected, "qualification boundary fixture")
    for expected in (
        "python3 scripts/verify-windows-gnullvm-bazel-child.py",
        '"bazel_child_source_contract":',
        '"setup_bazel_transport_token_authority": False',
        '"cached_bazel_digest_source_bound": True',
        '"windows_cached_bazel_digest_executed": False',
    ):
        require(boundary, expected, "qualification workflow")

    setup_step = setup_bazel.partition("    - name: Set up Bazel\n")[2]
    if not setup_step:
        fail("setup-bazel-ci action lacks the pinned setup-bazel step")
    setup_step = setup_step.partition("\n    - name:")[0]
    require(
        setup_step,
        f"bazel-contrib/setup-bazel@{SETUP_BAZEL_ACTION_COMMIT}",
        "setup-bazel step",
    )
    require(setup_step, "bazelisk-version: 1.28.1", "setup-bazel step")
    if "token:" in setup_step:
        fail("setup-bazel step must retain the upstream github.token default")

    print("PASS_WINDOWS_GNULLVM_Q0_30_BAZEL_CHILD_SOURCE")


if __name__ == "__main__":
    main()
