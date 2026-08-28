#!/usr/bin/env python3
"""Fail-closed, merge-ref-aware source gate for Hepta inference INF-0C."""
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
RECEIPT = DOCS / "HEPTA_INFERENCE_INF0C_SOURCE_RECEIPT_2026-08-28.json"
WORKFLOW = ROOT / ".github/workflows/hepta-inference-inf0.yml"
E2E = ROOT / "scripts/hepta-inference-inf0c-real-e2e.py"
EVIDENCE_V2 = ROOT / "scripts/hepta-inference-inf0c-evidence-v2.py"
BASE = "fe0889ecd46a5fc89de7b1ff3f28158c133a3502"
BASE_TREE = "636341eb865b7c6d669958a96e7959de74fee020"
BRANCH = "codex/hepta-inference-runtime-v2-20260828"
PLAN_BLOB = "4381207acce1bf6371c248dc3280fff1f2ae59ce"
PASS = "PASS_HEPTA_INFERENCE_INF0C_SOURCE_ONLY"
FALSE_AUTHORITY = (
    "production_listener", "production_writer", "provider_effect", "external_effect",
    "shared_kg_write", "memory_write", "route_write", "fleet_write", "model_npu",
    "remote_inference", "automatic_model_install", "operator_acceptance", "promotion", "release",
)


class GateError(RuntimeError):
    pass


def need(condition: bool, message: str) -> None:
    if not condition:
        raise GateError(message)


def read(path: Path) -> str:
    need(path.is_file(), f"missing {path.relative_to(ROOT)}")
    return path.read_text(encoding="utf-8")


def obj(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(read(path))
    except json.JSONDecodeError as error:
        raise GateError(f"invalid JSON in {path.relative_to(ROOT)}: {error}") from error
    need(isinstance(value, dict), f"{path.relative_to(ROOT)} must be an object")
    return value


def toml(path: Path) -> dict[str, Any]:
    try:
        return tomllib.loads(read(path))
    except tomllib.TOMLDecodeError as error:
        raise GateError(f"invalid TOML in {path.relative_to(ROOT)}: {error}") from error


def git(*args: str, check: bool = True) -> str:
    run = subprocess.run(["git", *args], cwd=ROOT, capture_output=True, text=True, check=False)
    if check and run.returncode:
        raise GateError(f"git {' '.join(args)} failed: {run.stderr.strip()}")
    return run.stdout.strip() if run.returncode == 0 else ""


def candidate_head() -> str:
    parents = git("rev-list", "--parents", "-n", "1", "HEAD").split()
    need(len(parents) in (2, 3), "unexpected checkout parent shape")
    candidate = git("rev-parse", "HEAD^2") if len(parents) == 3 else git("rev-parse", "HEAD")
    need(git("merge-base", "--is-ancestor", candidate, "HEAD", check=False) == "", "candidate ancestry check failed")
    return candidate


def markers(source: str, values: tuple[str, ...], label: str) -> None:
    for value in values:
        need(value in source, f"{label} missing {value}")


def closed(value: dict[str, Any], label: str) -> None:
    authority = value.get("authority")
    need(isinstance(authority, dict), f"{label}.authority missing")
    need(authority.get("qualification_only") is True, f"{label} not qualification-only")
    for field in FALSE_AUTHORITY:
        need(authority.get(field) is False, f"{label}.{field} must be false")


def main() -> int:
    candidate = candidate_head()
    matrix, status, receipt = obj(MATRIX), obj(STATUS), obj(RECEIPT)
    for label, value in (("matrix", matrix), ("status", status), ("receipt", receipt)):
        need(value.get("plan_git_blob_sha1") == PLAN_BLOB, f"{label} plan drift")
        binding = value.get("source_binding")
        need(isinstance(binding, dict), f"{label} binding missing")
        need(binding.get("commit") == BASE and binding.get("tree") == BASE_TREE, f"{label} base drift")
        closed(value, label)
    need(git("rev-parse", f"{candidate}:{PLAN.relative_to(ROOT)}") == PLAN_BLOB, "candidate plan drift")
    need(git("rev-parse", f"{candidate}:{RECEIPT.relative_to(ROOT)}") == git("rev-parse", f"HEAD:{RECEIPT.relative_to(ROOT)}"), "merge altered receipt")
    need(matrix.get("branch") == BRANCH and status.get("development_branch") == BRANCH, "branch drift")
    need(matrix.get("current_stage") == "INF-0C", "stage drift")
    need(matrix.get("overall_status") == status.get("status") == "SOURCE_PRESENT_NOT_RUN", "status drift")
    need(status.get("qualified") is False and receipt.get("qualified") is False, "qualified early")
    stages = {item.get("id"): item for item in matrix.get("stages", []) if isinstance(item, dict)}
    inf0c = stages.get("INF-0C", {})
    need(inf0c.get("source_complete") is True, "INF-0C source incomplete")
    need(inf0c.get("real_software_e2e_executed") is False, "E2E claimed early")
    need(inf0c.get("transport_disconnect_harness_source_complete") is True, "disconnect harness source incomplete")
    need(inf0c.get("controlled_restart_harness_source_complete") is True, "restart harness source incomplete")
    need(inf0c.get("backend_cancellation_acknowledged") is False, "backend cancellation acknowledged early")
    need(stages.get("INF-1", {}).get("status") == "NOT_STARTED", "INF-1 activated early")
    implemented = status.get("implemented", {})
    for flag in (
        "reference_contract_crate", "ollama_non_2xx_typed_failure",
        "ollama_pull_stream_fail_closed", "ollama_pull_idle_timeout",
        "lmstudio_cli_sha256_provenance", "lmstudio_sanitized_subprocess_environment",
        "responses_proxy_digest_only_dump", "responses_proxy_atomic_file_reservation",
        "local_provider_loopback_only", "local_provider_direct_no_proxy_no_redirect",
        "real_software_e2e_harness", "real_e2e_transport_disconnect_harness",
        "real_e2e_controlled_restart_harness", "real_e2e_trusted_control_helper",
        "control_response_body_bound",
    ):
        need(implemented.get(flag) is True, f"implemented flag false: {flag}")
    for flag in ("hepta_inferd", "native_worker", "real_model_e2e", "hardware_receipt"):
        need(implemented.get(flag) is False, f"status claims {flag}")

    ref_cargo = read(ROOT / "tools/hepta-inference-inf0/Cargo.toml")
    ref_rust = read(ROOT / "tools/hepta-inference-inf0/src/lib.rs")
    need("[workspace]" in ref_cargo and "[dependencies]" not in ref_cargo, "reference isolation drift")
    for banned in ("std::net", "reqwest", "tokio", "unsafe {", "raw_prompt", "MemoryWrite", "KgWrite"):
        need(banned not in ref_rust, f"reference contains {banned}")

    ollama_cargo = toml(ROOT / "codex-rs/ollama/Cargo.toml")
    ollama_http = read(ROOT / "codex-rs/ollama/src/client_http.inc.rs")
    ollama = "\n".join(read(path) for path in sorted((ROOT / "codex-rs/ollama/src").glob("client*.rs")))
    need("wiremock" not in ollama_cargo.get("dependencies", {}), "wiremock runtime dependency")
    need("wiremock" in ollama_cargo.get("dev-dependencies", {}), "wiremock dev dependency missing")
    markers(ollama, (
        "OLLAMA_HTTP_STATUS", "OLLAMA_PULL_TRANSPORT_ERROR", "OLLAMA_PULL_INVALID_JSON",
        "OLLAMA_PULL_FRAME_TOO_LARGE", "OLLAMA_PULL_IDLE_TIMEOUT", "MAX_CONTROL_RESPONSE_BYTES",
        "OLLAMA_BASE_URL_NOT_LOOPBACK_HTTP", "build_direct", "without_redirects",
    ), "Ollama")
    need("return Ok(Vec::new())" not in ollama_http, "Ollama model discovery folds failure into empty list")
    need("automatic model installation is disabled" in read(ROOT / "codex-rs/ollama/src/lib.rs"), "Ollama install fence missing")

    lm = "\n".join(read(path) for path in sorted((ROOT / "codex-rs/lmstudio/src").glob("client*.rs")))
    markers(lm, (
        "CODEX_LMS_CLI_SHA256", "LMSTUDIO_CLI_DIGEST_MISMATCH", "tokio::process::Command",
        ".kill_on_drop(true)", ".env_clear()", "apply_sanitized_environment",
        "LMSTUDIO_BASE_URL_NOT_LOOPBACK_HTTP", "MAX_CONTROL_RESPONSE_BYTES", "build_direct",
    ), "LM Studio")
    need("std::process::Command" not in lm, "LM Studio blocking command remains")
    need("automatic model installation is disabled" in read(ROOT / "codex-rs/lmstudio/src/lib.rs"), "LM Studio install fence missing")

    proxy = read(ROOT / "codex-rs/responses-api-proxy/src/dump.rs")
    markers(proxy, (
        "sha256_digest_v1", "byte_length", "complete", "create_new(true)", "0o600", "0o700",
        "DUMP_RETENTION", "MAX_DUMP_FILES", "reserve_capacity", "release_capacity",
        "symlink_metadata", "fs::remove_file(path)",
    ), "proxy dump")
    for banned in ("body: Vec<u8>", "dump_body(", "String::from_utf8_lossy(body)"):
        need(banned not in proxy, f"proxy raw-body path: {banned}")

    e2e, evidence, workflow = read(E2E), read(EVIDENCE_V2), read(WORKFLOW)
    markers(e2e, (
        "urllib.request.ProxyHandler({})", "NoRedirect", "endpoint host is not loopback",
        "implicit_download", "raw_model_output_persisted", "cancellation_executed",
        "controlled_restart_executed", "--execute",
    ), "real E2E")
    markers(evidence, (
        "http.client", "transport_disconnect_executed", "backend_cancellation_acknowledged",
        "HEPTA_INF0C_SERVICE_CONTROL_HELPER", "HEPTA_INF0C_SERVICE_CONTROL_HELPER_SHA256",
        "parse_sha256_binding", "resolve_control_helper", "subprocess.run",
        "stdout=subprocess.DEVNULL", "stderr=subprocess.DEVNULL", "shell=False",
        "sanitized_helper_environment", "controlled_restart_executed",
        "unavailable_observed", "raw_helper_output_persisted", "--self-test",
    ), "evidence v2")
    for command in (
        "cargo fmt --manifest-path tools/hepta-inference-inf0/Cargo.toml",
        "cargo test --manifest-path tools/hepta-inference-inf0/Cargo.toml",
        "cargo clippy --manifest-path tools/hepta-inference-inf0/Cargo.toml",
        "-p codex-responses-api-proxy", "python3 scripts/hepta-inference-inf0c-real-e2e.py",
        "python3 scripts/hepta-inference-inf0c-evidence-v2.py --self-test",
        "--run-controlled-restart",
    ):
        need(command in workflow, f"workflow missing {command}")

    source = git("rev-parse", f"{candidate}^")
    source_tree = git("show", "-s", "--format=%T", source)
    need(receipt.get("source_candidate_commit") == source, "receipt parent mismatch")
    need(receipt.get("source_candidate_tree") == source_tree, "receipt tree mismatch")
    changed = {line for line in git("diff", "--name-only", source, candidate).splitlines() if line}
    need(changed == {str(RECEIPT.relative_to(ROOT))}, "receipt commit must modify only receipt")
    need(receipt.get("claim") == "SOURCE_PRESENT_NOT_RUN", "receipt claim drift")
    need("SOURCE_PRESENT_NOT_RUN" in read(PLAN) and "qualified=false" in read(PLAN), "plan status drift")
    print(PASS)
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except GateError as error:
        print(f"FAIL_HEPTA_INFERENCE_INF0C_SOURCE_GATE: {error}", file=sys.stderr)
        raise SystemExit(1) from error
