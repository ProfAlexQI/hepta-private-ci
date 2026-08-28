#!/usr/bin/env python3
"""Static merge verifier for exact-source review candidate v2."""
from __future__ import annotations

import json
import pathlib
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
TOOL = ROOT / "scripts/hepta-servo-exact-source-review-candidate-v2.py"
TEST = ROOT / "scripts/tests/test_hepta_servo_exact_source_review_candidate_v2.py"
SCHEMA = ROOT / "docs/hepta-vnext/browser/hepta.servo.exact_source_review_candidate.v2.schema.json"
SPEC = ROOT / "docs/hepta-vnext/browser/C1_EXACT_SOURCE_REVIEW_CANDIDATE_V2.md"
STATUS = ROOT / "docs/hepta-vnext/browser/C1_EXACT_SOURCE_REVIEW_CANDIDATE_V2_STATUS.json"
WORKFLOW = ROOT / ".github/workflows/hepta-servo-exact-source-review-candidate-v2-contract.yml"
SOURCE_WORKFLOW = ROOT / ".github/workflows/hepta-servo-independent-source-qualification-v3.yml"


def fail(message: str) -> None:
    raise RuntimeError(message)


def main() -> int:
    try:
        for path in (TOOL, TEST, SCHEMA, SPEC, STATUS, WORKFLOW, SOURCE_WORKFLOW):
            if not path.is_file():
                fail(f"missing {path.relative_to(ROOT)}")
        tool = TOOL.read_text(encoding="utf-8")
        test = TEST.read_text(encoding="utf-8")
        workflow = WORKFLOW.read_text(encoding="utf-8")
        source_workflow = SOURCE_WORKFLOW.read_text(encoding="utf-8")
        spec = SPEC.read_text(encoding="utf-8")
        status = json.loads(STATUS.read_text(encoding="utf-8"))
        schema = json.loads(SCHEMA.read_text(encoding="utf-8"))
        if SCHEMA.read_bytes() != json.dumps(
            schema, sort_keys=True, separators=(",", ":"), ensure_ascii=False
        ).encode("utf-8"):
            fail("candidate schema is not compact canonical JSON")
        for token in (
            "workflow_run_projection",
            "workflow_jobs_projection",
            "workflow_artifacts_projection",
            "parse_checksums",
            "run_offline_verifier",
            "PENDING_SEPARATE_REVIEW",
            "candidate_accepted",
            "pointer_update_performed",
            "build_authorized",
            "runner_id",
            "required workflow steps",
            "artifact names",
            "EVIDENCE_COMPLETE_REVIEW_REQUIRED_BUILD_NOT_AUTHORIZED",
        ):
            if token not in tool:
                fail(f"candidate compiler is missing {token}")
        for token in (
            "test_zero_steps_are_rejected",
            "test_runner_id_zero_is_rejected",
            "test_expired_artifact_is_rejected",
            "test_absolute_checksum_paths_are_rejected",
            "test_unsorted_checksum_names_are_rejected",
            "test_duplicate_json_key_is_rejected",
            "test_hardlinked_source_file_is_rejected",
        ):
            if token not in test:
                fail(f"candidate fixture suite is missing {token}")
        for token in (
            "workflow_call:",
            "scripts/hepta-servo-exact-source-review-candidate-v2.py contract",
            "scripts/tests/test_hepta_servo_exact_source_review_candidate_v2.py -v",
            "scripts/verify-hepta-servo-exact-source-review-candidate-v2.py",
            "exact_servo_source_accepted=false",
            "build_authorized=false",
            "servo_built=false",
            "servo_runtime_qualified=false",
        ):
            if token not in workflow:
                fail(f"candidate workflow is missing {token}")
        if 'sha256sum "$output_dir"/*' in source_workflow:
            fail("exact-source workflow still emits runner absolute paths in SHA256SUMS")
        for token in (
            'for path in sorted(root.iterdir(), key=lambda item: item.name.encode("utf-8")):',
            'lines.append(f"{digest.hexdigest()}  {path.name}")',
            "SHA256SUMS",
        ):
            if token not in source_workflow:
                fail(f"exact-source workflow portable checksum step is missing {token}")
        if status.get("status") != "IMPLEMENTED_FIXTURE_ONLY_HOSTED_EVIDENCE_BLOCKED":
            fail("candidate status overclaims")
        if status.get("claims", {}).get("exact_servo_source_accepted") is not False:
            fail("candidate status accepted source without review")
        if any(value is not False for value in status.get("authority", {}).values()):
            fail("candidate status authority posture is open")
        if "Workflow success is necessary but not sufficient" not in spec:
            fail("candidate spec does not preserve separate review")
        result = subprocess.run(
            [sys.executable, str(TOOL), "contract"],
            cwd=ROOT,
            capture_output=True,
            text=True,
            timeout=30,
            check=False,
        )
        if result.returncode != 0:
            fail(f"candidate contract command failed: {(result.stderr or result.stdout)[-1000:]}")
        summary = json.loads(result.stdout)
        if summary.get("status") != "PASS_CONTRACT_ONLY":
            fail("candidate contract summary drifted")
    except (OSError, RuntimeError, json.JSONDecodeError, subprocess.TimeoutExpired) as error:
        print(f"HEPTA_SERVO_EXACT_SOURCE_REVIEW_CANDIDATE_V2_STATIC=FAIL: {error}", file=sys.stderr)
        return 1
    print(
        json.dumps(
            {
                "status": "HEPTA_SERVO_EXACT_SOURCE_REVIEW_CANDIDATE_V2_STATIC_PASS",
                "fixture_only": True,
                "exact_servo_source_accepted": False,
                "build_authorized": False,
                "authority": "all_false",
            },
            sort_keys=True,
            separators=(",", ":"),
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
