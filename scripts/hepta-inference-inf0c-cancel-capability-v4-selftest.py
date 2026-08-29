#!/usr/bin/env python3
from __future__ import annotations

import importlib.util
import pathlib
import sys
import threading
from http.server import BaseHTTPRequestHandler
from http.server import ThreadingHTTPServer
from types import ModuleType
from typing import Any


def load_probe() -> ModuleType:
    path = pathlib.Path(__file__).with_name("hepta-inference-inf0c-cancel-capability-v4.py")
    spec = importlib.util.spec_from_file_location("hepta_inf0c_cancel_v4", path)
    if spec is None or spec.loader is None:
        raise RuntimeError("failed to load v4 cancellation probe")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


PROBE = load_probe()


class Handler(BaseHTTPRequestHandler):
    protocol_version = "HTTP/1.1"

    def log_message(self, *_args: Any) -> None:
        return

    def do_GET(self) -> None:
        cases = {
            "/plain404": (404, "text/plain", b"not found"),
            "/html405": (405, "text/html", b"<html>unsupported</html>"),
            "/empty501": (501, "", b""),
            "/problem422": (422, "application/problem+json", b'{"error":"unsupported"}'),
            "/binary404": (404, "application/octet-stream", b"no"),
            "/plain200": (200, "text/plain", b"ok"),
            "/json200": (200, "application/json", b'{"ok":true}'),
        }
        status, content_type, body = cases.get(
            self.path, (404, "application/json", b'{"error":"unknown"}')
        )
        self.send_response(status)
        if content_type:
            self.send_header("Content-Type", content_type)
        self.send_header("Content-Length", str(len(body)))
        self.send_header("Connection", "close")
        self.end_headers()
        self.wfile.write(body)


def expect_failure(operation: Any, label: str) -> None:
    try:
        operation()
    except PROBE.QualificationError:
        return
    raise AssertionError(f"expected failure: {label}")


def main() -> int:
    server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()
    base = f"http://127.0.0.1:{server.server_address[1]}"
    try:
        for path, expected in (
            ("plain404", 404),
            ("html405", 405),
            ("empty501", 501),
            ("problem422", 422),
        ):
            result = PROBE.request_bounded_json("GET", f"{base}/{path}", 5.0)
            assert result.status == expected
        valid = PROBE.request_bounded_json("GET", f"{base}/json200", 5.0)
        assert PROBE.parse_object(valid, "valid") == {"ok": True}
        expect_failure(
            lambda: PROBE.request_bounded_json("GET", f"{base}/binary404", 5.0),
            "unsupported binary media type",
        )
        expect_failure(
            lambda: PROBE.request_bounded_json("GET", f"{base}/plain200", 5.0),
            "successful non-JSON response",
        )
    finally:
        server.shutdown()
        server.server_close()
        thread.join(timeout=2.0)
    print("PASS_HEPTA_INFERENCE_INF0C_CANCEL_CAPABILITY_V4_HTTP_CLASSIFICATION_SELF_TEST")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
