#!/usr/bin/env python3
"""Q0.40 composition of the three complementary Q0.39 source closures."""

from __future__ import annotations

import hashlib
import stat
import subprocess
from pathlib import Path
from typing import Final

import verify_windows_gnullvm_q039_contract as q039


ROOT = Path(__file__).resolve().parents[1]
Q039_CONTRACT = ROOT / "scripts" / "verify_windows_gnullvm_q039_contract.py"
Q039_BLOB_CONTRACT = ROOT / "scripts" / "hepta_q039_blob_contract.py"

EXPECTED_Q039_CONTRACT_BLOB: Final = (
    "220cc4be505a78f27ec3e9be1e425efe9d0c9692"
)
EXPECTED_Q039_BLOB_CONTRACT_BLOB: Final = (
    "c0657cc4a3dd171f0c76fa6a61a78f2998834bed"
)

DIRECT = """#!/usr/bin/env python3

from verify_windows_gnullvm_q040_composition import main


if __name__ == "__main__":
    main("direct-bazel")
"""
JOB = DIRECT.replace('main("direct-bazel")', 'main("job-executable")')

DELTA_BLOBS: Final = {
    ".github/scripts/run_bazel_q039_startup_order.py": (
        "e0923474a529b37ef416ab9af90cc0745079afe5"
    ),
    ".github/scripts/run_bazel_with_buildbuddy.py": (
        "cf5e7d990e1c649dac505ff98199cffa60def08d"
    ),
    ".github/scripts/test_run_bazel_startup_contract.py": (
        "20a37801e00df21fda03011102fde499963892bc"
    ),
    ".github/workflows/windows-setup-bazel-token-boundary.yml": (
        "55a7cd88fe7692ea57e49c4cb585b5b1974686fd"
    ),
    "scripts/verify-windows-gnullvm-startup-contract.py": (
        "de42d9bb3fbfef16ad880e0ab05345ef3731dfbd"
    ),
    "scripts/verify-windows-gnullvm-setup-token-receipt-truth.py": (
        "53a729fb4795fcecaf611c79b039c5dae7c0a6dc"
    ),
    "scripts/verify-windows-gnullvm-direct-bazel.py": (
        "a8fdc4f7015786f159bc2ff5a821d9d7da7c7e57"
    ),
    "scripts/verify-windows-gnullvm-job-executable.py": (
        "e42507786ba98dcf61507840870cebbccaacf4c8"
    ),
}
DELTA_EXECUTABLE: Final = (
    ".github/scripts/test_run_bazel_startup_contract.py",
    "scripts/verify-windows-gnullvm-startup-contract.py",
    "scripts/verify-windows-gnullvm-setup-token-receipt-truth.py",
    "scripts/verify-windows-gnullvm-direct-bazel.py",
    "scripts/verify-windows-gnullvm-job-executable.py",
    "scripts/verify_windows_gnullvm_q040_composition.py",
)

_composed = False


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


def require_git_executable(relative: str) -> None:
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
        require(token in text, f"{owner} lacks Q0.40 token: {token}")


def require_order(text: str, before: str, after: str, owner: str) -> None:
    require_tokens(text, (before, after), owner)
    require(
        text.index(before) < text.index(after),
        f"{owner} must place {before!r} before {after!r}",
    )


def _compose_q039_base() -> None:
    global _composed
    if _composed:
        return

    require(
        blob(Q039_CONTRACT) == EXPECTED_Q039_CONTRACT_BLOB,
        "immutable Q0.39 query/YAML contract drifted",
    )
    require(
        blob(Q039_BLOB_CONTRACT) == EXPECTED_Q039_BLOB_CONTRACT_BLOB,
        "immutable Q0.39 blob contract drifted",
    )

    q039.BLOBS.update(DELTA_BLOBS)
    q039.EXECUTABLE = tuple(
        dict.fromkeys((*q039.EXECUTABLE, *DELTA_EXECUTABLE))
    )
    q039.DIRECT = DIRECT
    q039.JOB = JOB

    original_load_legacy = q039.load_legacy

    def load_legacy(relative: str, name: str):
        module = original_load_legacy(relative, name)
        if hasattr(module, "EXPECTED_WRAPPER_BLOB"):
            module.EXPECTED_WRAPPER_BLOB = DELTA_BLOBS[
                ".github/scripts/run_bazel_with_buildbuddy.py"
            ]
        return module

    q039.load_legacy = load_legacy
    _composed = True


def validate_startup_order() -> None:
    helper = read(".github/scripts/run_bazel_q039_startup_order.py")
    wrapper = read(".github/scripts/run_bazel_with_buildbuddy.py")
    test = read(".github/scripts/test_run_bazel_startup_contract.py")
    verifier = read("scripts/verify-windows-gnullvm-startup-contract.py")

    require_tokens(
        helper,
        (
            "def canonicalize_keyless_windows_gnullvm_base_startup",
            "exact startup vector requires exactly",
            "exact startup vector rejects unreviewed",
            "return [expected_output_root, DISABLED_REPO_CONTENTS_CACHE]",
        ),
        "Q0.39 startup-order helper",
    )
    require_tokens(
        wrapper,
        (
            "from run_bazel_q039_startup_order import (",
            "canonicalize_keyless_windows_gnullvm_base_startup",
            "startup = canonicalize_keyless_windows_gnullvm_base_startup(",
            "strict_rc = [",
            "validate_keyless_windows_gnullvm_final_args(command[1:], env)",
        ),
        "public Bazel wrapper",
    )
    require_order(
        wrapper,
        "startup = canonicalize_keyless_windows_gnullvm_base_startup(",
        "strict_rc = [",
        "public Bazel wrapper",
    )
    require_tokens(
        test,
        (
            "from run_bazel_q017_policy import _git_blob_sha1",
            "test_real_ci_explicit_output_user_root_is_canonicalized",
            "test_reversed_exact_base_startup_is_canonicalized",
            "test_duplicate_output_user_root_fails_closed",
            "test_startup_order_helper_rejects_unreviewed_options",
        ),
        "startup regression",
    )
    require_tokens(
        verifier,
        (
            "Q039_POLICY",
            "EXPECTED_Q039_POLICY_BLOB",
            "EXPECTED_WRAPPER_BLOB",
            "EXPECTED_STARTUP_TEST_BLOB",
            "canonicalize_keyless_windows_gnullvm_base_startup",
            "test_real_ci_explicit_output_user_root_is_canonicalized",
        ),
        "startup source verifier",
    )


def validate_receipt_truth() -> None:
    workflow = read(
        ".github/workflows/windows-setup-bazel-token-boundary.yml"
    )
    verifier = read(
        "scripts/verify-windows-gnullvm-setup-token-receipt-truth.py"
    )
    truthful = (
        '"strict_step_parser_executed_before_setup_action": '
        'runner_os == "Linux"'
    )
    false_claim = (
        '"strict_step_parser_executed_before_setup_action": True'
    )
    require_tokens(
        workflow,
        (
            "python3 scripts/verify-windows-gnullvm-setup-token-"
            "cross-platform.py",
            "python3 scripts/verify-windows-gnullvm-setup-token-"
            "receipt-truth.py",
            truthful,
            '"cross_platform_verifier_executed_before_setup_action": True',
        ),
        "setup-token workflow",
    )
    require(false_claim not in workflow, "setup-token receipt overclaims a row")
    require_order(
        workflow,
        "python3 scripts/verify-windows-gnullvm-setup-token-"
        "cross-platform.py",
        "python3 scripts/verify-windows-gnullvm-setup-token-"
        "receipt-truth.py",
        "setup-token workflow",
    )
    require_order(
        workflow,
        "python3 scripts/verify-windows-gnullvm-setup-token-"
        "receipt-truth.py",
        "uses: ./.github/actions/setup-bazel-ci",
        "setup-token workflow",
    )
    require_tokens(
        verifier,
        (
            "TRUTHFUL_RECEIPT_FIELD",
            "FALSE_POSITIVE_RECEIPT_FIELD",
            "prove_false_positive_rejected",
            "prove_wrong_scope_rejected",
            "prove_truth_gate_order_rejected",
            "PASS_WINDOWS_GNULLVM_Q0_39_RECEIPT_STEP_TRUTH_SOURCE",
        ),
        "receipt-truth verifier",
    )


def validate_q040_increment() -> None:
    for relative, expected in DELTA_BLOBS.items():
        path = ROOT / relative
        require(path.is_file(), f"missing Q0.40 delta blob: {relative}")
        require(
            blob(path) == expected,
            f"Q0.40 delta blob drift: {relative}",
        )
    for relative in DELTA_EXECUTABLE:
        require_git_executable(relative)

    require(
        read("scripts/verify-windows-gnullvm-direct-bazel.py") == DIRECT,
        "Q0.40 direct-Bazel adapter drifted",
    )
    require(
        read("scripts/verify-windows-gnullvm-job-executable.py") == JOB,
        "Q0.40 job-executable adapter drifted",
    )
    validate_startup_order()
    validate_receipt_truth()


def main(owner: str = "q040") -> None:
    require(
        owner in {"q040", "direct-bazel", "job-executable"},
        f"unknown Q0.40 owner {owner!r}",
    )
    _compose_q039_base()
    if owner == "q040":
        q039.run_legacy("job-executable")
        q039.run_legacy("direct-bazel")
    else:
        q039.run_legacy(owner)
    q039.validate_increment()
    validate_q040_increment()
    print(
        "PASS_WINDOWS_GNULLVM_Q0_40_"
        f"{owner.upper().replace('-', '_')}_SOURCE"
    )


if __name__ == "__main__":
    main()
