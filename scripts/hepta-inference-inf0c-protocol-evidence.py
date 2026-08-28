#!/usr/bin/env python3
"""Qualification-only tool-call and protocol-fault evidence for INF-0C.

This harness verifies exact function-call structure, exact model inventory
stability, bounded strict SSE framing, timeout behavior, and malformed-event
rejection. It never downloads models and persists only digests, lengths, event
counts, timing, and closed authority.
"""

from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import pathlib
import socket
import sys
import time
import urllib.error
import urllib.request
from collections import Counter
from dataclasses import dataclass
from types import ModuleType
from typing import Any

TOOL_NAME = "hepta_probe"
TOOL_NONCE = "HEPTA_INF0C_TOOL"
TOOL_VALUE = 7
TOOL_PROMPT = (
    "Call the hepta_probe function exactly once with nonce "
    f"{TOOL_NONCE} and integer value {TOOL_VALUE}. Do not answer with text."
)
MAX_SSE_EVENT_BYTES = 256 * 1024
MAX_SSE_TOTAL_BYTES = 4 * 1024 * 1024
MAX_SSE_EVENTS = 4096
SSE_READ_CHUNK = 4096
ALLOWED_SSE_EVENT_TYPES = {
    "response.created",
    "response.in_progress",
    "response.output_item.added",
    "response.content_part.added",
    "response.function_call_arguments.delta",
    "response.function_call_arguments.done",
    "response.output_item.done",
    "response.completed",
    "error",
}


def load_minimal_harness() -> ModuleType:
    path = pathlib.Path(__file__).with_name("hepta-inference-inf0c-real-e2e.py")
    spec = importlib.util.spec_from_file_location("hepta_inf0c_protocol_base", path)
    if spec is None or spec.loader is None:
        raise RuntimeError("failed to load minimal INF-0C harness")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


BASE = load_minimal_harness()
QualificationError = BASE.QualificationError


@dataclass(frozen=True)
class Inventory:
    model_ids: tuple[str, ...]
    sha256: str
    byte_length: int


@dataclass(frozen=True)
class SseReceipt:
    status: int
    media_type: str
    total_bytes: int
    stream_sha256: str
    event_count: int
    event_type_counts: dict[str, int]
    completed: bool
    elapsed_ms: int


def require(condition: bool, message: str) -> None:
    if not condition:
        raise QualificationError(message)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--self-test", action="store_true")
    mode.add_argument("--execute", action="store_true", help="required real-service latch")
    parser.add_argument("--ollama-base", default="http://127.0.0.1:11434")
    parser.add_argument("--ollama-model")
    parser.add_argument("--lmstudio-base", default="http://127.0.0.1:1234/v1")
    parser.add_argument("--lmstudio-model")
    parser.add_argument("--timeout-seconds", type=float, default=60.0)
    parser.add_argument("--receipt", type=pathlib.Path)
    return parser.parse_args()


def canonical_inventory(model_ids: list[str], target_model: str, label: str) -> Inventory:
    require(model_ids, f"{label} model inventory is empty")
    validated: list[str] = []
    for model_id in model_ids:
        require(isinstance(model_id, str), f"{label} model inventory contains a non-string ID")
        BASE.validate_model_id(model_id)
        validated.append(model_id)
    require(len(validated) == len(set(validated)), f"{label} model inventory contains duplicate IDs")
    ordered = tuple(sorted(validated))
    require(target_model in ordered, f"{label} exact model is not installed")
    encoded = json.dumps(ordered, ensure_ascii=False, separators=(",", ":")).encode("utf-8")
    return Inventory(
        model_ids=ordered,
        sha256=hashlib.sha256(encoded).hexdigest(),
        byte_length=len(encoded),
    )


def inventory_receipt(inventory: Inventory) -> dict[str, Any]:
    return {
        "model_count": len(inventory.model_ids),
        "sha256": inventory.sha256,
        "byte_length": inventory.byte_length,
        "raw_model_ids_persisted": False,
    }


def fetch_ollama_inventory(base: str, model: str, timeout: float) -> tuple[Inventory, dict[str, Any]]:
    result = BASE.request_json("GET", f"{base}/api/tags", timeout)
    payload = BASE.parse_object(result, "Ollama models")
    entries = payload.get("models")
    require(isinstance(entries, list), "Ollama models array is missing")
    names: list[str] = []
    for entry in entries:
        require(isinstance(entry, dict), "Ollama model entry is not an object")
        name = entry.get("name")
        require(isinstance(name, str), "Ollama model entry is missing name")
        names.append(name)
    return canonical_inventory(names, model, "Ollama"), BASE.body_receipt(result)


def fetch_lmstudio_inventory(base: str, model: str, timeout: float) -> tuple[Inventory, dict[str, Any]]:
    result = BASE.request_json("GET", f"{base}/models", timeout)
    payload = BASE.parse_object(result, "LM Studio models")
    entries = payload.get("data")
    require(isinstance(entries, list), "LM Studio data array is missing")
    names: list[str] = []
    for entry in entries:
        require(isinstance(entry, dict), "LM Studio model entry is not an object")
        name = entry.get("id")
        require(isinstance(name, str), "LM Studio model entry is missing id")
        names.append(name)
    return canonical_inventory(names, model, "LM Studio"), BASE.body_receipt(result)


def tool_request(model: str) -> dict[str, Any]:
    return {
        "model": model,
        "input": TOOL_PROMPT,
        "tools": [
            {
                "type": "function",
                "name": TOOL_NAME,
                "description": "Qualification-only deterministic probe.",
                "parameters": {
                    "type": "object",
                    "additionalProperties": False,
                    "properties": {
                        "nonce": {"type": "string", "const": TOOL_NONCE},
                        "value": {"type": "integer", "const": TOOL_VALUE},
                    },
                    "required": ["nonce", "value"],
                },
                "strict": True,
            }
        ],
        "tool_choice": {"type": "function", "name": TOOL_NAME},
        "parallel_tool_calls": False,
        "max_output_tokens": 64,
        "stream": False,
    }


def _decode_arguments(value: Any, label: str) -> tuple[dict[str, Any], bytes]:
    if isinstance(value, str):
        encoded = value.encode("utf-8")
        try:
            decoded = json.loads(value)
        except json.JSONDecodeError as error:
            raise QualificationError(f"{label} function arguments are invalid JSON") from error
    elif isinstance(value, dict):
        decoded = value
        encoded = json.dumps(value, sort_keys=True, separators=(",", ":")).encode("utf-8")
    else:
        raise QualificationError(f"{label} function arguments have an unsupported type")
    require(isinstance(decoded, dict), f"{label} function arguments must be an object")
    return decoded, encoded


def extract_exact_tool_call(payload: dict[str, Any], label: str) -> dict[str, Any]:
    output = payload.get("output")
    require(isinstance(output, list), f"{label} output array is missing")
    calls: list[tuple[str, Any, str | None]] = []
    for item in output:
        if not isinstance(item, dict):
            continue
        item_type = item.get("type")
        if item_type not in {"function_call", "tool_call"}:
            continue
        function = item.get("function")
        if isinstance(function, dict):
            name = function.get("name")
            arguments = function.get("arguments")
        else:
            name = item.get("name")
            arguments = item.get("arguments")
        call_id = item.get("call_id") or item.get("id")
        if isinstance(name, str):
            calls.append((name, arguments, call_id if isinstance(call_id, str) else None))

    require(len(calls) == 1, f"{label} must return exactly one function call")
    name, raw_arguments, call_id = calls[0]
    require(name == TOOL_NAME, f"{label} function name mismatch")
    arguments, encoded = _decode_arguments(raw_arguments, label)
    require(
        set(arguments) == {"nonce", "value"},
        f"{label} function arguments contain missing or extra keys",
    )
    require(arguments.get("nonce") == TOOL_NONCE, f"{label} nonce mismatch")
    require(
        type(arguments.get("value")) is int and arguments["value"] == TOOL_VALUE,
        f"{label} integer value mismatch",
    )
    call_id_receipt: dict[str, Any] | None = None
    if call_id is not None:
        call_id_bytes = call_id.encode("utf-8")
        call_id_receipt = {
            "sha256": hashlib.sha256(call_id_bytes).hexdigest(),
            "byte_length": len(call_id_bytes),
            "raw_persisted": False,
        }
    return {
        "verified": True,
        "name": TOOL_NAME,
        "arguments_sha256": hashlib.sha256(encoded).hexdigest(),
        "arguments_byte_length": len(encoded),
        "raw_arguments_persisted": False,
        "call_id": call_id_receipt,
    }


def qualify_tool_call(
    label: str,
    base: str,
    model: str,
    timeout: float,
    fetch_inventory: Any,
    responses_path: str,
) -> dict[str, Any]:
    before, before_response = fetch_inventory(base, model, timeout)
    response = BASE.request_json(
        "POST",
        f"{base}{responses_path}",
        timeout,
        tool_request(model),
    )
    payload = BASE.parse_object(response, f"{label} Responses")
    tool_call = extract_exact_tool_call(payload, f"{label} Responses")
    after, after_response = fetch_inventory(base, model, timeout)
    require(
        before.sha256 == after.sha256
        and before.byte_length == after.byte_length
        and before.model_ids == after.model_ids,
        f"{label} model inventory changed during qualification",
    )
    return {
        "model_present": True,
        "model_inventory_unchanged": True,
        "inventory_before": inventory_receipt(before),
        "inventory_after": inventory_receipt(after),
        "inventory_before_response": before_response,
        "inventory_after_response": after_response,
        "tool_call": tool_call,
        "inference_response": BASE.body_receipt(response),
    }


def _decode_sse_event(block: bytes, sequence: int) -> tuple[str, dict[str, Any]]:
    require(len(block) <= MAX_SSE_EVENT_BYTES, "SSE event exceeded bounded size")
    try:
        text = block.decode("utf-8")
    except UnicodeDecodeError as error:
        raise QualificationError("SSE event is not valid UTF-8") from error
    event_name: str | None = None
    data_lines: list[str] = []
    for line in text.splitlines():
        if not line:
            continue
        if line.startswith(":"):
            continue
        field, separator, value = line.partition(":")
        require(bool(separator), "SSE line is missing a field separator")
        value = value[1:] if value.startswith(" ") else value
        if field == "event":
            require(event_name is None, "SSE event contains duplicate event fields")
            event_name = value
        elif field == "data":
            data_lines.append(value)
        else:
            raise QualificationError(f"SSE field {field!r} is not allowed")
    require(data_lines, "SSE event contains no data")
    data = "\n".join(data_lines)
    require(data != "[DONE]", "legacy [DONE] sentinel is not accepted")
    try:
        payload = json.loads(data)
    except json.JSONDecodeError as error:
        raise QualificationError("SSE data is invalid JSON") from error
    require(isinstance(payload, dict), "SSE data must be a JSON object")
    payload_type = payload.get("type")
    require(isinstance(payload_type, str), "SSE payload type is missing")
    if event_name is not None:
        require(event_name == payload_type, "SSE event and payload types differ")
    require(payload_type in ALLOWED_SSE_EVENT_TYPES, f"SSE event type {payload_type!r} is not allowed")
    if "sequence_number" in payload:
        require(
            type(payload["sequence_number"]) is int
            and payload["sequence_number"] == sequence,
            "SSE sequence number is not monotonic",
        )
    return payload_type, payload


def read_strict_sse(
    url: str,
    timeout: float,
    payload: dict[str, Any],
) -> SseReceipt:
    request = urllib.request.Request(
        url,
        method="POST",
        headers={
            "Accept": "text/event-stream",
            "Content-Type": "application/json",
            "Connection": "close",
            "X-Hepta-Qualification": "inf0c-strict-sse-v1",
        },
        data=json.dumps(payload, separators=(",", ":")).encode("utf-8"),
    )
    started = time.monotonic_ns()
    hasher = hashlib.sha256()
    total_bytes = 0
    buffer = bytearray()
    event_types: Counter[str] = Counter()
    completed = False
    status = 0
    media_type = ""
    try:
        with BASE.LOOPBACK_OPENER.open(request, timeout=timeout) as response:
            status = response.status
            require(200 <= status < 300, f"SSE request returned HTTP {status}")
            media_type = response.headers.get("Content-Type", "").split(";", 1)[0].strip().lower()
            require(media_type == "text/event-stream", "SSE response media type is not text/event-stream")
            while True:
                chunk = response.read(SSE_READ_CHUNK)
                if not chunk:
                    break
                total_bytes += len(chunk)
                require(total_bytes <= MAX_SSE_TOTAL_BYTES, "SSE stream exceeded total byte bound")
                hasher.update(chunk)
                buffer.extend(chunk)
                require(
                    len(buffer) <= MAX_SSE_EVENT_BYTES + 4,
                    "SSE event exceeded bounded size before delimiter",
                )
                while True:
                    delimiter = buffer.find(b"\n\n")
                    crlf_delimiter = buffer.find(b"\r\n\r\n")
                    if delimiter < 0 or (0 <= crlf_delimiter < delimiter):
                        delimiter = crlf_delimiter
                        delimiter_length = 4
                    else:
                        delimiter_length = 2
                    if delimiter < 0:
                        break
                    block = bytes(buffer[:delimiter])
                    del buffer[: delimiter + delimiter_length]
                    if not block.strip():
                        continue
                    require(sum(event_types.values()) < MAX_SSE_EVENTS, "SSE event count exceeded bound")
                    require(not completed, "SSE event appeared after response.completed")
                    event_type, event_payload = _decode_sse_event(
                        block,
                        sum(event_types.values()),
                    )
                    event_types[event_type] += 1
                    if event_type == "error":
                        error = event_payload.get("error")
                        raise QualificationError(
                            f"SSE error event: {type(error).__name__ if error is not None else 'unknown'}"
                        )
                    if event_type == "response.completed":
                        require(not completed, "SSE stream contains duplicate completion")
                        completed = True
    except urllib.error.HTTPError as error:
        raise QualificationError(f"SSE HTTP status {error.code}") from error
    except (urllib.error.URLError, TimeoutError, socket.timeout, OSError) as error:
        raise QualificationError(f"SSE request failed or timed out: {error}") from error
    require(not buffer.strip(), "SSE stream ended with an unterminated event")
    require(completed, "SSE stream ended without response.completed")
    elapsed_ms = max(0, (time.monotonic_ns() - started) // 1_000_000)
    return SseReceipt(
        status=status,
        media_type=media_type,
        total_bytes=total_bytes,
        stream_sha256=hasher.hexdigest(),
        event_count=sum(event_types.values()),
        event_type_counts=dict(sorted(event_types.items())),
        completed=completed,
        elapsed_ms=elapsed_ms,
    )


def sse_receipt(value: SseReceipt) -> dict[str, Any]:
    return {
        "status": value.status,
        "media_type": value.media_type,
        "total_bytes": value.total_bytes,
        "stream_sha256": value.stream_sha256,
        "event_count": value.event_count,
        "event_type_counts": value.event_type_counts,
        "completed": value.completed,
        "elapsed_ms": value.elapsed_ms,
        "raw_events_persisted": False,
    }


def expect_failure(operation: Any, label: str) -> None:
    try:
        operation()
    except QualificationError:
        return
    raise QualificationError(f"self-test expected failure: {label}")


def run_self_test() -> None:
    valid_payload = {
        "output": [
            {
                "type": "function_call",
                "name": TOOL_NAME,
                "arguments": json.dumps(
                    {"nonce": TOOL_NONCE, "value": TOOL_VALUE},
                    separators=(",", ":"),
                ),
                "call_id": "fixture-call",
            }
        ]
    }
    receipt = extract_exact_tool_call(valid_payload, "tool fixture")
    require(receipt.get("verified") is True, "tool-call parser self-test failed")
    expect_failure(
        lambda: extract_exact_tool_call(
            {
                "output": [
                    {
                        "type": "function_call",
                        "name": TOOL_NAME,
                        "arguments": json.dumps({"nonce": "wrong", "value": TOOL_VALUE}),
                    }
                ]
            },
            "nonce mismatch fixture",
        ),
        "tool nonce mismatch",
    )
    expect_failure(
        lambda: extract_exact_tool_call(
            {"output": valid_payload["output"] * 2},
            "duplicate tool fixture",
        ),
        "duplicate tool calls",
    )
    inventory = canonical_inventory(["z", "fixture", "a"], "fixture", "fixture")
    require(inventory.model_ids == ("a", "fixture", "z"), "inventory canonicalization failed")
    expect_failure(
        lambda: canonical_inventory(["fixture", "fixture"], "fixture", "fixture"),
        "duplicate model inventory",
    )
    print("PASS_HEPTA_INFERENCE_INF0C_PROTOCOL_PARSER_SELF_TEST")


def main() -> int:
    args = parse_args()
    if args.self_test:
        run_self_test()
        return 0

    require(args.execute, "--execute is required")
    require(args.timeout_seconds > 0, "timeout must be positive")
    require(args.receipt is not None, "--receipt is required")
    require(args.ollama_model is not None, "--ollama-model is required")
    require(args.lmstudio_model is not None, "--lmstudio-model is required")
    BASE.validate_model_id(args.ollama_model)
    BASE.validate_model_id(args.lmstudio_model)
    ollama_base = BASE.normalize_loopback_base(args.ollama_base)
    lmstudio_base = BASE.normalize_loopback_base(args.lmstudio_base)

    ollama = qualify_tool_call(
        "Ollama",
        ollama_base,
        args.ollama_model,
        args.timeout_seconds,
        fetch_ollama_inventory,
        "/v1/responses",
    )
    lmstudio = qualify_tool_call(
        "LM Studio",
        lmstudio_base,
        args.lmstudio_model,
        args.timeout_seconds,
        fetch_lmstudio_inventory,
        "/responses",
    )

    prompt = TOOL_PROMPT.encode("utf-8")
    expected_arguments = json.dumps(
        {"nonce": TOOL_NONCE, "value": TOOL_VALUE},
        sort_keys=True,
        separators=(",", ":"),
    ).encode("utf-8")
    receipt = {
        "schema": "hepta.inference.inf0c.protocol_evidence.v1",
        "source": {
            "commit": BASE.git_value("rev-parse", "HEAD"),
            "tree": BASE.git_value("rev-parse", "HEAD^{tree}"),
        },
        "scope": "QUALIFICATION_ONLY_TOOL_CALL_AND_PROTOCOL_EVIDENCE",
        "prompt": {
            "sha256": hashlib.sha256(prompt).hexdigest(),
            "byte_length": len(prompt),
            "raw_persisted": False,
        },
        "expected_tool": {
            "name": TOOL_NAME,
            "arguments_sha256": hashlib.sha256(expected_arguments).hexdigest(),
            "arguments_byte_length": len(expected_arguments),
            "raw_arguments_persisted": False,
        },
        "ollama": ollama,
        "lmstudio": lmstudio,
        "model_inventory_unchanged": True,
        "implicit_download": False,
        "malformed_event_fixture_executed": False,
        "timeout_fixture_executed": False,
        "raw_model_output_persisted": False,
        "raw_tool_arguments_persisted": False,
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
    print("PASS_HEPTA_INFERENCE_INF0C_PROTOCOL_EVIDENCE")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except QualificationError as error:
        print(f"FAIL_HEPTA_INFERENCE_INF0C_PROTOCOL_EVIDENCE: {error}", file=sys.stderr)
        raise SystemExit(1) from error
