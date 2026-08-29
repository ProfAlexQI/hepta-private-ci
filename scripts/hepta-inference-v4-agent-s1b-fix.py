#!/usr/bin/env python3
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def replace_once(path: Path, old: str, new: str) -> None:
    text = path.read_text(encoding="utf-8")
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"{path}: expected one occurrence, found {count}: {old[:100]!r}")
    path.write_text(text.replace(old, new, 1), encoding="utf-8")


def main() -> None:
    controller = ROOT / "codex-rs/hepta-infer-core/src/controller.rs"
    replace_once(
        controller,
        '''    pub fn expected_next_sequence(
        &self,
        request_id: &RequestId,
        request_generation: u64,
    ) -> Result<u64> {
        self.record(request_id, request_generation)?
            .last_sequence
            .checked_add(1)
            .ok_or(InferError::SequenceOverflow)
    }

''',
        '''    pub fn expected_next_sequence(
        &self,
        request_id: &RequestId,
        request_generation: u64,
    ) -> Result<u64> {
        self.record(request_id, request_generation)?
            .last_sequence
            .checked_add(1)
            .ok_or(InferError::SequenceOverflow)
    }

    pub fn validate_start_transition(
        &self,
        request_id: &RequestId,
        request_generation: u64,
        backend_generation: u64,
    ) -> Result<u64> {
        self.validate_backend_generation(backend_generation)?;
        let record = self.record(request_id, request_generation)?;
        Self::ensure_nonterminal(record)?;
        if record.state != LifecycleState::Queued {
            return Err(InferError::InvalidTransition);
        }
        if self.running_requests >= self.config.max_queue {
            return Err(InferError::RunningFull);
        }
        let tenant_running = self
            .running_per_tenant
            .get(&record.request.identity.tenant_id)
            .copied()
            .unwrap_or_default();
        if tenant_running >= self.config.max_per_tenant {
            return Err(InferError::TenantRunningFull);
        }
        record
            .last_sequence
            .checked_add(1)
            .ok_or(InferError::SequenceOverflow)
    }

''',
    )

    private = ROOT / "codex-rs/hepta-inferd/src/private.rs"
    replace_once(
        private,
        '''        if controller.backend_generation() != self.backend_generation {
            return Err(InferError::StaleBackendGeneration);
        }
''',
        '''        if controller.backend_generation() != self.backend_generation {
            return Err(InferError::StaleBackendGeneration);
        }
        if request.deadline_unix_ms <= now_unix_ms {
            return Err(InferError::DeadlineExpired);
        }
''',
    )
    replace_once(
        private,
        '''        if controller.expected_next_sequence(request_id, request_generation)? != sequence {
            return Err(InferError::StaleSequence);
        }
''',
        '''        if controller.validate_start_transition(
            request_id,
            request_generation,
            backend_generation,
        )? != sequence
        {
            return Err(InferError::StaleSequence);
        }
''',
    )
    replace_once(
        private,
        '''pub(crate) async fn run_private_worker_channel(
    mut attachment: PrivateWorkerAttachment,
    controller: Arc<Mutex<Controller>>,
    receipt_store: Arc<ReceiptStore>,
) -> io::Result<()> {
    loop {
        let bytes = time::timeout(
            attachment.server.config.io_timeout,
            read_private_frame(
                &mut *attachment.stream,
                attachment.server.config.max_frame_bytes,
            ),
''',
        '''pub(crate) async fn run_private_worker_channel(
    mut attachment: PrivateWorkerAttachment,
    controller: Arc<Mutex<Controller>>,
    receipt_store: Arc<ReceiptStore>,
) -> io::Result<()> {
    let io_timeout = attachment.server.config.io_timeout;
    let max_frame_bytes = attachment.server.config.max_frame_bytes;
    loop {
        let bytes = time::timeout(
            io_timeout,
            read_private_frame(&mut *attachment.stream, max_frame_bytes),
''',
    )
    replace_once(
        private,
        '''        time::timeout(
            attachment.server.config.io_timeout,
            write_private_frame(
                &mut *attachment.stream,
                &bytes,
                attachment.server.config.max_frame_bytes,
            ),
''',
        '''        time::timeout(
            io_timeout,
            write_private_frame(&mut *attachment.stream, &bytes, max_frame_bytes),
''',
    )
    replace_once(
        private,
        "            deadline_unix_ms: 10_000,\n",
        "            deadline_unix_ms: u64::MAX,\n",
    )
    replace_once(
        private,
        '''        assert!(matches!(
            controller.lock().await.terminal_receipt(&request_id),
            Err(InferError::UnknownRequest)
        ));
''',
        '''        let terminal = {
            let controller = controller.lock().await;
            controller.terminal_receipt(&request_id).cloned()
        };
        assert!(matches!(terminal, Err(InferError::UnknownRequest)));
''',
    )

    truth = ROOT / "scripts/hepta-inference-v4-source-truth.py"
    replace_once(
        truth,
        '    "codex-rs/hepta-infer-core/src/capability.rs",\n',
        '    "codex-rs/hepta-infer-core/src/capability.rs",\n    "codex-rs/hepta-infer-core/src/private_protocol.rs",\n',
    )
    replace_once(
        truth,
        '    "codex-rs/hepta-inferd/src/lib.rs",\n',
        '    "codex-rs/hepta-inferd/src/lib.rs",\n    "codex-rs/hepta-inferd/src/private.rs",\n',
    )
    replace_once(
        truth,
        '''        "controller": (
            ROOT / "codex-rs/hepta-infer-core/src/controller.rs"
        ).read_text(encoding="utf-8"),
''',
        '''        "private_protocol": (
            ROOT / "codex-rs/hepta-infer-core/src/private_protocol.rs"
        ).read_text(encoding="utf-8"),
        "controller": (
            ROOT / "codex-rs/hepta-infer-core/src/controller.rs"
        ).read_text(encoding="utf-8"),
''',
    )
    replace_once(
        truth,
        '''        "daemon": (
            ROOT / "codex-rs/hepta-inferd/src/lib.rs"
        ).read_text(encoding="utf-8"),
''',
        '''        "daemon": (
            ROOT / "codex-rs/hepta-inferd/src/lib.rs"
        ).read_text(encoding="utf-8"),
        "daemon_private": (
            ROOT / "codex-rs/hepta-inferd/src/private.rs"
        ).read_text(encoding="utf-8"),
''',
    )
    replace_once(
        truth,
        '''        "controller": [
            "inflight_requests",
''',
        '''        "private_protocol": [
            "WorkerBootstrapEnvelope",
            "WorkerToDaemon",
            "DaemonToWorker",
            "<redacted>",
        ],
        "controller": [
            "inflight_requests",
''',
    )
    replace_once(
        truth,
        '''        "daemon": [
            "Semaphore",
''',
        '''        "daemon": [
            "Semaphore",
            "serve_with_shutdown_and_private_worker",
''',
    )
    replace_once(
        truth,
        '''            "sync_directory",
        ],
''',
        '''            "sync_directory",
        ],
        "daemon_private": [
            "PrivateWorkerServer",
            "RequestGrantLedger",
            "run_private_worker_channel",
            "validate_start_transition",
        ],
''',
    )


if __name__ == "__main__":
    main()
