#!/usr/bin/env python3

from __future__ import annotations

import hashlib
import stat
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
WRAPPER = ROOT / ".github" / "scripts" / "run-bazel-ci.sh"
CORE = ROOT / ".github" / "scripts" / "run-bazel-ci-core.sh"
TEST = ROOT / ".github" / "scripts" / "test_run_bazel_ci_wrapper.py"
REPO_CHECKS = ROOT / ".github" / "workflows" / "repo-checks.yml"
TOOLCHAIN = ROOT / "bazel" / "toolchains" / "windows" / "BUILD.bazel"
BAZELRC = ROOT / ".bazelrc"
EXPECTED_Q0_13_CORE_BLOB = "2fe7cf37a0fddc1bb2f42f3e8a1e3b5a9e30f96b"


def fail(message: str) -> None:
    raise SystemExit(message)


def read(path: Path) -> str:
    if not path.is_file():
        fail(f"missing required path: {path.relative_to(ROOT)}")
    return path.read_text(encoding="utf-8")


def git_blob_sha(path: Path) -> str:
    content = path.read_bytes()
    payload = f"blob {len(content)}\0".encode() + content
    return hashlib.sha1(payload).hexdigest()


def require(text: str, needle: str, owner: str) -> None:
    if needle not in text:
        fail(f"{owner} is missing required contract text: {needle}")


def main() -> None:
    wrapper = read(WRAPPER)
    core = read(CORE)
    test = read(TEST)
    repo_checks = read(REPO_CHECKS)
    toolchain = read(TOOLCHAIN)
    bazelrc = read(BAZELRC)

    if not WRAPPER.stat().st_mode & stat.S_IXUSR:
        fail("run-bazel-ci.sh is not executable")
    if not CORE.stat().st_mode & stat.S_IXUSR:
        fail("run-bazel-ci-core.sh is not executable")
    if git_blob_sha(CORE) != EXPECTED_Q0_13_CORE_BLOB:
        fail("delegated Q0.13 core blob drifted")

    for needle in (
        'delegate="${script_dir}/run-bazel-ci-core.sh"',
        "ALLOW_WINDOWS_MSVC_FALLBACK is forbidden in GitHub Actions qualification jobs.",
        "--host_platform=//:local_windows_msvc",
        "--platforms=//:windows_x86_64_gnullvm",
        "--repo_env=BAZEL_DO_NOT_DETECT_CPP_TOOLCHAIN=0",
        "--extra_execution_platforms=//:windows_x86_64_msvc",
        "//:windows_gnullvm_tests_on_msvc_host_toolchain",
        "//bazel/toolchains/windows:local_msvc_cc_toolchain",
        "--strategy=TestRunner=local",
        "--strategy=V8Mksnapshot=local",
        "--test_env=RUST_TEST_THREADS=1",
    ):
        require(wrapper, needle, "run-bazel-ci.sh")

    require(
        bazelrc,
        "common --repo_env=BAZEL_DO_NOT_DETECT_CPP_TOOLCHAIN=1",
        ".bazelrc",
    )
    cross_skip = next(
        (
            line.removeprefix("common:ci-windows-cross ")
            for line in bazelrc.splitlines()
            if line.startswith(
                "common:ci-windows-cross --test_env=CODEX_BAZEL_TEST_SKIP_FILTERS="
            )
        ),
        None,
    )
    if cross_skip is None:
        fail(".bazelrc lacks the canonical Windows-cross skip contract")
    require(wrapper, cross_skip, "run-bazel-ci.sh")

    for needle in (
        'name = "local_msvc_cc_toolchain"',
        '"@llvm//constraints/windows/abi:msvc"',
        'toolchain = "@local_config_cc//:cc-compiler-x64_windows"',
        'toolchain_type = "@bazel_tools//tools/cpp:toolchain_type"',
    ):
        require(toolchain, needle, "local_msvc_cc_toolchain")

    required_tests = (
        "test_keyless_windows_cross_uses_split_local_toolchain_contract",
        "test_github_actions_rejects_ambient_msvc_fallback",
        "test_github_actions_rejects_disabled_local_cpp_discovery",
        "test_github_actions_rejects_later_conflicting_target_duplicate",
        "test_github_actions_rejects_later_conflicting_strategy_duplicate",
        "test_github_actions_rejects_missing_rust_bridge_toolchain",
        "test_github_actions_rejects_missing_msvc_exec_cc_toolchain",
        "test_authenticated_windows_cross_delegates_unchanged",
        "test_local_explicit_msvc_diagnostic_delegates_unchanged",
    )
    for test_name in required_tests:
        require(test, test_name, "test_run_bazel_ci_wrapper.py")

    require(
        repo_checks,
        "python3 scripts/verify-windows-gnullvm-qualification-contract.py",
        "repo-checks.yml",
    )
    require(
        repo_checks,
        "python3 -m unittest discover -s .github/scripts -p 'test_run_bazel*.py'",
        "repo-checks.yml",
    )

    for workflow in sorted((ROOT / ".github" / "workflows").glob("*.y*ml")):
        text = workflow.read_text(encoding="utf-8")
        if "ALLOW_WINDOWS_MSVC_FALLBACK" in text:
            fail(
                "qualification workflow contains ambient MSVC fallback marker: "
                f"{workflow.name}"
            )
        if "run-bazel-ci-core.sh" in text:
            fail(f"workflow bypasses the public Bazel wrapper: {workflow.name}")

    print("PASS_WINDOWS_GNULLVM_QUALIFICATION_CONTRACT_SOURCE")


if __name__ == "__main__":
    main()
