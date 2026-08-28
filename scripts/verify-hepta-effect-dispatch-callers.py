#!/usr/bin/env python3
"""Reject new raw production writer/dispatcher callers outside canonical adapters."""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
MANIFEST = ROOT / "docs/architecture/HEPTA_EFFECT_DISPATCH_CALLERS_V1.json"


class VerificationError(RuntimeError):
    pass


def require(condition: bool, message: str) -> None:
    if not condition:
        raise VerificationError(message)


def load_json_no_duplicates(path: Path) -> dict[str, Any]:
    def hook(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
        result: dict[str, Any] = {}
        for key, value in pairs:
            if key in result:
                raise VerificationError(f"duplicate JSON key {key!r} in {path}")
            result[key] = value
        return result

    try:
        value = json.loads(path.read_text(encoding="utf-8"), object_pairs_hook=hook)
    except (OSError, json.JSONDecodeError) as error:
        raise VerificationError(f"cannot parse {path}: {error}") from error
    require(isinstance(value, dict), "caller manifest must be a JSON object")
    return value


def source_paths() -> list[Path]:
    paths: list[Path] = []
    for path in (ROOT / "codex-rs").rglob("*.rs"):
        if any(part in {"target", ".git"} for part in path.parts):
            continue
        paths.append(path)
    return sorted(paths)


def verify_symbol(symbol: str, contract: dict[str, Any], paths: list[Path]) -> None:
    allowed_values = contract.get("allowed_paths")
    require(isinstance(allowed_values, list) and allowed_values, f"{symbol}: allowed_paths missing")
    allowed = {Path(value).as_posix() for value in allowed_values}
    require(len(allowed) == len(allowed_values), f"{symbol}: duplicate allowed path")
    require(contract.get("production_callers") == [], f"{symbol}: production callers must be empty")
    require(contract.get("runtime_attached") is False, f"{symbol}: runtime must remain detached")

    observed: set[str] = set()
    for path in paths:
        text = path.read_text(encoding="utf-8")
        if symbol in text:
            observed.add(path.relative_to(ROOT).as_posix())
    require(observed == allowed, f"{symbol}: caller set drifted; observed={sorted(observed)} expected={sorted(allowed)}")


def main() -> int:
    try:
        manifest = load_json_no_duplicates(MANIFEST)
        require(manifest.get("schema") == "hepta.effect_dispatch_callers.v1", "wrong schema")
        require(manifest.get("schema_version") == 1, "wrong schema version")
        symbols = manifest.get("symbols")
        require(isinstance(symbols, dict) and symbols, "symbols must be non-empty")
        require(
            set(symbols)
            == {"ProductionOutboxDispatcher::attach", "ProductionDurableWriter::open"},
            "caller symbol set changed",
        )
        policy = manifest.get("policy")
        require(isinstance(policy, dict), "policy must be an object")
        require(policy.get("agentd_raw_writer_accessor_allowed") is False, "raw writer accessor reopened")
        require(policy.get("direct_product_dispatcher_allowed") is False, "direct dispatcher reopened")
        require(policy.get("typed_external_effect_capability_required") is True, "typed effect gate disabled")
        require(policy.get("physical_memory_extraction_allowed") is False, "physical Memory extraction enabled")
        paths = source_paths()
        for symbol, contract in symbols.items():
            require(isinstance(contract, dict), f"{symbol}: contract must be an object")
            verify_symbol(symbol, contract, paths)
        host = (ROOT / "codex-rs/hepta-agentd/src/production_writer_host.rs").read_text(
            encoding="utf-8"
        )
        require("pub fn writer(&self)" not in host, "Agentd raw writer accessor is public")
        require(
            "external_effect: Authorized<ExternalEffectCapability>" in host,
            "Agentd target attachment lacks typed effect authority",
        )
    except VerificationError as error:
        print(f"FAIL_EFFECT_DISPATCH_CALLER_RATCHET: {error}", file=sys.stderr)
        return 1

    print(
        json.dumps(
            {
                "result": "PASS_EFFECT_DISPATCH_CALLER_RATCHET_SOURCE",
                "raw_dispatcher_callers": 2,
                "production_callers": 0,
                "runtime_attached": False,
                "external_effect_authority": False,
            },
            sort_keys=True,
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
