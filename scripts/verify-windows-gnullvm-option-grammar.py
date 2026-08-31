#!/usr/bin/env python3

from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
WRAPPER = ROOT / ".github" / "scripts" / "run-bazel-ci.sh"
TEST = ROOT / ".github" / "scripts" / "test_run_bazel_option_grammar.py"
FIXTURE = (
    ROOT
    / ".github"
    / "scripts"
    / "test_run_bazel_qualification_boundary.sh"
)
BOUNDARY = ROOT / ".github" / "workflows" / "windows-gnullvm-qualification-boundary.yml"
REPO_CHECKS = ROOT / ".github" / "workflows" / "repo-checks.yml"


def read(path: Path) -> str:
    if not path.is_file():
        raise SystemExit(f"missing option-grammar contract path: {path.relative_to(ROOT)}")
    return path.read_text(encoding="utf-8")


def require(text: str, expected: str, owner: str) -> None:
    if expected not in text:
        raise SystemExit(f"{owner} lacks option-grammar contract: {expected}")


def main() -> None:
    wrapper = read(WRAPPER)
    test = read(TEST)
    fixture = read(FIXTURE)
    boundary = read(BOUNDARY)
    repo_checks = read(REPO_CHECKS)

    require(wrapper, "reject_ci_separated_value_options", "Bazel wrapper")
    require(
        wrapper,
        "requires protected Bazel options in --option=value form",
        "Bazel wrapper",
    )
    if wrapper.index("reject_ci_separated_value_options") > wrapper.index(
        "require_ci_allowed_configs"
    ):
        raise SystemExit("option-grammar rejection must precede config validation")

    for option in (
        "--config",
        "--host_platform",
        "--platforms",
        "--repo_env",
        "--extra_execution_platforms",
        "--extra_toolchains",
        "--strategy",
        "--local_test_jobs",
        "--jobs",
        "--test_env",
    ):
        require(wrapper, option, "Bazel wrapper")
        require(test, option, "option-grammar regression")

    require(
        test,
        "test_github_actions_rejects_separated_protected_values",
        "option-grammar regression",
    )
    require(
        fixture,
        "python3 .github/scripts/test_run_bazel_option_grammar.py",
        "qualification fixture",
    )
    require(
        boundary,
        "python3 scripts/verify-windows-gnullvm-option-grammar.py",
        "qualification workflow",
    )
    require(
        repo_checks,
        "python3 -m unittest discover -s .github/scripts "
        "-p 'test_run_bazel*.py'",
        "ordinary repo-checks",
    )

    print("PASS_WINDOWS_GNULLVM_OPTION_GRAMMAR_SOURCE")


if __name__ == "__main__":
    main()
