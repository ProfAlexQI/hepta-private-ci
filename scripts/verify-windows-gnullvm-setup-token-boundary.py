#!/usr/bin/env python3

from __future__ import annotations

import hashlib
import os
import stat
import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
ACTION = ROOT / ".github" / "actions" / "setup-bazel-ci" / "action.yml"
ASSERTION = (
    ROOT / ".github" / "scripts" / "assert-bazelisk-setup-token-scrubbed.sh"
)
WORKFLOW = (
    ROOT / ".github" / "workflows" / "windows-setup-bazel-token-boundary.yml"
)
BLOCKING = ROOT / ".github" / "workflows" / "blocking-ci.yml"

EXPECTED_ACTION_BLOB = "890567be46f3fd78c11b89a20950bef2f7af4bf6"
PINNED_SETUP_BAZEL = (
    "bazel-contrib/setup-bazel@"
    "c5acdfb288317d0b5c0bbd7a396a3dc868bb0f86"
)
EXPECTED_STEP_HEADERS = [
    "- id: setup_ci",
    "- name: Set up Bazel",
    "- name: Scrub setup-only Bazelisk GitHub token",
    "- name: Configure Bazel repository cache",
    "- name: Expose MSVC SDK environment (Windows)",
    "- name: Compute cache-stable Windows Bazel PATH",
]
ASSERTION_PASS = "PASS_SETUP_BAZEL_TOKEN_SCRUBBED"


def fail(message: str) -> None:
    raise SystemExit(message)


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def read(path: Path) -> str:
    require(
        path.is_file(),
        f"missing Q0.34 setup-token boundary path: {path.relative_to(ROOT)}",
    )
    return path.read_text(encoding="utf-8")


def git_blob_sha(path: Path) -> str:
    content = path.read_bytes()
    framed = f"blob {len(content)}\0".encode("ascii") + content
    return hashlib.sha1(framed, usedforsecurity=False).hexdigest()


def require_executable(path: Path) -> None:
    require(
        bool(path.stat().st_mode & stat.S_IXUSR),
        f"required executable lost mode: {path.relative_to(ROOT)}",
    )


def require_order(text: str, before: str, after: str, owner: str) -> None:
    require(before in text, f"{owner} lacks required token: {before}")
    require(after in text, f"{owner} lacks required token: {after}")
    require(
        text.index(before) < text.index(after),
        f"{owner} must place {before!r} before {after!r}",
    )


def composite_step_headers(text: str) -> list[str]:
    marker = "runs:\n  using: composite\n  steps:\n"
    require(
        text.count(marker) == 1,
        "setup-bazel-ci must contain exactly one canonical composite steps mapping",
    )
    require(
        text.count("\nruns:\n") == 1,
        "setup-bazel-ci contains duplicate or noncanonical top-level runs mappings",
    )
    require(
        text.count("\n  steps:\n") == 1,
        "setup-bazel-ci contains duplicate or noncanonical steps mappings",
    )

    body = text.split(marker, 1)[1]
    headers: list[str] = []
    for line in body.splitlines():
        if line and not line.startswith("    "):
            break
        if line.startswith("    - "):
            headers.append(line.strip())
    return headers


def validate_setup_action(text: str) -> None:
    headers = composite_step_headers(text)
    require(
        headers == EXPECTED_STEP_HEADERS,
        "setup-bazel-ci step sequence drifted or contains an unnamed/flow-style step: "
        f"{headers!r}",
    )
    require(
        text.count(PINNED_SETUP_BAZEL) == 1,
        "setup-bazel must remain pinned exactly once",
    )

    setup_start = text.index("    - name: Set up Bazel")
    scrub_start = text.index(
        "    - name: Scrub setup-only Bazelisk GitHub token",
        setup_start,
    )
    setup_block = text[setup_start:scrub_start]
    require(
        setup_block.count("      uses: " + PINNED_SETUP_BAZEL) == 1,
        "setup-bazel step lost its exact pinned action",
    )
    require(
        "\n      run:" not in setup_block,
        "setup-bazel step may not mix repository-controlled run commands",
    )
    require(
        "BAZELISK_GITHUB_TOKEN" not in setup_block,
        "setup-bazel step must not copy or interpolate the transport token",
    )


def prove_anonymous_step_is_rejected(text: str) -> None:
    scrub = "    - name: Scrub setup-only Bazelisk GitHub token"
    injected = text.replace(
        scrub,
        "    - uses: ./.github/actions/untrusted-before-scrub\n\n" + scrub,
        1,
    )
    require(
        composite_step_headers(injected)[2]
        == "- uses: ./.github/actions/untrusted-before-scrub",
        "strict parser failed to observe an unnamed uses step",
    )
    try:
        validate_setup_action(injected)
    except SystemExit:
        return
    fail("strict setup action validator accepted an unnamed pre-scrub step")


def run_assertion(extra_env: dict[str, str]) -> subprocess.CompletedProcess[str]:
    env = {
        "PATH": os.environ.get("PATH", ""),
        "HOME": os.environ.get("HOME", str(ROOT)),
    }
    env.update(extra_env)
    return subprocess.run(
        [str(ASSERTION)],
        cwd=ROOT,
        env=env,
        check=False,
        capture_output=True,
        text=True,
    )


def validate_assertion_runtime() -> None:
    for env in ({}, {"BAZELISK_GITHUB_TOKEN": ""}):
        result = run_assertion(env)
        require(
            result.returncode == 0,
            "setup-token assertion rejected an absent/empty token: "
            f"{result.stderr!r}",
        )
        require(
            result.stdout.strip() == ASSERTION_PASS,
            "setup-token assertion did not emit its exact PASS marker",
        )

    for name in (
        "BAZELISK_GITHUB_TOKEN",
        "bazelisk_github_token",
        "BaZeLiSk_GiThUb_ToKeN",
    ):
        secret = f"q034-secret-{name}"
        result = run_assertion({name: secret})
        combined = result.stdout + result.stderr
        require(
            result.returncode != 0,
            f"setup-token assertion accepted nonempty case variant {name!r}",
        )
        require(
            secret not in combined,
            "setup-token assertion leaked the rejected token value",
        )
        require(
            "remained nonempty after scrub" in result.stderr,
            "setup-token assertion lacks a fixed fail-closed diagnostic",
        )

    duplicate = run_assertion(
        {
            "BAZELISK_GITHUB_TOKEN": "",
            "bazelisk_github_token": "q034-duplicate-secret",
        }
    )
    require(
        duplicate.returncode != 0,
        "setup-token assertion accepted a nonempty case-fold duplicate",
    )
    require(
        "q034-duplicate-secret" not in duplicate.stdout + duplicate.stderr,
        "setup-token assertion leaked a case-fold duplicate value",
    )


def main() -> None:
    action = read(ACTION)
    assertion = read(ASSERTION)
    workflow = read(WORKFLOW)
    blocking = read(BLOCKING)

    require_executable(ASSERTION)
    require(
        git_blob_sha(ACTION) == EXPECTED_ACTION_BLOB,
        "Q0.33 setup-bazel-ci action drifted before Q0.34 verification",
    )
    validate_setup_action(action)
    prove_anonymous_step_is_rejected(action)

    for token in (
        "#!/usr/bin/env bash",
        "set -euo pipefail",
        'if [[ "${name,,}" != "bazelisk_github_token" ]]',
        'if [[ -n "${!name}" ]]',
        "remained nonempty after scrub",
        ASSERTION_PASS,
        "compgen -e",
    ):
        require(token in assertion, f"assertion script lacks token: {token}")
    for forbidden in (
        'echo "$BAZELISK_GITHUB_TOKEN"',
        'printf "%s" "$BAZELISK_GITHUB_TOKEN"',
        "::debug::",
        "::notice::",
        "::warning::",
    ):
        require(
            forbidden not in assertion,
            f"assertion script contains forbidden token-output form: {forbidden}",
        )
    validate_assertion_runtime()

    require(
        workflow.startswith("name: Windows setup Bazel token boundary\n"),
        "Q0.34 workflow name drifted",
    )
    require(
        "on:\n  workflow_call:\n" in workflow,
        "Q0.34 workflow must remain reusable-only",
    )
    for forbidden_trigger in (
        "  pull_request:",
        "  push:",
        "  schedule:",
        "  workflow_dispatch:",
    ):
        require(
            forbidden_trigger not in workflow,
            f"Q0.34 workflow contains forbidden trigger: {forbidden_trigger.strip()}",
        )
    for token in (
        "permissions:\n  contents: read",
        "python3 scripts/verify-windows-gnullvm-setup-token-boundary.py",
        "uses: ./.github/actions/setup-bazel-ci",
        "target: x86_64-unknown-linux-gnu",
        ".github/scripts/assert-bazelisk-setup-token-scrubbed.sh",
        '"anonymous_step_between_setup_and_scrub_allowed": False',
        '"post_setup_nonempty_transport_token_observed": False',
        '"source_writeback": False',
        '"runtime_authority": False',
        '"production_authority": False',
        '"release_authority": False',
        '"callers_ratchet": False',
    ):
        require(token in workflow, f"Q0.34 workflow lacks token: {token}")
    require_order(
        workflow,
        "python3 scripts/verify-windows-gnullvm-setup-token-boundary.py",
        "uses: ./.github/actions/setup-bazel-ci",
        "Q0.34 workflow",
    )
    require_order(
        workflow,
        "uses: ./.github/actions/setup-bazel-ci",
        ".github/scripts/assert-bazelisk-setup-token-scrubbed.sh",
        "Q0.34 workflow",
    )

    job = "windows-setup-bazel-token-boundary:"
    uses = "uses: ./.github/workflows/windows-setup-bazel-token-boundary.yml"
    need = "      - windows-setup-bazel-token-boundary\n"
    require(blocking.count(job) == 1, "blocking-ci must define Q0.34 job once")
    require(blocking.count(uses) == 1, "blocking-ci must call Q0.34 workflow once")
    require(blocking.count(need) == 1, "CI required must depend on Q0.34 once")
    require_order(
        blocking,
        "windows-gnullvm-boundary:",
        job,
        "blocking-ci",
    )
    require_order(
        blocking,
        "      - windows-gnullvm-boundary\n",
        need,
        "blocking-ci needs",
    )

    print("PASS_WINDOWS_GNULLVM_Q0_34_STRICT_SETUP_TOKEN_BOUNDARY_SOURCE")


if __name__ == "__main__":
    main()
