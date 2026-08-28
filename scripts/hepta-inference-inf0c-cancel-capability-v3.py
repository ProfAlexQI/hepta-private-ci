#!/usr/bin/env python3
"""Qualification-only explicit provider cancellation capability probe.

The probe uses only a pinned loopback Responses endpoint. It distinguishes an
explicit background-response cancellation acknowledgement from a client-side
transport disconnect. Unsupported providers are classified fail-closed and are
never represented as having acknowledged cancellation.
"""

from __future__ import annotations

import argparse
import hashlib
import importlib.util
import pathlib
import re
import sys
import time
import urllib.error
import urllib.parse
import urllib.request
from dataclasses import dataclass
from types import ModuleType
from typing import Any

MAX_HTTP_BODY = 4 * 1024 * 1024
MAX_RESPONSE_ID_BYTES = 128
POLL_INTERVAL_SECONDS = 0.25
UNSUPPORTED_HTTP_STATUSES = {400, 404, 405, 409, 422, 501}
INITIAL_STATUSES = {"queued", "in_progress"}
CANCELLED_STATUSES = {"cancelled", "canceled"}
TERMINAL_NON_CANCELLED_STATUSES = {"completed", "failed", "incomplete"}
RESPONSE_ID_PATTERN = re.compile(r"^[A-Za-z0-9][A-Za-z0-9._-]{0,127}$")
CANCEL_PROMPT = (
    "Continue emitting the token HEPTA_INF0C_CANCEL_PROBE until the request "
    "is explicitly cancelled by the client."
)


def load_minimal_harness() -> ModuleType:
    path = pathlib.Path(__file__).with_name("hepta-inference-inf0c-real-e2e.py")
    spec = importlib.util.spec_from_file_location("hepta_inf0c_minimal_e2e", path)
    if spec is None or spec.loader is None:
        raise RuntimeError("failed to load minimal INF-0C harness")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


BASE = load_minimal_harness()
QualificationError = BASE.QualificationError


@dataclass(frozen=True)
class HttpResult:
    status: int
    media_type: str
    body: bytes
    elapsed_ms: int


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--execute", action="store_true", help="required safety latch")
    parser.add_argument("--ollama-base", default="http://127.0.0.1:11434")
    parser.add_argument("--ollama-model", required=True)
    parser.add_argument("--lmstudio-base", default="http://127.0.0.1:1234/v1")
    parser.add_argument("--lmstudio-model", required=True)
    parser.add_argument("--timeout-seconds", type=float, default=60.0)
    parser.add_argument("--poll-timeout-seconds", type=float, default=30.0)
    parser.add_argument("--receipt", type=pathlib.Path, required=True)
    return parser.parse_args()


def require(condition: bool, message: str) -> None:
    if not condition:
        raise QualificationError(message)


def normalize_status(value: Any, label: str) -> str:
    require(isinstance(value, str), f"{label} status is missing")
    normalized = value.strip().lower()
    require(normalized == value.lower(), f"{label} status is not canonical")
    require(
        0 < len(normalized) <= 64
        and all(character.isalnum() or character in "_-" for character in normalized),
        f"{label} status is invalid",
    )
    return normalized


def validate_response_id(value: Any, label: str) -> tuple[str, dict[str, Any]]:
    require(isinstance(value, str), f"{label} response id is missing")
    encoded = value.encode("utf-8")
    require(0 < len(encoded) <= MAX_RESPONSE_ID_BYTES, f"{label} response id length is invalid")
    require(value not in {".", ".."}, f"{label} response id is invalid")
    require(RESPONSE_ID_PATTERN.fullmatch(value) is not None, f"{label} response id is not canonical")
    return value, {
        "sha256": hashlib.sha256(encoded).hexdigest(),
        "byte_length": len(encoded),
        "raw_persisted": False,
    }


def media_type(headers: Any) -> str:
    return headers.get("Content-Type", "").split(";", 1)[0].strip().lower()


def request_bounded_json(
    method: str,
    url: str,
    timeout: float,
    payload: dict[str, Any] | None = None,
) -> HttpResult:
    headers = {"Accept": "application/json", "Connection": "close"}
    data = None
    if payload is not None:
        data = BASE.json.dumps(payload, separators=(",", ":")).encode("utf-8")
        headers["Content-Type"] = "application/json"
    request = urllib.request.Request(url, method=method, headers=headers, data=data)
    started = time.monotonic_ns()
    try:
        with BASE.LOOPBACK_OPENER.open(request, timeout=timeout) as response:
            body = response.read(MAX_HTTP_BODY + 1)
            status = response.status
            content_type = media_type(response.headers)
    except urllib.error.HTTPError as error:
        body = error.read(MAX_HTTP_BODY + 1)
        status = error.code
        content_type = media_type(error.headers)
    except (urllib.error.URLError, TimeoutError, OSError) as error:
        raise QualificationError(
            f"request failed for {urllib.parse.urlsplit(url).path}: {error}"
        ) from error
    elapsed_ms = max(0, (time.monotonic_ns() - started) // 1_000_000)
    require(len(body) <= MAX_HTTP_BODY, "HTTP response exceeded bounded body limit")
    require(
        content_type == "application/json",
        f"unexpected response media type {content_type or '<missing>'}",
    )
    return HttpResult(status=status, media_type=content_type, body=body, elapsed_ms=elapsed_ms)


def parse_object(result: HttpResult, label: str) -> dict[str, Any]:
    try:
        value = BASE.json.loads(result.body)
    except (UnicodeDecodeError, BASE.json.JSONDecodeError) as error:
        raise QualificationError(f"{label} returned invalid JSON") from error
    require(isinstance(value, dict), f"{label} must return a JSON object")
    return value


def http_receipt(result: HttpResult) -> dict[str, Any]:
    return {
        "status": result.status,
        "media_type": result.media_type,
        "byte_length": len(result.body),
        "sha256": hashlib.sha256(result.body).hexdigest(),
        "elapsed_ms": result.elapsed_ms,
    }


def model_binding(model: str) -> dict[str, Any]:
    BASE.validate_model_id(model)
    encoded = model.encode("utf-8")
    return {
        "sha256": hashlib.sha256(encoded).hexdigest(),
        "byte_length": len(encoded),
        "raw_persisted": False,
    }


def provider_paths(provider: str, normalized_base: str) -> tuple[str, str]:
    if provider == "ollama":
        return f"{normalized_base}/v1/responses", normalized_base
    if provider == "lmstudio":
        return f"{normalized_base}/responses", normalized_base
    raise QualificationError("unsupported provider identifier")


def unsupported_result(
    provider: str,
    model: str,
    phase: str,
    response: HttpResult,
    response_id: dict[str, Any] | None = None,
) -> dict[str, Any]:
    result: dict[str, Any] = {
        "provider": provider,
        "model": model_binding(model),
        "classification": "explicit_cancel_unsupported",
        "unsupported_phase": phase,
        "provider_cancel_capability_classified": True,
        "provider_cancel_acknowledged": False,
        "transport_disconnect_used": False,
        "response": http_receipt(response),
    }
    if response_id is not None:
        result["response_id"] = response_id
    return result


def probe_provider(
    provider: str,
    base: str,
    model: str,
    timeout: float,
    poll_timeout: float,
) -> dict[str, Any]:
    normalized_base = BASE.normalize_loopback_base(base)
    responses_url, _ = provider_paths(provider, normalized_base)
    create = request_bounded_json(
        "POST",
        responses_url,
        timeout,
        {
            "model": model,
            "input": CANCEL_PROMPT,
            "max_output_tokens": 512,
            "background": True,
            "store": True,
            "stream": False,
        },
    )
    if create.status in UNSUPPORTED_HTTP_STATUSES:
        return unsupported_result(provider, model, "background_create", create)
    require(200 <= create.status < 300, f"{provider} background create returned HTTP {create.status}")
    created = parse_object(create, f"{provider} background create")
    response_id, response_id_receipt = validate_response_id(created.get("id"), provider)
    initial_status = normalize_status(created.get("status"), f"{provider} background create")
    require(initial_status in INITIAL_STATUSES, f"{provider} background response was not cancellable")

    cancel_url = f"{responses_url}/{response_id}/cancel"
    cancel = request_bounded_json("POST", cancel_url, timeout, {})
    if cancel.status in UNSUPPORTED_HTTP_STATUSES:
        return unsupported_result(
            provider,
            model,
            "cancel_endpoint",
            cancel,
            response_id_receipt,
        )
    require(200 <= cancel.status < 300, f"{provider} cancel returned HTTP {cancel.status}")
    cancelled = parse_object(cancel, f"{provider} cancel")
    cancel_id, _ = validate_response_id(cancelled.get("id"), f"{provider} cancel")
    require(cancel_id == response_id, f"{provider} cancel response id mismatch")
    cancel_status = normalize_status(cancelled.get("status"), f"{provider} cancel")
    require(cancel_status in CANCELLED_STATUSES, f"{provider} cancel did not acknowledge cancellation")

    retrieve_url = f"{responses_url}/{response_id}"
    deadline = time.monotonic() + poll_timeout
    retrieve_attempts = 0
    terminal_result: HttpResult | None = None
    terminal_status = ""
    while time.monotonic() < deadline:
        retrieve_attempts += 1
        retrieved = request_bounded_json("GET", retrieve_url, timeout)
        if retrieved.status in UNSUPPORTED_HTTP_STATUSES:
            return unsupported_result(
                provider,
                model,
                "terminal_retrieve",
                retrieved,
                response_id_receipt,
            )
        require(200 <= retrieved.status < 300, f"{provider} retrieve returned HTTP {retrieved.status}")
        payload = parse_object(retrieved, f"{provider} retrieve")
        retrieved_id, _ = validate_response_id(payload.get("id"), f"{provider} retrieve")
        require(retrieved_id == response_id, f"{provider} retrieve response id mismatch")
        observed = normalize_status(payload.get("status"), f"{provider} retrieve")
        if observed in CANCELLED_STATUSES:
            terminal_result = retrieved
            terminal_status = observed
            break
        require(
            observed not in TERMINAL_NON_CANCELLED_STATUSES,
            f"{provider} reached terminal non-cancelled status {observed}",
        )
        time.sleep(POLL_INTERVAL_SECONDS)
    require(terminal_result is not None, f"{provider} cancellation terminal state timed out")

    return {
        "provider": provider,
        "model": model_binding(model),
        "classification": "explicit_background_cancel_acknowledged",
        "provider_cancel_capability_classified": True,
        "provider_cancel_acknowledged": True,
        "transport_disconnect_used": False,
        "response_id": response_id_receipt,
        "initial_status": initial_status,
        "cancel_status": cancel_status,
        "terminal_status": terminal_status,
        "terminal_retrieve_attempts": retrieve_attempts,
        "background_create": http_receipt(create),
        "cancel_response": http_receipt(cancel),
        "terminal_retrieve": http_receipt(terminal_result),
    }


def safe_probe(
    provider: str,
    base: str,
    model: str,
    timeout: float,
    poll_timeout: float,
) -> tuple[dict[str, Any], str | None]:
    try:
        return probe_provider(provider, base, model, timeout, poll_timeout), None
    except QualificationError as error:
        return (
            {
                "provider": provider,
                "model": model_binding(model),
                "classification": "failed_closed",
                "provider_cancel_capability_classified": False,
                "provider_cancel_acknowledged": False,
                "transport_disconnect_used": False,
                "error_digest": hashlib.sha256(str(error).encode("utf-8")).hexdigest(),
                "raw_error_persisted": False,
            },
            str(error),
        )


def main() -> int:
    args = parse_args()
    require(args.execute, "--execute is required")
    require(args.timeout_seconds > 0, "timeout must be positive")
    require(args.poll_timeout_seconds > 0, "poll timeout must be positive")
    BASE.validate_model_id(args.ollama_model)
    BASE.validate_model_id(args.lmstudio_model)

    ollama, ollama_error = safe_probe(
        "ollama",
        args.ollama_base,
        args.ollama_model,
        args.timeout_seconds,
        args.poll_timeout_seconds,
    )
    lmstudio, lmstudio_error = safe_probe(
        "lmstudio",
        args.lmstudio_base,
        args.lmstudio_model,
        args.timeout_seconds,
        args.poll_timeout_seconds,
    )
    providers = {"ollama": ollama, "lmstudio": lmstudio}
    all_classified = all(
        result["provider_cancel_capability_classified"] for result in providers.values()
    )
    all_acknowledged = all(
        result["provider_cancel_acknowledged"] for result in providers.values()
    )
    prompt = CANCEL_PROMPT.encode("utf-8")
    receipt = {
        "schema": "hepta.inference.inf0c.cancel_capability.v3",
        "source": {
            "commit": BASE.git_value("rev-parse", "HEAD"),
            "tree": BASE.git_value("rev-parse", "HEAD^{tree}"),
        },
        "scope": "QUALIFICATION_ONLY_EXPLICIT_PROVIDER_CANCEL_CAPABILITY",
        "prompt": {
            "sha256": hashlib.sha256(prompt).hexdigest(),
            "byte_length": len(prompt),
            "raw_persisted": False,
        },
        "providers": providers,
        "provider_cancel_capability_classified": all_classified,
        "backend_cancellation_acknowledged": all_acknowledged,
        "transport_disconnect_used": False,
        "raw_response_id_persisted": False,
        "raw_model_output_persisted": False,
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

    diagnostics = [message for message in (ollama_error, lmstudio_error) if message]
    if diagnostics:
        for diagnostic in diagnostics:
            print(f"CANCEL_CAPABILITY_DIAGNOSTIC: {diagnostic}", file=sys.stderr)
    if all_acknowledged:
        print("PASS_HEPTA_INFERENCE_INF0C_EXPLICIT_CANCEL_ACK_EVIDENCE")
        return 0
    print(
        "FAIL_HEPTA_INFERENCE_INF0C_EXPLICIT_CANCEL_ACK: "
        "one or more providers did not explicitly acknowledge cancellation",
        file=sys.stderr,
    )
    return 1


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except QualificationError as error:
        print(f"FAIL_HEPTA_INFERENCE_INF0C_CANCEL_CAPABILITY_V3: {error}", file=sys.stderr)
        raise SystemExit(1) from error
