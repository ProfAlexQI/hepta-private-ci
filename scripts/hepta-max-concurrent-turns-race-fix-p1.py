#!/usr/bin/env python3
"""Close Started/terminal and simultaneous turn-admission races."""

from __future__ import annotations

import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
THREAD_STATUS = ROOT / "codex-rs/app-server/src/thread_status.rs"
TURN = ROOT / "codex-rs/app-server/src/request_processors/turn_processor.rs"


def fail(message: str) -> None:
    raise SystemExit(f"FAIL_HEPTA_MAX_CONCURRENT_TURNS_RACE_FIX_P1: {message}")


def replace_once(text: str, old: str, new: str, label: str) -> str:
    count = text.count(old)
    if count != 1:
        fail(f"{label}: expected one marker, found {count}")
    return text.replace(old, new, 1)


def patch_thread_status() -> None:
    source = THREAD_STATUS.read_text(encoding="utf-8")
    if "pending_turn_permit_by_thread_id" in source:
        return
    old_methods = """    pub(crate) async fn try_reserve_new_turn(
        &self,
        thread_id: &str,
        core_reports_running: bool,
    ) -> Result<Option<OwnedSemaphorePermit>, usize> {
        let Some(limit) = self.max_concurrent_turns else {
            return Ok(None);
        };
        let runtime_reports_running = self
            .state
            .lock()
            .await
            .runtime_by_thread_id
            .get(thread_id)
            .is_some_and(|runtime| runtime.running);
        if core_reports_running || runtime_reports_running {
            return Ok(None);
        }
        let Some(capacity) = self.turn_capacity.as_ref() else {
            return Err(limit.get());
        };
        capacity
            .clone()
            .try_acquire_owned()
            .map(Some)
            .map_err(|_| limit.get())
    }

    pub(crate) async fn commit_turn_permit(
        &self,
        thread_id: &str,
        permit: OwnedSemaphorePermit,
    ) {
        let mut state = self.state.lock().await;
        let running = state
            .runtime_by_thread_id
            .get(thread_id)
            .is_some_and(|runtime| runtime.running);
        if running {
            state
                .turn_permit_by_thread_id
                .insert(thread_id.to_string(), permit);
        }
    }
"""
    new_methods = """    pub(crate) async fn try_reserve_new_turn(
        &self,
        thread_id: &str,
        core_reports_running: bool,
    ) -> Result<bool, usize> {
        let Some(limit) = self.max_concurrent_turns else {
            return Ok(false);
        };
        let Some(capacity) = self.turn_capacity.as_ref().cloned() else {
            return Err(limit.get());
        };
        let mut state = self.state.lock().await;
        let runtime_reports_running = state
            .runtime_by_thread_id
            .get(thread_id)
            .is_some_and(|runtime| runtime.running);
        if core_reports_running
            || runtime_reports_running
            || state.active_turn_permit_by_thread_id.contains_key(thread_id)
        {
            return Ok(false);
        }
        if state.pending_turn_permit_by_thread_id.contains_key(thread_id) {
            return Err(limit.get());
        }
        let permit = capacity
            .try_acquire_owned()
            .map_err(|_| limit.get())?;
        state
            .pending_turn_permit_by_thread_id
            .insert(thread_id.to_string(), permit);
        Ok(true)
    }

    pub(crate) async fn commit_turn_reservation(&self, thread_id: &str) {
        let mut state = self.state.lock().await;
        if let Some(permit) = state.pending_turn_permit_by_thread_id.remove(thread_id) {
            state
                .active_turn_permit_by_thread_id
                .insert(thread_id.to_string(), permit);
        }
    }

    pub(crate) async fn cancel_turn_reservation(&self, thread_id: &str) {
        self.state
            .lock()
            .await
            .pending_turn_permit_by_thread_id
            .remove(thread_id);
    }
"""
    source = replace_once(source, old_methods, new_methods, "reservation methods")
    source = replace_once(
        source,
        "    turn_permit_by_thread_id: HashMap<String, OwnedSemaphorePermit>,\n",
        "    pending_turn_permit_by_thread_id: HashMap<String, OwnedSemaphorePermit>,\n"
        "    active_turn_permit_by_thread_id: HashMap<String, OwnedSemaphorePermit>,\n",
        "reservation maps",
    )
    source = replace_once(
        source,
        "        self.turn_permit_by_thread_id.remove(thread_id);\n"
        "        self.update_status_watcher(thread_id, &ThreadStatus::NotLoaded);\n",
        "        self.pending_turn_permit_by_thread_id.remove(thread_id);\n"
        "        self.active_turn_permit_by_thread_id.remove(thread_id);\n"
        "        self.update_status_watcher(thread_id, &ThreadStatus::NotLoaded);\n",
        "remove release",
    )
    source = replace_once(
        source,
        "        if !running {\n"
        "            self.turn_permit_by_thread_id.remove(thread_id);\n"
        "        }\n",
        "        if !running {\n"
        "            self.pending_turn_permit_by_thread_id.remove(thread_id);\n"
        "            self.active_turn_permit_by_thread_id.remove(thread_id);\n"
        "        }\n",
        "terminal release",
    )
    old_tests = """    #[tokio::test]
    async fn concurrent_turn_capacity_is_shared_and_terminal_events_release_it() {
        let manager = ThreadWatchManager::new_with_options(None, Some(NonZeroUsize::MIN));
        let first = match manager
            .try_reserve_new_turn(INTERACTIVE_THREAD_ID, false)
            .await
        {
            Ok(Some(permit)) => permit,
            Ok(None) => panic!("new turn must reserve the configured capacity"),
            Err(limit) => panic!("first reservation unexpectedly hit limit {limit}"),
        };
        manager.note_turn_started(INTERACTIVE_THREAD_ID).await;
        manager
            .commit_turn_permit(INTERACTIVE_THREAD_ID, first)
            .await;

        assert!(matches!(
            manager
                .try_reserve_new_turn(NON_INTERACTIVE_THREAD_ID, false)
                .await,
            Err(1)
        ));

        manager
            .note_turn_completed(INTERACTIVE_THREAD_ID, false)
            .await;
        assert!(matches!(
            manager
                .try_reserve_new_turn(NON_INTERACTIVE_THREAD_ID, false)
                .await,
            Ok(Some(_))
        ));
    }

    #[tokio::test]
    async fn steering_an_existing_turn_does_not_consume_another_slot() {
        let manager = ThreadWatchManager::new_with_options(None, Some(NonZeroUsize::MIN));
        manager.note_turn_started(INTERACTIVE_THREAD_ID).await;
        assert!(matches!(
            manager
                .try_reserve_new_turn(INTERACTIVE_THREAD_ID, true)
                .await,
            Ok(None)
        ));
    }
"""
    new_tests = """    #[tokio::test]
    async fn concurrent_turn_capacity_is_shared_and_terminal_events_release_it() {
        let manager = ThreadWatchManager::new_with_options(None, Some(NonZeroUsize::MIN));
        assert_eq!(
            manager
                .try_reserve_new_turn(INTERACTIVE_THREAD_ID, false)
                .await,
            Ok(true)
        );
        manager.note_turn_started(INTERACTIVE_THREAD_ID).await;
        manager
            .commit_turn_reservation(INTERACTIVE_THREAD_ID)
            .await;

        assert!(matches!(
            manager
                .try_reserve_new_turn(NON_INTERACTIVE_THREAD_ID, false)
                .await,
            Err(1)
        ));

        manager
            .note_turn_completed(INTERACTIVE_THREAD_ID, false)
            .await;
        assert_eq!(
            manager
                .try_reserve_new_turn(NON_INTERACTIVE_THREAD_ID, false)
                .await,
            Ok(true)
        );
    }

    #[tokio::test]
    async fn terminal_before_commit_releases_pending_capacity() {
        let manager = ThreadWatchManager::new_with_options(None, Some(NonZeroUsize::MIN));
        assert_eq!(
            manager
                .try_reserve_new_turn(INTERACTIVE_THREAD_ID, false)
                .await,
            Ok(true)
        );
        manager.note_turn_started(INTERACTIVE_THREAD_ID).await;
        manager
            .note_turn_completed(INTERACTIVE_THREAD_ID, false)
            .await;
        manager
            .commit_turn_reservation(INTERACTIVE_THREAD_ID)
            .await;
        assert_eq!(
            manager
                .try_reserve_new_turn(NON_INTERACTIVE_THREAD_ID, false)
                .await,
            Ok(true)
        );
    }

    #[tokio::test]
    async fn simultaneous_same_thread_admission_cannot_overwrite_a_permit() {
        let manager = ThreadWatchManager::new_with_options(None, NonZeroUsize::new(2));
        assert_eq!(
            manager
                .try_reserve_new_turn(INTERACTIVE_THREAD_ID, false)
                .await,
            Ok(true)
        );
        assert!(matches!(
            manager
                .try_reserve_new_turn(INTERACTIVE_THREAD_ID, false)
                .await,
            Err(2)
        ));
        manager
            .cancel_turn_reservation(INTERACTIVE_THREAD_ID)
            .await;
    }

    #[tokio::test]
    async fn steering_an_existing_turn_does_not_consume_another_slot() {
        let manager = ThreadWatchManager::new_with_options(None, Some(NonZeroUsize::MIN));
        manager.note_turn_started(INTERACTIVE_THREAD_ID).await;
        assert_eq!(
            manager
                .try_reserve_new_turn(INTERACTIVE_THREAD_ID, true)
                .await,
            Ok(false)
        );
    }
"""
    source = replace_once(source, old_tests, new_tests, "reservation tests")
    THREAD_STATUS.write_text(source, encoding="utf-8")


def patch_turn_processor() -> None:
    source = TURN.read_text(encoding="utf-8")
    if "turn_capacity_reserved" in source:
        return
    start = source.find("        let thread_id_text = thread_id.to_string();\n")
    end = source.find("\n\n        // Eligible memory startup", start)
    if start < 0 or end < 0:
        fail("turn reservation block is missing")
    reservation = """        let thread_id_text = thread_id.to_string();
        let core_reports_running = matches!(thread.agent_status().await, AgentStatus::Running);
        let turn_capacity_reserved = match self
            .thread_watch_manager
            .try_reserve_new_turn(&thread_id_text, core_reports_running)
            .await
        {
            Ok(reserved) => reserved,
            Err(limit) => {
                let mut error = invalid_request(format!(
                    "Agent concurrent turn limit of {limit} has been reached"
                ));
                error.data = Some(serde_json::json!({
                    "resource": "max_concurrent_turns",
                    "limit": limit,
                }));
                self.track_error_response(&request_id, &error, /*error_type*/ None);
                return Err(error);
            }
        };"""
    source = source[:start] + reservation + source[end:]

    memory_submit_old = """            let submission = thread
                .start_or_steer_turn_and_capture_memory_policy(turn_input_request)
                .await
                .map_err(|err| {
                    let error = internal_error(format!("failed to submit turn input: {err}"));
                    self.track_error_response(&request_id, &error, /*error_type*/ None);
                    error
                })?;
"""
    memory_submit_new = """            let submission = match thread
                .start_or_steer_turn_and_capture_memory_policy(turn_input_request)
                .await
            {
                Ok(submission) => submission,
                Err(err) => {
                    if turn_capacity_reserved {
                        self.thread_watch_manager
                            .cancel_turn_reservation(&thread_id_text)
                            .await;
                    }
                    let error = internal_error(format!("failed to submit turn input: {err}"));
                    self.track_error_response(&request_id, &error, /*error_type*/ None);
                    return Err(error);
                }
            };
"""
    source = replace_once(source, memory_submit_old, memory_submit_new, "Memory submission error")
    source = source.replace(
        "                    if let Some(permit) = turn_capacity_permit.take() {\n"
        "                        self.thread_watch_manager\n"
        "                            .commit_turn_permit(&thread_id_text, permit)\n"
        "                            .await;\n"
        "                    }\n",
        "                    if turn_capacity_reserved {\n"
        "                        self.thread_watch_manager\n"
        "                            .commit_turn_reservation(&thread_id_text)\n"
        "                            .await;\n"
        "                    }\n",
    )
    source = replace_once(
        source,
        "                codex_core::MemoryTurnInputSubmission::Steered { turn_id } => turn_id,\n",
        "                codex_core::MemoryTurnInputSubmission::Steered { turn_id } => {\n"
        "                    if turn_capacity_reserved {\n"
        "                        self.thread_watch_manager\n"
        "                            .cancel_turn_reservation(&thread_id_text)\n"
        "                            .await;\n"
        "                    }\n"
        "                    turn_id\n"
        "                }\n",
        "Memory steered cancellation",
    )
    source = replace_once(
        source,
        "                codex_core::MemoryTurnInputSubmission::NotSubmitted { reason } => {\n"
        "                    let error = internal_error(format!(\"failed to submit turn input: {reason:?}\"));\n",
        "                codex_core::MemoryTurnInputSubmission::NotSubmitted { reason } => {\n"
        "                    if turn_capacity_reserved {\n"
        "                        self.thread_watch_manager\n"
        "                            .cancel_turn_reservation(&thread_id_text)\n"
        "                            .await;\n"
        "                    }\n"
        "                    let error = internal_error(format!(\"failed to submit turn input: {reason:?}\"));\n",
        "Memory not-submitted cancellation",
    )

    ordinary_submit_old = """            let submission = thread
                .start_or_steer_turn(turn_input_request)
                .await
                .map_err(|err| {
                    let error = internal_error(format!("failed to submit turn input: {err}"));
                    self.track_error_response(&request_id, &error, /*error_type*/ None);
                    error
                })?;
"""
    ordinary_submit_new = """            let submission = match thread.start_or_steer_turn(turn_input_request).await {
                Ok(submission) => submission,
                Err(err) => {
                    if turn_capacity_reserved {
                        self.thread_watch_manager
                            .cancel_turn_reservation(&thread_id_text)
                            .await;
                    }
                    let error = internal_error(format!("failed to submit turn input: {err}"));
                    self.track_error_response(&request_id, &error, /*error_type*/ None);
                    return Err(error);
                }
            };
"""
    source = replace_once(source, ordinary_submit_old, ordinary_submit_new, "ordinary submission error")
    source = replace_once(
        source,
        "                TurnInputSubmission::Steered { turn_id } => turn_id,\n",
        "                TurnInputSubmission::Steered { turn_id } => {\n"
        "                    if turn_capacity_reserved {\n"
        "                        self.thread_watch_manager\n"
        "                            .cancel_turn_reservation(&thread_id_text)\n"
        "                            .await;\n"
        "                    }\n"
        "                    turn_id\n"
        "                }\n",
        "ordinary steered cancellation",
    )
    source = replace_once(
        source,
        "                TurnInputSubmission::NotSubmitted { reason } => {\n"
        "                    let error = internal_error(format!(\"failed to submit turn input: {reason:?}\"));\n",
        "                TurnInputSubmission::NotSubmitted { reason } => {\n"
        "                    if turn_capacity_reserved {\n"
        "                        self.thread_watch_manager\n"
        "                            .cancel_turn_reservation(&thread_id_text)\n"
        "                            .await;\n"
        "                    }\n"
        "                    let error = internal_error(format!(\"failed to submit turn input: {reason:?}\"));\n",
        "ordinary not-submitted cancellation",
    )
    if "turn_capacity_permit" in source or "commit_turn_permit" in source:
        fail("obsolete local permit path remains after race fix")
    TURN.write_text(source, encoding="utf-8")


def main() -> int:
    for path in (THREAD_STATUS, TURN):
        if not path.is_file():
            fail(f"required file is missing: {path.relative_to(ROOT)}")
    patch_thread_status()
    patch_turn_processor()
    print("PASS_HEPTA_MAX_CONCURRENT_TURNS_RACE_FIX_P1_SOURCE")
    return 0


if __name__ == "__main__":
    sys.exit(main())
