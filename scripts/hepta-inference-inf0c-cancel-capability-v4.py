#!/usr/bin/env python3
"""Corrected explicit cancellation capability classification.

This qualification-only wrapper preserves the v3 request/receipt contract while
allowing a bounded, fixed set of non-JSON media types for known unsupported HTTP
statuses. Successful and unexpected responses remain JSON-only and fail closed.
The v3 receipt field ``transport_disconnect_used`` remains authoritative and this
wrapper never promotes a transport close to provider cancellation acknowledgement.
"""
from __future__ import annotations

import importlib.util
import pathlib
import sys
import time
import urllib.error
import urllib.parse
import urllib.request
from types import ModuleType
from typing import Any


def load_v3() -> ModuleType:
    path = pathlib.Path(__file__).with_name("hepta-inference-inf0c-cancel-capability-v3.py")
    spec = importlib.util.spec_from_file_location("hepta_inf0c_cancel_v3_base", path)
    if spec is None or spec.loader is None:
        raise RuntimeError("failed to load v3 cancellation capability probe")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


V3 = load_v3()
QualificationError = V3.QualificationError
UNSUPPORTED_MEDIA_TYPES = {
    "",
    "application/json",
    "application/problem+json",
    "text/plain",
    "text/html",
}


def request_bounded_json(
    method: str,
    url: str,
    timeout: float,
    payload: dict[str, Any] | None = None,
) -> Any:
    headers = {"Accept": "application/json", "Connection": "close"}
    data = None
    if payload is not None:
        data = V3.BASE.json.dumps(payload, separators=(",", ":")).encode("utf-8")
        headers["Content-Type"] = "application/json"
    request = urllib.request.Request(url, method=method, headers=headers, data=data)
    started = time.monotonic_ns()
    try:
        with V3.BASE.LOOPBACK_OPENER.open(request, timeout=timeout) as response:
            body = response.read(V3.MAX_HTTP_BODY + 1)
            status = response.status
            content_type = V3.media_type(response.headers)
    except urllib.error.HTTPError as error:
        body = error.read(V3.MAX_HTTP_BODY + 1)
        status = error.code
        content_type = V3.media_type(error.headers)
    except (urllib.error.URLError, TimeoutError, OSError) as error:
        raise QualificationError(
            f"request failed for {urllib.parse.urlsplit(url).path}: {error}"
        ) from error

    elapsed_ms = max(0, (time.monotonic_ns() - started) // 1_000_000)
    V3.require(len(body) <= V3.MAX_HTTP_BODY, "HTTP response exceeded bounded body limit")
    if status in V3.UNSUPPORTED_HTTP_STATUSES:
        V3.require(
            content_type in UNSUPPORTED_MEDIA_TYPES,
            f"unexpected unsupported-response media type {content_type or '<missing>'}",
        )
    else:
        V3.require(
            content_type == "application/json",
            f"unexpected response media type {content_type or '<missing>'}",
        )
    return V3.HttpResult(
        status=status,
        media_type=content_type,
        body=body,
        elapsed_ms=elapsed_ms,
    )


V3.request_bounded_json = request_bounded_json
probe_provider = V3.probe_provider
parse_object = V3.parse_object


def main() -> int:
    return V3.main()


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except QualificationError as error:
        print(f"FAIL_HEPTA_INFERENCE_INF0C_CANCEL_CAPABILITY_V4: {error}", file=sys.stderr)
        raise SystemExit(1) from error
