#!/usr/bin/env python3
"""Verify the repository-native Hepta browser plan and C1 safety scaffold.

The .yaml files intentionally contain JSON, which is valid YAML and keeps this
merge gate independent of third-party Python packages.
"""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path
from typing import Any, Iterable

ROOT = Path(__file__).resolve().parents[1]
BUNDLE = ROOT / "docs/hepta-vnext/browser"
CURRENT_PATH = BUNDLE / "CURRENT.yaml"
STAGE_PATH = BUNDLE / "STAGE_MATRIX.yaml"
TRACE_PATH = BUNDLE / "TRACEABILITY_MATRIX.yaml"
SERVO_PATH = BUNDLE / "SERVO_UPSTREAM_PIN.json"
TOPOLOGY_PATH = BUNDLE / "SERVO_SOURCE_IMPORT_TOPOLOGY.yaml"
PATCH_INVENTORY_PATH = ROOT / "third_party/servo-patches/PATCH_INVENTORY.json"
QUEUE_PATH = BUNDLE / "NEXT_WORK_QUEUE.yaml"
SCHEMA_PATH = BUNDLE / "hepta.browser.qualification_receipt.v1.schema.json"
SOURCE_SCHEMA_PATH = BUNDLE / "hepta.servo.source_receipt.v1.schema.json"
PLAN_PATH = BUNDLE / "EXECUTION_PLAN.md"
THREAT_PATH = BUNDLE / "THREAT_MODEL.md"
README_PATH = BUNDLE / "README.md"
PROVENANCE_DOC_PATH = BUNDLE / "SERVO_PROVENANCE.md"
PROVENANCE_GENERATOR_PATH = ROOT / "scripts/generate-hepta-servo-provenance.py"
PROVENANCE_TEST_PATH = ROOT / "scripts/test_generate_hepta_servo_provenance.py"
WORKFLOW_PATH = ROOT / ".github/workflows/hepta-vnext-qualification.yml"
OWNERS_PATH = ROOT / ".github/CODEOWNERS"
CARGO_PATH = ROOT / "codex-rs/hepta-shadow-qualification/Cargo.toml"
BAZEL_PATH = ROOT / "codex-rs/hepta-shadow-qualification/BUILD.bazel"
WORKER_PROTOCOL_PATH = ROOT / "codex-rs/hepta-shadow-qualification/src/browser_worker_protocol.rs"
WORKER_HARNESS_PATH = ROOT / "codex-rs/hepta-shadow-qualification/src/browser_worker_harness.rs"
WORKER_BINARY_PATH = (
    ROOT
    / "codex-rs/hepta-shadow-qualification/src/bin/hepta-browser-worker-qualification.rs"
)
WORKER_TEST_PATH = ROOT / "codex-rs/hepta-shadow-qualification/src/browser_worker_tests.rs"
WORKER_PROCESS_TEST_PATH = (
    ROOT / "codex-rs/hepta-shadow-qualification/tests/browser_worker_process.rs"
)
BROWSER_TEST_PATH = ROOT / "codex-rs/hepta-shadow-qualification/src/browser_tests.rs"

EXPECTED_STAGES = [f"WEB_C{i}" for i in range(8)]
EXPECTED_STAGE_STATUS = {
    "WEB_C0": "IMPLEMENTED_QUALIFICATION_ONLY",
    "WEB_C1": "IN_PROGRESS_PRIVATE_PROTOCOL_QUALIFICATION",
    "WEB_C2": "IMPLEMENTED_FIXTURE_ONLY",
    "WEB_C3": "IMPLEMENTED_FIXTURE_ONLY",
    "WEB_C4": "NOT_IMPLEMENTED",
    "WEB_C5": "NOT_IMPLEMENTED",
    "WEB_C6": "NOT_IMPLEMENTED",
    "WEB_C7": "NOT_IMPLEMENTED",
}
HEX40 = re.compile(r"^[0-9a-f]{40}$")
HEX64 = re.compile(r"^[0-9a-f]{64}$")
LOCAL_PATH_MARKERS = ("/Users/", "/Volumes/T5", "/home/qian", "C:\\Users\\")
AUTHORITY_KEYS = {
    "runtime_authority",
    "effect_authority",
    "production_caller",
    "production_writer",
    "external_network",
    "raw_cookie_export",
    "credential_export",
    "operator_acceptance",
    "promotion",
    "release_qualified",
}
PIN_AUTHORITY_KEYS = {
    "runtime_authority",
    "external_network",
    "production_caller",
    "promotion",
}
SOURCE_AUTHORITY_KEYS = {
    "runtime_authority",
    "production_caller",
    "production_writer",
    "external_network",
    "external_effect",
    "operator_acceptance",
    "promotion",
}


class VerificationError(RuntimeError):
    """Raised when the active plan or implementation widens its authority."""


def fail(message: str) -> None:
    raise VerificationError(message)


def load_json(path: Path) -> dict[str, Any]:
    if not path.is_file():
        fail(f"missing required file: {path.relative_to(ROOT)}")
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        fail(f"cannot parse {path.relative_to(ROOT)} as canonical JSON/YAML: {error}")
    if not isinstance(value, dict):
        fail(f"top-level value must be an object: {path.relative_to(ROOT)}")
    return value


def require_repo_file(value: Any, label: str) -> Path:
    if not isinstance(value, str) or not value or value.startswith(("/", "~")):
        fail(f"{label} must be a non-empty repository-relative path")
    candidate = Path(value)
    if ".." in candidate.parts:
        fail(f"{label} contains a parent traversal")
    path = ROOT / candidate
    if not path.is_file():
        fail(f"{label} points to a missing file: {value}")
    return path


def require_all_false(
    authority: Any,
    expected_keys: set[str],
    label: str,
) -> None:
    if not isinstance(authority, dict):
        fail(f"{label} authority must be an object")
    if set(authority) != expected_keys:
        fail(f"{label} authority keys differ: {sorted(set(authority) ^ expected_keys)}")
    enabled = sorted(key for key, value in authority.items() if value is not False)
    if enabled:
        fail(f"{label} attempted to enable authority: {enabled}")


def verify_no_local_paths(paths: Iterable[Path]) -> None:
    for path in paths:
        text = path.read_text(encoding="utf-8")
        for marker in LOCAL_PATH_MARKERS:
            if marker in text:
                fail(f"machine-local path marker {marker!r} found in {path.relative_to(ROOT)}")


def verify_python_syntax(paths: Iterable[Path]) -> None:
    for path in paths:
        try:
            compile(path.read_text(encoding="utf-8"), str(path), "exec")
        except (OSError, UnicodeError, SyntaxError) as error:
            fail(f"Python syntax check failed for {path.relative_to(ROOT)}: {error}")


def verify_current(current: dict[str, Any]) -> list[Path]:
    if current.get("schema") != "hepta.browser.current.v1" or current.get("schema_version") != 1:
        fail("CURRENT.yaml schema/version mismatch")
    if current.get("phase") != "DEVELOPMENT":
        fail("browser bundle must remain in DEVELOPMENT")
    if current.get("claim_level") != "L1_QUALIFICATION_ONLY":
        fail("browser bundle must remain qualification-only")
    if current.get("fail_closed") is not True:
        fail("CURRENT.yaml must declare fail_closed=true")
    require_all_false(current.get("authority"), AUTHORITY_KEYS, "CURRENT.yaml")

    stage_status = current.get("stage_status")
    if stage_status != EXPECTED_STAGE_STATUS:
        fail(f"CURRENT.yaml stage status drift: {stage_status}")
    implementation = current.get("implementation")
    if not isinstance(implementation, dict):
        fail("CURRENT.yaml implementation must be an object")
    expected_implementation = {
        "servo_source_topology": "frozen_isolated_verified_checkout",
        "private_worker_protocol": "implemented_qualification_only_unqualified",
        "qualification_process_harness": "implemented_private_stdio_pipe_unqualified",
        "unix_inherited_socketpair": "not_implemented",
        "windows_sid_named_pipe": "not_implemented",
        "servo_runtime": "not_integrated",
        "production_caller": "not_integrated",
    }
    for key, value in expected_implementation.items():
        if implementation.get(key) != value:
            fail(f"CURRENT.yaml implementation status drift for {key}")

    pointers = [
        "canonical_plan",
        "stage_matrix",
        "traceability_matrix",
        "threat_model",
        "receipt_schema",
        "servo_pin",
        "servo_source_topology",
        "servo_patch_inventory",
        "next_work_queue",
    ]
    return [require_repo_file(current.get(key), f"CURRENT.yaml#{key}") for key in pointers]


def verify_stages(matrix: dict[str, Any]) -> set[str]:
    if matrix.get("schema") != "hepta.browser.stage_matrix.v1" or matrix.get("schema_version") != 1:
        fail("STAGE_MATRIX schema/version mismatch")
    if matrix.get("execution_order") != EXPECTED_STAGES:
        fail("STAGE_MATRIX execution_order must be WEB_C0 through WEB_C7")
    stages = matrix.get("stages")
    if not isinstance(stages, list) or len(stages) != len(EXPECTED_STAGES):
        fail("STAGE_MATRIX must define exactly eight stages")
    by_id: dict[str, dict[str, Any]] = {}
    for stage in stages:
        if not isinstance(stage, dict) or not isinstance(stage.get("id"), str):
            fail("every stage must be an object with an id")
        stage_id = stage["id"]
        if stage_id in by_id:
            fail(f"duplicate stage: {stage_id}")
        by_id[stage_id] = stage
        for code_path in stage.get("code_paths", []):
            require_repo_file(code_path, f"{stage_id}.code_paths")
        for document_path in stage.get("document_paths", []):
            require_repo_file(document_path, f"{stage_id}.document_paths")
    if list(by_id) != EXPECTED_STAGES:
        fail(f"stage objects are not ordered C0-C7: {list(by_id)}")
    actual_status = {stage_id: stage.get("status") for stage_id, stage in by_id.items()}
    if actual_status != EXPECTED_STAGE_STATUS:
        fail(f"stage status drift: {actual_status}")

    for stage_id, stage in by_id.items():
        for dependency in stage.get("depends_on", []):
            if dependency not in by_id:
                fail(f"{stage_id} has unknown dependency {dependency}")
            if EXPECTED_STAGES.index(dependency) >= EXPECTED_STAGES.index(stage_id):
                fail(f"{stage_id} depends on non-prior stage {dependency}")
        if not isinstance(stage.get("receipt_kind"), str) or not stage["receipt_kind"]:
            fail(f"{stage_id} is missing receipt_kind")
    return set(by_id)


def verify_traceability(trace: dict[str, Any], stage_ids: set[str]) -> tuple[int, set[str]]:
    if trace.get("schema") != "hepta.browser.traceability_matrix.v1" or trace.get("schema_version") != 1:
        fail("TRACEABILITY_MATRIX schema/version mismatch")
    requirements = trace.get("requirements")
    if not isinstance(requirements, list) or not requirements:
        fail("TRACEABILITY_MATRIX must contain requirements")
    seen: set[str] = set()
    test_names: set[str] = set()
    for requirement in requirements:
        if not isinstance(requirement, dict):
            fail("traceability requirement must be an object")
        requirement_id = requirement.get("id")
        if not isinstance(requirement_id, str) or not requirement_id:
            fail("traceability requirement is missing id")
        if requirement_id in seen:
            fail(f"duplicate requirement id: {requirement_id}")
        seen.add(requirement_id)
        if requirement.get("stage") not in stage_ids:
            fail(f"{requirement_id} references unknown stage")
        if requirement.get("release_blocking") is not True:
            fail(f"{requirement_id} must remain release_blocking")
        for code_path in requirement.get("code", []):
            require_repo_file(code_path, f"{requirement_id}.code")
        for document_path in requirement.get("documents", []):
            require_repo_file(document_path, f"{requirement_id}.documents")
        for test_name in requirement.get("tests", []):
            if not isinstance(test_name, str) or not test_name:
                fail(f"{requirement_id} contains an invalid test name")
            test_names.add(test_name)
    return len(seen), test_names


def verify_servo(pin: dict[str, Any], topology: dict[str, Any]) -> None:
    if pin.get("schema") != "hepta.browser.servo_upstream_pin.v1" or pin.get("schema_version") != 1:
        fail("SERVO_UPSTREAM_PIN schema/version mismatch")
    if pin.get("repository") != "servo/servo":
        fail("Servo pin must target servo/servo")
    if not HEX40.fullmatch(str(pin.get("commit", ""))) or not HEX40.fullmatch(str(pin.get("tree", ""))):
        fail("Servo commit/tree must be exact lowercase 40-hex IDs")
    if pin.get("license") != "MPL-2.0":
        fail("Servo license binding must be MPL-2.0")
    if pin.get("integration_status") != "SOURCE_PIN_ONLY_NOT_IMPORTED":
        fail("Servo must remain source-pin-only in this slice")
    require_all_false(pin.get("authority"), PIN_AUTHORITY_KEYS, "Servo pin")

    if topology.get("schema") != "hepta.browser.servo_source_import_topology.v1":
        fail("Servo source topology schema mismatch")
    source = topology.get("source")
    if not isinstance(source, dict):
        fail("Servo source topology source is missing")
    for key in ("commit", "tree", "license"):
        if source.get(key) != pin.get(key):
            fail(f"Servo source topology differs from pin for {key}")
    if source.get("branch_tracking_allowed") is not False:
        fail("Servo source topology cannot track a branch")
    if source.get("unpinned_git_dependencies_allowed") is not False:
        fail("Servo source topology cannot allow unpinned Git dependencies")

    integration = topology.get("integration_topology")
    if not isinstance(integration, dict):
        fail("Servo source topology integration boundary is missing")
    if integration.get("mode") != "isolated_verified_source_checkout_and_worker_artifact":
        fail("Servo must remain an isolated source checkout and worker artifact")
    for key in (
        "main_cargo_workspace_dependency",
        "servo_source_inside_codex_rs_workspace",
        "servo_types_exposed_to_hepta_callers",
        "raw_webdriver_surface_exposed",
    ):
        if integration.get(key) is not False:
            fail(f"Servo integration topology widened {key}")

    reviewed = topology.get("reviewed_upstream_files")
    if not isinstance(reviewed, list) or len(reviewed) < 6:
        fail("Servo topology must contain the reviewed upstream blob inventory")
    paths: set[str] = set()
    for item in reviewed:
        if not isinstance(item, dict):
            fail("Servo reviewed file entry must be an object")
        path = item.get("path")
        blob = item.get("blob_sha")
        if not isinstance(path, str) or path in paths:
            fail("Servo reviewed file paths are missing or duplicated")
        if not HEX40.fullmatch(str(blob or "")):
            fail(f"Servo reviewed file has invalid blob SHA: {path}")
        paths.add(path)
    required_reviewed = {
        "Cargo.toml",
        "components/servo/Cargo.toml",
        "ports/servoshell/Cargo.toml",
        "ports/servoshell/lib.rs",
        "ports/servoshell/webdriver.rs",
        "components/webdriver_server/lib.rs",
    }
    if not required_reviewed <= paths:
        fail(f"Servo reviewed source inventory is incomplete: {sorted(required_reviewed - paths)}")
    excluded = topology.get("explicitly_excluded_initial_surface")
    if not isinstance(excluded, list) or "components/webdriver_server::start_server" not in excluded:
        fail("Servo WebDriver network server must remain explicitly excluded")
    require_all_false(topology.get("negative_authority"), SOURCE_AUTHORITY_KEYS, "Servo topology")


def verify_patch_inventory(inventory: dict[str, Any], pin: dict[str, Any]) -> None:
    if inventory.get("schema") != "hepta.browser.servo_patch_inventory.v1":
        fail("Servo patch inventory schema mismatch")
    if inventory.get("servo_commit") != pin.get("commit") or inventory.get("servo_tree") != pin.get("tree"):
        fail("Servo patch inventory source binding differs from the pin")
    if inventory.get("status") != "EMPTY_NO_SOURCE_IMPORTED":
        fail("Servo patch inventory must remain empty before source import")
    if inventory.get("patches") != []:
        fail("Servo patch inventory claims patches before source import")
    patch_files = sorted(PATCH_INVENTORY_PATH.parent.glob("*.patch"))
    if patch_files:
        fail(f"unregistered or premature Servo patch files exist: {[path.name for path in patch_files]}")
    next_patch = inventory.get("next_expected_patch")
    if not isinstance(next_patch, dict) or next_patch.get("status") != "NOT_CREATED":
        fail("the first Servo patch must remain not-created in this slice")
    policy = inventory.get("policy")
    if not isinstance(policy, dict) or any(value is not False for value in policy.values()):
        fail("Servo patch policy must keep every permission false")


def verify_queue(queue: dict[str, Any]) -> None:
    if queue.get("schema") != "hepta.browser.next_work_queue.v1" or queue.get("schema_version") != 1:
        fail("NEXT_WORK_QUEUE schema/version mismatch")
    if queue.get("authority") != "qualification_only":
        fail("NEXT_WORK_QUEUE must remain qualification-only")
    tasks = queue.get("tasks")
    if not isinstance(tasks, list) or [task.get("id") for task in tasks if isinstance(task, dict)] != [
        f"C1-{index:03d}" for index in range(1, 9)
    ]:
        fail("NEXT_WORK_QUEUE must define C1-001 through C1-008 in order")
    expected = {
        "C1-001": "COMPLETE_CONTRACT_FROZEN",
        "C1-002": "READY_IMPLEMENT_NEXT",
        "C1-003": "PARTIAL_QUALIFICATION_HARNESS_IMPLEMENTED",
        "C1-004": "BLOCKED_BY_C1-002_AND_C1-003_PRODUCTION_TRANSPORT",
        "C1-005": "BLOCKED_BY_C1-004",
        "C1-006": "BLOCKED_BY_C1-004",
        "C1-007": "BLOCKED_BY_C1-005_AND_C1-006",
        "C1-008": "BLOCKED_BY_C1-007",
    }
    actual = {task["id"]: task.get("status") for task in tasks if isinstance(task, dict)}
    if actual != expected:
        fail(f"NEXT_WORK_QUEUE status drift: {actual}")
    blocker = queue.get("environment_blocker")
    if not isinstance(blocker, dict) or blocker.get("status") != "GITHUB_ACTIONS_JOBS_FAIL_BEFORE_STEPS":
        fail("NEXT_WORK_QUEUE must preserve the current Actions environment blocker")


def verify_qualification_receipt_schema(schema: dict[str, Any]) -> None:
    if schema.get("$schema") != "https://json-schema.org/draft/2020-12/schema":
        fail("qualification receipt must use JSON Schema draft 2020-12")
    properties = schema.get("properties")
    if not isinstance(properties, dict):
        fail("qualification receipt schema has no properties")
    stage_enum = properties.get("stage", {}).get("enum")
    if stage_enum != EXPECTED_STAGES:
        fail("qualification receipt stage enum drift")
    authority = properties.get("authority", {})
    authority_properties = authority.get("properties", {})
    if set(authority_properties) != AUTHORITY_KEYS:
        fail("qualification receipt authority keys drift")
    for key, definition in authority_properties.items():
        if not isinstance(definition, dict) or definition.get("const") is not False:
            fail(f"qualification receipt may not enable authority field {key}")


def verify_source_receipt_schema(schema: dict[str, Any], pin: dict[str, Any]) -> None:
    if schema.get("$schema") != "https://json-schema.org/draft/2020-12/schema":
        fail("Servo source receipt must use JSON Schema draft 2020-12")
    properties = schema.get("properties")
    if not isinstance(properties, dict):
        fail("Servo source receipt schema has no properties")
    source_properties = properties.get("source", {}).get("properties", {})
    if source_properties.get("repository", {}).get("const") != "servo/servo":
        fail("Servo source receipt repository is not closed")
    if source_properties.get("license", {}).get("const") != "MPL-2.0":
        fail("Servo source receipt license is not closed")
    authority = schema.get("$defs", {}).get("authority", {}).get("properties", {})
    if set(authority) != SOURCE_AUTHORITY_KEYS:
        fail("Servo source receipt authority keys drift")
    for key, definition in authority.items():
        if not isinstance(definition, dict) or definition.get("const") is not False:
            fail(f"Servo source receipt may not enable authority field {key}")
    if pin.get("commit") != "0a48e298482659817eb50097df23841f2b8e3044":
        fail("Servo source pin changed without updating this plan successor")


def verify_test_names(test_names: set[str]) -> None:
    sources = "\n".join(
        path.read_text(encoding="utf-8")
        for path in (BROWSER_TEST_PATH, WORKER_TEST_PATH, WORKER_PROCESS_TEST_PATH)
    )
    missing = sorted(name for name in test_names if f"fn {name}" not in sources)
    if missing:
        fail(f"traceability references missing tests: {missing}")


def verify_worker_code() -> None:
    required = (
        WORKER_PROTOCOL_PATH,
        WORKER_HARNESS_PATH,
        WORKER_BINARY_PATH,
        WORKER_TEST_PATH,
        WORKER_PROCESS_TEST_PATH,
        CARGO_PATH,
        BAZEL_PATH,
    )
    for path in required:
        if not path.is_file():
            fail(f"missing worker implementation file: {path.relative_to(ROOT)}")
    protocol = WORKER_PROTOCOL_PATH.read_text(encoding="utf-8")
    harness = WORKER_HARNESS_PATH.read_text(encoding="utf-8")
    binary = WORKER_BINARY_PATH.read_text(encoding="utf-8")
    tests = WORKER_TEST_PATH.read_text(encoding="utf-8")
    process_test = WORKER_PROCESS_TEST_PATH.read_text(encoding="utf-8")
    cargo = CARGO_PATH.read_text(encoding="utf-8")
    bazel = BAZEL_PATH.read_text(encoding="utf-8")

    required_protocol_tokens = (
        "BROWSER_WORKER_PROTOCOL_SCHEMA_VERSION: u32 = 1",
        "MAX_BROWSER_WORKER_FRAME_BYTES: usize = 65_536",
        "BROWSER_WORKER_STARTUP_CAPABILITY_BYTES: usize = 32",
        "BrowserWorkerStartupCapability",
        "QualificationStdioPipe",
        "UnixInheritedSocketPair",
        "WindowsSidNamedPipe",
        "deny_unknown_fields",
        "read_browser_worker_frame",
        "write_browser_worker_frame",
        "WrongStartupCapability",
        "WrongSequence",
        "AuthorityOpen",
    )
    for token in required_protocol_tokens:
        if token not in protocol:
            fail(f"worker protocol is missing required token: {token}")
    for forbidden in ("TcpListener", "TcpStream", "0.0.0.0", "127.0.0.1", "WebSocket"):
        if forbidden in protocol or forbidden in harness or forbidden in binary:
            fail(f"worker scaffold contains forbidden network surface: {forbidden}")
    for prohibited in (".unwrap(", ".expect("):
        if prohibited in protocol or prohibited in harness or prohibited in binary or prohibited in tests or prohibited in process_test:
            fail(f"worker scaffold contains prohibited call path: {prohibited}")
    if "kill_on_drop(true)" not in harness or "Stdio::piped()" not in harness:
        fail("qualification worker harness is missing its private child process boundary")
    if "hepta-browser-worker-qualification" not in cargo or '"io-std"' not in cargo:
        fail("Cargo target or Tokio private-pipe feature is missing")
    if "hepta-browser-worker-qualification" not in bazel:
        fail("Bazel worker target is missing")


def verify_provenance_code() -> None:
    for path in (
        PROVENANCE_GENERATOR_PATH,
        PROVENANCE_TEST_PATH,
        SOURCE_SCHEMA_PATH,
        PROVENANCE_DOC_PATH,
    ):
        if not path.is_file():
            fail(f"missing provenance file: {path.relative_to(ROOT)}")
    generator = PROVENANCE_GENERATOR_PATH.read_text(encoding="utf-8")
    for token in (
        "--servo-source",
        "--output",
        "HEAD^{tree}",
        "--porcelain=v1",
        "--untracked-files=all",
        "hash-object",
        "machine_local_paths_included",
        "network_access_used",
        "os.O_EXCL",
        "os.fsync",
    ):
        if token not in generator:
            fail(f"Servo provenance generator is missing fail-closed behavior: {token}")
    for forbidden in ("urllib", "requests", "http.client", "socket.socket", "curl", "wget"):
        if forbidden in generator:
            fail(f"Servo provenance generator contains a network path: {forbidden}")


def verify_ci_and_ownership() -> None:
    workflow = WORKFLOW_PATH.read_text(encoding="utf-8")
    owners = OWNERS_PATH.read_text(encoding="utf-8")
    for required in (
        "integration/vnext-main-20260811",
        "pull_request:",
        "browser-c0-c3:",
        "scripts/verify-hepta-browser-plan.py",
    ):
        if required not in workflow:
            fail(f"Hepta workflow is missing {required}")
    for pattern in (
        "/codex-rs/hepta-* @ProfAlexQI",
        "/docs/hepta-vnext/ @ProfAlexQI",
        "/.github/workflows/hepta-vnext-qualification.yml @ProfAlexQI",
    ):
        if pattern not in owners:
            fail(f"CODEOWNERS is missing {pattern}")


def main() -> int:
    try:
        current = load_json(CURRENT_PATH)
        referenced_paths = verify_current(current)
        matrix = load_json(STAGE_PATH)
        stage_ids = verify_stages(matrix)
        requirement_count, test_names = verify_traceability(load_json(TRACE_PATH), stage_ids)
        pin = load_json(SERVO_PATH)
        topology = load_json(TOPOLOGY_PATH)
        verify_servo(pin, topology)
        verify_patch_inventory(load_json(PATCH_INVENTORY_PATH), pin)
        verify_queue(load_json(QUEUE_PATH))
        verify_qualification_receipt_schema(load_json(SCHEMA_PATH))
        verify_source_receipt_schema(load_json(SOURCE_SCHEMA_PATH), pin)
        verify_test_names(test_names)
        verify_worker_code()
        verify_provenance_code()
        verify_python_syntax(
            (
                Path(__file__),
                PROVENANCE_GENERATOR_PATH,
                PROVENANCE_TEST_PATH,
            )
        )
        verify_ci_and_ownership()
        verify_no_local_paths(
            (
                CURRENT_PATH,
                STAGE_PATH,
                TRACE_PATH,
                SERVO_PATH,
                TOPOLOGY_PATH,
                PATCH_INVENTORY_PATH,
                QUEUE_PATH,
                SCHEMA_PATH,
                SOURCE_SCHEMA_PATH,
                PLAN_PATH,
                THREAT_PATH,
                README_PATH,
                PROVENANCE_DOC_PATH,
            )
        )
    except VerificationError as error:
        print(f"hepta browser plan verification failed: {error}", file=sys.stderr)
        return 1

    summary = {
        "schema": "hepta.browser.plan_verification.v2",
        "status": "PASS_QUALIFICATION_PLAN_NOT_RUNTIME",
        "stages": EXPECTED_STAGES,
        "stage_status": EXPECTED_STAGE_STATUS,
        "requirements": requirement_count,
        "servo_commit": pin["commit"],
        "servo_tree": pin["tree"],
        "c1_source_topology": "frozen",
        "c1_worker_protocol": "implemented_unqualified",
        "c1_servo_runtime": "not_integrated",
        "authority": "all_false",
        "referenced_files": sorted(str(path.relative_to(ROOT)) for path in referenced_paths),
    }
    print(json.dumps(summary, sort_keys=True, separators=(",", ":")))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
