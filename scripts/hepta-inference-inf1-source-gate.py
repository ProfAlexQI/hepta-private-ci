#!/usr/bin/env python3
from __future__ import annotations

import json
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CORE = ROOT / "codex-rs" / "hepta-infer-core"
DAEMON = ROOT / "codex-rs" / "hepta-inferd"
STATUS = ROOT / "docs" / "hepta-vnext" / "inference" / "HEPTA_INFERENCE_INF1_SOURCE_STATUS_V1.json"
RECEIPT = ROOT / "docs" / "hepta-vnext" / "inference" / "HEPTA_INFERENCE_INF1_SOURCE_RECEIPT_2026-08-29.json"


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


def git(*args: str) -> str:
    return subprocess.check_output(["git", *args], cwd=ROOT, text=True).strip()


def main() -> None:
    for path in (
        CORE / "Cargo.toml",
        CORE / "src" / "lib.rs",
        CORE / "src" / "protocol.rs",
        CORE / "src" / "controller.rs",
        DAEMON / "Cargo.toml",
        DAEMON / "src" / "lib.rs",
        DAEMON / "src" / "main.rs",
        STATUS,
    ):
        require(path.is_file(), f"missing INF-1 source: {path.relative_to(ROOT)}")

    source = "\n".join(
        path.read_text(encoding="utf-8")
        for path in sorted((*CORE.rglob("*.rs"), *DAEMON.rglob("*.rs")))
    )
    forbidden = (
        "TcpListener",
        "TcpStream",
        "reqwest",
        "hyper::",
        "automatic_model_install: true",
        "production_listener: true",
        "remote_inference: true",
        "memory_write: true",
        "shared_kg_write: true",
        "raw_prompt",
        "prompt_text",
        "model_output",
    )
    for marker in forbidden:
        require(marker not in source, f"forbidden INF-1 source marker: {marker}")
    required = (
        "UnixListener",
        "ensure_current_user_peer",
        "MAX_FRAME_BYTES",
        "ProtocolNonCanonical",
        "StaleRequestGeneration",
        "StaleCancelGeneration",
        "StaleBackendGeneration",
        "QueueFull",
        "TenantQueueFull",
        "TerminalReceipt",
        "forced_worker_termination",
        "create_new(true)",
        "0o600",
        "prepare_private_socket_directory",
        "ConnectionTaskError::Peer",
        "ConnectionTaskError::Infrastructure",
        "truncated_connection_is_connection_local",
        "timeout(Duration::from_secs(5)",
    )
    for marker in required:
        require(marker in source, f"required INF-1 source marker missing: {marker}")

    status = json.loads(STATUS.read_text(encoding="utf-8"))
    require(status["stage"] == "INF-1", "INF-1 status stage drift")
    require(status["status"] == "SOURCE_PRESENT_NOT_RUN", "INF-1 status must remain source-only")
    require(status["activated"] is False, "INF-1 must remain inactive before qualification")
    require(status["qualified"] is False, "INF-1 must not self-qualify")
    require(all(value is False for value in status["authority"].values()), "INF-1 authority must remain closed")
    require(status["implemented"]["tcp_listener"] is False, "TCP listener must remain absent")
    require(status["implemented"]["product_caller_wiring"] is False, "product caller wiring must remain absent")

    if RECEIPT.exists():
        receipt = json.loads(RECEIPT.read_text(encoding="utf-8"))
        require(receipt["qualified"] is False, "source receipt must remain unqualified")
        require(all(value is False for value in receipt["authority"].values()), "receipt authority drift")
        head = git("rev-parse", "HEAD")
        parent = git("rev-parse", "HEAD^")
        parent_tree = git("show", "-s", "--format=%T", "HEAD^")
        require(receipt["receipt_commit"] == head, "receipt head binding mismatch")
        require(receipt["source_candidate_commit"] == parent, "receipt parent binding mismatch")
        require(receipt["source_candidate_tree"] == parent_tree, "receipt source tree mismatch")
        changed = git("diff", "--name-only", "HEAD^", "HEAD").splitlines()
        require(changed == [str(RECEIPT.relative_to(ROOT))], "receipt commit must be append-only")

    print("PASS_HEPTA_INFERENCE_INF1_SOURCE_ONLY")


if __name__ == "__main__":
    main()
