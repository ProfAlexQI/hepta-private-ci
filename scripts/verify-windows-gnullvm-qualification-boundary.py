#!/usr/bin/env python3

from __future__ import annotations

import hashlib
import stat
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
WORKFLOWS = REPO_ROOT / ".github" / "workflows"
WRAPPER = REPO_ROOT / ".github" / "scripts" / "run-bazel-ci.sh"
IMPLEMENTATION = REPO_ROOT / ".github" / "scripts" / "run-bazel-ci-impl.sh"
BOUNDARY_TEST = (
    REPO_ROOT
    / ".github"
    / "scripts"
    / "test_run_bazel_qualification_boundary.sh"
)
CI_WRAPPER_TEST = REPO_ROOT / ".github" / "scripts" / "test_run_bazel_ci_wrapper.py"
LOCAL_GNULLVM_TEST = (
    REPO_ROOT
    / ".github"
    / "scripts"
    / "test_run_bazel_local_windows_gnullvm.py"
)
DIAGNOSTIC_SCRIPT = REPO_ROOT / ".github" / "scripts" / "run-windows-msvc-diagnostic.sh"
TOOLCHAIN_BUILD = REPO_ROOT / "bazel" / "toolchains" / "windows" / "BUILD.bazel"
BOUNDARY_WORKFLOW = WORKFLOWS / "windows-gnullvm-qualification-boundary.yml"
DIAGNOSTIC_WORKFLOW = WORKFLOWS / "windows-msvc-nonqualifying-diagnostic.yml"
BLOCKING_WORKFLOW = WORKFLOWS / "blocking-ci.yml"
REPO_CHECKS = WORKFLOWS / "repo-checks.yml"
EXPECTED_IMPLEMENTATION_BLOB = "2fe7cf37a0fddc1bb2f42f3e8a1e3b5a9e30f96b"
RECEIPT_DIRECTORY_ENV = (
    "HEPTA_WINDOWS_MSVC_DIAGNOSTIC_RECEIPT_DIR: "
    "${{ runner.temp }}/hepta-windows-msvc-diagnostic"
)


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


def read(path: Path) -> str:
    require(
        path.is_file(),
        f"missing required file: {path.relative_to(REPO_ROOT)}",
    )
    return path.read_text(encoding="utf-8")


def git_blob_sha(path: Path) -> str:
    content = path.read_bytes()
    header = f"blob {len(content)}\0".encode()
    return hashlib.sha1(header + content).hexdigest()


def require_contains(text: str, expected: str, owner: str) -> None:
    require(expected in text, f"{owner} lacks required contract text: {expected}")


def main() -> None:
    wrapper = read(WRAPPER)
    implementation = read(IMPLEMENTATION)
    boundary_test = read(BOUNDARY_TEST)
    ci_wrapper_test = read(CI_WRAPPER_TEST)
    local_gnullvm_test = read(LOCAL_GNULLVM_TEST)
    diagnostic_script = read(DIAGNOSTIC_SCRIPT)
    toolchain_build = read(TOOLCHAIN_BUILD)
    boundary_workflow = read(BOUNDARY_WORKFLOW)
    diagnostic_workflow = read(DIAGNOSTIC_WORKFLOW)
    blocking_workflow = read(BLOCKING_WORKFLOW)
    repo_checks = read(REPO_CHECKS)

    require(
        wrapper.startswith("#!/usr/bin/env bash\n"),
        "wrapper must remain executable Bash",
    )
    require(
        bool(WRAPPER.stat().st_mode & stat.S_IXUSR),
        "run-bazel-ci.sh lost its executable bit",
    )
    require(
        bool(IMPLEMENTATION.stat().st_mode & stat.S_IXUSR),
        "run-bazel-ci-impl.sh lost its executable bit",
    )
    require(
        git_blob_sha(IMPLEMENTATION) == EXPECTED_IMPLEMENTATION_BLOB,
        "preserved Q0.13 Bazel implementation blob drifted",
    )
    require_contains(wrapper, "GITHUB_ACTIONS", "wrapper")
    require_contains(
        wrapper,
        "ALLOW_WINDOWS_MSVC_FALLBACK is forbidden in GitHub Actions "
        "qualification jobs",
        "wrapper",
    )
    require(
        "Automated Windows gnullvm qualification requires authenticated "
        "BuildBuddy/RBE" not in wrapper,
        "wrapper still blocks the source-controlled keyless gnullvm path",
    )

    required_wrapper_controls = (
        "--windows-msvc-host-platform",
        "--host_platform=//:local_windows_msvc",
        "--platforms=//:windows_x86_64_gnullvm",
        "--repo_env=BAZEL_DO_NOT_DETECT_CPP_TOOLCHAIN=0",
        "--extra_execution_platforms=//:windows_x86_64_msvc",
        "--extra_toolchains=//:windows_gnullvm_tests_on_msvc_host_toolchain,"
        "//bazel/toolchains/windows:local_msvc_cc_toolchain",
        "--strategy=TestRunner=local",
        "--strategy=V8Mksnapshot=local",
        "--local_test_jobs=8",
        "--jobs=8",
        "--test_env=RUST_TEST_THREADS=1",
        "--test_env=CODEX_BAZEL_TEST_SKIP_FILTERS="
        "command_safety::powershell_parser::tests::,"
        "suite::code_mode::code_mode_can_call_hidden_dynamic_tools,"
        "tests::windows_tests::"
        "conpty_ctrl_c_interrupts_powershell_foreground_child",
        "--build_metadata=TAG_windows_gnullvm_local=true",
        "require_exact_ci_arg",
        "require_ci_exact_list",
        "require_ci_allowed_configs",
        "canonicalize_ci_option",
        "canonicalize_exact_flag",
        "has_list_entry",
    )
    for expected in required_wrapper_controls:
        require_contains(wrapper, expected, "wrapper")

    require(
        "require_ci_list_contains" not in wrapper,
        "wrapper retains permissive list-membership validation",
    )
    require_contains(
        wrapper,
        "rejects non-allowlisted Bazel config",
        "wrapper",
    )
    require_contains(
        wrapper,
        'canonicalize_exact_flag "--config=ci-windows"',
        "wrapper",
    )
    require_contains(
        wrapper,
        'exec "$impl"',
        "wrapper",
    )
    require_contains(
        implementation,
        "ALLOW_WINDOWS_MSVC_FALLBACK",
        "preserved implementation",
    )

    for expected in (
        "PASS_WINDOWS_GNULLVM_QUALIFICATION_BOUNDARY_FIXTURES",
        "test_run_bazel_ci_wrapper.py",
        "test_run_bazel_local_windows_gnullvm.py",
    ):
        require_contains(boundary_test, expected, "boundary fixture")

    for expected in (
        "test_keyless_github_cross_uses_exact_split_toolchain_contract",
        "test_github_actions_rejects_ambient_msvc_fallback",
        "test_github_actions_rejects_conflicting_target",
        "test_github_actions_rejects_missing_required_toolchain",
        "test_github_actions_rejects_additional_execution_platform",
        "test_github_actions_rejects_additional_toolchain",
        "test_github_actions_rejects_duplicate_execution_platform",
        "test_github_actions_rejects_duplicate_toolchain",
        "test_github_actions_canonicalizes_exact_execution_lists",
        "test_github_actions_places_canonical_contract_after_configs",
        "test_github_actions_rejects_remote_config",
        "CI_TEST_FILTERS",
    ):
        require_contains(ci_wrapper_test, expected, "CI wrapper test")

    for expected in (
        "test_keyless_cross_uses_real_local_gnullvm_target",
        "test_conflicting_target_fails_before_bazel",
        "test_explicit_msvc_diagnostic_gets_real_local_cc_toolchain",
        "test_authenticated_cross_path_is_byte_for_byte_passthrough",
    ):
        require_contains(local_gnullvm_test, expected, "local gnullvm test")

    for expected in (
        'name = "local_msvc_cc_toolchain"',
        '"@local_config_cc//:cc-compiler-x64_windows"',
        '"@llvm//constraints/windows/abi:msvc"',
        'toolchain_type = "@bazel_tools//tools/cpp:toolchain_type"',
    ):
        require_contains(toolchain_build, expected, "MSVC exec toolchain")

    require(
        "qualification_mode" in diagnostic_script
        and "non_qualifying_msvc_diagnostic" in diagnostic_script,
        "diagnostic script lacks machine-readable non-qualifying mode",
    )
    for expected in (
        '"eligible_for_repository_admission": False',
        '"gnullvm_evidence": False',
        '"required_check": False',
        '"production_authority": False',
        '"release_authority": False',
    ):
        require_contains(diagnostic_script, expected, "diagnostic receipt")

    require(
        diagnostic_workflow.startswith(
            "name: Windows MSVC non-qualifying diagnostic\n"
        ),
        "manual diagnostic workflow name drifted",
    )
    require_contains(
        diagnostic_workflow,
        "on:\n  workflow_dispatch:",
        "manual diagnostic workflow",
    )
    for forbidden_trigger in (
        "  pull_request:",
        "  push:",
        "  schedule:",
        "  workflow_call:",
    ):
        require(
            forbidden_trigger not in diagnostic_workflow,
            "manual diagnostic contains forbidden trigger: "
            f"{forbidden_trigger.strip()}",
        )
    for expected in (
        "contents: read",
        "msvc-diagnostic:",
        "HEPTA_QUALIFICATION_MODE: non_qualifying_msvc_diagnostic",
        'ALLOW_WINDOWS_MSVC_FALLBACK: "1"',
        "run-windows-msvc-diagnostic.sh",
        "eligible_for_repository_admission=false",
    ):
        require_contains(diagnostic_workflow, expected, "manual workflow")

    job_prefix, separator, step_body = diagnostic_workflow.partition("    steps:\n")
    require(separator != "", "manual diagnostic workflow lacks a steps boundary")
    require(
        "${{ runner.temp }}" not in job_prefix,
        "manual diagnostic job-level env illegally references runner context",
    )
    require(
        step_body.count(RECEIPT_DIRECTORY_ENV) == 2,
        "manual diagnostic receipt directory must be bound in exactly two steps",
    )
    require_contains(
        step_body,
        "path: ${{ runner.temp }}/hepta-windows-msvc-diagnostic/receipt.json",
        "manual workflow artifact upload",
    )

    require(
        boundary_workflow.startswith(
            "name: Windows gnullvm qualification boundary\n"
        ),
        "qualification boundary workflow name drifted",
    )
    for expected in (
        "workflow_call:",
        "verify-windows-gnullvm-qualification-boundary.py",
        "test_run_bazel_qualification_boundary.sh",
        "PASS_WINDOWS_GNULLVM_QUALIFICATION_BOUNDARY_SOURCE",
    ):
        require_contains(boundary_workflow, expected, "boundary workflow")

    for workflow in sorted(WORKFLOWS.glob("*.y*ml")):
        text = workflow.read_text(encoding="utf-8")
        if workflow == DIAGNOSTIC_WORKFLOW:
            continue
        require(
            "ALLOW_WINDOWS_MSVC_FALLBACK" not in text,
            f"qualification workflow references fallback marker: {workflow.name}",
        )
        require(
            "run-bazel-ci-impl.sh" not in text,
            f"qualification workflow bypasses wrapper: {workflow.name}",
        )
        if workflow == BOUNDARY_WORKFLOW:
            require(
                text.count("run-windows-msvc-diagnostic.sh") == 1
                and "bash -n .github/scripts/run-windows-msvc-diagnostic.sh"
                in text,
                "boundary workflow may only syntax-check the diagnostic script",
            )
        else:
            require(
                "run-windows-msvc-diagnostic.sh" not in text,
                f"qualification workflow invokes diagnostic lane: {workflow.name}",
            )

    for expected in (
        "windows-gnullvm-boundary:",
        "uses: ./.github/workflows/windows-gnullvm-qualification-boundary.yml",
        "      - windows-gnullvm-boundary",
    ):
        require_contains(blocking_workflow, expected, "blocking-ci")

    require_contains(
        repo_checks,
        "python3 -m unittest discover -s .github/scripts "
        "-p 'test_run_bazel*.py'",
        "ordinary repo-checks",
    )

    print("PASS_WINDOWS_GNULLVM_QUALIFICATION_BOUNDARY_SOURCE")


if __name__ == "__main__":
    main()
