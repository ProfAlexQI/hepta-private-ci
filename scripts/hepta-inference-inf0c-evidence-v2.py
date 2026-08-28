#!/usr/bin/env python3
"""Qualification-only disconnect and controlled-restart evidence for INF-0C.

This companion imports the minimal real-software harness, reuses its loopback,
no-proxy, no-redirect, bounded-response and semantic-output checks, and adds:

* a bounded streaming transport disconnect followed by exact-model revalidation;
* an optional stop/start sequence through a digest-pinned helper that is
  revalidated immediately before and after every fixed-argv invocation.

No model is downloaded. No raw prompt, model output, stream prefix, helper
output, or secret is persisted. A transport disconnect is never represented as
provider/backend cancellation acknowledgement.
"""

from __future__ import annotations

import argparse
import hashlib
import http.client
import importlib.util
import os
import pathlib
import stat
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
ALLOWED_STREAM_MEDIA_TYPES = {"text/event-stream", "application/json"}
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


@dataclass(frozen=True)
class ControlHelper:
    path: pathlib.Path
    sha256: str
    size: int
    mtime_ns: int
    device: int
    inode: int
    mode: int
    parent_device: int
    parent_inode: int
    parent_mode: int


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
    normalized = BASE.normalize_loopback_base(value)
    parsed = BASE.urllib.parse.urlsplit(normalized)
    require(parsed.hostname is not None, "normalized endpoint host is missing")
    require(parsed.port is not None, "normalized endpoint port is missing")
    path_prefix = parsed.path.rstrip("/")
    return Endpoint(
        base=normalized,
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
    status = 0
    media_type = ""
    started = time.monotonic_ns()
    try:
        connection.request(
            "POST",
            endpoint.request_path(path),
            body=payload,
            headers={
                "Accept": "text/event-stream, application/json",
                "Content-Type": "application/json",
                "Connection": "close",
                "X-Hepta-Qualification": "inf0c-transport-disconnect-v2",
            },
        )
        response = connection.getresponse()
        require(200 <= response.status < 300, f"disconnect probe returned HTTP {response.status}")
        raw_media_type = response.getheader("Content-Type", "")
        media_type = raw_media_type.split(";", 1)[0].strip().lower()
        require(
            media_type in ALLOWED_STREAM_MEDIA_TYPES,
            f"disconnect probe returned unsupported media type {media_type or '<missing>'}",
        )
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
        "media_type": media_type,
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


def _same_canonical_path(left: pathlib.Path, right: pathlib.Path) -> bool:
    return os.path.normcase(os.path.abspath(str(left))) == os.path.normcase(
        os.path.abspath(str(right))
    )


def _secure_helper_metadata(path: pathlib.Path) -> tuple[os.stat_result, os.stat_result]:
    supplied_metadata = path.lstat()
    require(not stat.S_ISLNK(supplied_metadata.st_mode), "service-control helper path must not be a symlink")
    require(stat.S_ISREG(supplied_metadata.st_mode), "service-control helper is not a regular file")
    require(os.access(path, os.X_OK), "service-control helper is not executable")
    parent_metadata = path.parent.stat()
    require(stat.S_ISDIR(parent_metadata.st_mode), "service-control helper parent is not a directory")
    if os.name != "nt":
        require(
            stat.S_IMODE(supplied_metadata.st_mode) & 0o022 == 0,
            "service-control helper must not be group/other writable",
        )
        require(
            stat.S_IMODE(parent_metadata.st_mode) & 0o022 == 0,
            "service-control helper parent must not be group/other writable",
        )
        allowed_owners = {0, os.geteuid()}
        require(
            supplied_metadata.st_uid in allowed_owners,
            "service-control helper owner is not root or the current runner user",
        )
        require(
            parent_metadata.st_uid in allowed_owners,
            "service-control helper parent owner is not root or the current runner user",
        )
    return supplied_metadata, parent_metadata


def resolve_control_helper() -> ControlHelper:
    raw_path = os.environ.get(CONTROL_HELPER_ENV, "")
    raw_digest = os.environ.get(CONTROL_HELPER_SHA256_ENV, "")
    require(bool(raw_path), f"{CONTROL_HELPER_ENV} is required for controlled restart")
    expected = parse_sha256_binding(raw_digest)
    supplied = pathlib.Path(raw_path)
    require(supplied.is_absolute(), "service-control helper path must be absolute")
    try:
        canonical = supplied.resolve(strict=True)
    except OSError as error:
        raise QualificationError(f"failed to resolve service-control helper: {error}") from error
    require(
        _same_canonical_path(canonical, supplied),
        "service-control helper path must already be canonical",
    )
    metadata, parent_metadata = _secure_helper_metadata(canonical)
    actual = hash_file(canonical)
    require(actual == expected, "service-control helper SHA-256 mismatch")
    return ControlHelper(
        path=canonical,
        sha256=f"sha256:{actual}",
        size=metadata.st_size,
        mtime_ns=metadata.st_mtime_ns,
        device=metadata.st_dev,
        inode=metadata.st_ino,
        mode=stat.S_IMODE(metadata.st_mode),
        parent_device=parent_metadata.st_dev,
        parent_inode=parent_metadata.st_ino,
        parent_mode=stat.S_IMODE(parent_metadata.st_mode),
    )


def verify_control_helper(helper: ControlHelper) -> None:
    try:
        canonical = helper.path.resolve(strict=True)
    except OSError as error:
        raise QualificationError(f"failed to re-resolve service-control helper: {error}") from error
    require(
        _same_canonical_path(canonical, helper.path),
        "service-control helper canonical path changed",
    )
    metadata, parent_metadata = _secure_helper_metadata(helper.path)
    require(metadata.st_size == helper.size, "service-control helper size changed")
    require(metadata.st_mtime_ns == helper.mtime_ns, "service-control helper mtime changed")
    require(metadata.st_dev == helper.device, "service-control helper device changed")
    require(metadata.st_ino == helper.inode, "service-control helper inode changed")
    require(stat.S_IMODE(metadata.st_mode) == helper.mode, "service-control helper mode changed")
    require(parent_metadata.st_dev == helper.parent_device, "service-control helper parent device changed")
    require(parent_metadata.st_ino == helper.parent_inode, "service-control helper parent inode changed")
    require(
        stat.S_IMODE(parent_metadata.st_mode) == helper.parent_mode,
        "service-control helper parent mode changed",
    )
    require(
        f"sha256:{hash_file(helper.path)}" == helper.sha256,
        "service-control helper SHA-256 changed",
    )


def sanitized_helper_environment() -> dict[str, str]:
    return {
        name: value
        for name in ALLOWED_HELPER_ENV
        if (value := os.environ.get(name)) is not None
    }


def run_control_helper(helper: ControlHelper, action: str, service: str) -> dict[str, Any]:
    require(action in {"stop", "start"}, "unsupported control-helper action")
    require(service in {"ollama", "lmstudio"}, "unsupported control-helper service")
    verify_control_helper(helper)
    started = time.monotonic_ns()
    try:
        result = subprocess.run(
            [str(helper.path), action, service],
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
    verify_control_helper(helper)
    elapsed_ms = max(0, (time.monotonic_ns() - started) // 1_000_000)
    require(result.returncode == 0, f"service-control helper {action} exited {result.returncode}")
    return {
        "action": action,
        "service": service,
        "exit_code": result.returncode,
        "elapsed_ms": elapsed_ms,
        "helper_revalidated_before_and_after": True,
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
    helper: ControlHelper,
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
            "helper_sha256": helper.sha256,
            "helper_revalidated_per_invocation": True,
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


def _expect_qualification_failure(operation: Callable[[], Any], label: str) -> None:
    try:
        operation()
    except QualificationError:
        return
    raise QualificationError(f"self-test expected failure: {label}")


def run_self_test() -> None:
    endpoint = normalize_loopback_base("http://localhost:1")
    require(endpoint.host in {"127.0.0.1", "::1"}, "loopback literal pinning self-test failed")
    require(parse_sha256_binding("sha256:" + "a" * 64) == "a" * 64, "digest self-test failed")
    semantic = BASE.semantic_output_receipt(
        {"output": [{"content": [{"type": "output_text", "text": BASE.EXPECTED_OUTPUT}]}]},
        "semantic fixture",
    )
    require(semantic.get("verified") is True, "semantic output self-test failed")
    _expect_qualification_failure(
        lambda: BASE.semantic_output_receipt(
            {"output": [{"content": [{"type": "output_text", "text": "WRONG"}]}]},
            "semantic mismatch fixture",
        ),
        "semantic mismatch",
    )

    with tempfile.TemporaryDirectory() as directory:
        helper = pathlib.Path(directory) / "control-helper"
        helper.write_text(
            "#!/usr/bin/env python3\n"
            "import os\n"
            "import sys\n"
            "allowed = {\n"
            "    ('stop', 'ollama'), ('start', 'ollama'),\n"
            "    ('stop', 'lmstudio'), ('start', 'lmstudio'),\n"
            "}\n"
            "if 'HEPTA_INF0C_TEST_SECRET' in os.environ:\n"
            "    raise SystemExit(3)\n"
            "raise SystemExit(0 if tuple(sys.argv[1:]) in allowed else 2)\n",
            encoding="utf-8",
        )
        helper.chmod(0o700)
        previous_path = os.environ.get(CONTROL_HELPER_ENV)
        previous_digest = os.environ.get(CONTROL_HELPER_SHA256_ENV)
        previous_secret = os.environ.get("HEPTA_INF0C_TEST_SECRET")
        os.environ[CONTROL_HELPER_ENV] = str(helper.resolve())
        os.environ[CONTROL_HELPER_SHA256_ENV] = f"sha256:{hash_file(helper)}"
        os.environ["HEPTA_INF0C_TEST_SECRET"] = "must-not-leak"
        try:
            resolved = resolve_control_helper()
            require(resolved.path == helper.resolve(), "helper canonicalization self-test failed")
            require(
                resolved.sha256 == os.environ[CONTROL_HELPER_SHA256_ENV],
                "helper digest self-test failed",
            )
            for service in ("ollama", "lmstudio"):
                run_control_helper(resolved, "stop", service)
                run_control_helper(resolved, "start", service)
            _expect_qualification_failure(
                lambda: run_control_helper(resolved, "restart", "ollama"),
                "invalid helper action",
            )
            _expect_qualification_failure(
                lambda: run_control_helper(resolved, "stop", "other"),
                "invalid helper service",
            )
            helper.write_text("#!/usr/bin/env python3\nraise SystemExit(0)\n", encoding="utf-8")
            helper.chmod(0o700)
            _expect_qualification_failure(
                lambda: run_control_helper(resolved, "stop", "ollama"),
                "helper replacement revalidation",
            )
        finally:
            if previous_path is None:
                os.environ.pop(CONTROL_HELPER_ENV, None)
            else:
                os.environ[CONTROL_HELPER_ENV] = previous_path
            if previous_digest is None:
                os.environ.pop(CONTROL_HELPER_SHA256_ENV, None)
            else:
                os.environ[CONTROL_HELPER_SHA256_ENV] = previous_digest
            if previous_secret is None:
                os.environ.pop("HEPTA_INF0C_TEST_SECRET", None)
            else:
                os.environ["HEPTA_INF0C_TEST_SECRET"] = previous_secret
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
        helper = resolve_control_helper()
        restart = {
            "controlled_restart_executed": True,
            "helper_sha256": helper.sha256,
            "helper_revalidated_per_invocation": True,
            "ollama": controlled_restart(
                helper,
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
        "schema": "hepta.inference.inf0c.disconnect_restart_evidence.v3",
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
        "loopback_addresses_pinned_to_literals": True,
        "semantic_output_verified": True,
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
