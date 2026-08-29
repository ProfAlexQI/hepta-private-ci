#!/usr/bin/env python3
from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CRATE = ROOT / "codex-rs/hepta-infer-adapter"
SOURCE = CRATE / "src/lib.rs"
STATUS = ROOT / "docs/hepta-vnext/inference/HEPTA_INFERENCE_INF2A_ADAPTER_CONTRACT_STATUS_V1.json"


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(f"FAIL_HEPTA_INFERENCE_INF2A_ADAPTER_CONTRACT: {message}")


def main() -> None:
    require((CRATE / "Cargo.toml").is_file(), "adapter Cargo.toml missing")
    require(SOURCE.is_file(), "adapter source missing")
    require(STATUS.is_file(), "adapter status missing")
    source = SOURCE.read_text(encoding="utf-8")
    status = json.loads(STATUS.read_text(encoding="utf-8"))

    for marker in (
        "pub trait LocalInferenceAdapter",
        "pub struct AdapterRegistry",
        "CapabilityStatus::UnsupportedFailClosed",
        "AdapterError::CapabilityUnsupported",
        "remote_fallback_used: false",
        "implicit_model_install_used: false",
        "raw_prompt_persisted: false",
        "raw_output_persisted: false",
    ):
        require(marker in source, f"source marker missing: {marker}")

    for forbidden in (
        "TcpListener",
        "reqwest",
        "HttpClient",
        "download_model",
        "raw_prompt:",
        "raw_output:",
        "memory_write(",
        "kg_write(",
    ):
        require(forbidden not in source, f"forbidden source marker present: {forbidden}")

    require(status["stage"] == "INF-2A", "wrong stage")
    require(status["status"] == "SOURCE_PRESENT_NOT_RUN", "status must remain source-only")
    require(status["activated"] is False, "runtime activation must remain false")
    require(status["qualified"] is False, "qualification must remain false")
    require(all(value is False for value in status["authority"].values()), "authority must remain closed")
    require(status["implemented"]["real_ollama_adapter"] is False, "real Ollama adapter not yet present")
    require(status["implemented"]["real_lmstudio_adapter"] is False, "real LM Studio adapter not yet present")
    require(status["implemented"]["daemon_uds_fixture_route"] is False, "daemon route not yet present")
    require(status["implemented"]["product_caller_wiring"] is False, "product wiring must remain false")
    print("PASS_HEPTA_INFERENCE_INF2A_ADAPTER_CONTRACT_SOURCE_ONLY")


if __name__ == "__main__":
    main()
