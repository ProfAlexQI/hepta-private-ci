#!/usr/bin/env python3
"""Fail-closed verifier for the sole Hepta architecture catalog."""

from __future__ import annotations

import json
import pathlib
import re
import subprocess
import sys
from typing import Any, NoReturn

ROOT = pathlib.Path(__file__).resolve().parents[1]
CATALOG = ROOT / "docs/architecture/HEPTA_ARCHITECTURE_CATALOG_V1.json"
PRODUCT_GRAPH = ROOT / "codex-rs/hepta-contracts/src/product_graph.rs"
AUTHORITY = ROOT / "codex-rs/hepta-contracts/src/authority.rs"
STATUS = ROOT / "docs/architecture/HEPTA_ARCHITECTURE_EXECUTION_STATUS_V2.json"

SERVICE_STATES = {"required", "optional", "disabled"}
SOURCE_STATES = {"open", "partial", "closed"}
EXECUTION_STATES = {
    "not_run",
    "queued",
    "running",
    "passed",
    "failed",
    "blocked",
    "superseded",
}


def fail(message: str) -> NoReturn:
    raise SystemExit(f"FAIL_HEPTA_ARCHITECTURE_CATALOG_V1: {message}")


def load_json_no_duplicates(path: pathlib.Path) -> dict[str, Any]:
    def pairs_hook(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
        result: dict[str, Any] = {}
        for key, value in pairs:
            if key in result:
                fail(f"duplicate JSON key {key!r} in {path.relative_to(ROOT)}")
            result[key] = value
        return result

    try:
        value = json.loads(path.read_text(encoding="utf-8"), object_pairs_hook=pairs_hook)
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        fail(f"cannot parse {path.relative_to(ROOT)}: {error}")
    if not isinstance(value, dict):
        fail(f"{path.relative_to(ROOT)} must contain one JSON object")
    return value


def read(path: pathlib.Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except (OSError, UnicodeError) as error:
        fail(f"cannot read {path.relative_to(ROOT)}: {error}")


def unique_by_id(items: object, label: str) -> dict[str, dict[str, Any]]:
    if not isinstance(items, list) or not items:
        fail(f"{label} must be a non-empty list")
    result: dict[str, dict[str, Any]] = {}
    for item in items:
        if not isinstance(item, dict) or not isinstance(item.get("id"), str):
            fail(f"every {label} entry must have a string id")
        item_id = item["id"]
        if item_id in result:
            fail(f"duplicate {label} id {item_id!r}")
        result[item_id] = item
    return result


def verify_catalog_model(catalog: dict[str, Any]) -> None:
    if (
        catalog.get("schema") != "hepta.architecture-catalog.v1"
        or catalog.get("schemaVersion") != 1
    ):
        fail("unsupported catalog schema")

    authority = catalog.get("authority")
    if not isinstance(authority, dict):
        fail("catalog authority must be an object")
    if authority.get("status") != "sole_editable_architecture_source":
        fail("catalog is not declared as the sole editable architecture source")
    generated = authority.get("generatedViews")
    expected_generated = {
        "ARCHITECTURE.md",
        "docs/architecture/HEPTA_CURRENT_ARCHITECTURE_V1.json",
        "docs/architecture/DATA_AUTHORITY_MAP.md",
        "docs/architecture/HEPTA_RUNTIME_PROFILE_MATRIX_V1.json",
    }
    if not isinstance(generated, list) or set(generated) != expected_generated:
        fail("generated architecture view set drifted")
    for path in generated:
        if not (ROOT / path).is_file():
            fail(f"generated architecture view is missing: {path}")

    actions = unique_by_id(catalog.get("authorityActions"), "authority action")
    components = unique_by_id(catalog.get("components"), "component")
    domains = unique_by_id(catalog.get("dataDomains"), "data domain")
    profiles = unique_by_id(catalog.get("runtimeProfiles"), "runtime profile")

    durable_by_writer: dict[str, set[str]] = {component: set() for component in components}
    for domain_id, domain in domains.items():
        writer = domain.get("writer")
        if writer not in components:
            fail(f"data domain {domain_id} references unknown writer {writer!r}")
        migration_owner = domain.get("migrationOwner")
        if migration_owner is not None and migration_owner not in components:
            fail(
                f"data domain {domain_id} references unknown migration owner "
                f"{migration_owner!r}"
            )
        if domain.get("durability") != "ephemeral":
            durable_by_writer[writer].add(domain_id)
        if domain.get("rustProductGraph") is True:
            for field in (
                "rustDomainVariant",
                "rustWriterVariant",
                "rustStoreVariant",
            ):
                if not isinstance(domain.get(field), str):
                    fail(f"Rust-enforced domain {domain_id} is missing {field}")
        else:
            for field in (
                "rustDomainVariant",
                "rustWriterVariant",
                "rustStoreVariant",
            ):
                if domain.get(field) is not None:
                    fail(f"external domain {domain_id} unexpectedly declares {field}")

    for component_id, component in components.items():
        durable = component.get("durableDomains")
        if not isinstance(durable, list) or len(durable) != len(set(durable)):
            fail(f"component {component_id} durableDomains must be a unique list")
        if set(durable) != durable_by_writer[component_id]:
            fail(
                f"component {component_id} durable-domain ownership drifted: "
                f"declared={sorted(durable)} expected={sorted(durable_by_writer[component_id])}"
            )
        rust_variant = component.get("rustEnumVariant")
        if component.get("runtimeGraphMember") is True and not isinstance(rust_variant, str):
            fail(f"runtime graph component {component_id} has no Rust enum variant")

    component_ids = set(components)
    action_ids = set(actions)
    for profile_id, profile in profiles.items():
        profile_actions = profile.get("actions")
        services = profile.get("services")
        prerequisites = profile.get("qualificationPrerequisites")
        if (
            not isinstance(profile_actions, list)
            or len(profile_actions) != len(set(profile_actions))
            or not set(profile_actions).issubset(action_ids)
        ):
            fail(f"runtime profile {profile_id} has invalid actions")
        if not isinstance(services, dict) or set(services) != component_ids:
            fail(f"runtime profile {profile_id} must classify every component")
        invalid_states = {
            service: state for service, state in services.items() if state not in SERVICE_STATES
        }
        if invalid_states:
            fail(f"runtime profile {profile_id} has invalid service states: {invalid_states}")
        if (
            not isinstance(prerequisites, list)
            or len(prerequisites) != len(set(prerequisites))
            or not set(prerequisites).issubset(component_ids)
        ):
            fail(f"runtime profile {profile_id} has invalid qualification prerequisites")
        dangerous = [
            action
            for action in profile_actions
            if actions[action].get("externalOrRelease") is True
        ]
        if dangerous:
            fail(f"local runtime profile {profile_id} carries dangerous actions: {dangerous}")
        if profile.get("qualificationOnly") is not True and prerequisites:
            fail(f"non-qualification profile {profile_id} has qualification prerequisites")

    boundary = catalog.get("authorityBoundary")
    if not isinstance(boundary, dict) or not boundary or any(boundary.values()):
        fail("catalog authority boundary must remain fully closed")

    protocol = catalog.get("crossOwnerProtocol")
    if not isinstance(protocol, dict):
        fail("crossOwnerProtocol must be an object")
    if protocol.get("transport") != "transactional_outbox":
        fail("cross-owner transport must remain transactional_outbox")
    if protocol.get("blindRetryAfterBoundary") is not False:
        fail("blind retry after a crossed boundary must remain false")


def function_body(source: str, function_name: str) -> str:
    marker = f"    pub fn {function_name}("
    start = source.find(marker)
    if start < 0:
        fail(f"authority constructor is missing: {function_name}")
    candidates = [
        position
        for position in (
            source.find("\n    pub fn ", start + len(marker)),
            source.find("\n    fn new", start + len(marker)),
        )
        if position >= 0
    ]
    if not candidates:
        fail(f"authority constructor has no bounded end: {function_name}")
    return source[start : min(candidates)]


def verify_rust_parity(catalog: dict[str, Any]) -> None:
    product_graph = read(PRODUCT_GRAPH)
    authority = read(AUTHORITY)
    components = unique_by_id(catalog["components"], "component")
    domains = unique_by_id(catalog["dataDomains"], "data domain")
    actions = unique_by_id(catalog["authorityActions"], "authority action")
    profiles = unique_by_id(catalog["runtimeProfiles"], "runtime profile")

    components_start = product_graph.find("components: vec![")
    components_end = product_graph.find("],\n            edges:", components_start)
    if components_start < 0 or components_end < 0:
        fail("cannot locate ProductGraph component declaration")
    component_section = product_graph[components_start:components_end]

    for component_id, component in components.items():
        variant = component.get("rustEnumVariant")
        if isinstance(variant, str):
            if f'Self::{variant} => "{component_id}"' not in product_graph:
                fail(f"Rust component string mapping drifted: {component_id}")
            present = f"ProductComponentId::{variant}" in component_section
            if present != (component.get("runtimeGraphMember") is True):
                fail(
                    f"Rust runtime graph membership drifted for {component_id}: "
                    f"present={present}"
                )

    data_start = product_graph.find("data_authorities: vec![")
    data_end = product_graph.find("],\n        };", data_start)
    if data_start < 0 or data_end < 0:
        fail("cannot locate ProductGraph data-authority declaration")
    data_section = product_graph[data_start:data_end]

    rust_domains = [domain for domain in domains.values() if domain["rustProductGraph"]]
    if data_section.count("DataAuthority {") != len(rust_domains):
        fail(
            "Rust ProductGraph data-authority count drifted: "
            f"actual={data_section.count('DataAuthority {')} expected={len(rust_domains)}"
        )
    for domain in rust_domains:
        domain_id = domain["id"]
        domain_variant = domain["rustDomainVariant"]
        writer_variant = domain["rustWriterVariant"]
        store_variant = domain["rustStoreVariant"]
        if f'Self::{domain_variant} => "{domain_id}"' not in product_graph:
            fail(f"Rust data-domain string mapping drifted: {domain_id}")
        pattern = re.compile(
            r"DataAuthority\s*\{\s*"
            + rf"domain:\s*DataDomain::{re.escape(domain_variant)},\s*"
            + rf"writer:\s*ProductComponentId::{re.escape(writer_variant)},\s*"
            + rf"store:\s*DataStoreKind::{re.escape(store_variant)},\s*"
            + r"\}",
            re.MULTILINE,
        )
        if len(pattern.findall(data_section)) != 1:
            fail(f"Rust data authority does not exactly match catalog: {domain_id}")

    for profile_id, profile in profiles.items():
        body = function_body(authority, profile["rustConstructor"])
        variant = profile["rustVariant"]
        if f"RuntimeAuthorityProfile::{variant}" not in body:
            fail(f"Rust authority profile variant drifted: {profile_id}")
        observed = set(re.findall(r"AuthorityAction::([A-Za-z0-9_]+)", body))
        expected = {actions[action]["rustVariant"] for action in profile["actions"]}
        if observed != expected:
            fail(
                f"Rust authority actions drifted for {profile_id}: "
                f"observed={sorted(observed)} expected={sorted(expected)}"
            )


def verify_execution_status() -> None:
    status = load_json_no_duplicates(STATUS)
    if (
        status.get("schema") != "hepta.architecture-execution-status.v2"
        or status.get("schemaVersion") != 2
    ):
        fail("execution status schema drifted")
    policy = status.get("candidateBindingPolicy")
    if (
        not isinstance(policy, dict)
        or policy.get("mode") != "workflow_runtime_receipt_only"
        or policy.get("committedSourceFileMayClaimExecutableQualification") is not False
    ):
        fail("candidate binding policy is not runtime-receipt-only")
    execution = status.get("execution")
    if not isinstance(execution, dict):
        fail("execution status is missing")
    if set(execution.get("allowedStates", [])) != EXECUTION_STATES:
        fail("execution state vocabulary drifted")
    for key, value in execution.items():
        if key == "allowedStates":
            continue
        if value not in EXECUTION_STATES:
            fail(f"invalid execution state {key}={value!r}")
    authority = status.get("authority")
    if not isinstance(authority, dict) or not authority or any(authority.values()):
        fail("execution status widened authority")


def main() -> int:
    catalog = load_json_no_duplicates(CATALOG)
    verify_catalog_model(catalog)
    subprocess.run(
        [
            sys.executable,
            str(ROOT / "scripts/generate-hepta-architecture-views.py"),
            "--check",
        ],
        cwd=ROOT,
        check=True,
    )
    verify_rust_parity(catalog)
    verify_execution_status()
    print("PASS_HEPTA_ARCHITECTURE_CATALOG_V1")
    return 0


if __name__ == "__main__":
    sys.exit(main())
