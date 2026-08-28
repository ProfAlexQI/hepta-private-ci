#!/usr/bin/env python3
"""Qualification-only real-software smoke for fixed local inference services.

This harness never downloads models, never contacts a non-loopback host, and
never writes raw prompts or model outputs to its receipt. It proves exact model
presence plus one bounded Responses request whose digest-only semantic output
equals the fixed qualification marker for each pre-provisioned service.
Cancellation and controlled restart remain separate required evidence.
"""

from __future__ import annotations

import argparse
import hashlib
import ipaddress
import json
import os
import pathlib
import socket
import stat
import subprocess
import sys
import time
import urllib.error
import urllib.parse
import urllib.request
from dataclasses import dataclass
from typing import Any

EXPECTED_OUTPUT = "HEPTA_INF0C_OK"
PROMPT = f"Return exactly {EXPECTED_OUTPUT}."
MAX_HTTP_BODY = 4 * 1024 * 1024
ALLOWED_HOSTS = {"127.0.0.1", "localhost", "::1"}
ALLOWED_BASE_PATHS = {"", "/v1"}


class NoRedirect(urllib.request.HTTPRedirectHandler):
    def redirect_request(
        self,
        request: urllib.request.Request,
        file_pointer: Any,
        code: int,
        message: str,
        headers: Any,
        new_url: str,
    ) -> None:
        del request, file_pointer, code, message, headers, new_url
        return None


LOOPBACK_OPENER = urllib.request.build_opener(
    urllib.request.ProxyHandler({}),
    NoRedirect(),
)


class QualificationError(RuntimeError):
    pass


@dataclass(frozen=True)
class HttpResult:
    status: int
    body: bytes
    elapsed_ms: int
    media_type: str


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--execute", action="store_true", help="required safety latch")
    parser.add_argument("--ollama-base", default="http://127.0.0.1:11434")
    parser.add_argument("--ollama-model", required=True)
    parser.add_argument("--lmstudio-base", default="http://127.0.0.1:1234/v1")
    parser.add_argument("--lmstudio-model", required=True)
    parser.add_argument("--timeout-seconds", type=float, default=60.0)
    parser.add_argument("--receipt", type=pathlib.Path, required=True)
    return parser.parse_args()


def require(condition: bool, message: str) -> None:
    if not condition:
        raise QualificationError(message)


def _pin_loopback_literal(hostname: str, port: int) -> str:
    try:
        addresses = socket.getaddrinfo(hostname, port, type=socket.SOCK_STREAM)
    except OSError as error:
        raise QualificationError(f"failed to resolve loopback endpoint: {error}") from error
    require(bool(addresses), "endpoint resolved to no addresses")

    literals: list[str] = []
    for address in addresses:
        literal = address[4][0].split("%", 1)[0]
        try:
            parsed = ipaddress.ip_address(literal)
        except ValueError as error:
            raise QualificationError("endpoint resolved to an invalid IP address") from error
        require(parsed.is_loopback, "endpoint resolved outside loopback")
        canonical = parsed.compressed
        if canonical not in literals:
            literals.append(canonical)

    # Prefer IPv4 when both families are available so common local services
    # bound only to 127.0.0.1 remain reachable. The chosen literal is persisted
    # only as part of the request URL in memory, never in the receipt.
    return next((value for value in literals if ":" not in value), literals[0])


def normalize_loopback_base(value: str) -> str:
    require(value == value.strip(), "endpoint has surrounding whitespace")
    require(
        not any(ord(character) < 32 or ord(character) == 127 for character in value),
        "endpoint contains control characters",
    )
    try:
        parsed = urllib.parse.urlsplit(value)
        port = parsed.port
    except ValueError as error:
        raise QualificationError("endpoint URL or port is invalid") from error
    require(parsed.scheme == "http", "only loopback HTTP endpoints are allowed")
    require(parsed.hostname in ALLOWED_HOSTS, "endpoint host is not loopback")
    require(
        parsed.username is None and parsed.password is None,
        "endpoint credentials are forbidden",
    )
    require(not parsed.query and not parsed.fragment, "endpoint query/fragment is forbidden")
    require(port is not None and port != 0, "endpoint must include a non-zero port")
    path = parsed.path.rstrip("/")
    require(
        path in ALLOWED_BASE_PATHS,
        "only host root or /v1 endpoint paths are allowed",
    )
    literal = _pin_loopback_literal(parsed.hostname, port)
    rendered_host = f"[{literal}]" if ":" in literal else literal
    return f"http://{rendered_host}:{port}{path}"


def validate_model_id(model: str) -> None:
    require(model == model.strip(), "model identifier has surrounding whitespace")
    require(0 < len(model) <= 512, "model identifier length is invalid")
    require(not any(character.isspace() for character in model), "model identifier contains whitespace")
    require(
        not any(ord(character) < 32 or ord(character) == 127 for character in model),
        "model identifier contains control characters",
    )


def _media_type(headers: Any) -> str:
    raw = headers.get("Content-Type", "")
    return raw.split(";", 1)[0].strip().lower()


def request_json(
    method: str,
    url: str,
    timeout: float,
    payload: dict[str, Any] | None = None,
) -> HttpResult:
    headers = {"Accept": "application/json"}
    data = None
    if payload is not None:
        data = json.dumps(payload, separators=(",", ":")).encode("utf-8")
        headers["Content-Type"] = "application/json"
    request = urllib.request.Request(url, method=method, headers=headers, data=data)
    started = time.monotonic_ns()
    try:
        with LOOPBACK_OPENER.open(request, timeout=timeout) as response:
            body = response.read(MAX_HTTP_BODY + 1)
            status = response.status
            media_type = _media_type(response.headers)
    except urllib.error.HTTPError as error:
        raise QualificationError(
            f"HTTP status {error.code} from {urllib.parse.urlsplit(url).path}"
        ) from error
    except (urllib.error.URLError, TimeoutError, OSError) as error:
        raise QualificationError(
            f"request failed for {urllib.parse.urlsplit(url).path}: {error}"
        ) from error
    elapsed_ms = max(0, (time.monotonic_ns() - started) // 1_000_000)
    require(len(body) <= MAX_HTTP_BODY, "HTTP response exceeded bounded body limit")
    require(200 <= status < 300, f"unexpected HTTP status {status}")
    require(media_type == "application/json", f"unexpected response media type {media_type or '<missing>'}")
    return HttpResult(
        status=status,
        body=body,
        elapsed_ms=elapsed_ms,
        media_type=media_type,
    )


def parse_object(result: HttpResult, label: str) -> dict[str, Any]:
    try:
        value = json.loads(result.body)
    except (UnicodeDecodeError, json.JSONDecodeError) as error:
        raise QualificationError(f"{label} returned invalid JSON") from error
    require(isinstance(value, dict), f"{label} must return a JSON object")
    return value


def extract_response_text(payload: dict[str, Any], label: str) -> str:
    top_level = payload.get("output_text")
    if isinstance(top_level, str):
        require(top_level.strip(), f"{label} output_text is empty")
        return top_level

    output = payload.get("output")
    require(isinstance(output, list), f"{label} output array is missing")
    pieces: list[str] = []
    for item in output:
        if not isinstance(item, dict):
            continue
        content = item.get("content")
        if not isinstance(content, list):
            continue
        for part in content:
            if not isinstance(part, dict):
                continue
            if part.get("type") not in {"output_text", "text"}:
                continue
            text = part.get("text")
            if isinstance(text, str):
                pieces.append(text)
    combined = "".join(pieces)
    require(combined.strip(), f"{label} contains no output text")
    return combined


def semantic_output_receipt(payload: dict[str, Any], label: str) -> dict[str, Any]:
    output = extract_response_text(payload, label).strip()
    require(output == EXPECTED_OUTPUT, f"{label} semantic output mismatch")
    encoded = output.encode("utf-8")
    return {
        "verified": True,
        "sha256": hashlib.sha256(encoded).hexdigest(),
        "byte_length": len(encoded),
        "raw_persisted": False,
    }


def body_receipt(result: HttpResult) -> dict[str, Any]:
    return {
        "status": result.status,
        "media_type": result.media_type,
        "byte_length": len(result.body),
        "sha256": hashlib.sha256(result.body).hexdigest(),
        "elapsed_ms": result.elapsed_ms,
    }


def qualify_ollama(base: str, model: str, timeout: float) -> dict[str, Any]:
    version_result = request_json("GET", f"{base}/api/version", timeout)
    version = parse_object(version_result, "Ollama version").get("version")
    require(isinstance(version, str) and version.strip(), "Ollama version is missing")

    models_result = request_json("GET", f"{base}/api/tags", timeout)
    models_payload = parse_object(models_result, "Ollama models")
    entries = models_payload.get("models")
    require(isinstance(entries, list), "Ollama models array is missing")
    names = [entry.get("name") for entry in entries if isinstance(entry, dict)]
    require(model in names, "requested Ollama model is not pre-installed")

    response_result = request_json(
        "POST",
        f"{base}/v1/responses",
        timeout,
        {
            "model": model,
            "input": PROMPT,
            "max_output_tokens": 16,
            "stream": False,
        },
    )
    response_payload = parse_object(response_result, "Ollama Responses")
    semantic = semantic_output_receipt(response_payload, "Ollama Responses")
    return {
        "version": version,
        "model_present": True,
        "semantic_output": semantic,
        "version_response": body_receipt(version_result),
        "models_response": body_receipt(models_result),
        "inference_response": body_receipt(response_result),
    }


def qualify_lmstudio(base: str, model: str, timeout: float) -> dict[str, Any]:
    models_result = request_json("GET", f"{base}/models", timeout)
    models_payload = parse_object(models_result, "LM Studio models")
    entries = models_payload.get("data")
    require(isinstance(entries, list), "LM Studio data array is missing")
    names = [entry.get("id") for entry in entries if isinstance(entry, dict)]
    require(model in names, "requested LM Studio model is not pre-installed")

    response_result = request_json(
        "POST",
        f"{base}/responses",
        timeout,
        {
            "model": model,
            "input": PROMPT,
            "max_output_tokens": 16,
            "stream": False,
        },
    )
    response_payload = parse_object(response_result, "LM Studio Responses")
    semantic = semantic_output_receipt(response_payload, "LM Studio Responses")
    return {
        "model_present": True,
        "semantic_output": semantic,
        "models_response": body_receipt(models_result),
        "inference_response": body_receipt(response_result),
    }


def git_value(*args: str) -> str:
    completed = subprocess.run(
        ["git", *args],
        check=False,
        capture_output=True,
        text=True,
    )
    return completed.stdout.strip() if completed.returncode == 0 else "UNAVAILABLE"


def write_receipt(path: pathlib.Path, receipt: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    flags = os.O_WRONLY | os.O_CREAT | os.O_EXCL
    descriptor = os.open(path, flags, 0o600)
    try:
        with os.fdopen(descriptor, "w", encoding="utf-8") as handle:
            json.dump(receipt, handle, indent=2, sort_keys=True)
            handle.write("\n")
            handle.flush()
            os.fsync(handle.fileno())
    except BaseException:
        path.unlink(missing_ok=True)
        raise
    if os.name != "nt":
        require(
            stat.S_IMODE(path.stat().st_mode) == 0o600,
            "receipt permissions are not owner-only",
        )


def main() -> int:
    args = parse_args()
    require(args.execute, "--execute is required")
    require(args.timeout_seconds > 0, "timeout must be positive")
    validate_model_id(args.ollama_model)
    validate_model_id(args.lmstudio_model)
    ollama_base = normalize_loopback_base(args.ollama_base)
    lmstudio_base = normalize_loopback_base(args.lmstudio_base)

    ollama = qualify_ollama(ollama_base, args.ollama_model, args.timeout_seconds)
    lmstudio = qualify_lmstudio(lmstudio_base, args.lmstudio_model, args.timeout_seconds)
    prompt_bytes = PROMPT.encode("utf-8")
    receipt = {
        "schema": "hepta.inference.inf0c.real_software_e2e.v2",
        "source": {
            "commit": git_value("rev-parse", "HEAD"),
            "tree": git_value("rev-parse", "HEAD^{tree}"),
        },
        "scope": "QUALIFICATION_ONLY_MINIMAL_REAL_SOFTWARE_E2E",
        "expected_output": {
            "sha256": hashlib.sha256(EXPECTED_OUTPUT.encode("utf-8")).hexdigest(),
            "byte_length": len(EXPECTED_OUTPUT.encode("utf-8")),
            "raw_persisted": False,
        },
        "prompt": {
            "sha256": hashlib.sha256(prompt_bytes).hexdigest(),
            "byte_length": len(prompt_bytes),
            "raw_persisted": False,
        },
        "ollama": ollama,
        "lmstudio": lmstudio,
        "cancellation_executed": False,
        "controlled_restart_executed": False,
        "implicit_download": False,
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
    write_receipt(args.receipt, receipt)
    print("PASS_HEPTA_INFERENCE_INF0C_MINIMAL_REAL_SOFTWARE_E2E")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except QualificationError as error:
        print(f"FAIL_HEPTA_INFERENCE_INF0C_REAL_E2E: {error}", file=sys.stderr)
        raise SystemExit(1) from error
