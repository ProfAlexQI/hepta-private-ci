#!/usr/bin/env python3
"""Ratchet the Hepta Cargo dependency and composition-root policy."""

from __future__ import annotations

import json
import pathlib
import re
import sys
import tomllib
from dataclasses import dataclass
from typing import Any, Iterable

ROOT = pathlib.Path(__file__).resolve().parents[1]
POLICY_PATH = ROOT / "docs" / "architecture" / "HEPTA_DEPENDENCY_POLICY_V1.json"
CARGO_ROOT = ROOT / "codex-rs"


class PolicyError(RuntimeError):
    """A deterministic dependency-policy failure."""


@dataclass(frozen=True, order=True)
class Edge:
    source: str
    target: str


def no_duplicate_pairs(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise PolicyError(f"duplicate JSON key: {key}")
        result[key] = value
    return result


def load_policy() -> dict[str, Any]:
    try:
        value = json.loads(
            POLICY_PATH.read_text(encoding="utf-8"),
            object_pairs_hook=no_duplicate_pairs,
        )
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        raise PolicyError(f"cannot load dependency policy: {error}") from error
    if not isinstance(value, dict):
        raise PolicyError("dependency policy root must be an object")
    if value.get("schema") != "hepta.dependency-policy.v1":
        raise PolicyError("unsupported dependency policy schema")
    return value


def dependency_name(alias: str, value: Any) -> str:
    if isinstance(value, dict):
        package = value.get("package")
        if package is not None:
            if not isinstance(package, str) or not package:
                raise PolicyError(f"invalid Cargo dependency package for alias {alias}")
            return package
    return alias


def table_dependencies(table: Any) -> Iterable[str]:
    if table is None:
        return ()
    if not isinstance(table, dict):
        raise PolicyError("Cargo dependency table must be an object")
    return tuple(dependency_name(alias, value) for alias, value in table.items())


def target_runtime_dependencies(target: Any) -> Iterable[str]:
    if target is None:
        return ()
    if not isinstance(target, dict):
        raise PolicyError("Cargo target table must be an object")
    found: list[str] = []
    for target_name, table in target.items():
        if not isinstance(table, dict):
            raise PolicyError(f"Cargo target entry is not an object: {target_name}")
        found.extend(table_dependencies(table.get("dependencies")))
        found.extend(table_dependencies(table.get("build-dependencies")))
    return found


def load_runtime_graph() -> tuple[dict[str, pathlib.Path], set[Edge]]:
    manifests: dict[str, pathlib.Path] = {}
    edges: set[Edge] = set()
    for path in sorted(CARGO_ROOT.rglob("Cargo.toml")):
        if any(part in {"target", "vendor"} for part in path.parts):
            continue
        try:
            cargo = tomllib.loads(path.read_text(encoding="utf-8"))
        except (OSError, UnicodeError, tomllib.TOMLDecodeError) as error:
            raise PolicyError(f"invalid Cargo manifest {path.relative_to(ROOT)}: {error}") from error
        package = cargo.get("package")
        if not isinstance(package, dict):
            continue
        name = package.get("name")
        if not isinstance(name, str) or not name:
            raise PolicyError(f"Cargo package has no name: {path.relative_to(ROOT)}")
        previous = manifests.get(name)
        if previous is not None:
            raise PolicyError(
                f"duplicate Cargo package {name}: "
                f"{previous.relative_to(ROOT)} and {path.relative_to(ROOT)}"
            )
        manifests[name] = path
        dependencies = set(table_dependencies(cargo.get("dependencies")))
        dependencies.update(table_dependencies(cargo.get("build-dependencies")))
        dependencies.update(target_runtime_dependencies(cargo.get("target")))
        edges.update(Edge(name, dependency) for dependency in dependencies)
    return manifests, edges


def rule_violations(rule: dict[str, Any], edges: set[Edge]) -> set[Edge]:
    violations: set[Edge] = set()
    source_names = rule.get("from")
    if source_names is not None:
        if not isinstance(source_names, list) or not all(
            isinstance(value, str) for value in source_names
        ):
            raise PolicyError(f"invalid rule source set: {rule.get('id')}")
        forbidden_dependencies = set(rule.get("forbiddenDependencies", []))
        forbidden_prefixes = tuple(rule.get("forbiddenDependencyPrefixes", []))
        for edge in edges:
            if edge.source not in source_names:
                continue
            if edge.target in forbidden_dependencies or edge.target.startswith(
                forbidden_prefixes
            ):
                violations.add(edge)
    explicit = rule.get("forbiddenEdges", [])
    if not isinstance(explicit, list):
        raise PolicyError(f"invalid explicit edge list: {rule.get('id')}")
    for raw in explicit:
        if not (
            isinstance(raw, list)
            and len(raw) == 2
            and all(isinstance(value, str) for value in raw)
        ):
            raise PolicyError(f"invalid explicit forbidden edge: {raw!r}")
        edge = Edge(raw[0], raw[1])
        if edge in edges:
            violations.add(edge)
    return violations


def actual_source_pattern_debts(policy: dict[str, Any]) -> set[str]:
    found: set[str] = set()
    for rule in policy.get("rules", []):
        if not isinstance(rule, dict):
            raise PolicyError("dependency rule must be an object")
        for pattern in rule.get("forbiddenSourcePatterns", []):
            if not isinstance(pattern, dict):
                raise PolicyError("source pattern must be an object")
            path_value = pattern.get("path")
            regex_value = pattern.get("regex")
            if not isinstance(path_value, str) or not isinstance(regex_value, str):
                raise PolicyError("source pattern requires path and regex")
            path = ROOT / path_value
            if not path.is_file():
                continue
            if re.search(regex_value, path.read_text(encoding="utf-8")):
                found.add(path_value)
    return found


def debt_inventory(
    policy: dict[str, Any],
) -> tuple[dict[Edge, str], dict[str, str]]:
    edge_debts: dict[Edge, str] = {}
    path_debts: dict[str, str] = {}
    for debt in policy.get("baselineDebts", []):
        if not isinstance(debt, dict):
            raise PolicyError("baseline debt must be an object")
        debt_id = debt.get("id")
        if not isinstance(debt_id, str) or not debt_id:
            raise PolicyError("baseline debt has no ID")
        if debt.get("state") != "open":
            raise PolicyError(f"closed debt must be removed: {debt_id}")
        edge = debt.get("edge")
        path = debt.get("path")
        if edge is not None:
            if not (
                isinstance(edge, list)
                and len(edge) == 2
                and all(isinstance(value, str) for value in edge)
            ):
                raise PolicyError(f"invalid debt edge: {debt_id}")
            parsed = Edge(edge[0], edge[1])
            if parsed in edge_debts:
                raise PolicyError(f"duplicate debt edge: {parsed}")
            edge_debts[parsed] = debt_id
        elif isinstance(path, str):
            if path in path_debts:
                raise PolicyError(f"duplicate source-pattern debt: {path}")
            path_debts[path] = debt_id
        else:
            raise PolicyError(f"debt has neither edge nor path: {debt_id}")
    return edge_debts, path_debts


def main() -> int:
    policy = load_policy()
    manifests, edges = load_runtime_graph()

    all_violations: set[Edge] = set()
    for raw_rule in policy.get("rules", []):
        if not isinstance(raw_rule, dict):
            raise PolicyError("dependency rule must be an object")
        all_violations.update(rule_violations(raw_rule, edges))

    expected_edge_debts, expected_path_debts = debt_inventory(policy)
    actual_pattern_paths = actual_source_pattern_debts(policy)

    unknown_edges = sorted(all_violations - set(expected_edge_debts))
    stale_edge_debts = sorted(set(expected_edge_debts) - all_violations)
    unknown_paths = sorted(actual_pattern_paths - set(expected_path_debts))
    stale_path_debts = sorted(set(expected_path_debts) - actual_pattern_paths)

    if unknown_edges:
        raise PolicyError(f"new forbidden Cargo edges: {unknown_edges}")
    if stale_edge_debts:
        stale = [(edge, expected_edge_debts[edge]) for edge in stale_edge_debts]
        raise PolicyError(f"resolved Cargo debts must be removed from policy: {stale}")
    if unknown_paths:
        raise PolicyError(f"new forbidden composition-root patterns: {unknown_paths}")
    if stale_path_debts:
        stale = [(path, expected_path_debts[path]) for path in stale_path_debts]
        raise PolicyError(f"resolved source debts must be removed from policy: {stale}")

    referenced_packages = {
        package
        for edge in expected_edge_debts
        for package in (edge.source, edge.target)
        if package.startswith("codex-")
    }
    missing_packages = sorted(referenced_packages - set(manifests))
    if missing_packages:
        raise PolicyError(f"policy references missing Cargo packages: {missing_packages}")

    print(
        "PASS: dependency graph matches the exact open-debt baseline "
        f"({len(expected_edge_debts)} Cargo edges, "
        f"{len(expected_path_debts)} composition-root pattern)"
    )
    for edge, debt_id in sorted(expected_edge_debts.items()):
        print(f"OPEN-DEBT {debt_id}: {edge.source} -> {edge.target}")
    for path, debt_id in sorted(expected_path_debts.items()):
        print(f"OPEN-DEBT {debt_id}: {path}")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except PolicyError as error:
        print(f"FAIL: {error}", file=sys.stderr)
        raise SystemExit(1)
