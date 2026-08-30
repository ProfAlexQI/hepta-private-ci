#!/usr/bin/env python3

from __future__ import annotations

import hashlib
import os
import subprocess
from collections.abc import Mapping
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
ASSERTION = (
    ROOT / ".github" / "scripts" / "assert-bazelisk-setup-token-scrubbed.sh"
)
Q034_VERIFIER = (
    ROOT / "scripts" / "verify-windows-gnullvm-setup-token-boundary.py"
)
WORKFLOW = (
    ROOT
    / ".github"
    / "workflows"
    / "windows-setup-bazel-token-boundary.yml"
)
BLOCKING = ROOT / ".github" / "workflows" / "blocking-ci.yml"

EXPECTED_ASSERTION_BLOB = "b8611644aeeb3624d475f8a3bd222be48f753e91"
EXPECTED_Q034_VERIFIER_BLOB = "e43fd8d37edf4f1cd48f60498d39596420da4be1"
PASS = "PASS_SETUP_BAZEL_TOKEN_SCRUBBED"
TRANSPORT_TOKEN = "BAZELISK_GITHUB_TOKEN"
SUBPROCESS_ENV_ALLOWLIST = frozenset(
    {
        "COMSPEC",
        "HOME",
        "PATH",
        "PATHEXT",
        "SYSTEMROOT",
        "TEMP",
        "TMP",
        "USERPROFILE",
        "WINDIR",
    }
)
WINDOWS_ROW = (
    "          - os: windows-latest\n"
    "            target: x86_64-pc-windows-gnullvm\n"
)


def fail(message: str) -> None:
    raise SystemExit(message)


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def read(path: Path) -> str:
    require(path.is_file(), f"missing Q0.37 path: {path.relative_to(ROOT)}")
    return path.read_text(encoding="utf-8")


def blob(path: Path) -> str:
    data = path.read_bytes()
    framed = f"blob {len(data)}\0".encode("ascii") + data
    return hashlib.sha1(framed, usedforsecurity=False).hexdigest()


def require_git_executable(path: Path) -> None:
    result = subprocess.run(
        [
            "git",
            "ls-files",
            "--stage",
            "--",
            str(path.relative_to(ROOT)).replace("\\", "/"),
        ],
        cwd=ROOT,
        check=False,
        capture_output=True,
        text=True,
    )
    entries = result.stdout.splitlines()
    require(
        result.returncode == 0
        and len(entries) == 1
        and entries[0].split(maxsplit=1)[0] == "100755",
        f"required Q0.37 executable lost Git mode: {path.relative_to(ROOT)}",
    )


def controlled_subprocess_env(
    base: Mapping[str, str],
    extra: Mapping[str, str],
) -> dict[str, str]:
    """Build a bounded child environment without inheriting setup credentials."""

    env = {
        name: value
        for name, value in base.items()
        if name.upper() in SUBPROCESS_ENV_ALLOWLIST
    }
    env.update(extra)
    return env


def validate_controlled_subprocess_env() -> None:
    fixture_base = {
        "Path": r"C:\Program Files\Git\bin",
        "SystemRoot": r"C:\Windows",
        "ComSpec": r"C:\Windows\System32\cmd.exe",
        "TEMP": r"C:\Temp",
        "NOT_ALLOWLISTED": "must-not-cross",
        "BaZeLiSk_GiThUb_ToKeN": "must-not-cross",
    }
    env = controlled_subprocess_env(fixture_base, {})
    observed = {name.upper(): value for name, value in env.items()}

    require(
        observed.get("PATH") == fixture_base["Path"],
        "controlled subprocess environment lost PATH",
    )
    require(
        observed.get("SYSTEMROOT") == fixture_base["SystemRoot"],
        "controlled subprocess environment lost SystemRoot",
    )
    require(
        observed.get("COMSPEC") == fixture_base["ComSpec"],
        "controlled subprocess environment lost ComSpec",
    )
    require(
        "NOT_ALLOWLISTED" not in observed,
        "controlled subprocess environment retained an unapproved variable",
    )
    require(
        TRANSPORT_TOKEN not in observed,
        "controlled subprocess environment inherited setup transport token",
    )

    injected = controlled_subprocess_env(
        fixture_base,
        {"BaZeLiSk_GiThUb_ToKeN": "fixture-only"},
    )
    require(
        any(name.upper() == TRANSPORT_TOKEN for name in injected),
        "controlled subprocess environment cannot exercise token rejection",
    )


def run_assertion(extra: dict[str, str]) -> subprocess.CompletedProcess[str]:
    env = controlled_subprocess_env(os.environ, extra)
    if os.name == "nt":
        require(
            any(
                name.upper() == "SYSTEMROOT" and bool(value)
                for name, value in env.items()
            ),
            "Windows assertion subprocess requires a valid SystemRoot",
        )
    return subprocess.run(
        ["bash", str(ASSERTION)],
        cwd=ROOT,
        env=env,
        check=False,
        capture_output=True,
        text=True,
    )


def validate_runtime() -> None:
    validate_controlled_subprocess_env()

    for env in ({}, {TRANSPORT_TOKEN: ""}):
        result = run_assertion(env)
        require(
            result.returncode == 0 and result.stdout.strip() == PASS,
            f"assertion rejected absent/empty token: {result.stderr!r}",
        )

    for name in (
        TRANSPORT_TOKEN,
        "bazelisk_github_token",
        "BaZeLiSk_GiThUb_ToKeN",
    ):
        secret = f"q037-secret-{name}"
        result = run_assertion({name: secret})
        output = result.stdout + result.stderr
        require(result.returncode != 0, f"assertion accepted {name!r}")
        require(secret not in output, "assertion leaked rejected token value")
        require(
            "remained nonempty after scrub" in result.stderr,
            "assertion lacks fixed fail-closed diagnostic",
        )


def require_order(text: str, before: str, after: str) -> None:
    require(
        before in text and after in text,
        f"missing ordered tokens: {before!r}, {after!r}",
    )
    require(
        text.index(before) < text.index(after),
        f"invalid order: {before!r}, {after!r}",
    )


def validate_workflow(text: str) -> None:
    required = (
        "on:\n  workflow_call:\n",
        "permissions:\n  contents: read",
        "name: Setup Bazel token boundary on ${{ matrix.os }}",
        "          - os: ubuntu-24.04\n"
        "            target: x86_64-unknown-linux-gnu\n",
        WINDOWS_ROW,
        "runs-on: ${{ matrix.os }}",
        "if: runner.os == 'Linux'",
        "python3 scripts/verify-windows-gnullvm-setup-token-boundary.py",
        "python3 scripts/verify-windows-gnullvm-setup-token-cross-platform.py",
        "uses: ./.github/actions/setup-bazel-ci",
        "target: ${{ matrix.target }}",
        ".github/scripts/assert-bazelisk-setup-token-scrubbed.sh",
        '"matrix_os": os.environ["MATRIX_OS"]',
        '"matrix_target": os.environ["MATRIX_TARGET"]',
        '"cross_platform_verifier_executed_before_setup_action": True',
        '"post_setup_nonempty_transport_token_observed": False',
        '"source_writeback": False',
        '"production_authority": False',
        '"release_authority": False',
        '"callers_ratchet": False',
        "windows-setup-bazel-token-boundary-${{ matrix.target }}-${{ github.sha }}",
    )
    for token in required:
        require(token in text, f"Q0.37 workflow lacks token: {token}")

    for forbidden in (
        "  pull_request:",
        "  push:",
        "  schedule:",
        "  workflow_dispatch:",
    ):
        require(
            forbidden not in text,
            f"Q0.37 workflow has forbidden trigger: {forbidden.strip()}",
        )

    require(
        text.count("          - os: ") == 2,
        "Q0.37 matrix must contain exactly two rows",
    )
    require(
        text.count("uses: ./.github/actions/setup-bazel-ci") == 1,
        "setup action declaration drifted",
    )
    require(
        text.count("runs-on: ${{ matrix.os }}") == 1,
        "matrix runner binding drifted",
    )
    require_order(
        text,
        "python3 scripts/verify-windows-gnullvm-setup-token-boundary.py",
        "python3 scripts/verify-windows-gnullvm-setup-token-cross-platform.py",
    )
    require_order(
        text,
        "python3 scripts/verify-windows-gnullvm-setup-token-cross-platform.py",
        "uses: ./.github/actions/setup-bazel-ci",
    )
    require_order(
        text,
        "uses: ./.github/actions/setup-bazel-ci",
        ".github/scripts/assert-bazelisk-setup-token-scrubbed.sh",
    )


def prove_windows_required(text: str) -> None:
    require(WINDOWS_ROW in text, "Q0.37 workflow lacks exact Windows row")
    try:
        validate_workflow(text.replace(WINDOWS_ROW, "", 1))
    except SystemExit:
        return
    fail("Q0.37 validator accepted removal of the Windows runner")


def validate_blocking(text: str) -> None:
    job = "windows-setup-bazel-token-boundary:"
    uses = "uses: ./.github/workflows/windows-setup-bazel-token-boundary.yml"
    need = "      - windows-setup-bazel-token-boundary\n"
    require(text.count(job) == 1, "blocking-ci Q0.37 job count drifted")
    require(text.count(uses) == 1, "blocking-ci Q0.37 workflow call count drifted")
    require(text.count(need) == 1, "CI required Q0.37 dependency count drifted")


def main() -> None:
    assertion = read(ASSERTION)
    q034 = read(Q034_VERIFIER)
    workflow = read(WORKFLOW)
    blocking = read(BLOCKING)

    require_git_executable(ASSERTION)
    require_git_executable(Path(__file__).resolve())
    require(blob(ASSERTION) == EXPECTED_ASSERTION_BLOB, "Q0.34 assertion blob drifted")
    require(
        blob(Q034_VERIFIER) == EXPECTED_Q034_VERIFIER_BLOB,
        "Q0.34 verifier blob drifted",
    )
    require(
        "compgen -e" in assertion and PASS in assertion,
        "Q0.34 assertion contract drifted",
    )

    validate_runtime()
    validate_workflow(workflow)
    prove_windows_required(workflow)
    validate_blocking(blocking)

    print(
        "PASS_WINDOWS_GNULLVM_Q0_37_WINDOWS_SUBPROCESS_ENV_SETUP_TOKEN_SOURCE"
    )


if __name__ == "__main__":
    main()
