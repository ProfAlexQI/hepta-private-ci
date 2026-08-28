#!/usr/bin/env python3
"""Static verifier for Worker source/API topology acceptance pointer v1."""
from __future__ import annotations

import json
import pathlib
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
TOOL = ROOT / "scripts/hepta-servo-worker-source-topology-acceptance-pointer-v1.py"
PART_ROOT = ROOT / "scripts/hepta-servo-worker-source-topology-acceptance-v1"
PARTS = tuple(PART_ROOT / f"part{index:02d}.pyinc" for index in range(1, 6))
TEST = ROOT / "scripts/tests/test_hepta_servo_worker_source_topology_acceptance_pointer_v1.py"
TEST_PART_ROOT = ROOT / "scripts/tests/hepta_servo_worker_source_topology_acceptance_v1"
TEST_PARTS = tuple(TEST_PART_ROOT / f"part{index:02d}.pyinc" for index in range(1, 4))
POLICY = ROOT / "docs/hepta-vnext/browser/WORKER_SOURCE_TOPOLOGY_ACCEPTANCE_REVIEW_POLICY_V1.json"
POLICY_SCHEMA = ROOT / "docs/hepta-vnext/browser/hepta.servo.worker_source_topology_acceptance_review_policy.v1.schema.json"
CHALLENGE_SCHEMA = ROOT / "docs/hepta-vnext/browser/hepta.servo.worker_source_topology_acceptance_review_challenge.v1.schema.json"
POINTER_SCHEMA = ROOT / "docs/hepta-vnext/browser/hepta.servo.accepted_worker_source_topology_pointer.v1.schema.json"
STATUS = ROOT / "docs/hepta-vnext/browser/C1_WORKER_SOURCE_TOPOLOGY_ACCEPTANCE_POINTER_V1_STATUS.json"
CONTRACT = ROOT / ".github/workflows/hepta-servo-worker-source-topology-acceptance-pointer-v1-contract.yml"
LIVE = ROOT / ".github/workflows/hepta-servo-worker-source-topology-acceptance-live-review-v1.yml"
SOURCE_LIVE = ROOT / ".github/workflows/hepta-servo-exact-source-acceptance-live-review-v2.yml"
AGGREGATE = ROOT / ".github/workflows/hepta-browser-next-required-v9.yml"
CONTEXT_POLICY = ROOT / "docs/hepta-vnext/browser/CI_REQUIRED_CONTEXTS_V1.json"
CONTEXT_VERIFIER = ROOT / "scripts/verify-hepta-required-contexts.py"
CURRENT = ROOT / "docs/hepta-vnext/browser/CURRENT.yaml"
C1_CURRENT = ROOT / "docs/hepta-vnext/browser/C1_CURRENT_V7.json"
README = ROOT / "docs/hepta-vnext/browser/README.md"


def fail(message: str) -> None:
    raise RuntimeError(message)


def require(text: str, *tokens: str) -> None:
    for token in tokens:
        if token not in text:
            fail(f"missing token {token!r}")


def canonical(value: object) -> bytes:
    return json.dumps(
        value,
        sort_keys=True,
        separators=(",", ":"),
        ensure_ascii=False,
    ).encode("utf-8")


def run_json(path: pathlib.Path, *arguments: str) -> dict[str, object]:
    result = subprocess.run(
        [sys.executable, str(path), *arguments],
        cwd=ROOT,
        capture_output=True,
        text=True,
        timeout=90,
        check=False,
    )
    if result.returncode != 0:
        fail((result.stderr or result.stdout)[-1200:])
    return json.loads(result.stdout.strip().splitlines()[-1])


def main() -> int:
    try:
        paths = (
            TOOL,
            *PARTS,
            TEST,
            *TEST_PARTS,
            POLICY,
            POLICY_SCHEMA,
            CHALLENGE_SCHEMA,
            POINTER_SCHEMA,
            STATUS,
            CONTRACT,
            LIVE,
            SOURCE_LIVE,
            AGGREGATE,
            CONTEXT_POLICY,
            CONTEXT_VERIFIER,
            CURRENT,
            C1_CURRENT,
            README,
        )
        for path in paths:
            if not path.is_file():
                fail(f"missing {path.relative_to(ROOT)}")

        loader = TOOL.read_text(encoding="utf-8")
        require(loader, "part{index:02d}.pyinc", "exec(compile(")
        tool = "".join(path.read_text(encoding="utf-8") for path in PARTS)
        test_loader = TEST.read_text(encoding="utf-8")
        require(test_loader, "part{index:02d}.pyinc", "exec(compile(")
        test = "".join(path.read_text(encoding="utf-8") for path in TEST_PARTS)
        contract = CONTRACT.read_text(encoding="utf-8")
        live = LIVE.read_text(encoding="utf-8")
        source_live = SOURCE_LIVE.read_text(encoding="utf-8")
        aggregate = AGGREGATE.read_text(encoding="utf-8")
        readme = README.read_text(encoding="utf-8")
        policy = json.loads(POLICY.read_text(encoding="utf-8"))
        status = json.loads(STATUS.read_text(encoding="utf-8"))
        current = json.loads(CURRENT.read_text(encoding="utf-8"))
        c1 = json.loads(C1_CURRENT.read_text(encoding="utf-8"))
        contexts = json.loads(CONTEXT_POLICY.read_text(encoding="utf-8"))

        for schema_path in (POLICY_SCHEMA, CHALLENGE_SCHEMA, POINTER_SCHEMA):
            value = json.loads(schema_path.read_text(encoding="utf-8"))
            if schema_path.read_bytes() != canonical(value):
                fail(f"schema is not compact canonical JSON: {schema_path.name}")

        require(
            tool,
            "validate_source_pointer",
            "validate_topology_receipt",
            "source_bundle_receipt_sha256",
            "compressed_archive_sha256",
            "servoshell_dependency",
            "webdriver_server_dependency",
            "REQUIRES_LIVE_APPROVAL_EVIDENCE",
            "pointer_creation_command",
            "WORKER_SOURCE_TOPOLOGY_ACCEPTED_BUILD_RECIPE_REVIEW_REQUIRED_BUILD_NOT_AUTHORIZED",
            "HEPTA_WORKER_TOPOLOGY_ACCEPT_V1 ",
        )
        for forbidden in (
            'add_parser("accept")',
            'add_parser("create-pointer")',
            "git push",
            "update_ref",
            'subprocess.run(["cargo"',
        ):
            if forbidden in tool:
                fail(f"topology acceptance tool contains forbidden surface {forbidden!r}")
        require(
            test,
            "test_unaccepted_source_pointer_is_rejected",
            "test_source_bundle_digest_drift_is_rejected",
            "test_archive_digest_drift_is_rejected",
            "test_servoshell_widening_is_rejected",
            "test_webdriver_widening_is_rejected",
            "test_pointer_build_authority_is_rejected",
            "test_self_approval_is_rejected",
            "test_stale_approval_is_rejected",
            "test_current_head_change_request_is_rejected",
            "test_unknown_changed_path_is_rejected",
            "test_hardlinked_evidence_is_rejected",
        )
        require(
            contract,
            "workflow_call:",
            "hepta-servo-worker-source-topology-acceptance-pointer-v1.py contract",
            "test_hepta_servo_worker_source_topology_acceptance_pointer_v1.py -v",
            "pointer_creation_command=false",
            "worker_source_topology_accepted=false",
            "build_authorized=false",
        )
        require(
            live,
            "pull_request_target:",
            "name: Worker source/API topology accepted pointer live review",
            "ref: ${{ github.event.pull_request.base.sha }}",
            "PR-head code executed: false",
            "PASS_LIVE_REVIEW_WORKER_SOURCE_TOPOLOGY_ONLY",
        )
        require(
            source_live,
            "pull_request_target:",
            "name: Source-only accepted pointer live review",
            "ref: ${{ github.event.pull_request.base.sha }}",
            "PR-head code executed: false",
        )
        for workflow in (source_live, live):
            if "ref: ${{ github.event.pull_request.head.sha }}" in workflow:
                fail("live review executes PR-head verifier code")

        require(
            aggregate,
            "pull_request:",
            "name: Hepta Browser next required v9",
            "reusable workflow nesting: blocking-ci -> v9 -> leaf",
            "uses: ./.github/workflows/hepta-browser-ci.yml",
            "uses: ./.github/workflows/hepta-servo-independent-source-contract-v3.yml",
            "uses: ./.github/workflows/hepta-servo-exact-source-review-candidate-v2-contract.yml",
            "uses: ./.github/workflows/hepta-servo-worker-source-topology-contract.yml",
            "uses: ./.github/workflows/hepta-servo-exact-source-acceptance-pointer-v1-contract.yml",
            "uses: ./.github/workflows/hepta-servo-worker-source-topology-acceptance-pointer-v1-contract.yml",
            "uses: ./.github/workflows/hepta-servo-build-input-contract-v3.yml",
            "uses: ./.github/workflows/hepta-servo-build-preflight-contract.yml",
            '"worker_source_topology_accepted": False',
            '"build_authorized": False',
            '"servo_runtime_qualified": False',
        )
        if "uses: ./.github/workflows/hepta-browser-next-required-v8.yml" in aggregate:
            fail("canonical v9 is nested through obsolete v8")
        expected_context = {
            "check_name": "Hepta Browser next required v9",
            "purpose": "canonical WEB-C1 source, review, topology, build-input and preflight graph",
            "required": True,
            "workflow": ".github/workflows/hepta-browser-next-required-v9.yml",
        }
        if expected_context not in contexts.get("contexts", []):
            fail("required-context policy omits independent Browser v9 check")
        context_summary = run_json(CONTEXT_VERIFIER)
        if context_summary.get("status") != "PASS_VERSION_CONTROLLED_CONTEXT_CONTRACT":
            fail("required-context verifier status drifted")

        if policy.get("claims_after_acceptance", {}).get(
            "worker_source_topology_accepted"
        ) is not True:
            fail("topology policy does not describe topology-only acceptance")
        for key in ("build_authorized", "servo_built", "servo_runtime_qualified"):
            if policy.get("claims_after_acceptance", {}).get(key) is not False:
                fail(f"topology policy overclaims {key}")
        if any(value is not False for value in policy.get("authority", {}).values()):
            fail("topology policy authority posture is open")
        if status.get("evidence", {}).get("local_fixture_tests") != "29_PASS":
            fail("topology acceptance status does not record 29 fixture passes")
        for key in (
            "exact_servo_source_accepted",
            "worker_source_topology_accepted",
            "build_authorized",
        ):
            if status.get("claims", {}).get(key) is not False:
                fail(f"topology acceptance status overclaims {key}")

        if current.get("c1_current") != "docs/hepta-vnext/browser/C1_CURRENT_V7.json":
            fail("root CURRENT does not select C1 v7")
        if current.get("canonical_aggregate_workflow") != (
            ".github/workflows/hepta-browser-next-required-v9.yml"
        ):
            fail("root CURRENT does not select v9")
        if c1.get("canonical_aggregate_workflow") != (
            ".github/workflows/hepta-browser-next-required-v9.yml"
        ):
            fail("C1 v7 does not select v9")
        if c1.get("claims", {}).get("worker_source_topology_accepted") is not False:
            fail("C1 v7 overclaims topology acceptance")
        if c1.get("claims", {}).get("build_authorized") is not False:
            fail("C1 v7 overclaims build authority")
        if "Canonical C1 pointer: `C1_CURRENT_V7.json`" not in readme:
            fail("README does not identify C1 v7")

        summary = run_json(TOOL, "contract", "--policy", str(POLICY.resolve()))
        if summary.get("status") != "PASS_CONTRACT_ONLY":
            fail("topology acceptance contract status drifted")
        if summary.get("pointer_creation_command") is not False:
            fail("topology acceptance contract creates pointers")
    except (
        OSError,
        RuntimeError,
        UnicodeError,
        json.JSONDecodeError,
        subprocess.TimeoutExpired,
    ) as error:
        print(
            f"HEPTA_SERVO_WORKER_SOURCE_TOPOLOGY_ACCEPTANCE_STATIC=FAIL: {error}",
            file=sys.stderr,
        )
        return 1

    print(
        json.dumps(
            {
                "status": "HEPTA_SERVO_WORKER_SOURCE_TOPOLOGY_ACCEPTANCE_STATIC_PASS",
                "fixture_tests": "29_PASS",
                "independent_required_context": True,
                "trusted_base_source_review": True,
                "trusted_base_topology_review": True,
                "pointer_creation_command": False,
                "exact_servo_source_accepted": False,
                "worker_source_topology_accepted": False,
                "build_authorized": False,
                "servo_built": False,
                "servo_runtime_qualified": False,
                "authority": "all_false",
            },
            sort_keys=True,
            separators=(",", ":"),
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
