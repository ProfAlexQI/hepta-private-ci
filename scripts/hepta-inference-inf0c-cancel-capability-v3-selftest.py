#!/usr/bin/env python3
"""Hermetic tests for explicit provider cancellation capability evidence."""

from __future__ import annotations

import importlib.util
import json
import pathlib
import sys
import threading
from http.server import BaseHTTPRequestHandler
from http.server import ThreadingHTTPServer
from types import ModuleType
from typing import Any


def load_probe() -> ModuleType:
    path = pathlib.Path(__file__).with_name(
        "hepta-inference-inf0c-cancel-capability-v3.py"
    )
    spec = importlib.util.spec_from_file_location("hepta_inf0c_cancel_v3", path)
    if spec is None or spec.loader is None:
        raise RuntimeError("failed to load cancellation capability probe")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


PROBE = load_probe()
QualificationError = PROBE.QualificationError


class FixtureHandler(BaseHTTPRequestHandler):
    protocol_version = "HTTP/1.1"

    def log_message(self, *_args: Any) -> None:
        return

    def send_json(self, status: int, payload: dict[str, Any], media_type: str = "application/json") -> None:
        body = json.dumps(payload, separators=(",", ":")).encode("utf-8")
        self.send_response(status)
        self.send_header("Content-Type", media_type)
        self.send_header("Content-Length", str(len(body)))
        self.send_header("Connection", "close")
        self.end_headers()
        self.wfile.write(body)

    @property
    def mode(self) -> str:
        return getattr(self.server, "fixture_mode")

    def do_POST(self) -> None:
        length = int(self.headers.get("Content-Length", "0"))
        body = self.rfile.read(length)
        if self.path == "/v1/responses":
            payload = json.loads(body or b"{}")
            if self.mode == "unsupported_create":
                self.send_json(400, {"error": "background is unsupported"})
                return
            if self.mode == "wrong_media":
                self.send_json(200, {"id": "resp_fixture", "status": "in_progress"}, "text/plain")
                return
            response_id = "../escape" if self.mode == "invalid_id" else "resp_fixture"
            assert payload.get("background") is True
            assert payload.get("store") is True
            self.send_json(200, {"id": response_id, "status": "in_progress"})
            return
        if self.path == "/v1/responses/resp_fixture/cancel":
            if self.mode == "unsupported_cancel":
                self.send_json(404, {"error": "cancel is unsupported"})
                return
            response_id = "resp_other" if self.mode == "wrong_cancel_id" else "resp_fixture"
            self.send_json(200, {"id": response_id, "status": "cancelled"})
            return
        self.send_json(404, {"error": "unknown path"})

    def do_GET(self) -> None:
        if self.path == "/v1/responses/resp_fixture":
            status = "completed" if self.mode == "terminal_completed" else "cancelled"
            self.send_json(200, {"id": "resp_fixture", "status": status})
            return
        self.send_json(404, {"error": "unknown path"})


def expect_failure(operation: Any, label: str) -> None:
    try:
        operation()
    except QualificationError:
        return
    raise AssertionError(f"expected failure: {label}")


def run_fixture(mode: str) -> tuple[ThreadingHTTPServer, threading.Thread, str]:
    server = ThreadingHTTPServer(("127.0.0.1", 0), FixtureHandler)
    setattr(server, "fixture_mode", mode)
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()
    host, port = server.server_address
    return server, thread, f"http://{host}:{port}"


def probe(mode: str) -> dict[str, Any]:
    server, thread, base = run_fixture(mode)
    try:
        return PROBE.probe_provider(
            "ollama",
            base,
            "fixture-model",
            timeout=5.0,
            poll_timeout=2.0,
        )
    finally:
        server.shutdown()
        server.server_close()
        thread.join(timeout=2.0)


def main() -> int:
    acknowledged = probe("acknowledged")
    assert acknowledged["provider_cancel_acknowledged"] is True
    assert acknowledged["classification"] == "explicit_background_cancel_acknowledged"
    assert acknowledged["transport_disconnect_used"] is False
    assert acknowledged["response_id"]["raw_persisted"] is False

    unsupported_create = probe("unsupported_create")
    assert unsupported_create["classification"] == "explicit_cancel_unsupported"
    assert unsupported_create["unsupported_phase"] == "background_create"
    assert unsupported_create["provider_cancel_acknowledged"] is False

    unsupported_cancel = probe("unsupported_cancel")
    assert unsupported_cancel["classification"] == "explicit_cancel_unsupported"
    assert unsupported_cancel["unsupported_phase"] == "cancel_endpoint"
    assert unsupported_cancel["provider_cancel_acknowledged"] is False

    expect_failure(lambda: probe("invalid_id"), "invalid response id")
    expect_failure(lambda: probe("wrong_cancel_id"), "cancel id mismatch")
    expect_failure(lambda: probe("wrong_media"), "wrong media type")
    expect_failure(lambda: probe("terminal_completed"), "terminal non-cancelled state")
    expect_failure(
        lambda: PROBE.validate_response_id("../escape", "fixture"),
        "path traversal response id",
    )
    print("PASS_HEPTA_INFERENCE_INF0C_CANCEL_CAPABILITY_V3_SELF_TEST")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
