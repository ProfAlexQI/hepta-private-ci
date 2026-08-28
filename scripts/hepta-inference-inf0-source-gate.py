#!/usr/bin/env python3
"""Fail-closed source gate for Hepta inference INF-0C.

A PASS proves only exact source, receipt, privacy, and negative-authority shape.
It does not prove Rust compilation, real-model behavior, hardware performance,
operator acceptance, production authority, or INF-1 activation.
"""

from __future__ import annotations

import json
import subprocess
import sys
import tomllib
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
DOCS = ROOT / "docs/hepta-vnext/inference"
PLAN = DOCS / "HEPTA_INFERENCE_RUNTIME_DEVELOPMENT_PLAN_V2.md"
MATRIX = DOCS / "HEPTA_INFERENCE_STAGE_MATRIX_V2.json"
STATUS = DOCS / "HEPTA_INFERENCE_IMPLEMENTATION_STATUS_V1.json"
NOTE = DOCS / "HEPTA_INFERENCE_INF0C_STATUS_2026-08-28.md"
RECEIPT = DOCS / "HEPTA_INFERENCE_INF0C_SOURCE_RECEIPT_2026-08-28.json"
REFERENCE = ROOT / "tools/hepta-inference-inf0"
WORKFLOW = ROOT / ".github/workflows/hepta-inference-inf0.yml"
E2E = ROOT / "scripts/hepta-inference-inf0c-real-e2e.py"

BASE_COMMIT = "fe0889ecd46a5fc89de7b1ff3f28158c133a3502"
BASE_TREE = "636341eb865b7c6d669958a96e7959de74fee020"
BRANCH = "codex/hepta-inference-runtime-v2-20260828"
PLAN_BLOB = "4381207acce1bf6371c248dc3280fff1f2ae59ce"
PASS = "PASS_HEPTA_INFERENCE_INF0C_SOURCE_ONLY"

FALSE_AUTHORITY = (
    "production_listener", "production_writer", "provider_effect", "external_effect",
    "shared_kg_write", "memory_write", "route_write", "fleet_write", "model_npu",
    "remote_inference", "automatic_model_install", "operator_acceptance", "promotion", "release",
)
IMPLEMENTED = (
    "reference_contract_crate", "lmstudio_load_readiness_fence",
    "lmstudio_implicit_install_disabled", "lmstudio_async_subprocess_timeout",
    "lmstudio_cli_sha256_provenance", "lmstudio_bounded_stderr",
    "ollama_readiness_fail_closed", "ollama_non_2xx_typed_failure",
    "ollama_pull_stream_fail_closed", "ollama_pull_frame_bound",
    "ollama_pull_idle_timeout",
    "ollama_implicit_install_disabled", "unknown_oss_provider_fail_closed",
    "responses_proxy_digest_only_dump", "responses_proxy_owner_only_dump",
    "responses_proxy_dump_retention_and_count_bound",
    "responses_proxy_atomic_file_reservation", "responses_proxy_partial_write_cleanup",
    "real_software_e2e_harness", "real_e2e_proxy_and_redirect_disabled",
    "local_provider_loopback_only", "local_provider_direct_no_proxy_no_redirect",
    "lmstudio_sanitized_subprocess_environment", "control_response_body_bound",
)


class GateError(RuntimeError):
    pass


def require(condition: bool, message: str) -> None:
    if not condition:
        raise GateError(message)


def text(path: Path) -> str:
    require(path.is_file(), f"missing file: {path.relative_to(ROOT)}")
    return path.read_text(encoding="utf-8")


def object_json(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(text(path))
    except json.JSONDecodeError as error:
        raise GateError(f"invalid JSON in {path.relative_to(ROOT)}: {error}") from error
    require(isinstance(value, dict), f"{path.relative_to(ROOT)} must contain a JSOT object")
    return value


def table_toml(path: Path) -> dict[str, Any]:
    try:
        return tomllib.loads(text(path))
    except tomllib.TOMLDecodeError as error:
        raise GateError(f"invalid TOML in {path.relative_to(ROOT)}: {error}") from error


def git(*args: str) -> str:
    run = subprocess.run(["git", *args], cwd=ROOT, capture_output=True, text=True, check=False)
    if run.returncode:
        raise GateError(f"git {' '.join(args)} failed: {run.stderr.strip() or run.stdout.strip()}")
    return run.stdout.strip()


def tokens(source: str, required: tuple[str, ...], label: str) -> None:
    for token in required:
        require(token in source, f"{label} missing token: {token}")



def rust_bundle(directory: Path, stem: str) -> str:
    paths = sorted(directory.glob(f"{stem}*.rs"))
    require(paths, f"no Rust sources found for {directory.relative_to(ROOT)}/{stem}*.rs")
    return "\n".join(text(path) for path in paths)


def authority(value: dict[str, Any], label: str) -> None:
    block = value.get("authority")
    require(isinstance(block, dict), f"{label}.authority missing")
    require(block.get("qualification_only") is True, f"{label} not qualification-only")
    for field in FALSE_AUTHORITY:
        require(block.get(field) is False, f"{label}.authority.{field} must be false")


def main() -> int:
    plan = text(PLAN)
    matrix = object_json(MATRIX)
    status = object_json(STATUS)
    receipt = object_json(RECEIPT)
    note = text(NOTE)
    workflow = text(WORKFLOW)
    e2e = text(E2E)

    for label, value in (("matrix", matrix), ("status", status), ("receipt", receipt)):
        require(value.get("plan_git_blob_sha1") == PLAN_BLOB, f"{label} plan blob drift")
        binding = value.get("source_binding")
        require(isinstance(binding, dict), f"{label}.source_binding missing")
        require(binding.get("commit") == BASE_COMMIT, f"{label} base commit drift")
        require(binding.get("tree") == BASE_TREE, f"{label} base tree drift")
        authority(value, label)
    require(git("rev-parse", f"HEAD:{PLAN.relative_to(ROOT)}") == PLAN_BLOB, "plan blob drift")
    require(matrix.get("branch") == BRANCH and status.get("development_branch") == BRANCH, "branch drift")
    require(matrix.get("current_stage") == "INF-0C", "current stage must be INF-0C")
    require(matrix.get("overall_status") == status.get("status") == "SOURCE_PRESENT_NOT_RUN", "status drift")
    require(status.get("qualified") is False and receipt.get("qualified") is False, "qualified must remain false")

    stages = {stage.get("id"): stage for stage in matrix.get("stages", []) if isinstance(stage, dict)}
    for stage_id in ("INF-0A", "INF-0B", "INF-0C"):
        require(stages.get(stage_id, {}).get("status") == "SOURCE_PRESENT_NOT_RUN", f"{stage_id} status drift")
        require(stages.get(stage_id, {}).get("qualified") is False, f"{stage_id} qualified early")
    require(stages.get("INF-0C", {}).get("source_complete") is True, "INF-0C source incomplete")
    require(stages.get("INF-0C", {}).get("real_software_e2e_executed") is False, "real E2E claimed early")
    require(stages.get("INF-1", {}).get("status") == "NOT_STARTED", "INF-1 activated early")

    implemented = status.get("implemented")
    require(isinstance(implemented, dict), "status.implemented missing")
    for flag in IMPLEMENTED:
        require(implemented.get(flag) is True, f"implemented flag false: {flag}")
    for flag in ("hepta_inferd", "native_worker", "real_model_e2e", "hardware_receipt"):
        require(implemented.get(flag) is False, f"status claims {flag}")

    reference_cargo = text(REFERENCE / "Cargo.toml")
    reference_rust = text(REFERENCE / "src/lib.rs")
    require("[workspace]" in reference_cargo and "[dependencies]" not in reference_cargo, "reference crate isolation drift")
    for banned in ("std::net", "TcpListener", "UdpSocket", "reqwest", "tokio", "unsafe {", "raw_prompt", "MemoryWrite", "KgWrite"):
        require(banned not in reference_rust, f"reference crate contains {banned}")

    ollama_cargo = table_toml(ROOT / "codex-rs/ollama/Cargo.toml")
    ollama_lib = text(ROOT / "codex-rs/ollama/src/lib.rs")
    ollama_client = rust_bundle(ROOT / "codex-rs/ollama/src", "client")
    ollama_buffer = text(ROOT / "codex-rs/ollama/src/line_buffer.rs")
    ollama_tokio = ollama_cargo.get("dependencies", {}).get("tokio", {})
    require("time" in set(ollama_tokio.get("features", [])), "Ollama tokio time feature missing")
    require("wiremock" not in ollama_cargo.get("dependencies", {}), "wiremock runtime dependency")
    require("wiremock" in ollama_cargo.get("dev-dependencies", {}), "wiremock test dependency missing")
    tokens(ollama_client, (
        "OLLAMA_HTTP_STATUS", "OLLAMA_PULL_TRANSPORT_ERROR",
        "OLLAMA_PULL_INVALID_UTF8", "OLLAMA_PULL_INVALID_JSON",
        "OLLAMA_PULL_SERVER_ERROR", "OLLAMA_PULL_FRAME_TOO_LARGE",
        "OLLAMA_PULL_UNEXPECTED_EOF", "OLLAMA_PULL_IDLE_TIMEOUT",
        "MAX_PULL_FRAME_BYTES", "MAX_CONTROL_RESPONSE_BYTES",
        "OLLAMA_CONTROL_RESPONSE_TOO_LARGE",
        "OLLAMA_BASE_URL_NOT_LOOPBACK_HTTP", "validate_loopback_http_base_url",
        "build_direct", "without_redirects", "without_request_logging",
    ), "Ollama")
    require("return Ok(Vec::new())" not in ollama_client, "Ollama folds failure into empty list")
    require("pull_with_reporter(model" not in ollama_lib and "automatic model installation is disabled" in ollama_lib, "Ollama implicit install fence missing")
    require("take_remaining" in ollama_buffer, "Ollama trailing frame support missing")
    require("refusing to skip readiness checks" in text(ROOT / "codex-rs/utils/oss/src/lib.rs"), "unknown provider fence missing")

    lm_cargo = table_toml(ROOT / "codex-rs/lmstudio/Cargo.toml")
    lm_lib = text(ROOT / "codex-rs/lmstudio/src/lib.rs")
    lm_client = rust_bundle(ROOT / "codex-rs/lmstudio/src", "client")
    lm_sha = text(ROOT / "codex-rs/lmstudio/src/sha256.rs")
    tokio = lm_cargo.get("dependencies", {}).get("tokio", {})
    require({"io-util", "macros", "process", "rt", "time"}.issubset(set(tokio.get("features", []))), "LM Studio tokio features incomplete")
    tokens(lm_client, (
        "CODEX_LMS_CLI_SHA256", "LMSTUDIO_CLI_DIGEST_REQUIRED",
        "LMSTUDIO_CLI_DIGEST_MISMATCH", "tokio::process::Command",
        "tokio::time::timeout", ".kill_on_drop(true)",
        "read_bounded_stderr", "MAX_STDERR_BYTES",
        "LMSTUDIO_BASE_URL_NOT_LOOPBACK_HTTP", "validate_loopback_http_base_url",
        "build_direct", "without_redirects", "without_request_logging",
        ".env_clear()", "apply_sanitized_environment",
        "MAX_CONTROL_RESPONSE_BYTES", "LMSTUDIO_CONTROL_RESPONSE_TOO_LARGE",
    ), "LM Studio")
    require("std::process::Command" not in lm_client, "LM Studio still uses blocking Command")
    require("download_model(model)" not in lm_lib and "automatic model installation is disabled" in lm_lib, "LM Studio implicit install fence missing")
    tokens(lm_sha, ("known_answer_abc", "digest_reader"), "LM Studio SHA-256")

    proxy_lib = text(ROOT / "codex-rs/responses-api-proxy/src/lib.rs")
    proxy_dump = text(ROOT / "codex-rs/responses-api-proxy/src/dump.rs")
    proxy_sha = text(ROOT / "codex-rs/responses-api-proxy/src/sha256.rs")
    tokens(proxy_dump, (
        "sha256_digest_v1", "byte_length", "complete", "create_new(true)",
        "0o600", "0o700", "DUMP_RETENTION", "MAX_DUMP_FILES",
        "ResponseBodyDump", "reserve_capacity", "release_capacity",
        "symlink_metadata", "fs::remove_file(path)",
    ), "proxy dump")
    for banned in ("body: Vec<u8>", "dump_body(", "String::from_utf8_lossy(body)"):
        require(banned not in proxy_dump, f"proxy persists raw body via {banned}")
    require("Raw request and response bodies are never persisted" in proxy_lib, "proxy privacy wording missing")
    tokens(proxy_sha, ("known_answer_abc", "Sha256"), "proxy SHA-256")

    tokens(e2e, (
        "ALLOWED_HOSTS", "endpoint host is not loopback",
        "urllib.request.ProxyHandler({})", "NoRedirect",
        "implicit_download", "raw_model_output_persisted",
        "cancellation_executed", "controlled_restart_executed",
        "--execute", "urllib.request",
    ), "real E2E harness")
    require("SOURCE_PRESENT_NOT_RUN" in note, "status note evidence boundary missing")
    for token in ("cargo fmt --manifest-path tools/hepta-inference-inf0/Cargo.toml", "cargo test --manifest-path tools/hepta-inference-inf0/Cargo.toml", "cargo clippy --manifest-path tools/hepta-inference-inf0/Cargo.toml", "-p codex-responses-api-proxy", "python3 scripts/hepta-inference-inf0c-real-e2e.py"):
        require(token in workflow, f"workflow missing {token}")

    source_commit = git("rev-parse", "HEAD^")
    source_tree = git("show", "-s", "--format=%T", source_commit)
    require(receipt.get("source_candidate_commit") == source_commit, "receipt parent commit mismatch")
    require(receipt.get("source_candidate_tree") == source_tree, "receipt parent tree mismatch")
    changed = {line for line in git("diff", "--name-only", "HEAD^", "HEAD").splitlines() if line}
    require(changed == {str(RECEIPT.relative_to(ROOT))}, "receipt commit must add only INF-0C receipt")
    require("SOURCE_PRESENT_NOT_RUN" in plan and "qualified=false" in plan, "plan authority/status drift")
    require(receipt.get("claim") == "SOURCE_PRESENT_NOT_RUN", "receipt claim drift")
    print(PASS)
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except GateError as error:
        print(f"FAIL_HEPTA_INFERENCE_INF0C_SOURCE_GATE: {error}", file=sys.stderr)
        raise SystemExit(1) from error
