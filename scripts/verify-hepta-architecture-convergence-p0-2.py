#!/usr/bin/env python3
"""Fail-closed source verifier for Hepta architecture convergence P0.4."""

from __future__ import annotations

import json
import pathlib
import subprocess
import sys
from typing import Any, NoReturn

ROOT = pathlib.Path(__file__).resolve().parents[1]
ARCHITECTURE = ROOT / "docs/architecture/HEPTA_CURRENT_ARCHITECTURE_V1.json"
STATUS = ROOT / "docs/architecture/HEPTA_ARCHITECTURE_CONVERGENCE_P0_2_STATUS.json"
AUTHORITY_CALLERS = ROOT / "docs/architecture/HEPTA_AUTHORITY_CALLERS_V1.json"

REQUIRED_FILES = (
    ROOT / "ARCHITECTURE.md",
    ROOT / "docs/architecture/DATA_AUTHORITY_MAP.md",
    ROOT / "docs/architecture/RECOVERY_ORDER.md",
    ARCHITECTURE,
    STATUS,
    AUTHORITY_CALLERS,
    ROOT / "codex-rs/hepta-contracts/src/authority.rs",
    ROOT / "codex-rs/hepta-contracts/src/product_graph.rs",
    ROOT / "codex-rs/hepta-contracts/src/operation.rs",
    ROOT / "codex-rs/hepta-memory/src/cognitive_runtime.rs",
    ROOT / "codex-rs/hepta-memory-runtime/Cargo.toml",
    ROOT / "codex-rs/hepta-memory-runtime/src/lib.rs",
    ROOT / "codex-rs/hepta-memory-runtime/src/legacy_authority.rs",
    ROOT / "codex-rs/hepta-memory-runtime/src/production_writer.rs",
    ROOT / "codex-rs/hepta-agentd/src/composition.rs",
    ROOT / "codex-rs/hepta-agentd/src/memory_service.rs",
    ROOT / "codex-rs/hepta-agentd/src/production_authority_adapter.rs",
    ROOT / "codex-rs/hepta-agentd/src/production_writer_host.rs",
    ROOT / "codex-rs/hepta-agentd/src/runtime.rs",
    ROOT / "codex-rs/hepta-agentd/src/app_runtime.rs",
)

CLOSED_AUTHORITY_FIELDS = (
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
)

REQUIRED_DATA_WRITERS = {
    "fleet_registry": "supervisor",
    "agent_lifecycle": "supervisor",
    "release_promotion": "supervisor",
    "thread_session": "app_server",
    "memory_ledger": "memory_runtime",
    "knowledge_projection": "memory_runtime",
    "automation_schedule": "automation_runtime",
    "ingress_projection": "matrix_ingress",
    "runtime_health": "agentd",
}


def fail(message: str) -> NoReturn:
    raise SystemExit(f"FAIL_ARCHITECTURE_CONVERGENCE_P0_4: {message}")


def load_json(path: pathlib.Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        fail(f"cannot load {path.relative_to(ROOT)}: {error}")
    if not isinstance(value, dict):
        fail(f"{path.relative_to(ROOT)} must contain one JSON object")
    return value


def read(path: pathlib.Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except (OSError, UnicodeError) as error:
        fail(f"cannot read {path.relative_to(ROOT)}: {error}")


def require_source(path: pathlib.Path, needles: tuple[str, ...]) -> str:
    source = read(path)
    for needle in needles:
        if needle not in source:
            fail(f"{path.relative_to(ROOT)} is missing {needle!r}")
    return source


def require_absent(path: pathlib.Path, needles: tuple[str, ...]) -> None:
    source = read(path)
    for needle in needles:
        if needle in source:
            fail(f"{path.relative_to(ROOT)} still contains forbidden {needle!r}")


def verify_workspace_materialization() -> None:
    workspace = read(ROOT / "codex-rs/Cargo.toml")
    if workspace.count('    "hepta-memory-runtime",') != 1:
        fail("hepta-memory-runtime must be exactly one workspace member")
    if workspace.count(
        'codex-hepta-memory-runtime = { path = "hepta-memory-runtime" }'
    ) != 1:
        fail("hepta-memory-runtime must be exactly one workspace dependency")
    agentd_manifest = read(ROOT / "codex-rs/hepta-agentd/Cargo.toml")
    if agentd_manifest.count("codex-hepta-memory-runtime = { workspace = true }") != 1:
        fail("Agentd must depend exactly once on the Memory runtime facade")


def verify_architecture_documents() -> None:
    architecture = load_json(ARCHITECTURE)
    status = load_json(STATUS)
    caller_manifest = load_json(AUTHORITY_CALLERS)

    if architecture.get("schema") != "hepta.current-architecture.v1":
        fail("unexpected architecture schema")
    authority = architecture.get("authority")
    if not isinstance(authority, dict):
        fail("architecture authority must be an object")
    for field in (
        "externalPlanSnapshotsAreAuthority",
        "qualificationReceiptsAreProductAuthority",
        "draftPullRequestsAreProductAuthority",
    ):
        if authority.get(field) is not False:
            fail(f"{field} must remain false")

    components = architecture.get("components")
    if not isinstance(components, list) or not components:
        fail("components must be a non-empty list")
    component_by_id: dict[str, dict[str, Any]] = {}
    for component in components:
        if not isinstance(component, dict) or not isinstance(component.get("id"), str):
            fail("every component must have a string id")
        component_id = component["id"]
        if component_id in component_by_id:
            fail(f"duplicate component {component_id}")
        component_by_id[component_id] = component
    if component_by_id.get("agentd", {}).get("durableDomains") != []:
        fail("agentd must own no durable domain")
    if component_by_id.get("qualification_plane", {}).get("productDependencyAllowed") is not False:
        fail("qualification plane must not be a product dependency")

    data_authorities = architecture.get("dataAuthorities")
    if not isinstance(data_authorities, list):
        fail("dataAuthorities must be a list")
    writers: dict[str, str] = {}
    for entry in data_authorities:
        if not isinstance(entry, dict):
            fail("data authority entry must be an object")
        domain = entry.get("domain")
        writer = entry.get("writer")
        if not isinstance(domain, str) or not isinstance(writer, str):
            fail("data authority domain and writer must be strings")
        if domain in writers:
            fail(f"durable domain {domain} has more than one writer")
        writers[domain] = writer
    if writers != REQUIRED_DATA_WRITERS:
        fail(f"data writer map drifted: {writers!r}")

    protocol = architecture.get("crossOwnerProtocol")
    if not isinstance(protocol, dict):
        fail("crossOwnerProtocol must be an object")
    if protocol.get("transport") != "transactional_outbox":
        fail("cross-owner transport must remain transactional_outbox")
    if protocol.get("blindRetryAfterBoundary") is not False:
        fail("blind retry after a crossed boundary must remain false")

    status_authority = status.get("authority")
    if not isinstance(status_authority, dict):
        fail("status authority must be an object")
    for field in CLOSED_AUTHORITY_FIELDS:
        if status_authority.get(field) is not False:
            fail(f"status authority field {field} must remain false")
    qualification = status.get("qualification")
    if not isinstance(qualification, dict) or qualification.get("qualified") is not False:
        fail("source status cannot claim executable qualification")

    if caller_manifest.get("schemaVersion") != 3:
        fail("authority caller manifest is not at the P0.4 schema")
    legacy = caller_manifest.get("legacyProductionAdapter")
    if not isinstance(legacy, dict):
        fail("legacy production adapter declaration is missing")
    if legacy.get("rawWriterOpenPath") != (
        "codex-rs/hepta-memory-runtime/src/production_writer.rs"
    ):
        fail("raw production writer owner is not the Memory runtime")
    if legacy.get("agentdMayCallRawWriterOpen") is not False:
        fail("Agentd raw production writer opening must remain forbidden")


def verify_product_wiring() -> None:
    require_source(
        ROOT / "codex-rs/hepta-contracts/src/authority.rs",
        (
            "pub struct AuthorityGrant",
            "pub struct AuthorityLeaseBinding",
            "pub struct Authorized<C>",
            "pub fn authorize_verified_capability",
            "AuthorityAction::ExternalEffect",
            "AuthorityAction::PromoteRelease",
        ),
    )
    require_source(
        ROOT / "codex-rs/hepta-contracts/src/product_graph.rs",
        (
            "pub struct ProductGraph",
            "QualificationComponentInProductGraph",
            "DuplicateDataWriter",
            "DependencyCycle",
        ),
    )
    require_source(
        ROOT / "codex-rs/hepta-contracts/src/operation.rs",
        (
            "pub struct OperationBinding",
            "pub struct OutboxEnvelope",
            "RecoveryDecision::LookupOnly",
            "blindly_retries_after_delivery_boundary",
        ),
    )
    require_source(
        ROOT / "codex-rs/hepta-memory-runtime/src/legacy_authority.rs",
        (
            "pub struct ProductionCognitiveWriteAuthorization",
            "authorize_verified_capability::<CognitiveWriteCapability",
            "ProductionAuthorityVerifier",
            "AuthorityLeaseBinding::new(",
        ),
    )
    require_absent(
        ROOT / "codex-rs/hepta-memory-runtime/src/legacy_authority.rs",
        ("AuthorityGrant::qualification_cognitive_write(",),
    )
    require_source(
        ROOT / "codex-rs/hepta-memory-runtime/src/production_writer.rs",
        (
            "pub struct AuthorizedProductionWriter",
            "ProductionDurableWriter::open(",
            "authorization.capability().is_external()",
            "authorization.capability().subject_agent_id()",
            "authorization.capability().generation()",
        ),
    )
    require_source(
        ROOT / "codex-rs/hepta-agentd/src/composition.rs",
        (
            "pub(crate) struct AgentRuntimeComposition",
            "ProductGraph::agent_local",
            "AuthorityGrant::agent_local",
            "AgentMemoryService::open",
            "AgentAutomationService::open",
        ),
    )
    require_source(
        ROOT / "codex-rs/hepta-agentd/src/memory_service.rs",
        (
            "use codex_hepta_memory_runtime::AgentMemoryRuntime;",
            "AgentMemoryRuntime::open(",
            ".with_discovered_federation(",
            "runtime.into_cognitive_runtime()",
        ),
    )
    require_absent(
        ROOT / "codex-rs/hepta-agentd/src/memory_service.rs",
        ("CognitiveRuntime::open_agent_owned",),
    )
    require_source(
        ROOT / "codex-rs/hepta-agentd/src/production_authority_adapter.rs",
        (
            "pub(crate) use codex_hepta_memory_runtime::ProductionCognitiveWriteAuthorization;",
            "ProductionCognitiveWriteAuthorization::verify(",
        ),
    )
    require_absent(
        ROOT / "codex-rs/hepta-agentd/src/production_authority_adapter.rs",
        (
            "AuthorityLeaseBinding::new(",
            "authorize_verified_capability::<CognitiveWriteCapability",
        ),
    )
    writer_host = require_source(
        ROOT / "codex-rs/hepta-agentd/src/production_writer_host.rs",
        (
            "AuthorizedProductionWriter::open(",
            "AuthorizedProductionWriter::open_with_store(",
            "cognitive_write: Authorized<CognitiveWriteCapability>",
            "external_effect: Option<Authorized<ExternalEffectCapability>>",
            "external_effect: Authorized<ExternalEffectCapability>",
            "validate_external_effect_capability(",
        ),
    )
    if "ProductionDurableWriter::open(" in writer_host:
        fail("Agentd still calls the raw production writer opener")
    require_source(
        ROOT / "codex-rs/hepta-agentd/src/app_runtime.rs",
        (
            "cognitive_write: Option<Authorized<CognitiveWriteCapability>>",
            "authorize::<SessionServeCapability>()",
            "validate_cognitive_write_capability",
            "Agent App Server cannot consume external production cognitive-write authority",
        ),
    )
    require_source(
        ROOT / "codex-rs/hepta-agentd/src/runtime.rs",
        (
            "AgentRuntimeComposition::open(config)",
            "memory_service.into_runtime_parts()",
            "AgentAppServerService::new(",
            "let _product_graph = product_graph",
        ),
    )


def run_nested_gate(relative: str) -> None:
    result = subprocess.run(
        [sys.executable, str(ROOT / relative)],
        cwd=ROOT,
        text=True,
        check=False,
    )
    if result.returncode != 0:
        fail(f"nested source gate failed: {relative}")


def main() -> int:
    missing = [str(path.relative_to(ROOT)) for path in REQUIRED_FILES if not path.is_file()]
    if missing:
        fail(f"required files are absent: {missing}")

    verify_workspace_materialization()
    verify_architecture_documents()
    verify_product_wiring()
    run_nested_gate("scripts/verify-hepta-authority-callers-p0-1.py")

    print("PASS_ARCHITECTURE_CONVERGENCE_P0_4_CANONICAL_SOURCE_ONLY")
    return 0


if __name__ == "__main__":
    sys.exit(main())
