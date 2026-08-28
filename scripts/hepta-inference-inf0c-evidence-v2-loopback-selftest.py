#!/usr/bin/env python3
"""Hermetic loopback self-test for INF-0C disconnect and media-type fencing."""

from __future__ import annotations

import importlib.util
import pathlib
import sys
import threading
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from types import ModuleType
from typing import Any


def load_evidence() -> ModuleType:
    path = pathlib.Path(__file__).with_name("hepta-inference-inf0c-evidence-v2.py")
    spec = importlib.util.spec_from_file_location("hepta_inf0c_evidence_v2", path)
    if spec is None or spec.loader is None:
        raise RuntimeError("failed to load evidence-v2 harness")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


EVIDENCE = load_evidence()


class QuietServer(ThreadingHTTPServer):
    def handle_error(self, request: Any, client_address: Any) -> None:
        del request, client_address


class StreamingHandler(BaseHTTPRequestHandler):
    protocol_version = "HTTP/1.1"

    def log_message(self, format: str, *args: Any) -> None:
        del format, args

    def do_POST(self) -> None:
        length = int(self.headers.get("Content-Length", "0"))
        if length:
            self.rfile.read(length)

        if self.path == "/v1/responses":
            payload = b"data: HEPTA_INF0C_CANCEL\n\n" * 512
            media_type = "text/event-stream; charset=utf-8"
        elif self.path == "/v1/bad-media":
            payload = b"x" * 512
            media_type = "application/octet-stream"
        else:
            self.send_response(404)
            self.send_header("Content-Length", "0")
            self.end_headers()
            return

        self.send_response(200)
        self.send_header("Content-Type", media_type)
        self.send_header("Content-Length", str(len(payload)))
        self.end_headers()
        try:
            self.wfile.write(payload)
            self.wfile.flush()
        except (BrokenPipeError, ConnectionResetError):
            pass


def expect_failure(operation: Any, label: str) -> None:
    try:
        operation()
    except EVIDENCE.QualificationError:
        return
    raise RuntimeError(f"expected failure: {label}")


def main() -> int:
    for invalid in (
        "https://127.0.0.1:1",
        "http://example.com:80",
        "http://user@127.0.0.1:1",
        "http://127.0.0.1:1/v2",
        "http://127.0.0.1:0",
    ):
        expect_failure(
            lambda invalid=invalid: EVIDENCE.normalize_loopback_base(invalid),
            invalid,
        )

    server = QuietServer(("127.0.0.1", 0), StreamingHandler)
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()
    try:
        endpoint = EVIDENCE.normalize_loopback_base(
            f"http://localhost:{server.server_port}"
        )
        if endpoint.host != "127.0.0.1":
            raise RuntimeError("localhost was not pinned to a loopback IP literal")

        result = EVIDENCE.disconnect_stream(
            endpoint,
            "/v1/responses",
            "fixture-model",
            5.0,
            64,
        )
        if result.get("transport_disconnect_executed") is not True:
            raise RuntimeError("transport disconnect was not recorded")
        if result.get("backend_cancellation_acknowledged") is not False:
            raise RuntimeError("transport disconnect overclaimed backend acknowledgement")
        if result.get("prefix_byte_length") != 64:
            raise RuntimeError("bounded prefix length was not enforced")
        if result.get("media_type") != "text/event-stream":
            raise RuntimeError("stream media type was not normalized and recorded")
        if result.get("raw_prefix_persisted") is not False:
            raise RuntimeError("raw streaming prefix persistence was enabled")

        expect_failure(
            lambda: EVIDENCE.disconnect_stream(
                endpoint,
                "/v1/bad-media",
                "fixture-model",
                5.0,
                64,
            ),
            "unsupported stream media type",
        )
    finally:
        server.shutdown()
        server.server_close()
        thread.join(timeout=5.0)

    print("PASS_HEPTA_INFERENCE_INF0C_EVIDENCE_V2_LOOPBACK_SELF_TEST")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
