#!/usr/bin/env python3

from __future__ import annotations

from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
WORKFLOWS = REPO_ROOT / ".github" / "workflows"
WRAPPER = REPO_ROOT / ".github" / "scripts" / "run-bazel-ci.sh"
IMPLEMENTATION = REPO_ROOT / ".github" / "scripts" / "run-bazel-ci-impl.sh"
BOUNDARY_TEST = (
    REPO_ROOT / ".github" / "scripts" / "test_run_bazel_qualification_boundary.sh"
)
DIAGNOSTIC_SCRIPT = (
    REPO_ROOT / ".github" / "scripts" / "run-windows-msvc-diagnostic.sh"
)
BOUNDARY_WORKFLOW = WORKFLOWS / "windows-gnullvm-qualification-boundary.yml"
DIAGNOSTIC_WORKFLOW = WORKFLOWS / "windows-msvc-nonqualifying-diagnostic.yml"
BLOCKING_WORKFLOW = WORKFLOWS / "blocking-ci.yml"


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
    diagnostic_script = read(DIAGNOSTIC_SCRIPT)
    boundary_workflow = read(BOUNDARY_WORKFLOW)
    diagnostic_workflow = read(DIAGNOSTIC_WORKFLOW)
    blocking_workflow = read(BLOCKING_WORKFLOW)

    require(wrapper.startswith("#!/usr/bin/env bash\n"), "wrapper must remain executable Bash")
    require("GITHUB_ACTIONS" in wrapper, "wrapper lacks GitHub Actions boundary")
    require(
        "ALLOW_WINDOWS_MSVC_FALLBACK is forbidden in GitHub Actions qualification jobs"
        in wrapper,
        "wrapper lacks ambient fallback rejection",
    )
    require(
        "Automated Windows gnullvm qualification requires authenticated BuildBuddy/RBE"
        in wrapper,
        "wrapper lacks keyless automated rejection",
    )
    require(
        'exec "${script_dir}/run-bazel-ci-impl.sh" "$@"' in wrapper,
        "wrapper must delegate only after the boundary checks",
    )
    require(
        "ALLOW_WINDOWS_MSVC_FALLBACK" in implementation,
        "preserved implementation lost the explicit local diagnostic path",
    )

    require(
        "PASS_WINDOWS_GNULLVM_QUALIFICATION_BOUNDARY_FIXTURES" in boundary_test,
        "three-branch regression fixture is missing",
    )
    for expected in (
        "GITHUB_ACTIONS=true",
        "ALLOW_WINDOWS_MSVC_FALLBACK=1",
        "--config=ci-windows",
        "--config=ci-windows-cross",
        "--config=buildbuddy-generic-rbe",
        "--host_platform=//:local_windows_msvc",
        "--platforms=//:local_windows_msvc",
        "--host_platform=//:rbe",
    ):
        require(expected in boundary_test, f"fixture missing branch assertion: {expected}")

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
        require(expected in diagnostic_script, f"diagnostic receipt lacks {expected}")

    require(
        diagnostic_workflow.startswith("name: Windows MSVC non-qualifying diagnostic\n"),
        "manual diagnostic workflow name drifted",
    )
    require(
        "on:\n  workflow_dispatch:" in diagnostic_workflow,
        "manual diagnostic must be workflow_dispatch-only",
    )
    for forbidden_trigger in (
        "  pull_request:",
        "  push:",
        "  schedule:",
        "  workflow_call:",
    ):
        require(
            forbidden_trigger not in diagnostic_workflow,
            f"manual diagnostic contains forbidden trigger: {forbidden_trigger.strip()}",
        )
    for expected in (
        "contents: read",
        "msvc-diagnostic:",
        "HEPTA_QUALIFICATION_MODE: non_qualifying_msvc_diagnostic",
        "ALLOW_WINDOWS_MSVC_FALLBACK: \"1\"",
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

    print("PASS_WINDOWS_GNULLVM_QUALIFICATION_BOUNDARY_SOURCE")


if __name__ == "__main__":
    main()
