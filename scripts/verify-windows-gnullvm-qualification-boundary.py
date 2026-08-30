#!/usr/bin/env python3
from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
WF = ROOT / ".github/workflows"
SKIP = (
    "command_safety::powershell_parser::tests::,"
    "suite::code_mode::code_mode_can_call_hidden_dynamic_tools,"
    "tests::windows_tests::conpty_ctrl_c_interrupts_powershell_foreground_child"
)


def text(path: str) -> str:
    file = ROOT / path
    if not file.is_file():
        raise SystemExit(f"missing required file: {path}")
    return file.read_text(encoding="utf-8")


def contains(source: str, values: tuple[str, ...], label: str) -> None:
    for value in values:
        if value not in source:
            raise SystemExit(f"{label} lacks: {value}")


def main() -> None:
    wrapper = text(".github/scripts/run-bazel-ci.sh")
    impl = text(".github/scripts/run-bazel-ci-impl.sh")
    bazelrc = text(".bazelrc")
    boundary_fixture = text(".github/scripts/test_run_bazel_qualification_boundary.sh")
    wrapper_tests = text(".github/scripts/test_run_bazel_ci_wrapper.py")
    local_tests = text(".github/scripts/test_run_bazel_local_windows_gnullvm.py")
    toolchain = text("bazel/toolchains/windows/BUILD.bazel")
    diagnostic = text(".github/scripts/run-windows-msvc-diagnostic.sh")
    boundary_wf = text(".github/workflows/windows-gnullvm-qualification-boundary.yml")
    diagnostic_wf = text(".github/workflows/windows-msvc-nonqualifying-diagnostic.yml")
    blocking = text(".github/workflows/blocking-ci.yml")
    repo_checks = text(".github/workflows/repo-checks.yml")

    contains(wrapper, (
        "GITHUB_ACTIONS",
        "ALLOW_WINDOWS_MSVC_FALLBACK is forbidden in GitHub Actions qualification jobs",
        "--windows-local-gnullvm",
        "--host_platform=//:local_windows_msvc",
        "--platforms=//:windows_x86_64_gnullvm",
        "--repo_env=BAZEL_DO_NOT_DETECT_CPP_TOOLCHAIN=0",
        "--extra_execution_platforms=//:windows_x86_64_msvc",
        "--extra_toolchains=//:windows_gnullvm_tests_on_msvc_host_toolchain",
        "--extra_toolchains=//bazel/toolchains/windows:local_msvc_cc_toolchain",
        "--strategy=TestRunner=local",
        "--strategy=V8Mksnapshot=local",
        "--test_env=RUST_TEST_THREADS=1",
        f"--test_env=CODEX_BAZEL_TEST_SKIP_FILTERS={SKIP}",
        "reject_ci_owned_prefix",
        "reject_ci_forbidden_config",
        'exec "$impl"',
    ), "wrapper")
    if "Automated Windows gnullvm qualification requires authenticated BuildBuddy/RBE" in wrapper:
        raise SystemExit("wrapper still blocks keyless gnullvm qualification")
    local_flag = wrapper.index('wrapper_args+=("--windows-local-gnullvm")')
    config = wrapper.index('"--config=ci-windows"', local_flag)
    host = wrapper.index('"--host_platform=//:local_windows_msvc"', local_flag)
    if not local_flag < config < host:
        raise SystemExit("canonical keyless option ordering drifted")

    contains(impl, (
        "windows_local_gnullvm=0",
        "--windows-local-gnullvm",
        "pass_windows_target_build_env=0",
        "pass_windows_host_build_env=1",
        'post_config_bazel_args+=("--host_action_env=${env_var}")',
        'post_config_bazel_args+=("--action_env=${env_var}")',
        '"--host_action_env=PATH=${CODEX_BAZEL_WINDOWS_PATH}"',
        '"--test_env=PATH=${CODEX_BAZEL_WINDOWS_PATH}"',
        "local gnullvm compilation uses an MSVC platform only for",
    ), "implementation")
    if f"common:ci-windows-cross --test_env=CODEX_BAZEL_TEST_SKIP_FILTERS={SKIP}" not in bazelrc:
        raise SystemExit("ci-windows-cross canonical skip set drifted")

    contains(boundary_fixture, (
        "PASS_WINDOWS_GNULLVM_QUALIFICATION_BOUNDARY_FIXTURES",
        "test_run_bazel_ci_wrapper.py",
        "test_run_bazel_local_windows_gnullvm.py",
    ), "boundary fixture")
    contains(wrapper_tests, (
        "test_keyless_github_cross_uses_exact_split_toolchain_contract",
        "test_github_actions_rejects_ambient_msvc_fallback",
        "test_github_actions_rejects_every_caller_owned_critical_option",
        "test_github_actions_rejects_remote_or_platform_configs",
    ), "wrapper tests")
    contains(local_tests, (
        "test_github_keyless_contract_injects_canonical_cross_skip_set",
        "test_local_gnullvm_impl_keeps_msvc_sdk_host_only",
        "test_ordinary_local_windows_impl_preserves_target_and_host_env",
        "test_authenticated_cross_path_is_byte_for_byte_passthrough",
    ), "local gnullvm tests")
    contains(toolchain, (
        'name = "local_msvc_cc_toolchain"',
        '"@local_config_cc//:cc-compiler-x64_windows"',
        '"@llvm//constraints/windows/abi:msvc"',
    ), "MSVC exec toolchain")

    contains(diagnostic, (
        "non_qualifying_msvc_diagnostic",
        '"eligible_for_repository_admission": False',
        '"gnullvm_evidence": False',
        '"production_authority": False',
        '"release_authority": False',
    ), "diagnostic receipt")
    if not diagnostic_wf.startswith("name: Windows MSVC non-qualifying diagnostic\n"):
        raise SystemExit("diagnostic workflow name drifted")
    contains(diagnostic_wf, (
        "on:\n  workflow_dispatch:",
        'ALLOW_WINDOWS_MSVC_FALLBACK: "1"',
        "eligible_for_repository_admission=false",
    ), "diagnostic workflow")
    for trigger in ("  pull_request:", "  push:", "  schedule:", "  workflow_call:"):
        if trigger in diagnostic_wf:
            raise SystemExit(f"diagnostic workflow has forbidden trigger: {trigger.strip()}")

    contains(boundary_wf, (
        "workflow_call:",
        "verify-windows-gnullvm-qualification-boundary.py",
        "test_run_bazel_qualification_boundary.sh",
        "PASS_WINDOWS_GNULLVM_QUALIFICATION_BOUNDARY_SOURCE",
    ), "boundary workflow")
    for file in sorted(WF.glob("*.y*ml")):
        source = file.read_text(encoding="utf-8")
        if file.name == "windows-msvc-nonqualifying-diagnostic.yml":
            continue
        if "ALLOW_WINDOWS_MSVC_FALLBACK" in source or "run-bazel-ci-impl.sh" in source:
            raise SystemExit(f"qualification workflow bypasses boundary: {file.name}")
        if file.name != "windows-gnullvm-qualification-boundary.yml" and "run-windows-msvc-diagnostic.sh" in source:
            raise SystemExit(f"qualification workflow invokes diagnostic lane: {file.name}")

    contains(blocking, (
        "windows-gnullvm-boundary:",
        "uses: ./.github/workflows/windows-gnullvm-qualification-boundary.yml",
        "      - windows-gnullvm-boundary",
    ), "blocking-ci")
    if "python3 -m unittest discover -s .github/scripts -p 'test_run_bazel*.py'" not in repo_checks:
        raise SystemExit("repo-checks omits Bazel launcher regressions")

    print("PASS_WINDOWS_GNULLVM_QUALIFICATION_BOUNDARY_SOURCE")


if __name__ == "__main__":
    main()
