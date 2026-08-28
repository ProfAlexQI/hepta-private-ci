#!/usr/bin/env python3
"""Fail-closed source verifier for Hepta Architecture Convergence P0.2."""

from __future__ import annotations

import json
import sys
from collections import Counter, defaultdict, deque
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
MANIFEST = ROOT / "docs/architecture/HEPTA_PRODUCT_ARCHITECTURE_V1.json"


class VerificationError(RuntimeError):
    pass


def load_json_no_duplicates(path: Path) -> dict[str, Any]:
    def pairs_hook(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
        result: dict[str, Any] = {}
        for key, value in pairs:
            if key in result:
                raise VerificationError(f"duplicate JSON key {key!r} in {path}")
            result[key] = value
        return result

    try:
        value = json.loads(path.read_text(encoding="utf-8"), object_pairs_hook=pairs_hook)
    except (OSError, json.JSONDecodeError) as error:
        raise VerificationError(f"cannot parse {path}: {error}") from error
    if not isinstance(value, dict):
        raise VerificationError(f"{path} must contain one JSON object")
    return value


def require(condition: bool, message: str) -> None:
    if not condition:
        raise VerificationError(message)


def read(relative: str) -> str:
    path = ROOT / relative
    try:
        return path.read_text(encoding="utf-8")
    except OSError as error:
        raise VerificationError(f"cannot read {relative}: {error}") from error


def verify_graph(manifest: dict[str, Any]) -> None:
    components = manifest.get("components")
    edges = manifest.get("edges")
    authorities = manifest.get("data_authorities")
    require(isinstance(components, list) and components, "components must be a non-empty list")
    require(isinstance(edges, list) and edges, "edges must be a non-empty list")
    require(
        isinstance(authorities, list) and authorities,
        "data_authorities must be a non-empty list",
    )
    require(len(components) == len(set(components)), "component ids must be unique")
    required_components = {
        "supervisor",
        "agentd",
        "app_server",
        "memory_runtime",
        "automation_runtime",
        "matrix_ingress",
    }
    require(set(components) == required_components, "canonical product component set changed")
    require("qualification_plane" not in components, "qualification cannot be a product component")

    edge_keys: set[tuple[str, str, str]] = set()
    outgoing: dict[str, list[str]] = defaultdict(list)
    indegree = {component: 0 for component in components}
    for edge in edges:
        require(isinstance(edge, dict), "every edge must be an object")
        source = edge.get("from")
        target = edge.get("to")
        kind = edge.get("kind")
        require(source in indegree and target in indegree, "edge endpoint is not a component")
        require(source != target, "self edges are forbidden")
        key = (source, target, kind)
        require(key not in edge_keys, f"duplicate edge {key}")
        edge_keys.add(key)
        outgoing[source].append(target)
        indegree[target] += 1

    required_edges = {
        ("agentd", "supervisor", "depends_on"),
        ("agentd", "memory_runtime", "composes"),
        ("agentd", "automation_runtime", "composes"),
        ("agentd", "app_server", "hosts"),
        ("app_server", "memory_runtime", "reads_from"),
        ("automation_runtime", "app_server", "submits_to"),
        ("matrix_ingress", "agentd", "submits_to"),
    }
    require(edge_keys == required_edges, "canonical product edge set changed")

    ready = deque(sorted(component for component, degree in indegree.items() if degree == 0))
    visited = 0
    while ready:
        component = ready.popleft()
        visited += 1
        for target in outgoing[component]:
            indegree[target] -= 1
            if indegree[target] == 0:
                ready.append(target)
    require(visited == len(components), "product graph contains a dependency cycle")

    domains: list[str] = []
    for authority in authorities:
        require(isinstance(authority, dict), "every data authority must be an object")
        domain = authority.get("domain")
        writer = authority.get("writer")
        require(isinstance(domain, str) and domain, "data domain must be non-empty")
        require(writer in components, f"data writer {writer!r} is not a product component")
        domains.append(domain)
    duplicates = sorted(domain for domain, count in Counter(domains).items() if count != 1)
    require(not duplicates, f"data domains must have exactly one writer: {duplicates}")
    required_domains = {
        "fleet_registry",
        "agent_lifecycle",
        "thread_session",
        "memory_ledger",
        "knowledge_projection",
        "automation_schedule",
        "ingress_projection",
        "runtime_health",
    }
    require(set(domains) == required_domains, "machine data authority map is incomplete or widened")


def verify_negative_authority(manifest: dict[str, Any]) -> None:
    negative = manifest.get("negative_authority")
    require(isinstance(negative, dict), "negative_authority must be an object")
    required = {
        "production_caller",
        "production_writer",
        "model_invocation",
        "provider_dispatch",
        "external_effect",
        "fleet_mutation",
        "operator_acceptance",
        "promotion",
    }
    require(set(negative) == required, "negative-authority field set changed")
    require(all(value is False for value in negative.values()), "all P0 authority flags must be false")

    kernel = manifest.get("authority_kernel")
    require(isinstance(kernel, dict), "authority_kernel must be an object")
    require(kernel.get("contract_schema_version") == 2, "authority kernel contract must be v2")
    require(
        kernel.get("legacy_production_adapter")
        == "codex-hepta-agentd::production_authority_adapter",
        "legacy production authority adapter is not canonical",
    )
    forbidden = set(kernel.get("forbidden_actions", []))
    require(
        forbidden
        == {
            "invoke_model",
            "dispatch_provider",
            "external_effect",
            "mutate_fleet",
            "accept_operator",
            "promote_release",
        },
        "authority kernel forbidden-action set changed",
    )


def verify_source_wiring() -> None:
    contracts_lib = read("codex-rs/hepta-contracts/src/lib.rs")
    authority = read("codex-rs/hepta-contracts/src/authority.rs")
    graph = read("codex-rs/hepta-contracts/src/product_graph.rs")
    memory_runtime = read("codex-rs/hepta-memory/src/cognitive_runtime.rs")
    agentd_lib = read("codex-rs/hepta-agentd/src/lib.rs")
    composition = read("codex-rs/hepta-agentd/src/composition.rs")
    memory_service = read("codex-rs/hepta-agentd/src/memory_service.rs")
    automation_service = read("codex-rs/hepta-agentd/src/automation_service.rs")
    runtime = read("codex-rs/hepta-agentd/src/runtime.rs")
    app_runtime = read("codex-rs/hepta-agentd/src/app_runtime.rs")
    authority_adapter = read("codex-rs/hepta-agentd/src/production_authority_adapter.rs")
    production_host = read("codex-rs/hepta-agentd/src/production_writer_host.rs")

    for declaration in ("mod authority;", "mod product_graph;"):
        require(declaration in contracts_lib, f"missing contract module declaration: {declaration}")
    for constructor in (
        "pub fn snapshot_read_only",
        "pub fn agent_local",
        "pub fn qualification_cognitive_write",
    ):
        require(constructor in authority, f"missing closed-world authority constructor: {constructor}")
    require("AUTHORITY_KERNEL_SCHEMA_VERSION: u32 = 2" in authority, "authority kernel is not v2")
    require("pub struct Authorized<C>" in authority, "typed Authorized<C> capability token is missing")
    require("pub struct AuthorityLeaseBinding" in authority, "typed lease binding is missing")
    require("pub trait CapabilityVerifier" in authority, "capability verifier seam is missing")
    require(
        "pub fn authorize_verified_capability" in authority,
        "externally verified capability constructor is missing",
    )
    require("pub fn dangerous_actions" in authority, "authority escape audit is missing")

    require("pub struct ProductGraph" in graph, "product graph contract is missing")
    require("pub fn agent_local" in graph, "Agent product graph constructor is missing")
    require("DuplicateDataWriter" in graph, "single-writer validation is missing")
    require("DependencyCycle" in graph, "cycle validation is missing")

    require("pub async fn open_agent_owned" in memory_runtime, "Memory open facade is missing")
    require(
        "pub async fn with_discovered_federation" in memory_runtime,
        "Memory federation facade is missing",
    )
    for declaration in (
        "mod composition;",
        "mod memory_service;",
        "mod automation_service;",
        "mod production_authority_adapter;",
    ):
        require(declaration in agentd_lib, f"Agentd module is not registered: {declaration}")

    require(
        "ProductGraph::agent_local(&authority)" in composition,
        "Agentd does not consume the real ProductGraph",
    )
    require(
        "AgentMemoryService::open" in composition,
        "Agentd composition does not construct the Memory service",
    )
    require(
        "AgentAutomationService::open" in composition,
        "Agentd composition does not construct the Automation service",
    )
    require("CognitiveRuntime" not in composition, "composition still owns raw Memory runtime logic")
    require("AutomationStore" not in composition, "composition still owns raw Automation store logic")

    require(
        "CognitiveRuntime::open_agent_owned" in memory_service,
        "Memory service does not consume the Memory runtime facade",
    )
    require(
        "authorize::<CognitiveWriteCapability>" in memory_service,
        "Memory service does not retain typed cognitive-write capability",
    )
    require(
        "authorize::<AutomationMutationCapability>" in automation_service,
        "Automation service does not retain typed mutation capability",
    )
    require(
        "run_automation_scheduler" in automation_service,
        "Automation service does not own scheduler execution",
    )

    require(
        "AgentRuntimeComposition::open(config)" in runtime,
        "Agentd supervision loop bypasses the composition root",
    )
    require(
        "AgentAppServerService::new" in runtime,
        "Agentd supervision loop does not construct the App Server service",
    )
    require(
        "automation_service.run" in runtime,
        "Agentd supervision loop does not execute the Automation service",
    )
    require("CognitiveStore::open" not in runtime, "Agentd supervision loop still opens Memory directly")
    require("AutomationStore::open" not in runtime, "Agentd supervision loop still opens Automation directly")
    require("run_automation_scheduler" not in runtime, "Agentd supervision loop still owns scheduler internals")

    require("pub(crate) struct AgentAppServerService" in app_runtime, "App Server service is missing")
    require(
        "authorize::<SessionServeCapability>" in app_runtime,
        "App Server service does not retain typed session authority",
    )
    require(
        "COGNITIVE_WRITE_ENABLED" not in app_runtime,
        "App Server still uses a duplicated cognitive-write boolean",
    )
    require(
        "authority.allows(AuthorityAction::WriteCognitiveState)" in app_runtime,
        "App Server cognitive-write state is not derived from AuthorityGrant",
    )

    require(
        "authorize_verified_capability::<CognitiveWriteCapability" in authority_adapter,
        "legacy production lease is not mapped to typed cognitive-write authority",
    )
    require(
        "AuthorityAction::WriteCognitiveState" in authority_adapter,
        "legacy production adapter does not constrain the action",
    )
    require(
        "ExternalEffectCapability" not in authority_adapter,
        "legacy production adapter must not mint external-effect authority",
    )
    require(
        "Authorized<CognitiveWriteCapability>" in production_host,
        "production writer host does not retain typed cognitive-write authority",
    )
    require(
        "ExternalEffectCapability" not in production_host,
        "production writer host must not gain external-effect authority",
    )
    adapter_index = production_host.find("ProductionCognitiveWriteAuthorization::verify")
    store_index = production_host.find("CognitiveStore::open")
    require(adapter_index >= 0 and store_index >= 0, "production writer open sequence is incomplete")
    require(adapter_index < store_index, "typed authority must be verified before Cognitive store open")


def main() -> int:
    try:
        manifest = load_json_no_duplicates(MANIFEST)
        require(manifest.get("schema") == "hepta.product_architecture.v1", "wrong manifest schema")
        require(manifest.get("schema_version") == 1, "wrong manifest schema version")
        require(manifest.get("canonical_human_entry") == "ARCHITECTURE.md", "wrong canonical entry")
        require((ROOT / "ARCHITECTURE.md").is_file(), "ARCHITECTURE.md is missing")
        verify_graph(manifest)
        verify_negative_authority(manifest)
        verify_source_wiring()
    except VerificationError as error:
        print(f"FAIL_ARCHITECTURE_CONVERGENCE_P0_2: {error}", file=sys.stderr)
        return 1

    print(
        json.dumps(
            {
                "result": "PASS_ARCHITECTURE_CONVERGENCE_P0_2_SOURCE",
                "schema": manifest["schema"],
                "status": manifest["status"],
                "component_count": len(manifest["components"]),
                "data_authority_count": len(manifest["data_authorities"]),
                "typed_legacy_adapter": True,
                "service_builders": True,
                "runtime_authority": False,
                "external_effect": False,
                "promotion": False,
            },
            sort_keys=True,
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
