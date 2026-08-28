#!/usr/bin/env python3
"""Qualification-only cancellation and controlled-restart evidence for local inference.

The harness is deliberately isolated from product runtime. It accepts only direct
loopback HTTP services, never follows redirects or proxies, never downloads a
model, never invokes a shell, and persists only hashes, lengths, timings, and
boolean outcomes. A transport disconnect is recorded separately from backend
acknowledgement; this script never overclaims cancellation semantics.
"""

from __future__ import annotations

import argparse
import hashlib
import http.client
import json
import os
import pathlib
import socket
import stat
import subprocess
import sys
import tempfile
import threading
import time
import urllib.error
import urllib.parse
import urllib.request
from dataclasses import dataclass
from http.server import BaseHTTPRequestHandler
from http.server import ThreadingHTTPServer
from typing import Any

ALLOWED_HOSTS = {"127.0.0.1", "localhost", "::1"}
MAX_HTTP_BODY = 4 * 1024 * 1024
MAX_MODEL_ID = 512
CANCEL_PROMPT = "Emit HEPTA_CANCEL_STREAM_TOKEN repeatedly until stopped."
FOLLOWUP_PROMPT = "Return exactly HEPTA_POST_CANCEL_OK."
CONTROL_HELPER_ENV = "HEPTA_INF0C_CONTROL_HELPER"
CONTROL_HELPER_SHA_ENV = "HEPTA_INF0C_CONTROL_HELPER_SHA256"
SUBPROCESS_ENV_ALLOWLIST = (
    "HOME",
    "USERPROFILE",
    "APPDATA",
    "LOCALAPPDATA",
    "SystemRoot",
    "WINDIR",
    "TEMP",
    "TMP",
    "PATH",
    "LANG",
    "LC_ALL",
)


class QualificationError(RuntimeError):
    pass


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


@dataclass(frozen=True)
class Endpoint:
    base: str
    host: str
    port: int
    path: str


@dataclass(frozen=True)
class HttpResult:
    status: int
    body: bytes
    elapsed_ms: int


@dataclass(frozen=True)
class ControlHelper:
    path: pathlib.Path
    sha256: str


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--execute", action="store_true", help="required real-service latch")
    parser.add_argument("--self-test", action="store_true", help="run hermetic loopback fixtures")
    parser.add_argument("--controlled-restart", action="store_true")
    parser.add_argument("--ollama-base", default="http://127.0.0.1:11434")
    parser.add_argument("--ollama-model")
    parser.add_argument("--lmstudio-base", default="http://127.0.0.1:1234/v1")
    parser.add_argument("--lmstudio-model")
    parser.add_argument("--timeout-seconds", type=float, default=60.0)
    parser.add_argument("--restart-timeout-seconds", type=float, default=60.0)
    parser.add_argument("--cancel-read-bytes", type=int, default=64)
    parser.add_argument("--receipt", type=pathlib.Path, required=True)
    return parser.parse_args()


def require(condition: bool, message: str) -> None:
    if not condition:
        raise QualificationError(message)


def validate_model_id(model: str) -> None:
    require(model == model.strip(), "model identifier has surrounding whitespace")
    require(0 < len(model) <= MAX_MODEL_ID, "model identifier length is invalid")
    require(
        not any(ord(character) < 32 or ord(character) == 127 for character in model),
        "model identifier contains control characters",
    )


def normalize_loopback_base(value: str) -> Endpoint:
    parsed = urllib.parse.urlsplit(value)
    require(parsed.scheme == "http", "only loopback HTTP endpoints are allowed")
    require(parsed.hostname in ALLOWED_HOSTS, "endpoint host is not loopback")
    require(
        parsed.username is None and parsed.password is None,
        "endpoint credentials are forbidden",
    )
    require(not parsed.query and not parsed.fragment, "endpoint query/fragment is forbidden")
    require(parsed.port is not None and parsed.port != 0, "endpoint requires a non-zero port")
    path = parsed.path.rstrip("/")
    require(path in {"", "/v1"}, "only root or /v1 endpoint paths are allowed")
    try:
        addresses = socket.getaddrinfo(parsed.hostname, parsed.port, type=socket.SOCK_STREAM)
    except OSError as error:
        raise QualificationError(f"failed to resolve loopback endpoint: {error}") from error
    require(bool(addresses), "endpoint resolved to no addresses")
    for address in addresses:
        host = address[4][0]
        require(host.startswith("127.") or host == "::1", "endpoint resolved outside loopback")
    rendered_host = f"[{parsed.hostname}]" if ":" in parsed.hostname else parsed.hostname
    return Endpoint(
        base=f"http://{rendered_host}:{parsed.port}{path}",
        host=parsed.hostname,
        port=parsed.port,
        path=path,
    )


