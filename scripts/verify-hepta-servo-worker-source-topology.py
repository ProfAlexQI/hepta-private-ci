#!/usr/bin/env python3
"""Static merge-gate verifier for the WEB-C1 worker source/API topology."""
from __future__ import annotations

import importlib.util
import json
import pathlib
import sys
from types import ModuleType

ROOT = pathlib.Path(__file__).resolve().parents[1]
TOOL = ROOT / "scripts/hepta-servo-worker-source-topology.py"
TEST = ROOT / "scripts/tests/test_hepta_servo_worker_source_topology.py"
TOPOLOGY = ROOT / "docs/hepta-vnext/browser/SERVO_WORKER_SOURCE_TOPOLOGY_V1.json"
TOPOLOGY_SCHEMA = (
    ROOT / "docs/hepta-vnext/browser/hepta.servo.worker_source_topology.v1.schema.json"
)
RECEIPT_SCHEMA = (
    ROOT
    / "docs/hepta-vnext/browser/"
    "hepta.servo.worker_source_topology_verification.v1.schema.json"
)
ADR = ROOT / "docs/hepta-vnext/browser/ADR-0003-hepta-owned-servo-embedder.md"
DOC = ROOT / "docs/hepta-vnext/browser/C1_WORKER_SOURCE_TOPOLOGY.md"
STATUS = ROOT / "docs/hepta-vnext/browser/C1_WORKER_SOURCE_TOPOLOGY_STATUS.json"
CURRENT = ROOT / "docs/hepta-vnext/browser/C1_CURRENT_V4.json"
WORKFLOW = ROOT / ".github/workflows/hepta-servo-worker-source-topology-contract.yml"
AGGREGATE = ROOT / ".github/workflows/hepta-browser-next-required-v6.yml"


def fail(message: str) -> None:
    raise RuntimeError(message)


def load_tool() -> ModuleType:
    specification = importlib.util.spec_from_file_location(
        "hepta_servo_worker_source_topology_contract",
        TOOL,
    )
    if specification is None or specification.loader is None:
        fail("cannot load worker source-topology tool")
    module = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(module)
    return module


def load_canonical(path: pathlib.Path, label: str) -> dict[str, object]:
    raw = path.read_bytes()
    value = json.loads(raw)
    if not isinstance(value, dict):
        fail(f"{label} must be one JSON object")
    canonical = json.dumps(
        value,
        ensure_ascii=False,
        sort_keys=True,
        separators=(",", ":"),
    ).encode()
    if raw != canonical:
        fail(f"{label} is not compact canonical JSON")
    return value


def main() -> int:
    try:
        for path in (
            TOOL,
            TEST,
            TOPOLOGY,
            TOPOLOGY_SCHEMA,
            RECEIPT_SCHEMA,
            ADR,
            DOC,
            STATUS,
            CURRENT,
            WORKFLOW,
            AGGREGATE,
        ):
            if not path.is_file():
                fail(f"missing {path.relative_to(ROOT)}")

        tool = load_tool()
        topology = load_canonical(TOPOLOGY, "worker source topology")
        tool.validate_topology(topology)

        tool_source = TOOL.read_text(encoding="utf-8")
        for token in (
            "load_source_verifier",
            "source_verifier.verify_bundle",
            "git_blob_id",
            "scan_compressed_archive",
            "selected_git_blobs_match",
            "servoshell_conflict_anchors_match",
            "webdriver_wildcard_listener_anchor_match",
            "HEPTA_OWNED_EMBEDDER_TOPOLOGY_VERIFIED_BUILD_NOT_AUTHORIZED",
            "servoshell_build_root",
            "webdriver_server_dependency",
            "servo_built",
            "worker_artifact_created",
            "runtime_authority",
        ):
            if token not in tool_source:
                fail(f"worker source-topology tool is missing {token}")
        for forbidden in (
            "urllib",
            "requests",
            "http.client",
            "socket.socket",
            "subprocess.run",
            "os.system",
            "Popen(",
        ):
            if forbidden in tool_source:
                fail(f"worker source-topology tool contains dynamic surface {forbidden}")

        tests = TEST.read_text(encoding="utf-8")
        for name in (
            "test_canonical_topology_is_self_bound_and_closed",
            "test_exact_projection_scans_selected_and_reference_files",
            "test_missing_required_anchor_fails_closed",
            "test_blob_drift_fails_closed",
            "test_required_symlink_fails_closed",
            "test_any_hardlink_fails_closed",
            "test_duplicate_json_keys_fail_closed",
            "test_output_is_create_only_and_private",
            "test_servoshell_conflicts_are_explicitly_frozen",
            "test_contract_command_never_claims_source_or_runtime",
        ):
            if f"def {name}" not in tests:
                fail(f"worker source-topology tests are missing {name}")

        topology_schema = load_canonical(TOPOLOGY_SCHEMA, "topology schema")
        receipt_schema = load_canonical(RECEIPT_SCHEMA, "topology receipt schema")
        if topology_schema.get("$id") != tool.TOPOLOGY_SCHEMA:
            fail("topology schema ID drifted")
        if receipt_schema.get("$id") != tool.RECEIPT_SCHEMA:
            fail("topology verification receipt schema ID drifted")
        for schema, label in (
            (topology_schema, "topology schema"),
            (receipt_schema, "topology receipt schema"),
        ):
            authority = schema.get("$defs", {}).get("authority", {})
            properties = authority.get("properties", {})
            if set(properties) != set(tool.AUTHORITY):
                fail(f"{label} authority fields drifted")
            if any(value != {"const": False} for value in properties.values()):
                fail(f"{label} enables authority")

        status = load_canonical(STATUS, "topology status")
        if status.get("state") != (
            "CONTRACT_IMPLEMENTED_LOCAL_FIXTURES_PASS_EXACT_SOURCE_EVIDENCE_PENDING"
        ):
            fail("topology status overclaims or drifted")
        if status.get("authority") != tool.AUTHORITY:
            fail("topology status authority is open")
        if status.get("merge_authorized") is not False:
            fail("topology status authorized merge")

        current = load_canonical(CURRENT, "C1 current v4")
        if current.get("schema") != "hepta.browser.c1_current.v4":
            fail("C1 current v4 schema drifted")
        if current.get("canonical_worker_source_topology") != (
            "docs/hepta-vnext/browser/SERVO_WORKER_SOURCE_TOPOLOGY_V1.json"
        ):
            fail("C1 current v4 does not select the canonical topology")
        if current.get("canonical_worker_source_topology_tool") != (
            "scripts/hepta-servo-worker-source-topology.py"
        ):
            fail("C1 current v4 does not select the canonical topology tool")
        if current.get("canonical_aggregate_workflow") != (
            ".github/workflows/hepta-browser-next-required-v6.yml"
        ):
            fail("C1 current v4 does not select aggregate v6")
        if current.get("claims", {}).get("worker_source_topology_accepted") is not False:
            fail("C1 current v4 overclaims accepted source topology")
        if any(value is not False for value in current.get("authority", {}).values()):
            fail("C1 current v4 enables authority")

        adr = ADR.read_text(encoding="utf-8")
        for token in (
            "out-of-tree Hepta-owned worker crate",
            "unconditionally enables `background_hang_monitor`,",
            "`webdriver_server`",
            "`components/servo`",
            "Option<WebView>",
            "Reuse the upstream WebDriver server on loopback",
        ):
            if token not in adr:
                fail(f"embedder ADR is missing {token}")

        workflow = WORKFLOW.read_text(encoding="utf-8")
        for token in (
            "workflow_call:",
            "py_compile",
            "hepta-servo-worker-source-topology.py contract",
            "test_hepta_servo_worker_source_topology.py",
            "verify-hepta-servo-worker-source-topology.py",
            "servo_built=false",
            "runtime_authority=false",
            "operator_acceptance=false",
            "release_qualified=false",
        ):
            if token not in workflow:
                fail(f"source-topology workflow is missing {token}")

        aggregate = AGGREGATE.read_text(encoding="utf-8")
        for token in (
            "source-api-topology:",
            "hepta-servo-worker-source-topology-contract.yml",
            "- source-api-topology",
            "source_api_topology",
            "hepta_owned_embedder_v1",
            "worker_source_topology_accepted",
        ):
            if token not in aggregate:
                fail(f"aggregate v6 is missing {token}")
    except (OSError, UnicodeError, json.JSONDecodeError, RuntimeError) as error:
        print(
            f"HEPTA_SERVO_WORKER_SOURCE_TOPOLOGY_CONTRACT=FAIL: {error}",
            file=sys.stderr,
        )
        return 1

    print(
        json.dumps(
            {
                "schema": "hepta.servo.worker_source_topology_contract_verification.v1",
                "status": "PASS_CONTRACT_AND_LOCAL_FIXTURES_ONLY",
                "embedder_strategy": (
                    "out_of_tree_hepta_worker_using_public_servo_embedding_api"
                ),
                "servoshell_build_root": False,
                "webdriver_server_dependency": False,
                "exact_source_topology_receipt": False,
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
