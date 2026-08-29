#!/usr/bin/env python3
"""Fail-closed source verifier for Hepta architecture convergence P0.5."""

from __future__ import annotations

import json
import pathlib
import re
import subprocess
import sys
from typing import Any, NoReturn

ROOT = pathlib.Path(__file__).resolve().parents[1]
MODEL = ROOT / "docs/architecture/HEPTA_ARCHITECTURE_MODEL_V2.json"
STATUS = ROOT / "docs/architecture/HEPTA_QUALIFICATION_STATUS_V2.json"
LEDGER = ROOT / "docs/architecture/HEPTA_ARCHITECTURE_GAP_LEDGER_V2.json"
INDEX = ROOT / "docs/architecture/HEPTA_DOCUMENT_AUTHORITY_INDEX_V1.json"
LEGACY_ARCHITECTURE = ROOT / "docs/architecture/HEPTA_CURRENT_ARCHITECTURE_V1.json"
DATA_MAP = ROOT / "docs/architecture/DATA_AUTHORITY_MAP.md"
ARCHITECTURE = ROOT / "ARCHITECTURE.md"
PRODUCT_GRAPH = ROOT / "codex-rs/hepta-contracts/src/product_graph.rs"
AUTHORITY = ROOT / "codex-rs/hepta-contracts/src/authority.rs"
RUNTIME_PROFILE = ROOT / "codex-rs/hepta-agentd/src/runtime_profile.rs"
COMPOSITION = ROOT / "codex-rs/hepta-agentd/src/composition.rs"
MEMORY_SERVICE = ROOT / "codex-rs/hepta-agentd/src/memory_service.rs"
AUTOMATION_SERVICE = ROOT / "codex-rs/hepta-agentd/src/automation_service.rs"
WORKFLOW = ROOT / ".github/workflows/hepta-gap-closure-p0-5.yml"
BLOCKING_WORKFLOW = ROOT / ".github/workflows/blocking-ci.yml"

EXECUTION_STATES = {
    "not_run",
    "queued",
    "running",
    "passed",
    "failed",
    "blocked",
    "superseded",
}
EXTERNAL_STATES = {"not_issued", "issued", "rejected", "revoked", "superseded"}
CLOSED_AUTHORITY_FIELDS = {
    "productionCaller",
    "productionWriter",
    "effectAuthority",
    "externalEffect",
    "modelInvocationAuthority",
    "providerDispatchAuthority",
    "fleetMutationAuthority",
    "operatorAcceptance",
    "promotion",
    "release",
}


def fail(message: str) -> NoReturn:
    raise SystemExit(f"FAIL_HEPTA_P0_5_GAP_CLOSURE: {message}")


def load_json_no_duplicates(path: pathlib.Path) -> dict[str, Any]:
    def pairs_hook(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
        value: dict[str, Any] = {}
        for key, item in pairs:
            if key in value:
                fail(f"duplicate JSON key {key!r} in {path.relative_to(ROOT)}")
            value[key] = item
        return value

    try:
        data = json.loads(path.read_text(encoding="utf-8"), object_pairs_hook=pairs_hook)
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        fail(f"cannot parse {path.relative_to(ROOT)}: {error}")
    if not isinstance(data, dict):
        fail(f"{path.relative_to(ROOT)} must contain one JSON object")
    return data


def read(path: pathlib.Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except (OSError, UnicodeError) as error:
        fail(f"cannot read {path.relative_to(ROOT)}: {error}")


def require_files() -> None:
    required = (
        MODEL,
        STATUS,
        LEDGER,
        INDEX,
        LEGACY_ARCHITECTURE,
        DATA_MAP,
        ARCHITECTURE,
        PRODUCT_GRAPH,
        AUTHORITY,
        RUNTIME_PROFILE,
        COMPOSITION,
        MEMORY_SERVICE,
        AUTOMATION_SERVICE,
        WORKFLOW,
        BLOCKING_WORKFLOW,
        ROOT / "docs/architecture/HEPTA_ARCHITECTURE_CONVERGENCE_PLAN_V2.md",
        ROOT / "scripts/generate-hepta-architecture-projections.py",
    )
    missing = [str(path.relative_to(ROOT)) for path in required if not path.is_file()]
    if missing:
        fail(f"required P0.5 files are absent: {missing}")


def enum_variants(source: str, enum_name: str) -> list[str]:
    match = re.search(
        rf"(?:pub(?:\(crate\))?\s+)?enum\s+{re.escape(enum_name)}\s*\{{(?P<body>.*?)\n\}}",
        source,
        flags=re.DOTALL,
    )
    if match is None:
        fail(f"cannot locate Rust enum {enum_name}")
    variants = re.findall(r"^\s*([A-Z][A-Za-z0-9_]*)\s*,\s*$", match.group("body"), re.MULTILINE)
    if not variants:
        fail(f"Rust enum {enum_name} has no parsed variants")
    if len(variants) != len(set(variants)):
        fail(f"Rust enum {enum_name} has duplicate variants")
    return variants


def verify_projection_generation() -> None:
    result = subprocess.run(
        [sys.executable, "scripts/generate-hepta-architecture-projections.py", "--check"],
        cwd=ROOT,
        text=True,
        check=False,
    )
    if result.returncode != 0:
        fail("generated architecture projections drifted from the V2 model")


def verify_model() -> dict[str, Any]:
    model = load_json_no_duplicates(MODEL)
    if model.get("schema") != "hepta.architecture-model.v2" or model.get("schemaVersion") != 2:
        fail("wrong V2 architecture model schema")
    if model.get("status") != "normative_product_architecture_source":
        fail("V2 architecture model is not normative")

    projection = model.get("projectionPolicy")
    if not isinstance(projection, dict):
        fail("projectionPolicy must be an object")
    if projection.get("generator") != "scripts/generate-hepta-architecture-projections.py":
        fail("architecture projection generator drifted")
    if projection.get("handEditingGeneratedFilesAllowed") is not False:
        fail("generated architecture projections must not be hand-edited")

    components = model.get("components")
    if not isinstance(components, list) or not components:
        fail("model components must be a non-empty list")
    component_by_id: dict[str, dict[str, Any]] = {}
    for component in components:
        if not isinstance(component, dict) or not isinstance(component.get("id"), str):
            fail("invalid model component")
        identifier = component["id"]
        if identifier in component_by_id:
            fail(f"duplicate component {identifier}")
        component_by_id[identifier] = component
    if component_by_id.get("agentd", {}).get("durableDomains") != []:
        fail("Agentd must own no durable domain")
    for identifier in ("qualification_plane", "evidence_subsystem"):
        if component_by_id.get(identifier, {}).get("productGraphMember") is not False:
            fail(f"{identifier} cannot be a product graph member")
        if component_by_id.get(identifier, {}).get("productDependencyAllowed") is not False:
            fail(f"{identifier} cannot become a product dependency")

    domains = model.get("dataDomains")
    if not isinstance(domains, list) or not domains:
        fail("model dataDomains must be a non-empty list")
    domain_ids: set[str] = set()
    scopes: set[str] = set()
    for domain in domains:
        if not isinstance(domain, dict) or not isinstance(domain.get("id"), str):
            fail("invalid model data domain")
        identifier = domain["id"]
        if identifier in domain_ids:
            fail(f"duplicate data domain {identifier}")
        domain_ids.add(identifier)
        scope = domain.get("scope")
        if scope not in {"runtime_product_graph", "external_control", "qualification_evidence"}:
            fail(f"invalid data domain scope for {identifier}: {scope!r}")
        scopes.add(scope)
        writer = domain.get("writer")
        if writer not in component_by_id:
            fail(f"data domain {identifier} references unknown writer {writer!r}")
        if scope == "runtime_product_graph" and not isinstance(domain.get("rustVariant"), str):
            fail(f"runtime data domain {identifier} lacks a Rust variant")
        if scope != "runtime_product_graph" and domain.get("rustVariant") is not None:
            fail(f"non-runtime data domain {identifier} must not claim a ProductGraph Rust variant")
    if scopes != {"runtime_product_graph", "external_control", "qualification_evidence"}:
        fail(f"architecture data scopes drifted: {sorted(scopes)}")

    model_data_variants = [
        domain["rustVariant"] for domain in domains if domain["scope"] == "runtime_product_graph"
    ]
    rust_data_variants = enum_variants(read(PRODUCT_GRAPH), "DataDomain")
    if model_data_variants != rust_data_variants:
        fail(
            "V2 runtime data domains drifted from Rust DataDomain: "
            f"model={model_data_variants!r} rust={rust_data_variants!r}"
        )

    model_component_variants = [
        component["rustVariant"]
        for component in components
        if isinstance(component.get("rustVariant"), str)
    ]
    rust_component_variants = enum_variants(read(PRODUCT_GRAPH), "ProductComponentId")
    if model_component_variants != rust_component_variants:
        fail(
            "V2 components drifted from Rust ProductComponentId: "
            f"model={model_component_variants!r} rust={rust_component_variants!r}"
        )

    profiles = model.get("runtimeProfiles")
    if not isinstance(profiles, list) or len(profiles) != 3:
        fail("runtimeProfiles must contain the three closed profiles")
    model_profile_variants = [profile.get("rustVariant") for profile in profiles]
    rust_profile_variants = enum_variants(read(AUTHORITY), "RuntimeAuthorityProfile")
    if model_profile_variants != rust_profile_variants:
        fail(
            "V2 runtime profiles drifted from Rust RuntimeAuthorityProfile: "
            f"model={model_profile_variants!r} rust={rust_profile_variants!r}"
        )
    verify_runtime_profile_source(profiles, component_by_id)

    boundary = model.get("authorityBoundary")
    if not isinstance(boundary, dict) or set(boundary) != CLOSED_AUTHORITY_FIELDS:
        fail("model authority boundary field set drifted")
    if any(boundary.values()):
        fail("model widened the closed authority boundary")
    return model


def verify_runtime_profile_source(
    profiles: list[dict[str, Any]], component_by_id: dict[str, dict[str, Any]]
) -> None:
    source = read(RUNTIME_PROFILE)
    service_variants = enum_variants(source, "RuntimeServiceId")
    expected_services = [
        component["rustVariant"]
        for component in component_by_id.values()
        if component.get("productGraphMember") is True
    ]
    if service_variants != expected_services:
        fail(
            "runtime service enum drifted from product graph members: "
            f"rust={service_variants!r} model={expected_services!r}"
        )

    placement_variants = {
        "control_plane": "ControlPlane",
        "in_process": "InProcess",
        "adapter_process": "AdapterProcess",
        "dormant_boundary": "DormantBoundary",
    }
    constructor_for_requirement = {
        "required": "required",
        "optional": "optional",
        "disabled": "disabled",
    }
    for index, profile in enumerate(profiles):
        variant = profile.get("rustVariant")
        start_marker = f"RuntimeAuthorityProfile::{variant} => vec!["
        start = source.find(start_marker)
        if start < 0:
            fail(f"runtime profile source lacks {variant} branch")
        next_starts = [
            source.find(f"RuntimeAuthorityProfile::{other.get('rustVariant')} => vec![", start + 1)
            for other in profiles[index + 1 :]
        ]
        next_starts = [position for position in next_starts if position >= 0]
        end = min(next_starts) if next_starts else source.find("        };", start)
        if end < 0:
            fail(f"runtime profile source branch {variant} has no terminator")
        block = source[start:end]
        services = profile.get("services")
        if not isinstance(services, list):
            fail(f"profile {variant} services must be a list")
        positions: list[int] = []
        for service in services:
            identifier = service.get("id")
            component = component_by_id.get(identifier)
            if component is None or not isinstance(component.get("rustVariant"), str):
                fail(f"profile {variant} references unknown runtime service {identifier!r}")
            constructor = constructor_for_requirement.get(service.get("requirement"))
            placement = placement_variants.get(service.get("placement"))
            if constructor is None or placement is None:
                fail(f"profile {variant} contains an invalid service policy")
            marker = (
                f"RuntimeServicePolicy::{constructor}(\n"
                f"                    RuntimeServiceId::{component['rustVariant']},\n"
                f"                    RuntimeServicePlacement::{placement},"
            )
            position = block.find(marker)
            if position < 0:
                fail(f"runtime profile source lacks exact {variant}/{identifier} policy")
            positions.append(position)
            readiness = service.get("readinessRequired")
            failure_mode = service.get("failureMode")
            if service.get("requirement") == "required":
                readiness_marker = f"RuntimeServicePlacement::{placement},\n                    {str(readiness).lower()},"
                if readiness_marker not in block[position : position + len(marker) + 100]:
                    fail(f"runtime profile readiness drift for {variant}/{identifier}")
                if failure_mode != "fail_closed":
                    fail(f"required model service {variant}/{identifier} must fail closed")
            elif service.get("requirement") == "optional":
                if readiness is not False or failure_mode != "degraded":
                    fail(f"optional model service {variant}/{identifier} policy drifted")
            elif readiness is not False or failure_mode != "not_started":
                fail(f"disabled model service {variant}/{identifier} policy drifted")
        if positions != sorted(positions) or len(set(positions)) != len(positions):
            fail(f"runtime profile service order drifted for {variant}")

    composition = read(COMPOSITION)
    for marker in (
        "RuntimeProfileContract::for_authority(&authority)",
        ".validate_product_graph(&product_graph)",
        ".validate_composed_services(",
        "memory_service.is_available()",
        "automation_service.is_available()",
    ):
        if marker not in composition:
            fail(f"Agentd composition is missing runtime-profile binding {marker!r}")
    for path in (MEMORY_SERVICE, AUTOMATION_SERVICE):
        source = read(path)
        if "#[cfg(test)]\n    pub(crate) fn is_available" in source:
            fail(f"{path.relative_to(ROOT)} keeps runtime availability test-only")
        if "pub(crate) fn is_available(&self) -> bool" not in source:
            fail(f"{path.relative_to(ROOT)} lacks runtime availability observation")


def verify_status() -> None:
    status = load_json_no_duplicates(STATUS)
    if status.get("schema") != "hepta.qualification-status.v2" or status.get("schemaVersion") != 2:
        fail("wrong qualification status schema")
    if set(status.get("executionStateVocabulary", [])) != EXECUTION_STATES:
        fail("qualification execution state vocabulary drifted")
    if set(status.get("externalDecisionStateVocabulary", [])) != EXTERNAL_STATES:
        fail("external decision state vocabulary drifted")
    if status.get("claimLevel") != "source_present_unqualified":
        fail("source-controlled status may not claim executable qualification")
    qualification = status.get("qualification")
    if not isinstance(qualification, dict) or not qualification:
        fail("qualification status map is missing")
    for gate, record in qualification.items():
        if not isinstance(record, dict) or record.get("state") not in EXECUTION_STATES:
            fail(f"invalid qualification gate state: {gate}")
        if record.get("state") != "not_run":
            fail(f"source-controlled qualification gate must remain not_run: {gate}")
        for identity in ("runId", "jobId", "runnerId", "stepCount"):
            if record.get(identity) is not None:
                fail(f"source-controlled qualification gate self-issued {identity}: {gate}")
    decisions = status.get("externalDecisions")
    if not isinstance(decisions, dict) or not decisions:
        fail("external decision status map is missing")
    for gate, record in decisions.items():
        if not isinstance(record, dict) or record.get("state") not in EXTERNAL_STATES:
            fail(f"invalid external decision state: {gate}")
        if record.get("state") != "not_issued" or record.get("receipt") is not None:
            fail(f"source-controlled status self-issued external decision: {gate}")
    boundary = status.get("authorityBoundary")
    if not isinstance(boundary, dict) or set(boundary) != CLOSED_AUTHORITY_FIELDS:
        fail("status authority boundary field set drifted")
    if any(boundary.values()):
        fail("status widened authority")


def verify_ledger() -> None:
    ledger = load_json_no_duplicates(LEDGER)
    if ledger.get("schema") != "hepta.architecture-gap-ledger.v2" or ledger.get("schemaVersion") != 2:
        fail("wrong V2 gap-ledger schema")
    if ledger.get("overallState") != "source_gaps_closed_product_and_external_gates_open":
        fail("gap ledger overclaimed aggregate closure")
    closed = ledger.get("closedSourceGaps")
    if not isinstance(closed, dict) or not closed:
        fail("closedSourceGaps must be a non-empty object")
    for gap, record in closed.items():
        if not isinstance(record, dict) or record.get("state") != "source_implemented":
            fail(f"closed source gap has invalid state: {gap}")
        evidence = record.get("evidence")
        if not isinstance(evidence, list) or not evidence:
            fail(f"closed source gap has no evidence: {gap}")
        for relative in evidence:
            if not isinstance(relative, str) or not (ROOT / relative).is_file():
                fail(f"closed source gap evidence is absent: {gap}: {relative!r}")
    product = ledger.get("openProductGaps")
    if not isinstance(product, dict) or not product:
        fail("openProductGaps must remain explicit")
    for gap, record in product.items():
        if not isinstance(record, dict) or record.get("state") not in {"open", "partial"}:
            fail(f"product gap is not honestly open/partial: {gap}")
        if not record.get("closureCriteria"):
            fail(f"product gap has no closure criteria: {gap}")
    external = ledger.get("openExternalGates")
    if not isinstance(external, dict) or not external:
        fail("openExternalGates must remain explicit")
    for gap, record in external.items():
        state = record.get("state") if isinstance(record, dict) else None
        if state not in {"not_run", "not_issued"}:
            fail(f"external gate was self-issued or overclaimed: {gap}: {state!r}")
    boundary = ledger.get("authorityBoundary")
    if not isinstance(boundary, dict) or set(boundary) != CLOSED_AUTHORITY_FIELDS:
        fail("ledger authority boundary field set drifted")
    if any(boundary.values()):
        fail("ledger widened authority")


def verify_document_authority(model: dict[str, Any]) -> None:
    index = load_json_no_duplicates(INDEX)
    if index.get("schemaVersion") != 2:
        fail("document authority index must be schemaVersion 2")
    normative = index.get("normative")
    expected_normative = {
        "architectureModel": "docs/architecture/HEPTA_ARCHITECTURE_MODEL_V2.json",
        "executionPlan": "docs/architecture/HEPTA_ARCHITECTURE_CONVERGENCE_PLAN_V2.md",
        "gapLedger": "docs/architecture/HEPTA_ARCHITECTURE_GAP_LEDGER_V2.json",
        "qualificationStatus": "docs/architecture/HEPTA_QUALIFICATION_STATUS_V2.json",
    }
    if normative != expected_normative:
        fail("document authority normative set drifted")
    generated = index.get("generatedProjections")
    if not isinstance(generated, list) or len(generated) != 3:
        fail("document authority generated projection set drifted")
    for record in generated:
        if not isinstance(record, dict) or record.get("source") != expected_normative["architectureModel"]:
            fail("generated projection is not bound to the V2 model")
    policy = index.get("pullRequestPolicy")
    if not isinstance(policy, dict) or any(policy.values()):
        fail("document authority pull-request policy must remain entirely false")

    legacy = load_json_no_duplicates(LEGACY_ARCHITECTURE)
    projection = legacy.get("projection")
    if not isinstance(projection, dict):
        fail("V1 compatibility architecture lacks projection metadata")
    if projection.get("status") != "generated_compatibility_projection":
        fail("V1 compatibility architecture still claims normative authority")
    if projection.get("normativeModel") != expected_normative["architectureModel"]:
        fail("V1 compatibility architecture points to the wrong normative model")
    if projection.get("runtimeAuthority") is not False or projection.get("qualificationAuthority") is not False:
        fail("V1 compatibility projection incorrectly claims authority")

    architecture = read(ARCHITECTURE)
    if "GENERATED FILE — do not hand edit" not in architecture:
        fail("ARCHITECTURE.md is not marked generated")
    data_map = read(DATA_MAP)
    if "GENERATED FILE — do not hand edit" not in data_map:
        fail("DATA_AUTHORITY_MAP.md is not marked generated")
    for domain in model["dataDomains"]:
        if domain["displayName"] not in data_map:
            fail(f"data authority projection omits domain {domain['id']}")


def verify_workflows() -> None:
    workflow = read(WORKFLOW)
    for marker in (
        "permissions:\n  contents: read",
        "Exact source-head P0.5 closure",
        "Merge-candidate P0.5 closure",
        "Hepta P0.5 gap closure required",
        "python3 scripts/generate-hepta-architecture-projections.py --check",
        "python3 scripts/verify-hepta-p0-5-gap-closure.py",
        "cargo test --locked -p codex-hepta-agentd runtime_profile::tests",
        "cargo test --locked -p codex-hepta-agentd composition::tests",
        "cargo clippy --locked --all-targets -p codex-hepta-agentd -- -D warnings",
        '"source_mutation": False',
    ):
        if marker not in workflow:
            fail(f"P0.5 qualification workflow is missing {marker!r}")
    for forbidden in (
        "contents: write",
        "persist-credentials: true",
        "git push",
        "git commit",
        "git update-ref",
    ):
        if forbidden in workflow:
            fail(f"P0.5 qualification workflow contains source mutation: {forbidden}")

    blocking = read(BLOCKING_WORKFLOW)
    if "uses: ./.github/workflows/hepta-gap-closure-p0-5.yml" not in blocking:
        fail("blocking-ci does not invoke P0.5 gap closure")
    if "- hepta-gap-closure" not in blocking:
        fail("blocking-ci required aggregate omits P0.5 gap closure")


def main() -> int:
    require_files()
    verify_projection_generation()
    model = verify_model()
    verify_status()
    verify_ledger()
    verify_document_authority(model)
    verify_workflows()
    print("PASS_HEPTA_P0_5_GAP_CLOSURE_SOURCE_ONLY")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
