#!/usr/bin/env python3
"""Verify process-wide, race-safe concurrent-turn budget wiring."""

from __future__ import annotations

import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
APP_LIB = ROOT / "codex-rs/app-server/src/lib.rs"
MESSAGE = ROOT / "codex-rs/app-server/src/message_processor.rs"
TURN = ROOT / "codex-rs/app-server/src/request_processors/turn_processor.rs"
THREAD_STATUS = ROOT / "codex-rs/app-server/src/thread_status.rs"
AGENTD_APP = ROOT / "codex-rs/hepta-agentd/src/app_runtime.rs"


def fail(message: str) -> None:
    raise SystemExit(f"FAIL_HEPTA_MAX_CONCURRENT_TURNS_P1: {message}")


def require(path: pathlib.Path, markers: tuple[str, ...]) -> str:
    try:
        source = path.read_text(encoding="utf-8")
    except OSError as error:
        fail(f"cannot read {path.relative_to(ROOT)}: {error}")
    for marker in markers:
        if marker not in source:
            fail(f"{path.relative_to(ROOT)} is missing {marker!r}")
    return source


def main() -> int:
    app = require(
        APP_LIB,
        (
            "pub max_concurrent_turns: Option<NonZeroUsize>",
            'field("max_concurrent_turns", &self.max_concurrent_turns)',
            "max_concurrent_turns: None",
            "max_concurrent_turns: runtime_options.max_concurrent_turns",
        ),
    )
    if app.count("pub max_concurrent_turns: Option<NonZeroUsize>") != 1:
        fail("AppServerRuntimeOptions contains duplicate concurrent-turn fields")

    require(
        MESSAGE,
        (
            "pub(crate) max_concurrent_turns: Option<NonZeroUsize>",
            "ThreadWatchManager::new_with_outgoing_and_turn_limit(",
            "max_concurrent_turns,",
        ),
    )
    turn = require(
        TURN,
        (
            "try_reserve_new_turn(&thread_id_text, core_reports_running)",
            "let turn_capacity_reserved = match self",
            "commit_turn_reservation(&thread_id_text)",
            "cancel_turn_reservation(&thread_id_text)",
            '"resource": "max_concurrent_turns"',
            "TurnInputSubmission::Steered { turn_id } => {",
            "MemoryTurnInputSubmission::Steered { turn_id } => {",
        ),
    )
    if turn.count("commit_turn_reservation(&thread_id_text)") != 2:
        fail("both Memory and ordinary Started paths must commit the reservation")
    if turn.count("cancel_turn_reservation(&thread_id_text)") < 6:
        fail("steered, rejected, and error paths do not all cancel reservations")
    for forbidden in ("turn_capacity_permit", "commit_turn_permit"):
        if forbidden in turn:
            fail(f"obsolete local permit path remains: {forbidden}")

    status = require(
        THREAD_STATUS,
        (
            "turn_capacity: Option<Arc<Semaphore>>",
            "max_concurrent_turns: Option<NonZeroUsize>",
            "pub(crate) async fn try_reserve_new_turn(",
            "pub(crate) async fn commit_turn_reservation(",
            "pub(crate) async fn cancel_turn_reservation(",
            "pending_turn_permit_by_thread_id: HashMap<String, OwnedSemaphorePermit>",
            "active_turn_permit_by_thread_id: HashMap<String, OwnedSemaphorePermit>",
            "self.pending_turn_permit_by_thread_id.remove(thread_id);",
            "self.active_turn_permit_by_thread_id.remove(thread_id);",
            "terminal_before_commit_releases_pending_capacity",
            "simultaneous_same_thread_admission_cannot_overwrite_a_permit",
            "steering_an_existing_turn_does_not_consume_another_slot",
        ),
    )
    if "acquire_owned().await" in status:
        fail("turn admission must fail fast rather than wait indefinitely for capacity")
    if ".try_acquire_owned()" not in status:
        fail("turn admission does not use an atomic non-blocking semaphore reservation")
    if "turn_permit_by_thread_id: HashMap" in status:
        fail("obsolete single-map permit ownership remains")

    require(
        AGENTD_APP,
        (
            "identity.resources.max_concurrent_turns",
            "max_concurrent_turns: Some(max_concurrent_turns)",
            "options.max_concurrent_turns",
        ),
    )

    print("PASS_HEPTA_MAX_CONCURRENT_TURNS_P1")
    return 0


if __name__ == "__main__":
    sys.exit(main())
