#!/usr/bin/env python3
"""Qualification-only disconnect and controlled-restart evidence for INF-0C.

This companion imports the minimal real-software harness, reuses its loopback,
no-proxy, no-redirect, bounded-response checks, and adds two explicit probes:

* close a streaming Responses connection after a bounded prefix, then prove the
  exact pre-installed model remains usable;
* optionally stop and start each provider through one operator-pinned helper
  executable, observe unavailability, and prove exact-model readiness returns.

No model is downloaded. No raw prompt, model output, helper output, or secret is
persisted. A transport disconnect is not misrepresented as backend cancellation
acknowledgement.
"""

from __future__ import annotations

import argparse
import hashlib
import http.client
import importlib.util
import os
import pathlib
import socket
import subprocess
import sys
import tempfile
import time
from dataclasses import dataclass
from types import ModuleType
from typing import Any, Callable

CONTROL_HELPER_ENV = "HEPTA_INF0C_SERVICE_CONTROL_HELPER"
CONTROL_HELPER_SHA256_ENV = "HEPTA_INF0C_SERVICE_CONTROL_HELPER_SHA256"
CONTROL_HELPER_TIMEOUT_SECONDS = 60.0
MAX_DISCONNECT_PREFIX = 4096
POLL_INTERVAL_SECONDS = 0.25
CANCEL_PROMPT = "Emit HEPTA_INF0C_CANCEL repeatedly until the client disconnects."
ALLOWED_HELPER_ENV = (
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
class Endpoint:
    base: str
    host: str
    port: int
    path_prefix: str

    def url(self, suffix: str) -> str:
        return f"{self.base}{suffix}"

    def request_path(self, suffix: str) -> str:
        return f"{self.path_prefix}{suffix}" or "/"


def require(condition: bool, message: str) -> None:
    if not condition:
        raise QualificationError(message)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--self-test", action="store_true")
    mode.add_argument("--execute", action="store_true", help="required real-service safety latch")
    parser.add_argument("--ollama-base", default="http://127.0.0.1:11434")
    parser.add_argument("--ollama-model")
    parser.add_argument("--lmstudio-base", default="http://127.0.0.1:1234/v1")
    parser.add_argument("--lmstudio-model")
    parser.add_argument("--timeout-seconds", type=float, default=60.0)
    parser.add_argument("--restart-timeout-seconds", type=float, default=120.0)
    parser.add_argument("--disconnect-read-bytes", type=int, default=512)
    parser.add_argument("--run-controlled-restart", action="store_true")
    parser.add_argument("--receipt", type=pathlib.Path)
    return parser.parse_args()


def normalize_loopback_base(value: str) -> Endpoint:
    parsed = BASE.urllib.parse.urlsplit(value)
    require(parsed.scheme == "http", "only loopback HTTP endpoints are allowed")
    require(parsed.hostname in BASE.ALLOWED_HOSTS, "endpoint host is not loopback")
    require(parsed.username is None and parsed.password is None, "endpoint credentials are forbidden")
    require(not parsed.query and not parsed.fragment, "endpoint query/fragment is forbidden")
    require(parsed.port is not None and parsed.port != 0, "endpoint must include a non-zero port")
    try:
        addresses = socket.getaddrinfo(parsed.hostname, parsed.port, type=socket.SOCK_STREAM)
    except OSError as error:
        raise QualificationError(f"failed to resolve loopback endpoint: {error}") from error
    require(bool(addresses), "endpoint resolved to no addresses")
    for address in addresses:
        host = address[4][0]
        require(host.startswith("127.") or host == "::1", "endpoint resolved outside loopback")
    path_prefix = parsed.path.rstrip("/")
    require(path_prefix in ("", "/v1"), "only host root or /v1 endpoint paths are allowed")
    rendered_host = f"[{parsed.hostname}]" if ":" in parsed.hostname else parsed.hostname
    return Endpoint(
        base=f"http://{rendered_host}:{parsed.port}{path_prefix}",
        host=parsed.hostname,
        port=parsed.port,
        path_prefix=path_prefix,
    )


def disconnect_stream(
    endpoint: Endpoint,
    path: str,
    model: str,
    timeout: float,
    read_bytes: int,
) -> dict[str, Any]:
    require(0 < read_bytes <= MAX_DISCONNECT_PREFIX, "disconnect prefix bound is invalid")
    payload = BASE.json.dumps(
        {
            "model": model,
            "input": CANCEL_PROMPT,
            "max_output_tokens": 256,
            "stream": True,
        },
        separators=(",", ":"),
    ).encode("utf-8")
    connection = http.client.HTTPConnection(endpoint.host, endpoint.port, timeout=timeout)
    response: http.client.HTTPResponse | None = None
    prefix = b""
    started = time.monotonic_ns()
    try:
        connection.request(
            "POST",
            endpoint.request_path(path),
            body=payload,
            headers={
                "Accept": "text/event-stream, application/json",
                "Content-Type": "application/json",
                "X-Hepta-Qualification": "inf0c-transport-disconnect-v1",
            },
        )
        response = connection.getresponse()
        require(200 <= response.status < 300, f"disconnect probe returned HTTP {response.status}")
        prefix = response.read(read_bytes)
        require(bool(prefix), "disconnect probe returned no bytes before close")
        status = response.status
    except (OSError, TimeoutError, http.client.HTTPException) as error:
        raise QualificationError(f"transport-disconnect probe failed: {error}") from error
    finally:
        if response is not None:
            response.close()
        connection.close()
    elapsed_ms = max(0, (time.monotonic_ns() - started) // 1_000_000)
    return {
        "transport_disconnect_executed": True,
        "backend_cancellation_acknowledged": False,
        "status": status,
        "prefix_byte_length": len(prefix),
        "prefix_sha256": hashlib.sha256(prefix).hexdigest(),
        "elapsed_ms": elapsed_ms,
        "raw_prefix_persisted": False,
    }


def parse_sha256_binding(value: str) -> str:
    require(value.startswith("sha256:"), "helper digest must use sha256:<64 lowercase hex>")
    digest = value.removeprefix("sha256:")
    require(
        len(digest) == 64 and all(character in "0123456789abcdef" for character in digest),
        "helper digest must use sha256:<64 lowercase hex>",
    )
    return digest


def hash_file(path: pathlib.Path) -> str:
    hasher = hashlib.sha256()
    with path.open("rb") as handle:
        while chunk := handle.read(1024 * 1024):
            hasher.update(chunk)
    return hasher.hexdigest()


def resolve_control_helper() -> tuple[pathlib.Path, str]:
    raw_path = os.environ.get(CONTROL_HELPER_ENV, "")
    raw_digest = os.environ.get(CONTROL_HELPER_SHA256_ENV, "")
    require(bool(raw_path), f"{CONTROL_HELPER_ENV} is required for controlled restart")
    expected = parse_sha256_binding(raw_digest)
    supplied = pathlib.Path(raw_path)
    require(not supplied.is_symlink(), "service-control helper path must not be a symlink")
    try:
        canonical = supplied.resolve(strict=True)
    except OSError as error:
        raise QualificationError(f"failed to resolve service-control helper: {error}") from error
    require(canonical.is_file(), "service-control helper is not a regular file")
    actual = hash_file(canonical)
    require(actual == expected, "service-control helper SHA-256 mismatch")
    return canonical, f"sha256:{actual}"


def sanitized_helper_environment() -> dict[str, str]:
    return {
        name: value
        for name in ALLOWED_HELPER_ENV
        if (value := os.environ.get(name)) is not None
    }


def run_control_helper(helper: pathlib.Path, action: str, service: str) -> dict[str, Any]:
    require(action in {"stop", "start"}, "unsupported control-helper action")
    require(service in {"ollama", "lmstudio"}, "unsupported control-helper service")
    started = time.monotonic_ns()
    try:
        result = subprocess.run(
            [str(helper), action, service],
            stdin=subprocess.DEVNULL,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
            shell=False,
            close_fds=True,
            env=sanitized_helper_environment(),
            timeout=CONTROL_HELPER_TIMEOUT_SECONDS,
            check=False,
        )
    except (OSError, subprocess.TimeoutExpired) as error:
        raise QualificationError(f"service-control helper {action} failed: {error}") from error
    elapsed_ms = max(0, (time.monotonic_ns() - started) // 1_000_000)
    require(result.returncode == 0, f"service-control helper {action} exited {result.returncode}")
    return {
        "action": action,
        "service": service,
        "exit_code": result.returncode,
        "elapsed_ms": elapsed_ms,
    }


def endpoint_reachable(endpoint: Endpoint, path: str, timeout: float = 1.0) -> bool:
    request = BASE.urllib.request.Request(endpoint.url(path), method="GET")
    try:
        with BASE.LOOPBACK_OPENER.open(request, timeout=timeout):
            return True
    except BASE.urllib.error.HTTPError:
        return True
    except (BASE.urllib.error.URLError, TimeoutError, OSError):
        return False


def wait_until(predicate: Callable[[], bool], timeout: float, failure: str) -> int:
    started = time.monotonic_ns()
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        if predicate():
            return max(0, (time.monotonic_ns() - started) // 1_000_000)
        time.sleep(POLL_INTERVAL_SECONDS)
    raise QualificationError(failure)


def controlled_restart(
    helper: pathlib.Path,
    helper_digest: str,
    service: str,
    endpoint: Endpoint,
    health_path: str,
    qualify: Callable[[], dict[str, Any]],
    timeout: float,
) -> dict[str, Any]:
    stop = run_control_helper(helper, "stop", service)
    stopped = True
    try:
        unavailable_ms = wait_until(
            lambda: not endpoint_reachable(endpoint, health_path),
            timeout,
            f"{service} did not become unavailable after stop",
        )
        start = run_control_helper(helper, "start", service)
        stopped = False
        post_result: dict[str, Any] | None = None

        def qualified_again() -> bool:
            nonlocal post_result
            try:
                post_result = qualify()
                return True
            except QualificationError:
                return False

        ready_ms = wait_until(
            qualified_again,
            timeout,
            f"{service} did not regain exact-model readiness after start",
        )
        require(post_result is not None, f"{service} post-restart result is missing")
        return {
            "controlled_restart_executed": True,
            "unavailable_observed": True,
            "helper_sha256": helper_digest,
            "stop": stop,
            "unavailable_after_ms": unavailable_ms,
            "start": start,
            "ready_after_ms": ready_ms,
            "post_restart": post_result,
        }
    finally:
        if stopped:
            try:
                run_control_helper(helper, "start", service)
            except QualificationError:
                pass


def run_self_test() -> None:
    endpoint = normalize_loopback_base("http://127.0.0.1:1")
    require(endpoint.host == "127.0.0.1", "base harness import self-test failed")
    require(parse_sha256_binding("sha256:" + "a" * 64) == "a" * 64, "digest self-test failed")

    with tempfile.TemporaryDirectory() as directory:
        helper = pathlib.Path(directory) / "control-helper"
        helper.write_text(
            "#!/usr/bin/env python3\nimport sys\n"
            "raise SystemExit(0 if sys.argv[1:] in "
            "([\"stop\", \"ollama\"], [\"start\", \"ollama\"]) else 2)\n",
            encoding="utf-8",
        )
        helper.chmod(0o700)
        previous_path = os.environ.get(CONTROL_HELPER_ENV)
        previous_digest = os.environ.get(CONTROL_HELPER_SHA256_ENV)
        os.environ[CONTROL_HELPER_ENV] = str(helper)
        os.environ[CONTROL_HELPER_SHA256_ENV] = f"sha256:{hash_file(helper)}"
        try:
            resolved, digest = resolve_control_helper()
            require(resolved == helper.resolve(), "helper canonicalization self-test failed")
            require(digest == os.environ[CONTROL_HELPER_SHA256_ENV], "helper digest self-test failed")
            run_control_helper(resolved, "stop", "ollama")
            run_control_helper(resolved, "start", "ollama")
        finally:
            if previous_path is None:
                os.environ.pop(CONTROL_HELPER_ENV, None)
            else:
                os.environ[CONTROL_HELPER_ENV] = previous_path
            if previous_digest is None:
                os.environ.pop(CONTROL_HELPER_SHA256_ENV, None)
            else:
                os.environ[CONTROL_HELPER_SHA256_ENV] = previous_digest
    print("PASS_HEPTA_INFERENCE_INF0C_EVIDENCE_V2_SELF_TEST")


def main() -> int:
    args = parse_args()
    if args.self_test:
        run_self_test()
        return 0

    require(args.execute, "--execute is required")
    require(args.timeout_seconds > 0, "timeout must be positive")
    require(args.restart_timeout_seconds > 0, "restart timeout must be positive")
    require(args.receipt is not None, "--receipt is required")
    require(args.ollama_model is not None, "--ollama-model is required")
    require(args.lmstudio_model is not None, "--lmstudio-model is required")
    BASE.validate_model_id(args.ollama_model)
    BASE.validate_model_id(args.lmstudio_model)
    ollama_endpoint = normalize_loopback_base(args.ollama_base)
    lmstudio_endpoint = normalize_loopback_base(args.lmstudio_base)

    baseline = {
        "ollama": BASE.qualify_ollama(
            ollama_endpoint.base, args.ollama_model, args.timeout_seconds
        ),
        "lmstudio": BASE.qualify_lmstudio(
            lmstudio_endpoint.base, args.lmstudio_model, args.timeout_seconds
        ),
    }
    cancellation: dict[str, Any] = {
        "ollama": disconnect_stream(
            ollama_endpoint,
            "/v1/responses",
            args.ollama_model,
            args.timeout_seconds,
            args.disconnect_read_bytes,
        ),
        "lmstudio": disconnect_stream(
            lmstudio_endpoint,
            "/responses",
            args.lmstudio_model,
            args.timeout_seconds,
            args.disconnect_read_bytes,
        ),
        "backend_cancellation_acknowledged": False,
    }
    cancellation["ollama"]["post_disconnect"] = BASE.qualify_ollama(
        ollama_endpoint.base, args.ollama_model, args.timeout_seconds
    )
    cancellation["lmstudio"]["post_disconnect"] = BASE.qualify_lmstudio(
        lmstudio_endpoint.base, args.lmstudio_model, args.timeout_seconds
    )

    restart: dict[str, Any] = {"controlled_restart_executed": False}
    if args.run_controlled_restart:
        helper, helper_digest = resolve_control_helper()
        restart = {
            "controlled_restart_executed": True,
            "helper_sha256": helper_digest,
            "ollama": controlled_restart(
                helper,
                helper_digest,
                "ollama",
                ollama_endpoint,
                "/api/version",
                lambda: BASE.qualify_ollama(
                    ollama_endpoint.base, args.ollama_model, args.timeout_seconds
                ),
                args.restart_timeout_seconds,
            ),
            "lmstudio": controlled_restart(
                helper,
                helper_digest,
                "lmstudio",
                lmstudio_endpoint,
                "/models",
                lambda: BASE.qualify_lmstudio(
                    lmstudio_endpoint.base, args.lmstudio_model, args.timeout_seconds
                ),
                args.restart_timeout_seconds,
            ),
        }

    prompt = CANCEL_PROMPT.encode("utf-8")
    receipt = {
        "schema": "hepta.inference.inf0c.disconnect_restart_evidence.v2",
        "source": {
            "commit": BASE.git_value("rev-parse", "HEAD"),
            "tree": BASE.git_value("rev-parse", "HEAD^{tree}"),
        },
        "scope": "QUALIFICATION_ONLY_DISCONNECT_AND_CONTROLLED_RESTART",
        "prompt": {
            "sha256": hashlib.sha256(prompt).hexdigest(),
            "byte_length": len(prompt),
            "raw_persisted": False,
        },
        "baseline": baseline,
        "cancellation": cancellation,
        "controlled_restart": restart,
        "implicit_download": False,
        "raw_model_output_persisted": False,
        "raw_helper_output_persisted": False,
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
    print("PASS_HEPTA_INFERENCE_INF0C_DISCONNECT_RESTART_EVIDENCE")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except QualificationError as error:
        print(f"FAIL_HEPTA_INFERENCE_INF0C_EVIDENCE_V2: {error}", file=sys.stderr)
        raise SystemExit(1) from error
