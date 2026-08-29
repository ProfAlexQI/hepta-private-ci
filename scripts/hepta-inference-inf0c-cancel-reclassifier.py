#!/usr/bin/env python3
"""Reclassify the immutable INF-0C v8.2 cancellation observation.

The real-service probe returned a bounded 2xx JSON response with a canonical
response id and status, but the requested background operation did not enter
one of the contractually cancellable states (queued/in_progress). The v3 probe
therefore emitted a digest-only diagnostic rather than a capability result.

For that exact model/provider/probe tuple, this gate conservatively classifies
explicit provider cancellation as UNSUPPORTED_FAIL_CLOSED. It never turns that
classification into cancellation acknowledgement, never treats transport
disconnect as acknowledgement, and requires cancel-required work to be rejected
before backend dispatch.
"""
from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
from typing import Any

EXPECTED_ARTIFACT_SHA256 = "83f5346ee3d7107b794e6466a5aef007d3b087f30ff2850074a73d8a71e353ad"
EXPECTED_SOURCE_COMMIT = "cd3344d767667b177d6d21c7198bade228d010bb"
EXPECTED_SOURCE_TREE = "be25744e47d3b2396aceaf4dcac81b720b5112e3"
PROVIDERS = ("ollama", "lmstudio")


class ClassificationError(RuntimeError):
    pass


def need(condition: bool, message: str) -> None:
    if not condition:
        raise ClassificationError(message)


def load_object(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise ClassificationError(f"invalid receipt: {error}") from error
    need(isinstance(value, dict), "receipt must be a JSON object")
    return value


def diagnostic_digest(provider: str) -> str:
    message = f"{provider} background response was not cancellable"
    return hashlib.sha256(message.encode("utf-8")).hexdigest()


def classify(receipt: dict[str, Any]) -> dict[str, Any]:
    need(receipt.get("schema") == "hepta.inference.inf0c.runtime_closure.v1", "schema drift")
    evidence = receipt.get("immutable_evidence")
    need(isinstance(evidence, dict), "immutable evidence missing")
    need(evidence.get("artifact_sha256") == EXPECTED_ARTIFACT_SHA256, "artifact digest drift")
    source = evidence.get("source")
    need(isinstance(source, dict), "source binding missing")
    need(source.get("commit") == EXPECTED_SOURCE_COMMIT, "source commit drift")
    need(source.get("tree") == EXPECTED_SOURCE_TREE, "source tree drift")

    cancellation = receipt.get("explicit_provider_cancellation")
    need(isinstance(cancellation, dict), "cancellation section missing")
    providers = cancellation.get("providers")
    need(isinstance(providers, dict) and set(providers) == set(PROVIDERS), "provider set drift")

    output: dict[str, Any] = {}
    for provider in PROVIDERS:
        value = providers[provider]
        need(isinstance(value, dict), f"{provider} observation missing")
        need(
            value.get("original_error_digest") == diagnostic_digest(provider),
            f"{provider} diagnostic digest mismatch",
        )
        need(value.get("original_probe_classification") == "failed_closed", f"{provider} original class drift")
        need(value.get("transport_disconnect_used") is False, f"{provider} disconnect conflation")
        need(value.get("provider_cancel_acknowledged") is False, f"{provider} acknowledgement invented")
        need(value.get("classification") == "UNSUPPORTED_FAIL_CLOSED", f"{provider} class drift")
        need(value.get("provider_cancel_capability_classified") is True, f"{provider} unclassified")
        output[provider] = {
            "classification": "UNSUPPORTED_FAIL_CLOSED",
            "provider_cancel_capability_classified": True,
            "provider_cancel_acknowledged": False,
            "transport_disconnect_used": False,
        }

    policy = cancellation.get("dispatch_policy")
    need(isinstance(policy, dict), "dispatch policy missing")
    need(policy.get("cancel_required_request") == "REJECT_BEFORE_BACKEND_DISPATCH", "dispatch policy drift")
    for field in ("text_fallback", "remote_fallback", "implicit_model_switch", "implicit_model_install"):
        need(policy.get(field) is False, f"dispatch policy enables {field}")

    need(cancellation.get("backend_cancellation_acknowledged") is False, "backend acknowledgement invented")
    need(cancellation.get("transport_disconnect_used_as_ack") is False, "disconnect promoted to acknowledgement")
    return output


def self_test() -> None:
    expected = {
        provider: {
            "classification": "UNSUPPORTED_FAIL_CLOSED",
            "provider_cancel_capability_classified": True,
            "provider_cancel_acknowledged": False,
            "transport_disconnect_used": False,
        }
        for provider in PROVIDERS
    }
    fixture = {
        "schema": "hepta.inference.inf0c.runtime_closure.v1",
        "immutable_evidence": {
            "artifact_sha256": EXPECTED_ARTIFACT_SHA256,
            "source": {"commit": EXPECTED_SOURCE_COMMIT, "tree": EXPECTED_SOURCE_TREE},
        },
        "explicit_provider_cancellation": {
            "providers": {
                provider: {
                    "original_error_digest": diagnostic_digest(provider),
                    "original_probe_classification": "failed_closed",
                    "classification": "UNSUPPORTED_FAIL_CLOSED",
                    "provider_cancel_capability_classified": True,
                    "provider_cancel_acknowledged": False,
                    "transport_disconnect_used": False,
                }
                for provider in PROVIDERS
            },
            "backend_cancellation_acknowledged": False,
            "transport_disconnect_used_as_ack": False,
            "dispatch_policy": {
                "cancel_required_request": "REJECT_BEFORE_BACKEND_DISPATCH",
                "text_fallback": False,
                "remote_fallback": False,
                "implicit_model_switch": False,
                "implicit_model_install": False,
            },
        },
    }
    need(classify(fixture) == expected, "positive fixture failed")
    fixture["explicit_provider_cancellation"]["providers"]["ollama"]["provider_cancel_acknowledged"] = True
    try:
        classify(fixture)
    except ClassificationError:
        return
    raise ClassificationError("negative acknowledgement fixture did not fail closed")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--receipt", type=Path)
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    need(args.self_test or args.receipt is not None, "--self-test or --receipt is required")
    if args.self_test:
        self_test()
    if args.receipt is not None:
        classify(load_object(args.receipt))
    print("PASS_HEPTA_INFERENCE_INF0C_CANCEL_RECLASSIFICATION")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except ClassificationError as error:
        print(f"FAIL_HEPTA_INFERENCE_INF0C_CANCEL_RECLASSIFICATION: {error}")
        raise SystemExit(1) from error
