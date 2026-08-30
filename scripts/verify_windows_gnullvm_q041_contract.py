"""Q0.41 additive ratchet for verified direct-Bazel query parsing."""

from __future__ import annotations

import hashlib
import importlib.util
import stat
import subprocess
import sys
from pathlib import Path
from typing import Final

from hepta_q041_blob_contract import (
    BLOBS,
    DIRECT,
    EXECUTABLE,
    JOB,
    Q040_BLOB_OVERRIDES,
)


ROOT = Path(__file__).resolve().parents[1]
BLOB_CONTRACT = ROOT / "scripts" / "hepta_q041_blob_contract.py"
BLOB_CONTRACT_SHA: Final = "a94aec13f821c9c583309bdda4e3f9d17b70e7b5"
BAZELISK_LINUX_SHA256: Final = (
    "22e7d3a188699982f661cf4687137ee52d1f24fec1ec893d91a6c4d791a75de8"
)
BAZEL_LINUX_SHA256: Final = (
    "c44a93f25398c68f904fa1d19b61d321de6c0d2f09dca375d7bc0dc9b9428403"
)


def fail(message: str) -> None:
    raise SystemExit(message)


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def read(relative: str) -> str:
    path = ROOT / relative
    require(path.is_file(), f"missing Q0.41 path: {relative}")
    return path.read_text(encoding="utf-8")


def blob(path: Path) -> str:
    data = path.read_bytes()
    framed = f"blob {len(data)}\0".encode("ascii") + data
    return hashlib.sha1(framed, usedforsecurity=False).hexdigest()


def require_executable(relative: str) -> None:
    path = ROOT / relative
    require(
        bool(path.stat().st_mode & stat.S_IXUSR),
        f"lost filesystem executable mode: {relative}",
    )
    result = subprocess.run(
        ["git", "ls-files", "--stage", "--", relative],
        cwd=ROOT,
        capture_output=True,
        text=True,
        check=False,
    )
    entries = result.stdout.splitlines()
    require(
        result.returncode == 0
        and len(entries) == 1
        and entries[0].split(maxsplit=1)[0] == "100755",
        f"lost Git executable mode: {relative}",
    )


def require_tokens(text: str, tokens: tuple[str, ...], owner: str) -> None:
    for token in tokens:
        require(token in text, f"{owner} lacks Q0.41 token: {token}")


def load_source(relative: str, name: str):
    spec = importlib.util.spec_from_file_location(name, ROOT / relative)
    require(spec is not None and spec.loader is not None, f"cannot load {relative}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


def run_adapted_q040(owner: str) -> None:
    module = load_source(
        "scripts/verify_windows_gnullvm_q040_contract.py",
        "_hepta_q041_q040_base",
    )
    module.BLOBS = {**module.BLOBS, **Q040_BLOB_OVERRIDES}
    module.DIRECT = DIRECT
    module.JOB = JOB
    module.main(owner)


def validate_increment() -> None:
    require(
        blob(BLOB_CONTRACT) == BLOB_CONTRACT_SHA,
        "Q0.41 blob contract drifted",
    )
    for relative, expected in BLOBS.items():
        path = ROOT / relative
        require(path.is_file(), f"missing Q0.41 blob: {relative}")
        require(blob(path) == expected, f"Q0.41 blob drift: {relative}")
    for relative in EXECUTABLE:
        require_executable(relative)

    require(
        read("scripts/verify-windows-gnullvm-direct-bazel.py") == DIRECT,
        "Q0.41 direct verifier wrapper drifted",
    )
    require(
        read("scripts/verify-windows-gnullvm-job-executable.py") == JOB,
        "Q0.41 job verifier wrapper drifted",
    )

    query = read(
        "scripts/verify-windows-gnullvm-bazel-query-executable.py"
    )
    require_tokens(
        query,
        (
            'BAZEL_VERSION: Final = "9.0.0"',
            'BAZELISK_VERSION: Final = "1.28.1"',
            BAZELISK_LINUX_SHA256,
            BAZEL_LINUX_SHA256,
            "unreviewed Bazelisk control",
            "def resolve_verified_linux_bazel",
            '[str(bazelisk), "--print_env"]',
            "Bazelisk executable changed during child resolution",
            "cached Bazel executable SHA-256 drifted",
            "content-addressed layout",
            "def _direct_environment",
            "def _validate_direct_environment",
            "def _validate_direct_bazel",
            "direct Bazel executable changed before parser launch",
            "BASE.parser_smoke_command",
            "BASE.validate_source()",
            "PASS_WINDOWS_GNULLVM_Q0_41_DIRECT_BAZEL_QUERY_EXECUTED",
        ),
        "verified query executable",
    )

    tests = read(".github/scripts/test_run_bazel_query_executable.py")
    require_tokens(
        tests,
        (
            "test_resolver_and_direct_launch_contract",
            "test_bazelisk_and_child_identity_fail_closed",
            "test_bazelisk_mutation_and_direct_execution",
            'poisoned["BAZELISK_USER_AGENT"]',
            "changed before parser launch",
            "changed during child resolution",
        ),
        "verified query executable regression",
    )

    fixture = read(".github/scripts/test_run_bazel_qualification_boundary.sh")
    for command in (
        "python3 .github/scripts/test_run_bazel_query_executable.py",
        "python3 scripts/verify-windows-gnullvm-bazel-query-executable.py",
    ):
        require(
            fixture.count(command) == 1,
            f"qualification fixture must execute exactly once: {command}",
        )

    workflow = read(
        ".github/workflows/windows-gnullvm-qualification-boundary.yml"
    )
    setup = "uses: ./.github/actions/setup-bazel-ci"
    legacy = (
        "python3 scripts/verify-windows-gnullvm-bazel-query-vector.py --execute"
    )
    direct = (
        "python3 scripts/verify-windows-gnullvm-bazel-query-executable.py --execute"
    )
    source = "python3 scripts/verify-windows-gnullvm-bazel-query-executable.py"
    for command in (legacy, direct):
        require(
            workflow.count(command) == 1,
            f"qualification workflow must execute exactly once: {command}",
        )
    require(
        workflow.index(setup) < workflow.index(legacy) < workflow.index(direct),
        "verified direct parser execution order drifted",
    )
    require(
        workflow.count(source) == 2,
        "query executable must run once as source check and once with --execute",
    )
    require_tokens(
        workflow,
        (
            '"bazelisk_version": "1.28.1"',
            BAZELISK_LINUX_SHA256,
            BAZEL_LINUX_SHA256,
            '"bazelisk_executable_verified": True',
            '"bazelisk_child_path_parsed_exactly_once": True',
            '"cached_bazel_cas_path_verified": True',
            '"cached_bazel_sha256_verified": True',
            '"direct_bazel_query_parser_executed": True',
            '"direct_bazel_rehashed_immediately_before_parser": True',
            '"cached_bazel_executed_on_this_linux_source_job": True',
            '"runtime_authority": False',
            '"production_authority": False',
            '"promotion": False',
            '"release_authority": False',
            '"callers_ratchet": False',
        ),
        "Q0.41 qualification receipt source",
    )


def main(owner: str = "q041") -> None:
    require(
        owner in {"q041", "direct-bazel", "job-executable"},
        f"unknown Q0.41 owner {owner!r}",
    )
    q040_owner = "q040" if owner == "q041" else owner
    run_adapted_q040(q040_owner)
    validate_increment()
    suffix = owner.upper().replace("-", "_")
    print(f"PASS_WINDOWS_GNULLVM_Q0_41_{suffix}_SOURCE")


if __name__ == "__main__":
    main()
