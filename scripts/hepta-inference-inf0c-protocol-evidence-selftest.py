#!/usr/bin/env python3
"""Hermetic strict-SSE, malformed-event, and timeout self-test for INF-0C."""

from __future__ import annotations

import importlib.util
import pathlib
import sys
import threading
import time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from types import ModuleType
from typing import Any


def load_protocol() -> ModuleType:
    path = pathlib.Path(__file__).with_name(
        "hepta-inference-inf0c-protocol-evidence.py"
    )
    spec = importlib.util.spec_from_file_location("hepta_inf0c_protocol", path)
    if spec is None or spec.loader is None:
        raise RuntimeError("failed to load protocol evidence harness")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


PROTOCOL = load_protocol()


def event(event_type: str, sequence: int, extra: str = "") -> bytes:
    data = (
        '{"type":"'
        + event_type
        + '","sequence_number":'
        + str(sequence)
        + extra
        + "}"
    )
    return f"event: {event_type}\ndata: {data}\n\n".encode("utf-8")


class QuietServer(ThreadingHTTPServer):
    def handle_error(self, request: Any, client_address: Any) -> None:
        del request, client_address


class ProtocolHandler(BaseHTTPRequestHandler):
    protocol_version = "HTTP/1.1"

    def log_message(self, format: str, *args: Any) -> None:
        del format, args

    def do_POST(self) -> None:
        length = int(self.headers.get("Content-Length", "0"))
        if length:
            self.rfile.read(length)

        media_type = "text/event-stream; charset=utf-8"
        if self.path == "/valid":
            payload = event("response.created", 0) + event(
                "response.completed", 1, ',"response":{"status":"completed"}'
            )
        elif self.path == "/malformed-json":
            payload = b"event: response.created\ndata: {not-json}\n\n"
        elif self.path == "/unknown-event":
            payload = b'event: response.unknown\ndata: {"type":"response.unknown"}\n\n'
        elif self.path == "/event-type-mismatch":
            payload = b'event: response.created\ndata: {"type":"response.completed"}\n\n'
        elif self.path == "/truncated":
            payload = event("response.created", 0)
        elif self.path == "/unterminated":
            payload = b'event: response.created\ndata: {"type":"response.created"}'
        elif self.path == "/duplicate-completion":
            payload = event("response.completed", 0) + event(
                "response.completed", 1
            )
        elif self.path == "/event-after-completion":
            payload = event("response.completed", 0) + event(
                "response.created", 1
            )
        elif self.path == "/bad-sequence":
            payload = event("response.created", 7) + event(
                "response.completed", 8
            )
        elif self.path == "/legacy-done":
            payload = b"data: [DONE]\n\n"
        elif self.path == "/unknown-field":
            payload = b'x-extra: value\ndata: {"type":"response.created"}\n\n'
        elif self.path == "/oversized":
            payload = (
                b"event: response.created\ndata: "
                + b"x" * (PROTOCOL.MAX_SSE_EVENT_BYTES + 1)
                + b"\n\n"
            )
        elif self.path == "/bad-media":
            payload = event("response.completed", 0)
            media_type = "application/json"
        elif self.path == "/slow":
            payload = event("response.completed", 0)
        else:
            self.send_response(404)
            self.send_header("Content-Length", "0")
            self.end_headers()
            return

        self.send_response(200)
        self.send_header("Content-Type", media_type)
        self.send_header("Content-Length", str(len(payload)))
        self.end_headers()
        if self.path == "/slow":
            time.sleep(0.3)
        try:
            self.wfile.write(payload)
            self.wfile.flush()
        except (BrokenPipeError, ConnectionResetError):
            pass


def expect_failure(operation: Any, label: str) -> None:
    try:
        operation()
    except PROTOCOL.QualificationError:
        return
    raise RuntimeError(f"expected failure: {label}")


def main() -> int:
    server = QuietServer(("127.0.0.1", 0), ProtocolHandler)
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()
    try:
        base = f"http://127.0.0.1:{server.server_port}"
        valid = PROTOCOL.read_strict_sse(
            f"{base}/valid",
            2.0,
            {"fixture": True},
        )
        if valid.completed is not True or valid.event_count != 2:
            raise RuntimeError("valid strict SSE stream was not accepted")
        if valid.event_type_counts != {
            "response.completed": 1,
            "response.created": 1,
        }:
            raise RuntimeError("SSE event type counts are incorrect")
        if valid.media_type != "text/event-stream":
            raise RuntimeError("SSE media type was not normalized")
        if not valid.stream_sha256 or valid.total_bytes <= 0:
            raise RuntimeError("SSE digest receipt is missing")

        for path in (
            "/malformed-json",
            "/unknown-event",
            "/event-type-mismatch",
            "/truncated",
            "/unterminated",
            "/duplicate-completion",
            "/event-after-completion",
            "/bad-sequence",
            "/legacy-done",
            "/unknown-field",
            "/oversized",
            "/bad-media",
        ):
            expect_failure(
                lambda path=path: PROTOCOL.read_strict_sse(
                    f"{base}{path}",
                    2.0,
                    {"fixture": True},
                ),
                path,
            )
        expect_failure(
            lambda: PROTOCOL.read_strict_sse(
                f"{base}/slow",
                0.05,
                {"fixture": True},
            ),
            "stream timeout",
        )
    finally:
        server.shutdown()
        server.server_close()
        thread.join(timeout=5.0)

    print("PASS_HEPTA_INFERENCE_INF0C_PROTOCOL_SSE_SELF_TEST")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
