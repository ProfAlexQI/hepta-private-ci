#!/usr/bin/env python3
from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
LIB = ROOT / "codex-rs/hepta-inferd/src/lib.rs"
TESTS = ROOT / "codex-rs/hepta-inferd/src/tests.rs"
GATE = ROOT / "scripts/hepta-inference-inf1-source-gate.py"
STATUS = ROOT / "docs/hepta-vnext/inference/HEPTA_INFERENCE_INF1_SOURCE_STATUS_V1.json"


def replace_once(text: str, old: str, new: str, label: str) -> str:
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"{label}: expected one match, found {count}")
    return text.replace(old, new, 1)


def patch_lib() -> None:
    text = LIB.read_text(encoding="utf-8")
    text = replace_once(
        text,
        "use tokio::task::JoinSet;\n\n",
        "use tokio::task::JoinSet;\n\n"
        "#[derive(Debug)]\n"
        "enum ConnectionTaskError {\n"
        "    Peer,\n"
        "    Infrastructure(io::Error),\n"
        "}\n\n",
        "connection error type",
    )
    text = replace_once(
        text,
        "                let stream = accepted?;\n"
        "                stream.ensure_current_user_peer()?;\n",
        "                let stream = accepted?;\n"
        "                if stream.ensure_current_user_peer().is_err() {\n"
        "                    continue;\n"
        "                }\n",
        "peer isolation",
    )
    text = replace_once(
        text,
        "                    Some(Ok(Ok(()))) => {}\n"
        "                    Some(Ok(Err(error))) => return Err(error),\n",
        "                    Some(Ok(Ok(()))) => {}\n"
        "                    Some(Ok(Err(ConnectionTaskError::Peer))) => {}\n"
        "                    Some(Ok(Err(ConnectionTaskError::Infrastructure(error)))) => return Err(error),\n",
        "join classification",
    )
    text = replace_once(
        text,
        ") -> io::Result<()> {\n"
        "    let request = read_message(&mut stream, max_frame_bytes).await?;\n",
        ") -> Result<(), ConnectionTaskError> {\n"
        "    let request = read_message(&mut stream, max_frame_bytes)\n"
        "        .await\n"
        "        .map_err(|_| ConnectionTaskError::Peer)?;\n",
        "connection result",
    )
    text = replace_once(
        text,
        "        let now_unix_ms = unix_time_ms()?;\n",
        "        let now_unix_ms = unix_time_ms().map_err(ConnectionTaskError::Infrastructure)?;\n",
        "clock classification",
    )
    text = replace_once(
        text,
        "    persist_terminal_responses(&receipt_store, &response).await?;\n"
        "    write_message(&mut stream, &response, max_frame_bytes).await?;\n"
        "    stream.shutdown().await\n",
        "    persist_terminal_responses(&receipt_store, &response)\n"
        "        .await\n"
        "        .map_err(ConnectionTaskError::Infrastructure)?;\n"
        "    write_message(&mut stream, &response, max_frame_bytes)\n"
        "        .await\n"
        "        .map_err(|_| ConnectionTaskError::Peer)?;\n"
        "    stream.shutdown().await.map_err(|_| ConnectionTaskError::Peer)\n",
        "write classification",
    )
    text = text.replace("use std::os::unix::fs::OpenOptionsExt;\n", "")
    if "OpenOptionsExt" in text:
        raise SystemExit("unused OpenOptionsExt import remains")
    LIB.write_text(text, encoding="utf-8")


def patch_tests() -> None:
    text = TESTS.read_text(encoding="utf-8")
    text = replace_once(
        text,
        "use tokio::time::sleep;\n",
        "use tokio::time::sleep;\nuse tokio::time::timeout;\n",
        "timeout import",
    )
    text = replace_once(
        text,
        "            match task.await {\n",
        "            match must(timeout(Duration::from_secs(5), task).await) {\n",
        "harness shutdown timeout",
    )
    text = replace_once(
        text,
        "    let error = match serve_with_shutdown(harness.config.clone(), async {\n"
        "        let _ = receiver.await;\n"
        "    })\n"
        "    .await\n"
        "    {\n",
        "    let second = timeout(\n"
        "        Duration::from_secs(5),\n"
        "        serve_with_shutdown(harness.config.clone(), async {\n"
        "            let _ = receiver.await;\n"
        "        }),\n"
        "    )\n"
        "    .await;\n"
        "    let error = match must(second) {\n",
        "second instance timeout",
    )
    text = replace_once(
        text,
        "    match task.await {\n",
        "    match must(timeout(Duration::from_secs(5), task).await) {\n",
        "restart shutdown timeout",
    )
    text += """

#[tokio::test]
async fn truncated_connection_is_connection_local() {
    let mut harness = Harness::start().await;
    let stream = must(UnixStream::connect(&harness.config.socket_path).await);
    drop(stream);
    sleep(Duration::from_millis(25)).await;
    assert_eq!(
        exchange(&harness.config.socket_path, ClientMessage::Ping { nonce: 19 }).await,
        ServerMessage::Pong { nonce: 19 }
    );
    harness.stop().await;
}
"""
    TESTS.write_text(text, encoding="utf-8")


def patch_gate() -> None:
    text = GATE.read_text(encoding="utf-8")
    text = replace_once(
        text,
        '        "prepare_private_socket_directory",\n',
        '        "prepare_private_socket_directory",\n'
        '        "ConnectionTaskError::Peer",\n'
        '        "ConnectionTaskError::Infrastructure",\n'
        '        "truncated_connection_is_connection_local",\n'
        '        "timeout(Duration::from_secs(5)",\n',
        "source gate markers",
    )
    GATE.write_text(text, encoding="utf-8")


def write_status() -> None:
    STATUS.parent.mkdir(parents=True, exist_ok=True)
    authority_names = (
        "production_listener",
        "production_writer",
        "provider_effect",
        "external_effect",
        "shared_kg_write",
        "memory_write",
        "route_write",
        "fleet_write",
        "model_npu",
        "remote_inference",
        "automatic_model_install",
        "operator_acceptance",
        "promotion",
        "release",
    )
    value = {
        "schema": "hepta.inference.inf1_source_status.v2",
        "plan_id": "HEPTA-INFERENCE-RUNTIME-V2",
        "stage": "INF-1",
        "status": "SOURCE_PRESENT_NOT_RUN",
        "activated": False,
        "qualified": False,
        "implemented": {
            "backend_neutral_core": True,
            "uds_control_daemon": True,
            "same_user_peer_gate": True,
            "canonical_bounded_cbor": True,
            "request_generation_fence": True,
            "backend_generation_fence": True,
            "cancel_generation_fence": True,
            "terminal_receipt_store": True,
            "per_tenant_queue_bound": True,
            "forced_worker_termination_receipt": True,
            "connection_local_protocol_failure_isolation": True,
            "infrastructure_failure_escalation": True,
            "controlled_restart_generation_increment": True,
            "tcp_listener": False,
            "native_model_worker": False,
            "product_caller_wiring": False,
            "remote_inference": False,
            "automatic_model_install": False,
        },
        "authority": {name: False for name in authority_names},
    }
    STATUS.write_text(json.dumps(value, indent=2) + "\n", encoding="utf-8")


def main() -> None:
    patch_lib()
    patch_tests()
    patch_gate()
    write_status()
    print("PASS_HEPTA_INFERENCE_INF1_CONNECTION_ISOLATION_REPAIR_V4")


if __name__ == "__main__":
    main()
