#!/usr/bin/env python3
"""Fail closed if setup-token receipts overclaim row-specific source execution."""

from __future__ import annotations

import stat
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
WORKFLOW = (
    ROOT
    / ".github"
    / "workflows"
    / "windows-setup-bazel-token-boundary.yml"
)

STRICT_STEP = (
    "      - name: Verify strict setup-token source boundary\n"
    "        if: runner.os == 'Linux'\n"
    "        shell: bash\n"
    "        run: python3 scripts/verify-windows-gnullvm-setup-token-boundary.py\n"
)
CROSS_PLATFORM_STEP = (
    "      - name: Verify cross-platform setup-token boundary\n"
    "        shell: bash\n"
    "        run: |\n"
    "          python3 scripts/verify-windows-gnullvm-setup-token-cross-platform.py\n"
    "          python3 scripts/verify-windows-gnullvm-setup-token-receipt-truth.py\n"
)
SETUP_ACTION = "      - name: Exercise pinned setup-bazel composite action\n"
TRUTHFUL_RECEIPT_FIELD = (
    '              "strict_step_parser_executed_before_setup_action": '
    'runner_os == "Linux",\n'
)
FALSE_POSITIVE_RECEIPT_FIELD = (
    '              "strict_step_parser_executed_before_setup_action": True,\n'
)


def fail(message: str) -> None:
    raise SystemExit(message)


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def require_order(text: str, *tokens: str) -> None:
    positions = []
    for token in tokens:
        require(token in text, f"missing ordered token: {token!r}")
        positions.append(text.index(token))
    require(
        positions == sorted(positions) and len(positions) == len(set(positions)),
        f"invalid or ambiguous order for tokens: {tokens!r}",
    )


def validate(text: str) -> None:
    require(
        text.count(STRICT_STEP) == 1,
        "Linux-only strict source-parser step must be present exactly once",
    )
    require(
        text.count(CROSS_PLATFORM_STEP) == 1,
        "cross-platform truth verifier step must be present exactly once",
    )
    require(
        text.count(TRUTHFUL_RECEIPT_FIELD) == 1,
        "receipt must derive strict-parser execution from runner_os == Linux",
    )
    require(
        FALSE_POSITIVE_RECEIPT_FIELD not in text,
        "receipt must not claim the Linux-only strict parser executed on every row",
    )
    require(
        text.count(
            '"cross_platform_verifier_executed_before_setup_action": True,'
        )
        == 1,
        "cross-platform verifier execution claim must remain exact",
    )
    require(
        text.count('"runner_os": runner_os,') == 1,
        "receipt must bind the runner_os used by the row-specific claim",
    )
    require_order(
        text,
        STRICT_STEP,
        CROSS_PLATFORM_STEP,
        SETUP_ACTION,
        TRUTHFUL_RECEIPT_FIELD,
    )


def prove_false_positive_rejected(text: str) -> None:
    mutated = text.replace(
        TRUTHFUL_RECEIPT_FIELD,
        FALSE_POSITIVE_RECEIPT_FIELD,
        1,
    )
    try:
        validate(mutated)
    except SystemExit:
        return
    fail("validator accepted a hard-coded strict-parser success claim")


def prove_wrong_scope_rejected(text: str) -> None:
    mutated = text.replace(
        "        if: runner.os == 'Linux'\n",
        "        if: runner.os == 'Windows'\n",
        1,
    )
    try:
        validate(mutated)
    except SystemExit:
        return
    fail("validator accepted a strict parser scoped to the wrong matrix row")


def prove_truth_gate_order_rejected(text: str) -> None:
    without = text.replace(CROSS_PLATFORM_STEP, "", 1)
    moved = without.replace(
        SETUP_ACTION,
        SETUP_ACTION + CROSS_PLATFORM_STEP,
        1,
    )
    try:
        validate(moved)
    except SystemExit:
        return
    fail("validator accepted receipt-truth verification after setup execution")


def main() -> None:
    require(WORKFLOW.is_file(), f"missing workflow: {WORKFLOW.relative_to(ROOT)}")
    require(
        Path(__file__).stat().st_mode & stat.S_IXUSR != 0,
        "Q0.39 receipt-truth verifier lost executable mode",
    )
    text = WORKFLOW.read_text(encoding="utf-8")
    validate(text)
    prove_false_positive_rejected(text)
    prove_wrong_scope_rejected(text)
    prove_truth_gate_order_rejected(text)
    print("PASS_WINDOWS_GNULLVM_Q0_39_RECEIPT_STEP_TRUTH_SOURCE")


if __name__ == "__main__":
    main()
