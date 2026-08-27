#!/usr/bin/env python3
"""Verify that every focused Hepta Browser slice is a real CI-required gate."""
from __future__ import annotations

import json
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
BLOCKING = ROOT / ".github/workflows/blocking-ci.yml"
CARGO = ROOT / "tools/hepta-browser-c1-protocol/Cargo.toml"
OWNERS = ROOT / ".github/CODEOWNERS"
WORKFLOWS = {
    "hepta-browser-c0-c3": ".github/workflows/hepta-browser-c0-c3.yml",
    "hepta-browser-c1-protocol": ".github/workflows/hepta-browser-c1-protocol.yml",
    "hepta-servo-artifact-contract": ".github/workflows/hepta-servo-artifact-contract.yml",
    "hepta-servo-build-manifest-contract": ".github/workflows/hepta-servo-build-manifest-contract.yml",
    "hepta-servo-source-bundle-contract": ".github/workflows/hepta-servo-source-bundle-contract.yml",
    "hepta-servo-source-contract": ".github/workflows/hepta-servo-source-contract.yml",
    "hepta-vnext": ".github/workflows/hepta-vnext-qualification.yml",
}


def fail(message: str) -> None:
    raise RuntimeError(message)


def main() -> int:
    try:
        for path in (BLOCKING, CARGO, OWNERS):
            if not path.is_file(): fail(f"missing {path.relative_to(ROOT)}")
        blocking = BLOCKING.read_text(); owners = OWNERS.read_text(); cargo = CARGO.read_text()
        if "continue-on-error" in blocking: fail("blocking CI may not weaken focused gates")
        for job, relative in WORKFLOWS.items():
            path = ROOT / relative
            if not path.is_file(): fail(f"missing reusable workflow {relative}")
            workflow = path.read_text()
            if "workflow_call:" not in workflow: fail(f"{relative} is not reusable")
            for token in (f"  {job}:\n", f"uses: ./{relative}", f"      - {job}\n"):
                if token not in blocking: fail(f"blocking CI does not require {job}: missing {token!r}")
        for token in (
            'autobins = false', 'name = "hepta-browser-c1-process-trial"',
            'path = "src/bin/hepta-browser-c1-process-trial.rs"',
        ):
            if token not in cargo: fail(f"C1 Cargo manifest is missing {token}")
        for token in (
            "/tools/hepta-browser-c1-protocol/ @ProfAlexQI",
            "/scripts/hepta-servo-*.py @ProfAlexQI",
            "/scripts/tests/test_hepta_servo_*.py @ProfAlexQI",
            "/.github/workflows/hepta-*.yml @ProfAlexQI",
        ):
            if token not in owners: fail(f"CODEOWNERS is missing {token}")
    except (OSError, RuntimeError) as error:
        print(f"HEPTA_FOCUSED_GATES=FAIL: {error}", file=sys.stderr); return 1
    print(json.dumps({"focused_gates": sorted(WORKFLOWS), "status": "HEPTA_FOCUSED_GATES_PASS"}, sort_keys=True, separators=(",", ":")))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
