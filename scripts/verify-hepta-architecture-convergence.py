#!/usr/bin/env python3
"""Fail-closed source verifier for Hepta Architecture Convergence P0."""

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
    memory = read("codex-rs/hepta-memory/src/cognitive_runtime.rs")
    agentd_lib = read("codex-rs/hepta-agentd/src/lib.rs")
    composition = read("codex-rs/hepta-agentd/src/composition.rs")
    runtime = read("codex-rs/hepta-agentd/src/runtime.rs")
    app_runtime = read("codex-rs/hepta-agentd/src/app_runtime.rs")

    for declaration in ("mod authority;", "mod product_graph;"):
        require(declaration in contracts_lib, f"missing contract module declaration: {declaration}")
    for constructor in (
        "pub fn snapshot_read_only",
        "pub fn agent_local",
        "pub fn qualification_cognitive_write",
    ):
        require(constructor in authority, f"missing closed-world authority constructor: {constructor}")
    require("pub struct Authorized<C>" in authority, "typed Authorized<C> capability token is missing")
    require("pub fn dangerous_actions" in authority, "authority escape audit is missing")

    require("pub struct ProductGraph" in graph, "product graph contract is missing")
    require("pub fn agent_local" in graph, "Agent product graph constructor is missing")
    require("DuplicateDataWriter" in graph, "single-writer validation is missing")
    require("DependencyCycle" in graph, "cycle validation is missing")

    require("pub async fn open_agent_owned" in memory, "Memory open facade is missing")
    require(
        "pub async fn with_discovered_federation" in memory,
        "Memory federation facade is missing",
    )
    require("mod composition;" in agentd_lib, "Agentd composition module is not registered")
    require(
        "ProductGraph::agent_local(&authority)" in composition,
        "Agentd does not consume the real ProductGraph",
    )
    require(
        "CognitiveRuntime::open_agent_owned" in composition,
        "Agentd does not consume the Memory runtime facade",
    )
    require(
        "AgentRuntimeComposition::open(config)" in runtime,
        "Agentd supervision loop bypasses the composition root",
    )
    require("CognitiveStore::open" not in runtime, "Agentd supervision loop still opens Memory directly")
    require(
        "FederatedRecallSet::discover" not in runtime,
        "Agentd supervision loop still discovers federation directly",
    )
    require(
        "COGNITIVE_WRITE_ENABLED" not in app_runtime,
        "App Server still uses a duplicated cognitive-write boolean",
    )
    require(
        "authority.allows(AuthorityAction::WriteCognitiveState)" in app_runtime,
        "App Server cognitive-write state is not derived from AuthorityGrant",
    )


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
        print(f"FAIL_ARCHITECTURE_CONVERGENCE_P0: {error}", file=sys.stderr)
        return 1

    print(
        json.dumps(
            {
                "result": "PASS_ARCHITECTURE_CONVERGENCE_P0_SOURCE",
                "schema": manifest["schema"],
                "status": manifest["status"],
                "component_count": len(manifest["components"]),
                "data_authority_count": len(manifest["data_authorities"]),
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
