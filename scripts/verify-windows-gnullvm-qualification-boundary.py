#!/usr/bin/env python3

from __future__ import annotations

from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
WORKFLOWS = REPO_ROOT / ".github" / "workflows"
WRAPPER = REPO_ROOT / ".github" / "scripts" / "run-bazel-ci.sh"
IMPLEMENTATION = REPO_ROOT / ".github" / "scripts" / "run-bazel-ci-impl.sh"
BOUNDARY_TEST = REPO_ROOT / ".github" / "scripts" / "test_run_bazel_qualification_boundary.sh"
CI_WRAPPER_TEST = REPO_ROOT / ".github" / "scripts" / "test_run_bazel_ci_wrapper.py"
LOCAL_GNULLVM_TEST = REPO_ROOT / ".github" / "scripts" / "test_run_bazel_local_windows_gnullvm.py"
DIAGNOSTIC_SCRIPT = REPO_ROOT / ".github" / "scripts" / "run-windows-msvc-diagnostic.sh"
TOOLCHAIN_BUILD = REPO_ROOT / "bazel" / "toolchains" / "windows" / "BUILD.bazel"
BOUNDARY_WORKFLOW = WORKFLOWS / "windows-gnullvm-qualification-boundary.yml"
DIAGNOSTIC_WORKFLOW = WORKFLOWS / "windows-msvc-nonqualifying-diagnostic.yml"
BLOCKING_WORKFLOW = WORKFLOWS / "blocking-ci.yml"
REPO_CHECKS = WORKFLOWS / "repo-checks.yml"


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


def read(path: Path) -> str:
    require(path.is_file(), f"missing required file: {path.relative_to(REPO_ROOT)}")
    return path.read_text(encoding="utf-8")


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

    require(wrapper.startswith("#!/usr/bin/env bash\n"), "wrapper must remain executable Bash")
    require("GITHUB_ACTIONS" in wrapper, "wrapper lacks GitHub Actions boundary")
    require(
        "ALLOW_WINDOWS_MSVC_FALLBACK is forbidden in GitHub Actions qualification jobs" in wrapper,
        "wrapper lacks ambient MSVC fallback rejection",
    )
    require(
        "Automated Windows gnullvm qualification requires authenticated BuildBuddy/RBE" not in wrapper,
        "wrapper still blocks the source-controlled keyless gnullvm path",
    )
    for expected in (
        "--windows-msvc-host-platform",
        "--host_platform=//:local_windows_msvc",
        "--platforms=//:windows_x86_64_gnullvm",
        "--repo_env=BAZEL_DO_NOT_DETECT_CPP_TOOLCHAIN=0",
        "--extra_execution_platforms=//:windows_x86_64_msvc",
        "--extra_toolchains=//:windows_gnullvm_tests_on_msvc_host_toolchain,//bazel/toolchains/windows:local_msvc_cc_toolchain",
        "--strategy=TestRunner=local",
        "--strategy=V8Mksnapshot=local",
        "--local_test_jobs=8",
        "--jobs=8",
        "--test_env=RUST_TEST_THREADS=1",
        "--test_env=CODEX_BAZEL_TEST_SKIP_FILTERS=command_safety::powershell_parser::tests::,suite::code_mode::code_mode_can_call_hidden_dynamic_tools,tests::windows_tests::conpty_ctrl_c_interrupts_powershell_foreground_child",
        "--build_metadata=TAG_windows_gnullvm_local=true",
        "require_exact_ci_arg",
        "require_ci_exact_list",
        "require_ci_allowed_configs",
        "canonicalize_ci_option",
        "canonicalize_exact_flag",
        "has_list_entry",
    ):
        require(expected in wrapper, f"wrapper lacks exact local gnullvm control: {expected}")
    require(
        "require_ci_list_contains" not in wrapper,
        "wrapper retains permissive list-membership validation",
    )
    require(
        "rejects non-allowlisted Bazel config" in wrapper,
        "wrapper lacks keyless qualification config allowlist",
    )
    require(
        'canonicalize_exact_flag "--config=ci-windows"' in wrapper,
        "wrapper does not place ci-windows before canonical authority options",
    )
    require('exec "$impl"' in wrapper, "wrapper must delegate only after exact boundary construction")
    require(
        "ALLOW_WINDOWS_MSVC_FALLBACK" in implementation,
        "preserved implementation lost the manual diagnostic path",
    )

    for expected in (
        "PASS_WINDOWS_GNULLVM_QUALIFICATION_BOUNDARY_FIXTURES",
        "test_run_bazel_ci_wrapper.py",
        "test_run_bazel_local_windows_gnullvm.py",
    ):
        require(expected in boundary_test, f"boundary fixture lacks {expected}")
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
        require(expected in ci_wrapper_test, f"CI wrapper test lacks {expected}")
    for expected in (
        "test_keyless_cross_uses_real_local_gnullvm_target",
        "test_conflicting_target_fails_before_bazel",
        "test_explicit_msvc_diagnostic_gets_real_local_cc_toolchain",
        "test_authenticated_cross_path_is_byte_for_byte_passthrough",
    ):
        require(expected in local_gnullvm_test, f"local gnullvm test lacks {expected}")

    for expected in (
        'name = "local_msvc_cc_toolchain"',
        '"@local_config_cc//:cc-compiler-x64_windows"',
        '"@llvm//constraints/windows/abi:msvc"',
        'toolchain_type = "@bazel_tools//tools/cpp:toolchain_type"',
    ):
        require(expected in toolchain_build, f"MSVC exec toolchain lacks {expected}")

    require(
        "qualification_mode" in diagnostic_script and "non_qualifying_msvc_diagnostic" in diagnostic_script,
        "diagnostic script lacks machine-readable non-qualifying mode",
    )
    for expected in (
        '"eligible_for_repository_admission": False',
        '"gnullvm_evidence": False',
        '"required_check": False',
        '"production_authority": False',
        '"release_authority": False',
    ):
        require(expected in diagnostic_script, f"diagnostic receipt lacks {expected}")

    require(
        diagnostic_workflow.startswith("name: Windows MSVC non-qualifying diagnostic\n"),
        "manual diagnostic workflow name drifted",
    )
    require("on:\n  workflow_dispatch:" in diagnostic_workflow, "manual diagnostic must be workflow_dispatch-only")
    for forbidden_trigger in ("  pull_request:", "  push:", "  schedule:", "  workflow_call:"):
        require(
            forbidden_trigger not in diagnostic_workflow,
            f"manual diagnostic contains forbidden trigger: {forbidden_trigger.strip()}",
        )
    for expected in (
        "contents: read",
        "msvc-diagnostic:",
        "HEPTA_QUALIFICATION_MODE: non_qualifying_msvc_diagnostic",
        'ALLOW_WINDOWS_MSVC_FALLBACK: "1"',
        "run-windows-msvc-diagnostic.sh",
        "eligible_for_repository_admission=false",
    ):
        require(expected in diagnostic_workflow, f"manual workflow lacks {expected}")

    require(
        boundary_workflow.startswith("name: Windows gnullvm qualification boundary\n"),
        "qualification boundary workflow name drifted",
    )
    for expected in (
        "workflow_call:",
        "verify-windows-gnullvm-qualification-boundary.py",
        "test_run_bazel_qualification_boundary.sh",
        "PASS_WINDOWS_GNULLVM_QUALIFICATION_BOUNDARY_SOURCE",
    ):
        require(expected in boundary_workflow, f"boundary workflow lacks {expected}")

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
                and "bash -n .github/scripts/run-windows-msvc-diagnostic.sh" in text,
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
        require(expected in blocking_workflow, f"blocking-ci lacks ratchet: {expected}")

    require(
        "python3 -m unittest discover -s .github/scripts -p 'test_run_bazel*.py'" in repo_checks,
        "ordinary repo-checks does not execute Bazel launcher regressions",
    )

    print("PASS_WINDOWS_GNULLVM_QUALIFICATION_BOUNDARY_SOURCE")


if __name__ == "__main__":
    main()
