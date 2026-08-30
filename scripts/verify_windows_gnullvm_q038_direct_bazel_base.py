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
Q034_POLICY = SCRIPTS / "run_bazel_q034_execution_manifest.py"
WRAPPER = SCRIPTS / "run_bazel_with_buildbuddy.py"
DIRECT_TEST = SCRIPTS / "test_run_bazel_direct_bazel.py"
SETUP_TOKEN_TEST = SCRIPTS / "test_run_bazel_setup_token_boundary.py"
EXECUTION_TEST = SCRIPTS / "test_run_bazel_execution_manifest.py"
BOUNDARY = SCRIPTS / "test_run_bazel_qualification_boundary.sh"
QUALIFICATION_WORKFLOW = (
    ROOT / ".github" / "workflows" / "windows-gnullvm-qualification-boundary.yml"
)
BAZEL_WORKFLOW = ROOT / ".github" / "workflows" / "bazel.yml"
SETUP_BAZEL_ACTION = (
    ROOT / ".github" / "actions" / "setup-bazel-ci" / "action.yml"
)
CLIPPY_TARGETS = ROOT / "scripts" / "list-bazel-clippy-targets.sh"
RELEASE_TARGETS = ROOT / "scripts" / "list-bazel-release-targets.sh"
BAZELVERSION = ROOT / ".bazelversion"

EXPECTED_Q026_BLOB = "e0729bd796b342568c624d15faf3638a1372d01d"
EXPECTED_Q028_BLOB = "86225acd9158132df8cd5ae9dc6720205a7c47a6"
EXPECTED_Q029_BLOB = "2d57d5e222b87a89b2f8b1c93c476f450b03e646"
EXPECTED_Q030_BLOB = "1614f53c9572cdda3b1d7cf227f3a730e27b2adb"
EXPECTED_Q034_BLOB = "6e0f9b873b1bf3585781ef1c12e41f29eeb0403a"
EXPECTED_WRAPPER_BLOB = "2e7d3f1a2a27e2c310efaa1448be8447a36bfdbb"
EXPECTED_DIRECT_TEST_BLOB = "f03bb5d31ce5bca1c82f9a6349b506387e43b8e7"
EXPECTED_SETUP_TOKEN_TEST_BLOB = "5778dd884ef087362a99b665fbb1c60cf2dce5f0"
EXPECTED_EXECUTION_TEST_BLOB = "d6fb968f7bdc1721152f48e81bdd2415ff50f6f4"
EXPECTED_BOUNDARY_BLOB = "ef870de5dbb9dbcc560d930051cc9f1c464cea68"
EXPECTED_QUALIFICATION_WORKFLOW_BLOB = (
    "3ec965f7483deb177556b83a1ecef36702c3e680"
)
EXPECTED_BAZEL_WORKFLOW_BLOB = "55c470b88085fea874fca38573d49fd0c1d18cfe"
EXPECTED_SETUP_BAZEL_ACTION_BLOB = "890567be46f3fd78c11b89a20950bef2f7af4bf6"
EXPECTED_CLIPPY_TARGETS_BLOB = "a7f17d0aaffc280d711a57f64dfdaa10e8c12c58"
EXPECTED_RELEASE_TARGETS_BLOB = "154f0b3580f3ba3216b6e4b840a3a7364e24e007"
EXPECTED_BAZELVERSION_BLOB = "f7ee06693c17a06e2a0f51ef7eb2a61866e77b8e"


def fail(message: str) -> None:
    raise SystemExit(message)


def read(path: Path) -> str:
    if not path.is_file():
        fail(f"missing Q0.30-Q0.34 contract path: {path.relative_to(ROOT)}")
    return path.read_text(encoding="utf-8")


def git_blob_sha(path: Path) -> str:
    content = path.read_bytes()
    framed = f"blob {len(content)}\0".encode("ascii") + content
    return hashlib.sha1(framed, usedforsecurity=False).hexdigest()


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def require_token(text: str, token: str, owner: str) -> None:
    require(token in text, f"{owner} lacks Q0.30-Q0.34 token: {token}")


def reject_token(text: str, token: str, owner: str) -> None:
    require(token not in text, f"{owner} contains forbidden token: {token}")


def require_order(text: str, before: str, after: str, owner: str) -> None:
    require_token(text, before, owner)
    require_token(text, after, owner)
    require(
        text.index(before) < text.index(after),
        f"{owner} must place {before!r} before {after!r}",
    )


def require_blob(path: Path, expected: str, owner: str) -> None:
    observed = git_blob_sha(path)
    require(
        observed == expected,
        f"{owner} drifted: expected {expected}, observed {observed}",
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
    q034 = read(Q034_POLICY)
    wrapper = read(WRAPPER)
    direct_test = read(DIRECT_TEST)
    setup_token_test = read(SETUP_TOKEN_TEST)
    execution_test = read(EXECUTION_TEST)
    boundary = read(BOUNDARY)
    qualification_workflow = read(QUALIFICATION_WORKFLOW)
    bazel_workflow = read(BAZEL_WORKFLOW)
    setup_bazel_action = read(SETUP_BAZEL_ACTION)
    clippy_targets = read(CLIPPY_TARGETS)
    release_targets = read(RELEASE_TARGETS)

    for path in (
        WRAPPER,
        DIRECT_TEST,
        SETUP_TOKEN_TEST,
        EXECUTION_TEST,
        BOUNDARY,
        CLIPPY_TARGETS,
        RELEASE_TARGETS,
    ):
        require_executable(path)

    for path, expected, owner in (
        (Q026_POLICY, EXPECTED_Q026_BLOB, "Q0.26 compatibility policy"),
        (Q028_POLICY, EXPECTED_Q028_BLOB, "Q0.28 startup policy"),
        (Q029_POLICY, EXPECTED_Q029_BLOB, "Q0.29 job policy"),
        (Q030_POLICY, EXPECTED_Q030_BLOB, "Q0.32 direct Bazel policy"),
        (Q034_POLICY, EXPECTED_Q034_BLOB, "Q0.34 execution manifest"),
        (WRAPPER, EXPECTED_WRAPPER_BLOB, "public Bazel wrapper"),
        (DIRECT_TEST, EXPECTED_DIRECT_TEST_BLOB, "Q0.32 direct Bazel test"),
        (
            SETUP_TOKEN_TEST,
            EXPECTED_SETUP_TOKEN_TEST_BLOB,
            "Q0.33 setup-token test",
        ),
        (
            EXECUTION_TEST,
            EXPECTED_EXECUTION_TEST_BLOB,
            "Q0.34 execution-manifest test",
        ),
        (BOUNDARY, EXPECTED_BOUNDARY_BLOB, "qualification fixture"),
        (
            QUALIFICATION_WORKFLOW,
            EXPECTED_QUALIFICATION_WORKFLOW_BLOB,
            "qualification workflow",
        ),
        (BAZEL_WORKFLOW, EXPECTED_BAZEL_WORKFLOW_BLOB, "Bazel workflow"),
        (
            SETUP_BAZEL_ACTION,
            EXPECTED_SETUP_BAZEL_ACTION_BLOB,
            "setup-bazel-ci action",
        ),
        (
            CLIPPY_TARGETS,
            EXPECTED_CLIPPY_TARGETS_BLOB,
            "Clippy target generator",
        ),
        (
            RELEASE_TARGETS,
            EXPECTED_RELEASE_TARGETS_BLOB,
            "release target generator",
        ),
    ):
        require_blob(path, expected, owner)

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
        'TEST_JOB = "test-windows-shard"',
        'CLIPPY_JOB = "clippy"',
        'RELEASE_JOB = "verify-release-build"',
        "prepare_bazelisk_environment",
        "_validate_bazelisk_inputs",
        "_validate_runner_identity",
        "_validate_paths",
        "_validate_job_binding",
        "exact canonical release target payload",
    ):
        require_token(q029, token, "Q0.29 job policy")

    for token in (
        "Q0.32 direct Bazel CAS, transport-token, and pre-launch authority",
        'SETUP_BAZEL_TRANSPORT_TOKEN = "BAZELISK_GITHUB_TOKEN"',
        "consume_setup_bazel_transport_token(env)",
        "Q0.32 Bazelisk preparation",
        "resolve_verified_bazel_command",
        'run(\n        [str(bazelisk), "--print_env"]',
        "retained the setup-only transport token",
        "Bazelisk executable changed during child resolution",
        "cached Bazel executable SHA-256 drifted",
        "content-addressed store",
        'env["PATH"] = child_path',
        "final PATH head is not the verified cached Bazel directory",
        "validate_keyless_windows_gnullvm_command",
        "verified direct Bazel executable changed before launch",
    ):
        require_token(q030, token, "Q0.32 direct Bazel policy")
    reject_token(q030, "result.stderr", "Q0.32 direct Bazel policy")

    for token in (
        "Q0.34 exact workspace and target-manifest launch authority",
        "WINDOWS_TEST_QUERY",
        "CLIPPY_MANUAL_TEST_QUERY",
        "CRC32_POLYNOMIAL = 0x04C11DB7",
        "def _posix_cksum",
        "GITHUB_WORKSPACE must not be a symlink",
        "def _query_labels",
        "cwd=workspace",
        "env=dict(env)",
        "target-manifest Bazel query failed",
        "sorted(labels)",
        'not target.endswith("-test-bin")',
        "def validate_keyless_windows_gnullvm_execution",
        "final Bazel target manifest is not exact",
        "the last validation before launch still rehashes",
    ):
        require_token(q034, token, "Q0.34 execution manifest")
    require(
        q034.count("_validate_q032(command, env)") == 2,
        "Q0.34 must validate Q0.32 before and after manifest discovery",
    )
    reject_token(q034, "result.stderr", "Q0.34 execution manifest")

    for token in (
        "Q0.17-Q0.30 qualification ratchets",
        "from run_bazel_q030_direct_bazel import prepare_bazelisk_environment",
        "from run_bazel_q030_direct_bazel import resolve_verified_bazel_command",
        "validate_keyless_windows_gnullvm_command(command, os.environ)",
        "from run_bazel_q034_execution_manifest import (",
        "validate_keyless_windows_gnullvm_execution",
        "launch_cwd: Path | None = None",
        "cwd=launch_cwd",
        "os.execvpe(command[0], command, os.environ)",
    ):
        require_token(wrapper, token, "public Bazel wrapper")
    for before, after in (
        (
            "prepare_bazelisk_environment(os.environ)",
            "command = resolve_verified_bazel_command(command, os.environ)",
        ),
        (
            "command = resolve_verified_bazel_command(command, os.environ)",
            "validate_keyless_windows_gnullvm_command(command, os.environ)",
        ),
        (
            "validate_keyless_windows_gnullvm_command(command, os.environ)",
            "launch_cwd = validate_keyless_windows_gnullvm_execution(",
        ),
        (
            "launch_cwd = validate_keyless_windows_gnullvm_execution(",
            "result = subprocess.run(command, check=False, cwd=launch_cwd)",
        ),
    ):
        require_order(wrapper, before, after, "public Bazel wrapper")

    for token in (
        "test_posix_cksum_matches_reviewed_shell_algorithm",
        "test_exact_windows_test_manifest_and_workspace_pass",
        "test_test_target_omission_addition_and_reorder_fail_closed",
        "test_exact_clippy_manifest_filters_only_native_test_helpers",
        "test_clippy_target_omission_addition_and_reorder_fail_closed",
        "test_release_target_manifest_remains_exact_without_query",
        "test_query_failure_fails_closed_without_echoing_output",
        "test_workspace_symlink_fails_closed",
        "test_padded_duplicate_and_nonworkspace_query_output_fail_closed",
        "test_qualifying_windows_launch_uses_canonical_workspace",
        "test_nonqualifying_windows_launch_retains_legacy_cwd",
    ):
        require_token(execution_test, token, "Q0.34 regression")

    for token in (
        "python3 .github/scripts/test_run_bazel_direct_bazel.py",
        "python3 .github/scripts/test_run_bazel_setup_token_boundary.py",
        "python3 .github/scripts/test_run_bazel_execution_manifest.py",
        "python3 scripts/verify-windows-gnullvm-direct-bazel.py",
    ):
        require_token(boundary, token, "qualification boundary fixture")

    for token in (
        "python3 scripts/verify-windows-gnullvm-direct-bazel.py",
        '"final_bazel_cwd_bound_to_canonical_workspace": True',
        '"windows_test_target_manifest_recomputed_before_launch": True',
        '"windows_clippy_target_manifest_recomputed_before_launch": True',
        '"windows_release_target_manifest_exact_before_launch": True',
        '"target_manifest_executed_on_this_linux_source_job": False',
    ):
        require_token(
            qualification_workflow,
            token,
            "qualification workflow",
        )

    for token in (
        "bazel_test_query='tests(//...) except tests(//third_party/v8:all)",
        "| LC_ALL=C sort",
        "| cksum",
        "BAZEL_TEST_SHARD_COUNT: 4",
        "./scripts/list-bazel-clippy-targets.sh",
        "bash ./scripts/list-bazel-release-targets.sh",
    ):
        require_token(bazel_workflow, token, "Bazel workflow")

    for token in (
        "Resolve and sort the dynamic targets",
        "| LC_ALL=C sort",
        'grep -v -- \'-test-bin$\'',
        '"//codex-rs/..."',
        '"-//codex-rs/v8-poc:all"',
    ):
        require_token(clippy_targets, token, "Clippy target generator")

    for token in (
        '"//codex-rs/..."',
        '"-//codex-rs/core/tests/remote_env_windows:smoke-test"',
        '"-//codex-rs/v8-poc:all"',
    ):
        require_token(release_targets, token, "release target generator")

    for token in (
        "bazel-contrib/setup-bazel@c5acdfb288317d0b5c0bbd7a396a3dc868bb0f86",
        "- name: Scrub setup-only Bazelisk GitHub token",
        "printf '%s\\n' 'BAZELISK_GITHUB_TOKEN='",
        "unset BAZELISK_GITHUB_TOKEN",
    ):
        require_token(setup_bazel_action, token, "setup-bazel-ci action")

    require(
        BAZELVERSION.read_bytes() == b"9.0.0\n",
        ".bazelversion bytes drifted",
    )
    require_blob(
        BAZELVERSION,
        EXPECTED_BAZELVERSION_BLOB,
        ".bazelversion",
    )

    print("PASS_WINDOWS_GNULLVM_Q0_30_DIRECT_BAZEL_SOURCE")
    print("PASS_WINDOWS_GNULLVM_Q0_32_TRANSPORT_AND_PATH_SOURCE")
    print("PASS_WINDOWS_GNULLVM_Q0_33_SETUP_TOKEN_JOB_BOUNDARY_SOURCE")
    print("PASS_WINDOWS_GNULLVM_Q0_34_CWD_AND_TARGET_MANIFEST_SOURCE")


if __name__ == "__main__":
    main()
