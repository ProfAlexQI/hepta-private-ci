#!/usr/bin/env python3
"""Classify exact local-provider protocol capabilities without overclaiming.

For each fixed provider/model tuple this qualification-only harness records one
of two acceptable terminal capability states:

* QUALIFIED: the exact contract executed successfully.
* UNSUPPORTED_FAIL_CLOSED: the exact tuple did not satisfy the contract and
  must be rejected before backend dispatch.

Transport failures, timeouts, invalid inventory, or mutable model inventory are
FAILED_CLOSED and keep the qualification run red. Receipts contain only bounded
metadata and digests; raw prompts, model output, SSE frames, tool arguments and
failure text are never persisted.
"""

from __future__ import annotations

import argparse
import hashlib
import http.server
import importlib.util
import json
import pathlib
import socket
import sys
import threading
import time
from dataclasses import asdict
from types import ModuleType
from typing import Any, Callable

QUALIFIED = "QUALIFIED"
UNSUPPORTED = "UNSUPPORTED_FAIL_CLOSED"
FAILED = "FAILED_CLOSED"
DISPATCH_REJECT = "REJECT_BEFORE_BACKEND_DISPATCH"


def load_module(name: str, filename: str) -> ModuleType:
    path = pathlib.Path(__file__).with_name(filename)
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"failed to load {filename}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


BASE = load_module("hepta_inf0c_capability_base", "hepta-inference-inf0c-real-e2e.py")
PROTOCOL = load_module(
    "hepta_inf0c_capability_protocol",
    "hepta-inference-inf0c-protocol-evidence.py",
)
QualificationError = BASE.QualificationError


def require(condition: bool, message: str) -> None:
    if not condition:
        raise QualificationError(message)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--self-test", action="store_true")
    mode.add_argument("--execute", action="store_true")
    parser.add_argument("--ollama-base", default="http://127.0.0.1:11434")
    parser.add_argument("--ollama-model")
    parser.add_argument("--lmstudio-base", default="http://127.0.0.1:1234/v1")
    parser.add_argument("--lmstudio-model")
    parser.add_argument("--timeout-seconds", type=float, default=180.0)
    parser.add_argument("--receipt", type=pathlib.Path)
    return parser.parse_args()


def digest_text(value: str) -> dict[str, Any]:
    encoded = value.encode("utf-8")
    return {
        "sha256": hashlib.sha256(encoded).hexdigest(),
        "byte_length": len(encoded),
        "raw_persisted": False,
    }


def error_receipt(error: BaseException) -> dict[str, Any]:
    # The message can contain endpoint details or provider diagnostics. Persist
    # only its digest and the exception class.
    return {
        "class": type(error).__name__,
        "message": digest_text(str(error)),
    }


def is_transport_failure(message: str) -> bool:
    lowered = message.lower()
    return any(
        marker in lowered
        for marker in (
            "request failed",
            "failed or timed out",
            "timed out",
            "timeout",
            "connection refused",
            "connection reset",
            "name or service not known",
            "temporary failure",
            "http response exceeded",
            "stream exceeded total byte bound",
        )
    )


def classify_contract_error(error: BaseException) -> str:
    return FAILED if is_transport_failure(str(error)) else UNSUPPORTED


def inventory_equal(before: Any, after: Any) -> bool:
    return (
        before.model_ids == after.model_ids
        and before.sha256 == after.sha256
        and before.byte_length == after.byte_length
    )


def capability_result(
    status: str,
    *,
    evidence: dict[str, Any] | None = None,
    error: BaseException | None = None,
) -> dict[str, Any]:
    require(status in {QUALIFIED, UNSUPPORTED, FAILED}, "invalid capability status")
    result: dict[str, Any] = {
        "status": status,
        "qualified": status == QUALIFIED,
        "unsupported_fail_closed": status == UNSUPPORTED,
        "dispatch_policy": None if status == QUALIFIED else DISPATCH_REJECT,
        "text_fallback_allowed": False,
        "remote_fallback_allowed": False,
        "implicit_model_switch_allowed": False,
        "implicit_model_install_allowed": False,
        "raw_provider_output_persisted": False,
    }
    if evidence is not None:
        result["evidence"] = evidence
    if error is not None:
        result["failure"] = error_receipt(error)
    return result


def qualify_tool_capability(
    label: str,
    base: str,
    model: str,
    timeout: float,
    fetch_inventory: Callable[..., Any],
    responses_path: str,
) -> dict[str, Any]:
    before, before_response = fetch_inventory(base, model, timeout)
    status = FAILED
    evidence: dict[str, Any] | None = None
    captured: BaseException | None = None
    try:
        response = BASE.request_json(
            "POST",
            f"{base}{responses_path}",
            timeout,
            PROTOCOL.tool_request(model),
        )
        payload = BASE.parse_object(response, f"{label} Responses")
        tool_call = PROTOCOL.extract_exact_tool_call(payload, f"{label} Responses")
        status = QUALIFIED
        evidence = {
            "tool_call": tool_call,
            "response": BASE.body_receipt(response),
        }
    except QualificationError as error:
        captured = error
        status = classify_contract_error(error)

    after, after_response = fetch_inventory(base, model, timeout)
    if not inventory_equal(before, after):
        captured = QualificationError(f"{label} inventory changed during capability classification")
        status = FAILED
        evidence = None

    result = capability_result(status, evidence=evidence, error=captured)
    result["inventory_unchanged"] = inventory_equal(before, after)
    result["inventory_before"] = PROTOCOL.inventory_receipt(before)
    result["inventory_after"] = PROTOCOL.inventory_receipt(after)
    result["inventory_before_response"] = before_response
    result["inventory_after_response"] = after_response
    return result


def semantic_stream_request(model: str) -> dict[str, Any]:
    return {
        "model": model,
        "input": BASE.PROMPT,
        "max_output_tokens": 32,
        "stream": True,
    }


def qualify_sse_capability(
    base: str,
    model: str,
    timeout: float,
    responses_path: str,
) -> dict[str, Any]:
    try:
        receipt = PROTOCOL.read_strict_sse(
            f"{base}{responses_path}",
            timeout,
            semantic_stream_request(model),
        )
        return capability_result(
            QUALIFIED,
            evidence={
                "stream": PROTOCOL.sse_receipt(receipt),
                "terminal_completion_verified": True,
            },
        )
    except QualificationError as error:
        return capability_result(classify_contract_error(error), error=error)


class FixtureHandler(http.server.BaseHTTPRequestHandler):
    protocol_version = "HTTP/1.1"

    def log_message(self, format: str, *args: object) -> None:
        del format, args

    @staticmethod
    def event(event_type: str, sequence: int, payload: dict[str, Any] | None = None) -> bytes:
        value = {"type": event_type, "sequence_number": sequence}
        if payload:
            value.update(payload)
        encoded = json.dumps(value, separators=(",", ":")).encode("utf-8")
        return b"event: " + event_type.encode("utf-8") + b"\ndata: " + encoded + b"\n\n"

    def do_POST(self) -> None:
        length = int(self.headers.get("Content-Length", "0"))
        if length:
            self.rfile.read(length)
        if self.path == "/timeout":
            time.sleep(0.25)
            return
        self.send_response(200)
        self.send_header("Content-Type", "text/event-stream")
        self.send_header("Connection", "close")
        self.end_headers()
        if self.path == "/valid":
            payload = self.event("response.created", 0) + self.event("response.completed", 1)
        elif self.path == "/duplicate-terminal":
            payload = self.event("response.completed", 0) + self.event("response.completed", 1)
        elif self.path == "/post-terminal":
            payload = self.event("response.completed", 0) + self.event("response.created", 1)
        elif self.path == "/truncated":
            payload = b'event: response.created\ndata: {"type":"response.created"}'
        elif self.path == "/legacy-done":
            payload = b"data: [DONE]\n\n"
        elif self.path == "/unknown":
            payload = self.event("response.unknown", 0)
        elif self.path == "/mismatch":
            payload = b'event: response.created\ndata: {"type":"response.completed","sequence_number":0}\n\n'
        elif self.path == "/malformed-json":
            payload = b"event: response.created\ndata: {not-json}\n\n"
        elif self.path == "/oversized":
            payload = b"event: response.created\ndata: " + b"x" * (PROTOCOL.MAX_SSE_EVENT_BYTES + 1) + b"\n\n"
        else:
            self.send_error(404)
            return
        try:
            self.wfile.write(payload)
            self.wfile.flush()
        except BrokenPipeError:
            pass


def expect_failure(operation: Callable[[], Any], label: str) -> None:
    try:
        operation()
    except QualificationError:
        return
    raise QualificationError(f"fixture unexpectedly passed: {label}")


def run_sse_fixture_suite() -> dict[str, bool]:
    server = http.server.ThreadingHTTPServer(("127.0.0.1", 0), FixtureHandler)
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()
    host, port = server.server_address
    root = f"http://{host}:{port}"
    payload = {"model": "fixture", "input": "fixture", "stream": True}
    try:
        valid = PROTOCOL.read_strict_sse(f"{root}/valid", 2.0, payload)
        require(valid.completed and valid.event_count == 2, "valid SSE fixture failed")
        cases = {
            "malformed_json": "/malformed-json",
            "unknown_event": "/unknown",
            "event_payload_mismatch": "/mismatch",
            "oversized_event": "/oversized",
            "truncated_event": "/truncated",
            "duplicate_completion": "/duplicate-terminal",
            "post_terminal_event": "/post-terminal",
            "legacy_done": "/legacy-done",
        }
        for label, path in cases.items():
            expect_failure(
                lambda path=path: PROTOCOL.read_strict_sse(f"{root}{path}", 2.0, payload),
                label,
            )
        expect_failure(
            lambda: PROTOCOL.read_strict_sse(f"{root}/timeout", 0.05, payload),
            "timeout",
        )
        return {
            "valid_stream": True,
            **{label: True for label in cases},
            "timeout": True,
        }
    finally:
        server.shutdown()
        server.server_close()
        thread.join(timeout=2.0)


def run_self_test() -> None:
    fixtures = run_sse_fixture_suite()
    require(all(fixtures.values()), "SSE fixture suite was incomplete")
    require(classify_contract_error(QualificationError("SSE HTTP status 400")) == UNSUPPORTED, "HTTP contract rejection classification failed")
    require(classify_contract_error(QualificationError("request failed for /responses: refused")) == FAILED, "transport failure classification failed")
    print("PASS_HEPTA_INFERENCE_INF0C_CAPABILITY_CLASSIFIER_SELF_TEST")


def main() -> int:
    args = parse_args()
    if args.self_test:
        run_self_test()
        return 0

    require(args.execute, "--execute is required")
    require(args.receipt is not None, "--receipt is required")
    require(args.ollama_model is not None, "--ollama-model is required")
    require(args.lmstudio_model is not None, "--lmstudio-model is required")
    require(args.timeout_seconds > 0, "timeout must be positive")
    BASE.validate_model_id(args.ollama_model)
    BASE.validate_model_id(args.lmstudio_model)
    ollama_base = BASE.normalize_loopback_base(args.ollama_base)
    lmstudio_base = BASE.normalize_loopback_base(args.lmstudio_base)

    fixtures = run_sse_fixture_suite()
    providers = {
        "ollama": {
            "tool_call": qualify_tool_capability(
                "Ollama",
                ollama_base,
                args.ollama_model,
                args.timeout_seconds,
                PROTOCOL.fetch_ollama_inventory,
                "/v1/responses",
            ),
            "strict_sse": qualify_sse_capability(
                ollama_base,
                args.ollama_model,
                args.timeout_seconds,
                "/v1/responses",
            ),
        },
        "lmstudio": {
            "tool_call": qualify_tool_capability(
                "LM Studio",
                lmstudio_base,
                args.lmstudio_model,
                args.timeout_seconds,
                PROTOCOL.fetch_lmstudio_inventory,
                "/responses",
            ),
            "strict_sse": qualify_sse_capability(
                lmstudio_base,
                args.lmstudio_model,
                args.timeout_seconds,
                "/responses",
            ),
        },
    }
    statuses = [
        capability["status"]
        for provider in providers.values()
        for capability in provider.values()
    ]
    matrix_complete = all(status in {QUALIFIED, UNSUPPORTED} for status in statuses)
    receipt = {
        "schema": "hepta.inference.inf0c.capability_classification.v2",
        "source": {
            "commit": BASE.git_value("rev-parse", "HEAD"),
            "tree": BASE.git_value("rev-parse", "HEAD^{tree}"),
        },
        "scope": "QUALIFICATION_ONLY_EXACT_TUPLE_CAPABILITY_CLASSIFICATION",
        "providers": providers,
        "strict_sse_fault_fixtures": fixtures,
        "capability_matrix_complete": matrix_complete,
        "accepted_terminal_states": [QUALIFIED, UNSUPPORTED],
        "unsupported_policy": {
            "dispatch": DISPATCH_REJECT,
            "text_fallback": False,
            "remote_fallback": False,
            "implicit_model_switch": False,
            "implicit_model_install": False,
        },
        "raw_prompt_persisted": False,
        "raw_model_output_persisted": False,
        "raw_tool_arguments_persisted": False,
        "raw_sse_events_persisted": False,
        "authority": {
            "production": False,
            "effect": False,
            "memory_write": False,
            "kg_write": False,
            "remote_inference": False,
            "promotion": False,
        },
        "qualified": False,
    }
    BASE.write_receipt(args.receipt, receipt)
    if not matrix_complete:
        print("FAIL_HEPTA_INFERENCE_INF0C_CAPABILITY_MATRIX", file=sys.stderr)
        return 1
    print("PASS_HEPTA_INFERENCE_INF0C_CAPABILITY_MATRIX_COMPLETE")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except QualificationError as error:
        print(
            "FAIL_HEPTA_INFERENCE_INF0C_CAPABILITY_CLASSIFICATION: "
            f"{type(error).__name__}",
            file=sys.stderr,
        )
        raise SystemExit(1) from error
