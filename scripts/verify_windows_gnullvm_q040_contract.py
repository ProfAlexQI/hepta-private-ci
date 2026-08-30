"""Q0.40 additive ratchet for direct Bazel query parsing and receipt truth."""

from __future__ import annotations

import hashlib
import importlib.util
import stat
import subprocess
import sys
from pathlib import Path
from typing import Final

from hepta_q040_blob_contract import (
    BLOBS,
    DIRECT,
    EXECUTABLE,
    JOB,
    Q039_BLOB_OVERRIDES,
)


ROOT = Path(__file__).resolve().parents[1]
BLOB_CONTRACT = ROOT / "scripts" / "hepta_q040_blob_contract.py"
BLOB_CONTRACT_SHA: Final = "b6c19d669ebd08cb779392b0920b443a8bca46d9"
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
    require(path.is_file(), f"missing Q0.40 path: {relative}")
    return path.read_text(encoding="utf-8")


def blob(path: Path) -> str:
    data = path.read_bytes()
    return hashlib.sha1(
        f"blob {len(data)}\0".encode("ascii") + data,
        usedforsecurity=False,
    ).hexdigest()


def require_executable(relative: str) -> None:
    path = ROOT / relative
    require(path.stat().st_mode & stat.S_IXUSR, f"lost mode: {relative}")
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
        require(token in text, f"{owner} lacks Q0.40 token: {token}")


def load_module(relative: str, name: str):
    spec = importlib.util.spec_from_file_location(name, ROOT / relative)
    require(spec is not None and spec.loader is not None, f"cannot load {relative}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


def run_adapted_q039(owner: str) -> None:
    module = load_module(
        "scripts/verify_windows_gnullvm_q039_contract.py",
        "_hepta_q040_q039_base",
    )
    module.BLOBS = {**module.BLOBS, **Q039_BLOB_OVERRIDES}
    module.DIRECT = DIRECT
    module.JOB = JOB
    module.main(owner)


def validate_increment() -> None:
    require(blob(BLOB_CONTRACT) == BLOB_CONTRACT_SHA, "Q0.40 blob contract drifted")
    for relative, expected in BLOBS.items():
        path = ROOT / relative
        require(path.is_file(), f"missing Q0.40 blob: {relative}")
        require(blob(path) == expected, f"Q0.40 blob drift: {relative}")
    for relative in EXECUTABLE:
        require_executable(relative)

    require(
        read("scripts/verify-windows-gnullvm-direct-bazel.py") == DIRECT,
        "Q0.40 direct wrapper drifted",
    )
    require(
        read("scripts/verify-windows-gnullvm-job-executable.py") == JOB,
        "Q0.40 job wrapper drifted",
    )

    query = read("scripts/verify-windows-gnullvm-bazel-query-executable.py")
    require_tokens(
        query,
        (
            'BAZELISK_VERSION: Final = "1.28.1"',
            BAZELISK_LINUX_SHA256,
            BAZEL_LINUX_SHA256,
            "def resolve_verified_linux_bazel",
            '[str(bazelisk), "--print_env"]',
            "Bazelisk executable SHA-256 drifted",
            "Bazelisk executable changed during child resolution",
            "cached Bazel executable SHA-256 drifted",
            "content-addressed layout",
            "def _validate_direct_bazel",
            "direct Bazel executable changed before parser launch",
            "def _validate_direct_environment",
            "BASE.parser_smoke_command",
            "BASE.validate_source()",
        ),
        "query executable verifier",
    )
    query_tests = read(".github/scripts/test_run_bazel_query_executable.py")
    require_tokens(
        query_tests,
        (
            "test_resolver_and_direct_launch_contract",
            "test_bazelisk_and_child_identity_fail_closed",
            "test_bazelisk_mutation_and_direct_execution",
        ),
        "query executable tests",
    )

    workflow = read(".github/workflows/windows-gnullvm-qualification-boundary.yml")
    legacy = "python3 scripts/verify-windows-gnullvm-bazel-query-vector.py --execute"
    direct = (
        "python3 scripts/verify-windows-gnullvm-bazel-query-executable.py --execute"
    )
    require_tokens(
        workflow,
        (
            "Execute legacy and verified direct Bazel 9 query parser smoke",
            legacy,
            direct,
            '"bazelisk_version": "1.28.1"',
            BAZELISK_LINUX_SHA256,
            BAZEL_LINUX_SHA256,
            '"bazelisk_executable_verified": True',
            '"cached_bazel_cas_path_verified": True',
            '"cached_bazel_sha256_verified": True',
            '"direct_bazel_query_parser_executed": True',
            '"direct_bazel_rehashed_immediately_before_parser": True',
            '"cached_bazel_executed_on_this_linux_source_job": True',
        ),
        "qualification workflow",
    )
    setup = "uses: ./.github/actions/setup-bazel-ci"
    require(
        workflow.index(setup) < workflow.index(legacy) < workflow.index(direct),
        "query parser execution ordering drifted",
    )
    require(
        workflow.count(legacy) == 1 and workflow.count(direct) == 1,
        "query parser implementations must each execute exactly once",
    )

    setup_workflow = read(
        ".github/workflows/windows-setup-bazel-token-boundary.yml"
    )
    require_tokens(
        setup_workflow,
        (
            "if: runner.os == 'Linux'",
            "verify-windows-gnullvm-setup-token-receipt-truth.py",
            '"strict_step_parser_executed_before_setup_action": runner_os == "Linux"',
            '"cross_platform_verifier_executed_before_setup_action": True',
        ),
        "setup-token workflow",
    )
    truth = read("scripts/verify-windows-gnullvm-setup-token-receipt-truth.py")
    require_tokens(
        truth,
        (
            "FALSE_POSITIVE_RECEIPT_FIELD",
            "prove_false_positive_rejected",
            "prove_wrong_scope_rejected",
            "prove_truth_gate_order_rejected",
        ),
        "receipt-truth verifier",
    )

    fixture = read(".github/scripts/test_run_bazel_qualification_boundary.sh")
    for command in (
        "python3 .github/scripts/test_run_bazel_query_executable.py",
        "python3 scripts/verify-windows-gnullvm-bazel-query-executable.py",
        "python3 scripts/verify-windows-gnullvm-setup-token-receipt-truth.py",
    ):
        require(
            fixture.count(command) == 1,
            f"qualification fixture must execute once: {command}",
        )


def main(owner: str = "q040") -> None:
    require(
        owner in {"q040", "direct-bazel", "job-executable"},
        f"unknown Q0.40 owner {owner!r}",
    )
    if owner == "q040":
        run_adapted_q039("job-executable")
        run_adapted_q039("direct-bazel")
    else:
        run_adapted_q039(owner)
    validate_increment()
    label = owner.upper().replace("-", "_")
    print(f"PASS_WINDOWS_GNULLVM_Q0_40_{label}_SOURCE")


if __name__ == "__main__":
    main()
