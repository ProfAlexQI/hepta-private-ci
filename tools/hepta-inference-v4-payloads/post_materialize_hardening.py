#!/usr/bin/env python3
"""Post-materialization hardening for the unified Hepta inference V4 candidate."""

from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def load(relative: str) -> tuple[Path, str]:
    path = ROOT / relative
    return path, path.read_text(encoding="utf-8")


def save(path: Path, source: str) -> None:
    path.write_text(source, encoding="utf-8")


def replace_once(source: str, old: str, new: str, label: str) -> str:
    count = source.count(old)
    if count != 1:
        raise SystemExit(f"{label}: expected one marker, found {count}: {old!r}")
    return source.replace(old, new, 1)


def insert_before_last_module_brace(source: str, insertion: str, label: str) -> str:
    marker = "\n}"
    index = source.rfind(marker)
    if index < 0:
        raise SystemExit(f"{label}: final module brace not found")
    return source[:index] + insertion.rstrip() + "\n" + source[index:]


def harden_worker_sequences() -> None:
    path, source = load("codex-rs/hepta-infer-worker-host/src/lib.rs")
    if "expected_token_sequence" not in source:
        source = replace_once(
            source,
            "        self.write(&WorkerFrame::Submit {\n",
            "        let expected_token_sequence = initial_sequence\n"
            "            .checked_add(1)\n"
            "            .ok_or(WorkerHostError::Config)?;\n"
            "        self.write(&WorkerFrame::Submit {\n",
            "worker initial sequence",
        )
        source = replace_once(
            source,
            "sequence == initial_sequence.saturating_add(1)",
            "sequence == expected_token_sequence",
            "worker token fence",
        )
    if "expected_complete_sequence" not in source:
        source = replace_once(
            source,
            "        let expected_result_digest = next_token_chain_digest(\n",
            "        let expected_complete_sequence = token_sequence\n"
            "            .checked_add(1)\n"
            "            .ok_or(WorkerHostError::ProtocolFence)?;\n"
            "        let expected_result_digest = next_token_chain_digest(\n",
            "worker completion sequence",
        )
        source = replace_once(
            source,
            "sequence == token_sequence.saturating_add(1)",
            "sequence == expected_complete_sequence",
            "worker complete fence",
        )
    if ".saturating_add(1)" in source:
        raise SystemExit("worker host still contains saturating sequence arithmetic")
    save(path, source)

    test_path, tests = load("codex-rs/hepta-infer-worker-host/tests/process.rs")
    if "initial_sequence_overflow_is_rejected_before_worker_dispatch" not in tests:
        tests += r'''

#[tokio::test]
async fn initial_sequence_overflow_is_rejected_before_worker_dispatch() {
    let mut worker = spawn("success", Duration::from_secs(2)).await;
    assert_eq!(
        worker
            .submit_fixture(
                must(RequestId::parse("request-worker-sequence-overflow")),
                2,
                u64::MAX,
                digest('e'),
                digest('b'),
                digest('c'),
                8,
            )
            .await,
        Err(WorkerHostError::Config)
    );
    must(worker.shutdown().await);
}
'''
    save(test_path, tests)


def harden_provider_evidence_and_http() -> None:
    path, source = load("codex-rs/hepta-inferd/src/provider_host.rs")
    source = replace_once(
        source,
        "        let real_provider_executed =\n"
        "            self.manifest.execution_class == ProviderExecutionClass::RealProcessUnattested;\n",
        "        // Repository source and a caller-selected execution class cannot attest a\n"
        "        // real provider process. Only an independent exact-process artifact may\n"
        "        // elevate that evidence layer.\n"
        "        let real_provider_executed = false;\n",
        "provider evidence truth",
    )
    old_headers = '''        if name.eq_ignore_ascii_case("content-length") {
            content_length = Some(
                value
                    .trim()
                    .parse::<usize>()
                    .map_err(|_| ProviderHostError::HttpShape)?,
            );
        }
        if name.eq_ignore_ascii_case("content-type") {
            content_type = Some(value.trim().to_ascii_lowercase());
        }
'''
    new_headers = '''        if name.eq_ignore_ascii_case("content-length") {
            if content_length.is_some() {
                return Err(ProviderHostError::HttpShape);
            }
            content_length = Some(
                value
                    .trim()
                    .parse::<usize>()
                    .map_err(|_| ProviderHostError::HttpShape)?,
            );
        }
        if name.eq_ignore_ascii_case("content-type") {
            if content_type.is_some() {
                return Err(ProviderHostError::HttpShape);
            }
            content_type = Some(value.trim().to_ascii_lowercase());
        }
'''
    source = replace_once(source, old_headers, new_headers, "provider duplicate headers")
    source = replace_once(
        source,
        '''    if !content_type
        .as_deref()
        .is_some_and(|value| value.starts_with("application/json"))
    {
''',
        '''    if !content_type.as_deref().is_some_and(|value| {
        matches!(value, "application/json" | "application/json; charset=utf-8")
    }) {
''',
        "provider content type",
    )
    if "unattested_execution_class_cannot_self_attest" not in source:
        source = insert_before_last_module_brace(
            source,
            r'''

    #[tokio::test]
    async fn unattested_execution_class_cannot_self_attest() {
        let provider = spawn_fake_provider(Scenario::Success).await;
        let mut candidate = manifest(provider.endpoint);
        candidate.execution_class = ProviderExecutionClass::RealProcessUnattested;
        let host = must(ProviderRuntimeHost::new(candidate));
        let execution = must(host.execute("bounded evidence truth").await);
        assert_eq!(
            execution.receipt.execution_class,
            ProviderExecutionClass::RealProcessUnattested
        );
        assert!(!execution.receipt.real_provider_executed);
    }

    #[test]
    fn duplicate_identity_headers_and_json_prefix_are_rejected() {
        let duplicate_length = b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 2\r\nContent-Length: 2\r\n\r\n{}";
        assert_eq!(
            parse_http_json(duplicate_length, 64),
            Err(ProviderHostError::HttpShape)
        );
        let duplicate_type = b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Type: application/json\r\nContent-Length: 2\r\n\r\n{}";
        assert_eq!(
            parse_http_json(duplicate_type, 64),
            Err(ProviderHostError::HttpShape)
        );
        let json_prefix = b"HTTP/1.1 200 OK\r\nContent-Type: application/jsonp\r\nContent-Length: 2\r\n\r\n{}";
        assert_eq!(
            parse_http_json(json_prefix, 64),
            Err(ProviderHostError::ContentType)
        );
    }
''',
            "provider tests",
        )
    save(path, source)


def harden_receipt_directory_shape() -> None:
    path, source = load("codex-rs/hepta-inferd/src/receipt_retention.rs")
    source = replace_once(
        source,
        '''        if !entry.file_type().await?.is_file() {
            continue;
        }
''',
        '''        if !entry.file_type().await?.is_file() {
            return Err(invalid_data("INF_RECEIPT_RETENTION_NON_REGULAR_ENTRY"));
        }
''',
        "receipt non-regular entry",
    )
    save(path, source)

    test_path, tests = load("codex-rs/hepta-inferd/src/retention_tests.rs")
    if "non_regular_store_entry_fails_closed" not in tests:
        tests = insert_before_last_module_brace(
            tests,
            r'''

#[cfg(unix)]
#[tokio::test]
async fn non_regular_store_entry_fails_closed() {
    use std::os::unix::fs::symlink;

    let root = must(TempDir::new());
    let outside = must(TempDir::new());
    let target = outside.path().join("receipt-target.cbor");
    must(std::fs::write(&target, b"not-a-receipt"));
    must(symlink(&target, root.path().join("receipt-symlink.cbor")));
    let result = receipt_retention::compact_and_recover(
        root.path(),
        ReceiptRetentionPolicy {
            minimum_retention: Duration::from_millis(1),
            compact_on_start: false,
        },
        1,
        &HashSet::new(),
        32,
        1024 * 1024,
    )
    .await;
    let error = result.expect_err("symlink receipt entries must fail closed");
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
}
''',
            "retention tests",
        )
    save(test_path, tests)


def harden_scheduler_overflow() -> None:
    path, source = load("codex-rs/hepta-infer-core/src/scheduler.rs")
    old_selection = '''        let selected = self
            .pending
            .values()
            .filter(|pending| pending.request.deadline_unix_ms > now_unix_ms)
            .filter(|pending| self.can_reserve(&pending.request))
            .map(|pending| {
                let starved = now_tick.saturating_sub(pending.enqueued_tick)
                    >= self.config.max_starvation_ticks;
                let virtual_finish = self.virtual_finish(&pending.request);
                (
                    !starved,
                    pending.request.deadline_unix_ms,
                    virtual_finish,
                    pending.enqueue_order,
                    pending.request.request_id.clone(),
                )
            })
            .min()
            .ok_or(SchedulerError::NoDispatchableRequest)?;
'''
    new_selection = '''        let mut candidates = Vec::new();
        for pending in self
            .pending
            .values()
            .filter(|pending| pending.request.deadline_unix_ms > now_unix_ms)
            .filter(|pending| self.can_reserve(&pending.request))
        {
            let starved = now_tick.saturating_sub(pending.enqueued_tick)
                >= self.config.max_starvation_ticks;
            let virtual_finish = self.virtual_finish(&pending.request)?;
            candidates.push((
                !starved,
                pending.request.deadline_unix_ms,
                virtual_finish,
                pending.enqueue_order,
                pending.request.request_id.clone(),
            ));
        }
        let selected = candidates
            .into_iter()
            .min()
            .ok_or(SchedulerError::NoDispatchableRequest)?;
'''
    source = replace_once(source, old_selection, new_selection, "scheduler selection")
    source = replace_once(
        source,
        "            let expected_finish = self.virtual_finish(&pending.request);\n",
        "            let expected_finish = self.virtual_finish(&pending.request)?;\n",
        "scheduler recovery finish",
    )
    old_finish = '''    fn virtual_finish(&self, request: &SchedulingRequest) -> u128 {
        let current = self
            .tenant_virtual_finish
            .get(&request.tenant_id)
            .copied()
            .unwrap_or_default();
        let service = (u128::from(request.cost_units) * VIRTUAL_FINISH_SCALE)
            .div_ceil(u128::from(request.tenant_weight));
        current.saturating_add(service)
    }
'''
    new_finish = '''    fn virtual_finish(&self, request: &SchedulingRequest) -> SchedulerResult<u128> {
        let current = self
            .tenant_virtual_finish
            .get(&request.tenant_id)
            .copied()
            .unwrap_or_default();
        let service = (u128::from(request.cost_units) * VIRTUAL_FINISH_SCALE)
            .div_ceil(u128::from(request.tenant_weight));
        current
            .checked_add(service)
            .ok_or(SchedulerError::ReservationOverflow)
    }
'''
    source = replace_once(source, old_finish, new_finish, "scheduler virtual finish")
    if "virtual_finish_overflow_fails_closed" not in source:
        source = insert_before_last_module_brace(
            source,
            r'''

    #[test]
    fn virtual_finish_overflow_fails_closed() {
        let mut scheduler = must(DeterministicScheduler::new(config(), 13));
        let candidate = request("request-overflow", "tenant-a", 100, 1);
        scheduler
            .tenant_virtual_finish
            .insert(candidate.tenant_id.clone(), u128::MAX);
        must(scheduler.enqueue(candidate, 1, 0));
        assert_eq!(
            scheduler.reserve_next(2, 1),
            Err(SchedulerError::ReservationOverflow)
        );
    }
''',
            "scheduler tests",
        )
    save(path, source)


def verify_cross_package_truth() -> None:
    required = {
        "codex-rs/hepta-infer-core/src/lib.rs": (
            "mod capability;",
            "mod scheduler;",
            "pub use controller::RunningCancelPhase;",
            "pub use controller::initial_token_chain_digest;",
            "pub use scheduler::DeterministicScheduler;",
        ),
        "codex-rs/hepta-infer-client/src/lib.rs": (
            "mod shadow_bridge;",
            "pub use shadow_bridge::ProductShadowBridge;",
        ),
        "codex-rs/hepta-inferd/src/lib.rs": (
            "mod cancel_supervisor;",
            "mod private_control;",
            "mod provider_host;",
            "mod receipt_retention;",
        ),
        "codex-rs/Cargo.toml": (
            '"hepta-infer-worker-host"',
            'codex-hepta-infer-worker-host = { path = "hepta-infer-worker-host" }',
            'libloading = "0.8.9"',
        ),
    }
    for relative, markers in required.items():
        text = (ROOT / relative).read_text(encoding="utf-8")
        for marker in markers:
            if text.count(marker) != 1:
                raise SystemExit(f"cross-package marker drift: {relative}: {marker!r}")

    provider = (ROOT / "codex-rs/hepta-inferd/src/provider_host.rs").read_text(
        encoding="utf-8"
    )
    if "let real_provider_executed = false;" not in provider:
        raise SystemExit("provider source can still self-attest real execution")
    scheduler = (ROOT / "codex-rs/hepta-infer-core/src/scheduler.rs").read_text(
        encoding="utf-8"
    )
    if "current.saturating_add(service)" in scheduler:
        raise SystemExit("scheduler still saturates virtual-time overflow")


def main() -> None:
    harden_worker_sequences()
    harden_provider_evidence_and_http()
    harden_receipt_directory_shape()
    harden_scheduler_overflow()
    verify_cross_package_truth()
    print("PASS_HEPTA_INFERENCE_V4_POST_MATERIALIZATION_HARDENING")


if __name__ == "__main__":
    main()
