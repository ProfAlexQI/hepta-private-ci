#!/usr/bin/env python3

from __future__ import annotations

import hashlib
import stat
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
WRAPPER = ROOT / ".github" / "scripts" / "run-bazel-ci.sh"
IMPLEMENTATION = ROOT / ".github" / "scripts" / "run-bazel-ci-impl.sh"
TEST = ROOT / ".github" / "scripts" / "test_run_bazel_local_windows_gnullvm.py"
REPO_CHECKS = ROOT / ".github" / "workflows" / "repo-checks.yml"
TOOLCHAIN = ROOT / "bazel" / "toolchains" / "windows" / "BUILD.bazel"
BAZELRC = ROOT / ".bazelrc"
MODULE = ROOT / "MODULE.bazel"
RULES_CC_PATCH = ROOT / "patches" / "rules_cc_rusty_v8_custom_libcxx.patch"
EXPECTED_Q0_13_IMPLEMENTATION_BLOB = "2fe7cf37a0fddc1bb2f42f3e8a1e3b5a9e30f96b"
WINDOWS_GNULLVM_TEST_TAG_FILTERS = (
    "-nolinux,-noarm64,-noautodeps,-requires-python,"
    "-requires-powershell-parser,-requires-code-mode-powershell,"
    "-requires-conpty,-requires-unix-socket-bind,-requires-uds"
)
GENERIC_LOCAL_TOOLCHAIN_REGISTRATION = (
    '    "@local_config_cc_toolchains//:all",'
)


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


def reject(text: str, needle: str, owner: str) -> None:
    if needle in text:
        fail(f"{owner} contains forbidden contract text: {needle}")


def main() -> None:
    wrapper = read(WRAPPER)
    implementation = read(IMPLEMENTATION)
    test = read(TEST)
    repo_checks = read(REPO_CHECKS)
    toolchain = read(TOOLCHAIN)
    bazelrc = read(BAZELRC)
    module = read(MODULE)
    rules_cc_patch = read(RULES_CC_PATCH)

    if not WRAPPER.stat().st_mode & stat.S_IXUSR:
        fail("run-bazel-ci.sh is not executable")
    if not IMPLEMENTATION.stat().st_mode & stat.S_IXUSR:
        fail("run-bazel-ci-impl.sh is not executable")
    if git_blob_sha(IMPLEMENTATION) != EXPECTED_Q0_13_IMPLEMENTATION_BLOB:
        fail("delegated Q0.13 implementation blob drifted")

    required_wrapper_text = (
        'impl="$(dirname "${BASH_SOURCE[0]}")/run-bazel-ci-impl.sh"',
        "ALLOW_WINDOWS_MSVC_FALLBACK is forbidden in GitHub Actions qualification jobs.",
        "normalize_windows_local_bazel_args",
        "--host_platform|--platforms|--repo_env|--extra_execution_platforms|--extra_toolchains|--strategy|--test_env|--test_tag_filters|--config)",
        "Credential-free Windows option ${arg} is missing its value.",
        "Credential-free Windows qualification forbids competing --extra_toolchains value",
        "Credential-free Windows qualification forbids non-local --strategy value",
        "--host_platform=//:local_windows_msvc",
        "--platforms=//:windows_x86_64_gnullvm",
        "--repo_env=BAZEL_DO_NOT_DETECT_CPP_TOOLCHAIN=0",
        "--extra_execution_platforms=//:windows_x86_64_msvc",
        "//:windows_gnullvm_tests_on_msvc_host_toolchain",
        "//bazel/toolchains/windows:local_msvc_cc_toolchain",
        "--strategy=TestRunner=local",
        "--strategy=V8Mksnapshot=local",
        "--test_env=RUST_TEST_THREADS=1",
        "--test_env=BAZEL_SH",
        "--test_tag_filters=${WINDOWS_GNULLVM_TEST_TAG_FILTERS}",
        "refusing conflicting option",
    )
    for needle in required_wrapper_text:
        require(wrapper, needle, "run-bazel-ci.sh")

    require(
        wrapper,
        f'WINDOWS_GNULLVM_TEST_TAG_FILTERS="{WINDOWS_GNULLVM_TEST_TAG_FILTERS}"',
        "run-bazel-ci.sh",
    )
    for tag in (
        "-requires-powershell-parser",
        "-requires-code-mode-powershell",
        "-requires-conpty",
        "-requires-unix-socket-bind",
        "-requires-uds",
    ):
        require(wrapper, tag, "run-bazel-ci.sh")
        require(bazelrc, tag, ".bazelrc")

    reject(
        wrapper,
        "CODEX_BAZEL_TEST_SKIP_FILTERS",
        "run-bazel-ci.sh",
    )
    require(
        bazelrc,
        "common --repo_env=BAZEL_DO_NOT_DETECT_CPP_TOOLCHAIN=1",
        ".bazelrc",
    )

    for needle in (
        'name = "local_msvc_cc_toolchain"',
        '"@platforms//cpu:x86_64"',
        '"@platforms//os:windows"',
        '"@llvm//constraints/windows/abi:msvc"',
        'toolchain = "@local_config_cc//:cc-compiler-x64_windows"',
        'toolchain_type = "@bazel_tools//tools/cpp:toolchain_type"',
    ):
        require(toolchain, needle, "local_msvc_cc_toolchain")

    require(
        module,
        '"//patches:rules_cc_rusty_v8_custom_libcxx.patch"',
        "MODULE.bazel",
    )
    for needle in (
        "diff --git a/MODULE.bazel b/MODULE.bazel",
        'cc_configure = use_extension("//cc:extensions.bzl", "cc_configure_extension")',
        'use_repo(cc_configure, "local_config_cc", "local_config_cc_toolchains")',
        f"-{GENERIC_LOCAL_TOOLCHAIN_REGISTRATION}",
        '"//cc/private/toolchain/test:default_test_runner_toolchain"',
    ):
        require(
            rules_cc_patch,
            needle,
            "rules_cc_rusty_v8_custom_libcxx.patch",
        )
    reject(
        rules_cc_patch,
        f"+{GENERIC_LOCAL_TOOLCHAIN_REGISTRATION}",
        "rules_cc_rusty_v8_custom_libcxx.patch",
    )

    required_tests = (
        "test_keyless_cross_uses_real_local_gnullvm_target",
        "test_canonical_split_form_is_normalized_once",
        "test_unrelated_split_repo_env_and_strategy_are_preserved",
        "test_conflicting_target_fails_before_bazel",
        "test_later_conflicting_target_also_fails_before_bazel",
        "test_conflicting_split_target_fails_before_bazel",
        "test_conflicting_split_host_fails_before_bazel",
        "test_conflicting_split_repo_env_fails_before_bazel",
        "test_competing_split_toolchain_fails_before_bazel",
        "test_conflicting_test_strategy_fails_before_bazel",
        "test_conflicting_split_test_strategy_fails_before_bazel",
        "test_unlisted_remote_strategy_fails_before_bazel",
        "test_conflicting_split_test_env_fails_before_bazel",
        "test_conflicting_split_bazel_shell_fails_before_bazel",
        "test_conflicting_split_test_filter_fails_before_bazel",
        "test_missing_split_value_fails_before_bazel",
        "test_explicit_msvc_diagnostic_gets_real_local_cc_toolchain",
        "test_github_actions_rejects_ambient_msvc_diagnostic",
        "test_authenticated_cross_path_is_byte_for_byte_passthrough",
    )
    for test_name in required_tests:
        require(test, test_name, "test_run_bazel_local_windows_gnullvm.py")

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
        if workflow.name == "hepta-q0-17-source-publisher.yml":
            continue
        if "ALLOW_WINDOWS_MSVC_FALLBACK" in text:
            fail(
                "qualification workflow contains ambient MSVC fallback marker: "
                f"{workflow.name}"
            )
        if "run-bazel-ci-impl.sh" in text:
            fail(f"workflow bypasses the public Bazel wrapper: {workflow.name}")

    print("PASS_WINDOWS_GNULLVM_QUALIFICATION_CONTRACT_SOURCE")


if __name__ == "__main__":
    main()
