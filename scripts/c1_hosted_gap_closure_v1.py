#!/usr/bin/env python3
"""Exact-head, fail-closed producer for the current WEB-C1 hosted gaps.

This script is invoked only by the temporary ARM64 workflow. It patches a
small allowlisted surface, validates every changed contract with Rust 1.95 and
Python fixtures, commits only after all checks pass, and pushes only when the
remote branch still equals EXPECTED_HEAD. It does not merge, accept source or
topology, authorize a build, qualify runtime, promote, or release.
"""

from __future__ import annotations

import os
import pathlib
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
EXPECTED_BRANCH = "codex/hepta-vnext-plan-browser-c0-c3-20260827"
EXPECTED_HEAD = os.environ.get("EXPECTED_HEAD", "")
HEAD_REF = os.environ.get("HEAD_REF", "")


class ClosureError(RuntimeError):
    pass


def fail(message: str) -> None:
    raise ClosureError(message)


def run(*args: str, cwd: pathlib.Path = ROOT) -> None:
    print("+", " ".join(args), flush=True)
    subprocess.run(args, cwd=cwd, check=True)


def output(*args: str, cwd: pathlib.Path = ROOT) -> str:
    return subprocess.check_output(args, cwd=cwd, text=True, encoding="utf-8").strip()


def replace_once(path: str, old: str, new: str) -> None:
    target = ROOT / path
    text = target.read_text(encoding="utf-8")
    count = text.count(old)
    if count != 1:
        fail(f"{path}: expected one preimage, observed {count}")
    target.write_text(text.replace(old, new, 1), encoding="utf-8")


def replace_all(path: str, old: str, new: str) -> None:
    target = ROOT / path
    text = target.read_text(encoding="utf-8")
    if old not in text:
        fail(f"{path}: missing replacement preimage {old!r}")
    updated = text.replace(old, new)
    if old in updated:
        fail(f"{path}: replacement incomplete for {old!r}")
    target.write_text(updated, encoding="utf-8")


def verify_exact_head() -> None:
    if HEAD_REF != EXPECTED_BRANCH:
        fail(f"unexpected branch {HEAD_REF!r}")
    if len(EXPECTED_HEAD) != 40:
        fail("EXPECTED_HEAD is not one full Git SHA")
    if output("git", "rev-parse", "HEAD") != EXPECTED_HEAD:
        fail("local checkout is not EXPECTED_HEAD")
    run(
        "git",
        "fetch",
        "--no-tags",
        "origin",
        f"refs/heads/{HEAD_REF}:refs/remotes/origin/{HEAD_REF}",
    )
    if output("git", "rev-parse", f"refs/remotes/origin/{HEAD_REF}") != EXPECTED_HEAD:
        fail("remote branch moved before qualification")
    if output("git", "status", "--porcelain"):
        fail("worktree is not clean before closure")


def patch_contracts() -> None:
    replace_once(
        "scripts/tests/test_hepta_servo_build_input_seal_v3.py",
        '            "features": ["baked-in-resources", "background-hang-monitor"],\n',
        '            "features": ["background-hang-monitor", "baked-in-resources"],\n',
    )
    replace_once(
        "scripts/hepta-servo-source-bundle-verify.py",
        '''def safe_member_path(prefix: str, name: str) -> tuple[str, ...]:
    if not name.startswith(prefix) or "\\\\" in name or "\\0" in name:
        fail(f"archive path is outside frozen prefix: {name!r}")
    relative = name[len(prefix) :]
''',
        '''def safe_member_path(prefix: str, name: str) -> tuple[str, ...]:
    if "\\\\" in name or "\\0" in name:
        fail(f"archive path is unsafe: {name!r}")
    root = prefix.rstrip("/")
    if name == root:
        return ()
    if not name.startswith(prefix):
        fail(f"archive path is outside frozen prefix: {name!r}")
    relative = name[len(prefix) :]
''',
    )
    replace_once(
        "scripts/verify-hepta-servo-build-preflight-contract.py",
        '''        "workflow_call:",
        "pull_request:",
        "python3 scripts/tests/test_hepta_servo_build_preflight.py",
''',
        '''        "workflow_call:",
        "workflow_dispatch:",
        "python3 scripts/tests/test_hepta_servo_build_preflight.py",
''',
    )
    replace_once(
        "scripts/verify-hepta-servo-build-preflight-contract.py",
        '''    if "scripts/hepta-servo-build-preflight-v2.py --source-verification" in text:
        fail("contract workflow must not run a real build preflight")
''',
        '''    for forbidden_trigger in ("\\n  pull_request:", "\\n  push:"):
        if forbidden_trigger in text:
            fail(
                "build-preflight leaf workflow contains retired trigger "
                f"{forbidden_trigger.strip()}"
            )
    if "scripts/hepta-servo-build-preflight-v2.py --source-verification" in text:
        fail("contract workflow must not run a real build preflight")
''',
    )
    replace_once(
        "tools/hepta-browser-c1-protocol/tests/secret_redaction.rs",
        '''    for rendered in (
        format!("{expected:?}"),
        format!("{acknowledgement:?}"),
        format!("{confirmation:?}"),
    ) {
''',
        '''    for rendered in [
        format!("{expected:?}"),
        format!("{acknowledgement:?}"),
        format!("{confirmation:?}"),
    ] {
''',
    )
    replace_once(
        "scripts/verify-hepta-browser-plan.py",
        '''        blocking_tokens = (
            "pull_request:",
            "integration/vnext-main-20260811",
            "runner-preflight:",
            "hepta-vnext:",
            "uses: ./.github/workflows/hepta-vnext-qualification.yml",
            "- hepta-vnext",
        )
        for token in blocking_tokens:
            if token not in blocking:
                fail(f"blocking CI is missing {token}")
''',
        '''        blocking_tokens = (
            "pull_request:",
            "integration/vnext-main-20260811",
            "runner-preflight:",
            "name: CI required",
            "scripts/verify-hepta-required-contexts.py",
        )
        for token in blocking_tokens:
            if token not in blocking:
                fail(f"blocking CI is missing {token}")
        for forbidden in (
            "uses: ./.github/workflows/hepta-vnext-qualification.yml",
            "uses: ./.github/workflows/hepta-browser-next-required-v9.yml",
        ):
            if forbidden in blocking:
                fail(f"blocking CI reintroduced nested required graph {forbidden}")
        if "name: Hepta vNext required" not in hepta:
            fail("Hepta qualification is missing the independent required context")
''',
    )
    replace_all("scripts/verify-hepta-browser-plan.py", "@ProfAlexQI", "@ProfHepta")
    replace_all(".github/CODEOWNERS", "@ProfAlexQI", "@ProfHepta")


def validate() -> None:
    run("cargo", "fmt", "--manifest-path", "tools/hepta-browser-c1-protocol/Cargo.toml")
    run("cargo", "fmt", "--manifest-path", "tools/hepta-browser-c1-startup-bridge/Cargo.toml")

    python_files = (
        "scripts/hepta-servo-source-bundle-verify.py",
        "scripts/verify-hepta-servo-build-preflight-contract.py",
        "scripts/verify-hepta-browser-plan.py",
        "scripts/tests/test_hepta_servo_build_input_seal_v3.py",
    )
    run("python3", "-m", "py_compile", *python_files)
    for script in (
        "scripts/tests/test_hepta_servo_build_input_seal_v3.py",
        "scripts/tests/test_hepta_servo_source_bundle_verify.py",
        "scripts/tests/test_hepta_servo_source_bundle_verify_v2.py",
        "scripts/tests/test_hepta_servo_build_preflight.py",
        "scripts/tests/test_hepta_servo_build_preflight_v2.py",
        "scripts/verify-hepta-servo-build-preflight-contract.py",
        "scripts/verify-hepta-required-contexts.py",
        "scripts/verify-hepta-browser-plan.py",
    ):
        run("python3", script)

    for manifest in (
        "tools/hepta-browser-c1-protocol/Cargo.toml",
        "tools/hepta-browser-c1-startup-bridge/Cargo.toml",
    ):
        run("cargo", "fmt", "--manifest-path", manifest, "--", "--check")
        run("cargo", "test", "--locked", "--manifest-path", manifest)
        run(
            "cargo",
            "clippy",
            "--locked",
            "--manifest-path",
            manifest,
            "--all-targets",
            "--",
            "-D",
            "warnings",
        )
    run("python3", "scripts/verify-hepta-browser-c1-protocol.py")
    run("python3", "scripts/verify-hepta-browser-c1-startup-bridge.py")
    run("git", "diff", "--check")


def commit_and_push() -> None:
    allowlisted = (
        ".github/CODEOWNERS",
        "scripts/verify-hepta-browser-plan.py",
        "scripts/tests/test_hepta_servo_build_input_seal_v3.py",
        "scripts/hepta-servo-source-bundle-verify.py",
        "scripts/verify-hepta-servo-build-preflight-contract.py",
        "tools/hepta-browser-c1-protocol/src",
        "tools/hepta-browser-c1-protocol/tests",
        "tools/hepta-browser-c1-startup-bridge/src",
        "tools/hepta-browser-c1-startup-bridge/tests",
    )
    run("git", "add", "--", *allowlisted)
    staged = output("git", "diff", "--cached", "--name-only").splitlines()
    if not staged:
        fail("closure produced no staged changes")
    allowed_prefixes = (
        ".github/CODEOWNERS",
        "scripts/verify-hepta-browser-plan.py",
        "scripts/tests/test_hepta_servo_build_input_seal_v3.py",
        "scripts/hepta-servo-source-bundle-verify.py",
        "scripts/verify-hepta-servo-build-preflight-contract.py",
        "tools/hepta-browser-c1-protocol/src/",
        "tools/hepta-browser-c1-protocol/tests/",
        "tools/hepta-browser-c1-startup-bridge/src/",
        "tools/hepta-browser-c1-startup-bridge/tests/",
    )
    unexpected = [
        path
        for path in staged
        if not any(
            path == prefix or (prefix.endswith("/") and path.startswith(prefix))
            for prefix in allowed_prefixes
        )
    ]
    if unexpected:
        fail(f"unexpected staged paths: {unexpected}")
    run("git", "config", "user.name", "Hepta Qualification Bot")
    run(
        "git",
        "config",
        "user.email",
        "102159240+ProfHepta@users.noreply.github.com",
    )
    run("git", "commit", "-m", "fix(hepta-browser): close hosted qualification gaps")
    run(
        "git",
        "fetch",
        "--no-tags",
        "origin",
        f"refs/heads/{HEAD_REF}:refs/remotes/origin/{HEAD_REF}",
    )
    if output("git", "rev-parse", f"refs/remotes/origin/{HEAD_REF}") != EXPECTED_HEAD:
        fail("remote branch moved after validation; refusing push")
    run("git", "push", "origin", f"HEAD:refs/heads/{HEAD_REF}")
    print(f"NEW_HEAD={output('git', 'rev-parse', 'HEAD')}")


def main() -> int:
    verify_exact_head()
    patch_contracts()
    validate()
    commit_and_push()
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (ClosureError, OSError, UnicodeError, subprocess.CalledProcessError) as error:
        print(f"HEPTA_C1_GAP_CLOSURE=FAIL: {error}", file=sys.stderr)
        raise SystemExit(1)
