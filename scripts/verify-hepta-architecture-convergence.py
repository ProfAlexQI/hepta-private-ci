#!/usr/bin/env python3
"""Fail-closed source verifier for Hepta Architecture Convergence P0.2."""

from __future__ import annotations

import json
import sys
from collections import Counter, defaultdict, deque
from pathlib import Path
from typing import Any, Iterable

ROOT = Path(__file__).resolve().parents[1]
MANIFEST = ROOT / "docs/architecture/HEPTA_PRODUCT_ARCHITECTURE_V1.json"


class VerificationError(RuntimeError):
    pass


def require(condition: bool, message: str) -> None:
    if not condition:
        raise VerificationError(message)


def read(relative: str) -> str:
    path = ROOT / relative
    try:
        return path.read_text(encoding="utf-8")
    except OSError as error:
        raise VerificationError(f"cannot read {relative}: {error}") from error


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
    require(isinstance(value, dict), f"{path} must contain one JSON object")
    return value


def require_all(source: str, needles: Iterable[str], label: str) -> None:
    missing = [needle for needle in needles if needle not in source]
    require(not missing, f"{label} is missing required source contracts: {missing}")


def require_none(source: str, needles: Iterable[str], label: str) -> None:
    present = [needle for needle in needles if needle in source]
    require(not present, f"{label} contains forbidden source contracts: {present}")


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

    required_components = {
        "supervisor",
        "agentd",
        "app_server",
        "memory_runtime",
        "automation_runtime",
        "matrix_ingress",
    }
    require(len(components) == len(set(components)), "component ids must be unique")
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
    require(
        set(domains)
        == {
            "fleet_registry",
            "agent_lifecycle",
            "thread_session",
            "memory_ledger",
            "knowledge_projection",
            "automation_schedule",
            "ingress_projection",
            "runtime_health",
        },
        "machine data authority map is incomplete or widened",
    )


def verify_machine_authority(manifest: dict[str, Any]) -> None:
    negative = manifest.get("negative_authority")
    require(isinstance(negative, dict), "negative_authority must be an object")
    require(
        set(negative)
        == {
            "production_caller",
            "production_writer",
            "model_invocation",
            "provider_dispatch",
            "external_effect",
            "fleet_mutation",
            "operator_acceptance",
            "promotion",
        },
        "negative-authority field set changed",
    )
    require(all(value is False for value in negative.values()), "all P0 authority flags must be false")

    kernel = manifest.get("authority_kernel")
    require(isinstance(kernel, dict), "authority_kernel must be an object")
    require(kernel.get("contract_schema_version") == 2, "authority kernel contract must be v2")
    require(
        kernel.get("verified_capability_constructor") == "authorize_verified_capability",
        "external capability constructor is not canonical",
    )
    require(
        kernel.get("legacy_production_adapter")
        == "codex-hepta-agentd::production_authority_adapter",
        "legacy production authority adapter is not canonical",
    )
    require(
        set(kernel.get("forbidden_actions", []))
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

    qualification = manifest.get("qualification")
    require(isinstance(qualification, dict), "qualification must be an object")
    for field in (
        "uses_real_product_modules",
        "exact_head_required",
        "real_agent_private_sqlite_open_required",
        "legacy_production_verifier_required",
        "typed_cognitive_write_witness_required",
        "typed_external_effect_witness_required_at_target_attach",
        "typed_external_effect_witness_revalidated_at_dispatch",
        "merge_candidate_gate_separate",
    ):
        require(qualification.get(field) is True, f"qualification field {field} must remain true")
    require(
        qualification.get("physical_memory_schema_extraction_allowed") is False,
        "physical Memory schema extraction must remain blocked",
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
    qualification_writer = read("codex-rs/hepta-agentd/src/qualification_writer.rs")
    authority_adapter = read("codex-rs/hepta-agentd/src/production_authority_adapter.rs")
    production_host = read("codex-rs/hepta-agentd/src/production_writer_host.rs")

    require_all(
        contracts_lib,
        (
            "mod authority;",
            "mod product_graph;",
            "pub use authority::AuthorityLeaseBinding;",
            "pub use authority::CapabilityVerifier;",
            "pub use authority::authorize_verified_capability;",
        ),
        "contracts root",
    )
    require_all(
        authority,
        (
            "AUTHORITY_KERNEL_SCHEMA_VERSION: u32 = 2",
            "pub fn snapshot_read_only",
            "pub fn agent_local",
            "pub fn qualification_cognitive_write",
            "pub struct AuthorityLeaseBinding",
            "pub struct CapabilityVerificationRequest",
            "pub trait CapabilityVerifier",
            "pub fn authorize_verified_capability",
            "pub struct Authorized<C>",
            "AuthorizationSource::ExternalLease",
            "pub fn external_lease_binding",
            "pub fn dangerous_actions",
        ),
        "authority kernel",
    )
    require_none(
        authority,
        ("Deserialize for Authorized", "pub fn from_status"),
        "typed authority token",
    )

    require_all(
        graph,
        (
            "pub struct ProductGraph",
            "pub fn agent_local",
            "DuplicateDataWriter",
            "DependencyCycle",
            "QualificationComponentInProductGraph",
        ),
        "product graph",
    )
    require_all(
        memory_runtime,
        (
            "pub async fn open_agent_owned",
            "pub async fn with_discovered_federation",
            "pub fn cognitive_write_store_available",
        ),
        "Memory runtime facade",
    )
    require_all(
        agentd_lib,
        (
            "mod composition;",
            "mod memory_service;",
            "mod automation_service;",
            "mod production_authority_adapter;",
        ),
        "Agentd module graph",
    )

    require_all(
        composition,
        (
            "ProductGraph::agent_local(&authority)",
            "AgentMemoryService::open",
            "AgentAutomationService::open",
        ),
        "Agentd composition root",
    )
    require_none(
        composition,
        ("CognitiveRuntime", "CognitiveStore", "AutomationStore", "run_automation_scheduler"),
        "Agentd composition root",
    )

    require_all(
        memory_service,
        (
            "CognitiveRuntime::open_agent_owned",
            "authorize::<MemoryReadCapability>",
            "authorize::<CognitiveWriteCapability>",
            "Option<Authorized<CognitiveWriteCapability>>",
            "into_runtime_parts",
            "with_discovered_federation",
        ),
        "Memory service",
    )
    require_all(
        automation_service,
        (
            "authorize::<AutomationMutationCapability>",
            "AutomationStore::open",
            "run_automation_scheduler",
        ),
        "Automation service",
    )

    require_all(
        runtime,
        (
            "AgentRuntimeComposition::open(config)",
            "memory_service.into_runtime_parts()",
            "AgentAppServerService::new",
            "automation_service.run",
        ),
        "Agentd supervision loop",
    )
    require_none(
        runtime,
        ("CognitiveStore::open", "AutomationStore::open", "run_automation_scheduler"),
        "Agentd supervision loop",
    )

    require_all(
        app_runtime,
        (
            "pub(crate) struct AgentAppServerService",
            "authorize::<SessionServeCapability>",
            "cognitive_write: Option<Authorized<CognitiveWriteCapability>>",
            "validate_cognitive_write_capability",
            "Agent App Server cannot consume external production cognitive-write authority",
            "cognitive_write.as_ref()",
        ),
        "App Server service",
    )
    require_none(app_runtime, ("COGNITIVE_WRITE_ENABLED",), "App Server service")

    require_all(
        qualification_writer,
        (
            "cognitive_write: Option<&Authorized<CognitiveWriteCapability>>",
            "let cognitive_write = cognitive_write.cloned()?;",
            "cognitive_write.subject_agent_id() != &identity.agent_id",
            "cognitive_write.generation() != identity.spawn_generation",
            "let cognitive_write = cognitive_write.clone();",
            "hepta-agentd:qualification-turn-writer:v2",
        ),
        "qualification writer host",
    )

    require_all(
        authority_adapter,
        (
            "authorize_verified_capability::<CognitiveWriteCapability",
            "AuthorityAction::WriteCognitiveState",
            "self.verifier",
            "ProductionCognitiveWriteAuthorization",
        ),
        "legacy production authority adapter",
    )
    require_none(
        authority_adapter,
        ("ExternalEffectCapability", "ProviderDispatchCapability"),
        "legacy production authority adapter",
    )

    require_all(
        production_host,
        (
            "Authorized<CognitiveWriteCapability>",
            "Authorized<ExternalEffectCapability>",
            "pub async fn admit",
            "pub async fn status",
            "external_effect: Authorized<ExternalEffectCapability>",
            "validate_external_effect_capability",
            "external_effect.external_lease_binding()",
            "cognitive_write.external_lease_binding()",
            "effect_binding.authority_epoch() != cognitive_binding.authority_epoch()",
            "effect_binding.owner_epoch() != cognitive_binding.owner_epoch()",
        ),
        "production writer host",
    )
    require_none(
        production_host,
        ("pub fn writer(&self)", "pub fn attach_target(mut self, target"),
        "production writer host public surface",
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
        verify_machine_authority(manifest)
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
                "typed_external_effect_gate": True,
                "typed_writer_capability_end_to_end": True,
                "raw_writer_escape_closed": True,
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
