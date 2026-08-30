#!/usr/bin/env python3
"""Q0.43 composition of startup, Git-mode receipt, and direct-query closure."""

from __future__ import annotations

import hashlib
import subprocess
from pathlib import Path
from typing import Final

import verify_windows_gnullvm_q041_contract as q041
from hepta_q043_blob_contract import BLOBS, DIRECT, EXECUTABLE, JOB


ROOT = Path(__file__).resolve().parents[1]
BLOB_CONTRACT = ROOT / "scripts" / "hepta_q043_blob_contract.py"
BLOB_CONTRACT_SHA: Final = "f0300d515f0a9a9a93b34980c5edac81f7c8c00a"
Q041_BLOB_CONTRACT = "scripts/hepta_q041_blob_contract.py"
Q041_CONTRACT = "scripts/verify_windows_gnullvm_q041_contract.py"
BOUNDARY = ".github/scripts/test_run_bazel_qualification_boundary.sh"
WORKFLOW = ".github/workflows/windows-gnullvm-qualification-boundary.yml"
SETUP_WORKFLOW = ".github/workflows/windows-setup-bazel-token-boundary.yml"
RECEIPT = "scripts/verify-windows-gnullvm-setup-token-receipt-truth.py"
QUERY_BASE = "scripts/verify-windows-gnullvm-bazel-query-vector.py"
QUERY = "scripts/verify-windows-gnullvm-bazel-query-executable.py"
QUERY_TEST = ".github/scripts/test_run_bazel_query_executable.py"
DIRECT_ADAPTER = "scripts/verify-windows-gnullvm-direct-bazel.py"
JOB_ADAPTER = "scripts/verify-windows-gnullvm-job-executable.py"
BAZELISK_SHA256: Final = (
    "22e7d3a188699982f661cf4687137ee52d1f24fec1ec893d91a6c4d791a75de8"
)
BAZEL_SHA256: Final = (
    "c44a93f25398c68f904fa1d19b61d321de6c0d2f09dca375d7bc0dc9b9428403"
)


def fail(message: str) -> None:
    raise SystemExit(message)


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def read(relative: str) -> str:
    path = ROOT / relative
    require(path.is_file(), f"missing Q0.43 path: {relative}")
    return path.read_text(encoding="utf-8")


def blob(path: Path) -> str:
    data = path.read_bytes()
    framed = f"blob {len(data)}\0".encode("ascii") + data
    return hashlib.sha1(framed, usedforsecurity=False).hexdigest()


def require_git_executable(relative: str) -> None:
    path = ROOT / relative
    require(path.is_file(), f"missing executable Q0.43 path: {relative}")
    result = subprocess.run(
        ["git", "ls-files", "--stage", "--", relative.replace("\\", "/")],
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
        f"Q0.43 requires one Git 100755 entry: {relative}",
    )


def require_tokens(text: str, tokens: tuple[str, ...], owner: str) -> None:
    for token in tokens:
        require(token in text, f"{owner} lacks Q0.43 token: {token}")


def require_order(text: str, *tokens: str) -> None:
    positions: list[int] = []
    for token in tokens:
        require(token in text, f"missing ordered Q0.43 token: {token!r}")
        positions.append(text.index(token))
    require(
        positions == sorted(positions) and len(positions) == len(set(positions)),
        f"invalid or ambiguous Q0.43 order: {tokens!r}",
    )


def patch_parent_contracts() -> None:
    require(
        blob(ROOT / Q041_BLOB_CONTRACT) == BLOBS[Q041_BLOB_CONTRACT],
        "selected Q0.41 blob contract drifted",
    )
    require(
        blob(ROOT / Q041_CONTRACT) == BLOBS[Q041_CONTRACT],
        "selected fixed Q0.41 source contract drifted",
    )

    q041.BLOBS[BOUNDARY] = BLOBS[BOUNDARY]
    q041.BLOBS[WORKFLOW] = BLOBS[WORKFLOW]
    q041.BLOBS[DIRECT_ADAPTER] = BLOBS[DIRECT_ADAPTER]
    q041.BLOBS[JOB_ADAPTER] = BLOBS[JOB_ADAPTER]
    q041.DIRECT = DIRECT
    q041.JOB = JOB

    q041.q040.BLOBS[BOUNDARY] = BLOBS[BOUNDARY]
    q041.q040.BLOBS[WORKFLOW] = BLOBS[WORKFLOW]
    q041.q040.BLOBS[RECEIPT] = BLOBS[RECEIPT]
    q041.q040.BLOBS[DIRECT_ADAPTER] = BLOBS[DIRECT_ADAPTER]
    q041.q040.BLOBS[JOB_ADAPTER] = BLOBS[JOB_ADAPTER]


def validate_receipt_git_mode() -> None:
    receipt = read(RECEIPT)
    require_tokens(
        receipt,
        (
            '["git", "ls-files", "--stage", "--", relative]',
            'str(path.relative_to(ROOT)).replace("\\\\", "/")',
            'entries[0].split(maxsplit=1)[0] == "100755"',
            "Q0.42 receipt-truth verifier must be one Git 100755 entry",
            "PASS_WINDOWS_GNULLVM_Q0_42_RECEIPT_GIT_MODE_TRUTH_SOURCE",
        ),
        "setup-token receipt-truth verifier",
    )
    for forbidden in (
        "stat.S_IXUSR",
        ".stat().st_mode",
        "os.access(",
    ):
        require(
            forbidden not in receipt,
            f"receipt verifier retained host-mode authority: {forbidden}",
        )


def validate_direct_query() -> None:
    query = read(QUERY)
    test = read(QUERY_TEST)
    require_tokens(
        query,
        (
            'BAZEL_VERSION: Final = "9.0.0"',
            'BAZELISK_VERSION: Final = "1.28.1"',
            BAZELISK_SHA256,
            BAZEL_SHA256,
            "def resolve_verified_linux_bazel(",
            '[str(bazelisk), "--print_env"]',
            "Bazelisk executable changed during child resolution",
            "def _require_bazel_cas_identity",
            "expected = (",
            '"downloads",',
            '"sha256",',
            "def _validate_direct_bazel(",
            "direct Bazel executable changed before parser launch",
            "direct Bazel PATH head is not the verified CAS directory",
            "command = BASE.parser_smoke_command(bazel, workspace)",
            'observed == ["//:probe"]',
        ),
        "verified direct Bazel query source",
    )
    require_tokens(
        test,
        (
            "test_resolver_and_direct_launch_contract",
            "test_bazelisk_and_child_identity_fail_closed",
            "test_bazelisk_mutation_and_direct_execution",
            "changed before parser launch",
            "changed during child resolution",
            "content-addressed layout",
        ),
        "verified direct Bazel query regression",
    )

    query_module = q041.q040.load_source(
        QUERY,
        "_hepta_q043_verified_query",
    )
    query_module.BASE.validate_source()


def validate_boundary() -> None:
    boundary = read(BOUNDARY)
    commands = (
        "python3 .github/scripts/test_run_bazel_startup_contract.py",
        "python3 scripts/verify-windows-gnullvm-startup-contract.py",
        "python3 .github/scripts/test_run_bazel_query_executable.py",
        "python3 scripts/verify-windows-gnullvm-bazel-query-executable.py",
        "python3 scripts/verify-windows-gnullvm-setup-token-receipt-truth.py",
        "python3 scripts/verify-windows-gnullvm-direct-bazel.py",
        "python3 scripts/verify-windows-gnullvm-job-executable.py",
    )
    for command in commands:
        require(
            boundary.count(command) == 1,
            f"Q0.43 boundary command count drifted: {command}",
        )
    require_order(
        boundary,
        "python3 .github/scripts/test_run_bazel_startup_contract.py",
        "python3 .github/scripts/test_run_bazel_query_executable.py",
        "python3 scripts/verify-windows-gnullvm-startup-contract.py",
        "python3 scripts/verify-windows-gnullvm-bazel-query-executable.py",
        "python3 scripts/verify-windows-gnullvm-setup-token-receipt-truth.py",
    )


def validate_workflow() -> None:
    workflow = read(WORKFLOW)
    setup = "uses: ./.github/actions/setup-bazel-ci"
    legacy = (
        "python3 scripts/verify-windows-gnullvm-bazel-query-vector.py "
        "--execute"
    )
    direct_query = (
        "python3 scripts/verify-windows-gnullvm-bazel-query-executable.py "
        "--execute"
    )
    require_tokens(
        workflow,
        (
            "permissions:\n  contents: read",
            "python3 scripts/verify-windows-gnullvm-bazel-query-executable.py",
            "python3 scripts/verify-windows-gnullvm-setup-token-receipt-truth.py",
            legacy,
            direct_query,
            '"schema": "hepta_windows_gnullvm_qualification_boundary_v3"',
            f'"{BAZELISK_SHA256}"',
            f'"{BAZEL_SHA256}"',
            '"cached_bazel_cas_path_verified": True',
            '"direct_bazel_query_parser_executed": True',
            '"direct_bazel_rehashed_immediately_before_parser": True',
            '"source_writeback": False',
            '"runtime_authority": False',
            '"production_authority": False',
            '"operator_acceptance": False',
            '"promotion": False',
            '"release_authority": False',
            '"callers_ratchet": False',
        ),
        "Windows gnullvm qualification workflow",
    )
    require(workflow.count(direct_query) == 1, "direct query execution count drifted")
    require_order(workflow, setup, legacy, direct_query)
    for forbidden in (
        "contents: write",
        "pull-requests: write",
        "actions: write",
        "id-token: write",
    ):
        require(
            forbidden not in workflow,
            f"qualification workflow gained write authority: {forbidden}",
        )

    setup_workflow = read(SETUP_WORKFLOW)
    require_tokens(
        setup_workflow,
        (
            "python3 scripts/verify-windows-gnullvm-setup-token-receipt-truth.py",
            '"strict_step_parser_executed_before_setup_action": '
            'runner_os == "Linux"',
            '"cross_platform_verifier_executed_before_setup_action": True',
        ),
        "setup-token matrix workflow",
    )


def validate_increment() -> None:
    require(
        blob(BLOB_CONTRACT) == BLOB_CONTRACT_SHA,
        "Q0.43 blob contract drifted",
    )
    for relative, expected in BLOBS.items():
        path = ROOT / relative
        require(path.is_file(), f"missing Q0.43 blob: {relative}")
        require(blob(path) == expected, f"Q0.43 blob drift: {relative}")
    for relative in EXECUTABLE:
        require_git_executable(relative)

    require(read(DIRECT_ADAPTER) == DIRECT, "Q0.43 direct adapter drifted")
    require(read(JOB_ADAPTER) == JOB, "Q0.43 job adapter drifted")
    require((ROOT / ".bazelversion").read_bytes() == b"9.0.0\n", "Bazel pin drifted")

    validate_receipt_git_mode()
    validate_direct_query()
    validate_boundary()
    validate_workflow()


def main(owner: str = "q043") -> None:
    require(
        owner in {"q043", "direct-bazel", "job-executable"},
        f"unknown Q0.43 owner {owner!r}",
    )
    patch_parent_contracts()
    q041.main("q041" if owner == "q043" else owner)
    validate_increment()
    suffix = owner.upper().replace("-", "_")
    print(f"PASS_WINDOWS_GNULLVM_Q0_43_{suffix}_SOURCE")


if __name__ == "__main__":
    main()
