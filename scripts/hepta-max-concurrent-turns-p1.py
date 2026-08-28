#!/usr/bin/env python3
"""Wire the Agent manifest concurrent-turn budget into the real App Server."""

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


def replace_once(text: str, old: str, new: str, label: str) -> str:
    count = text.count(old)
    if count != 1:
        fail(f"{label}: expected one marker {old!r}, found {count}")
    return text.replace(old, new, 1)


def patch_app_lib() -> None:
    source = APP_LIB.read_text(encoding="utf-8")
    if "pub max_concurrent_turns: Option<NonZeroUsize>" in source:
        return
    source = replace_once(
        source,
        "    pub turn_queue_capacity: Option<NonZeroUsize>,\n",
        "    pub turn_queue_capacity: Option<NonZeroUsize>,\n"
        "    /// Optional process-wide cap for concurrently running assistant turns.\n"
        "    ///\n"
        "    /// This is an embedding-owned admission boundary. Existing running\n"
        "    /// turns may still be steered without consuming another slot.\n"
        "    pub max_concurrent_turns: Option<NonZeroUsize>,\n",
        "AppServerRuntimeOptions field",
    )
    source = replace_once(
        source,
        '            .field("turn_queue_capacity", &self.turn_queue_capacity)\n',
        '            .field("turn_queue_capacity", &self.turn_queue_capacity)\n'
        '            .field("max_concurrent_turns", &self.max_concurrent_turns)\n',
        "AppServerRuntimeOptions Debug",
    )
    source = replace_once(
        source,
        "            && self.turn_queue_capacity == other.turn_queue_capacity\n",
        "            && self.turn_queue_capacity == other.turn_queue_capacity\n"
        "            && self.max_concurrent_turns == other.max_concurrent_turns\n",
        "AppServerRuntimeOptions PartialEq",
    )
    source = replace_once(
        source,
        "            turn_queue_capacity: None,\n",
        "            turn_queue_capacity: None,\n"
        "            max_concurrent_turns: None,\n",
        "AppServerRuntimeOptions Default",
    )
    source = replace_once(
        source,
        "            turn_queue_capacity: runtime_options.turn_queue_capacity,\n",
        "            turn_queue_capacity: runtime_options.turn_queue_capacity,\n"
        "            max_concurrent_turns: runtime_options.max_concurrent_turns,\n",
        "MessageProcessorArgs construction",
    )
    APP_LIB.write_text(source, encoding="utf-8")


def patch_message_processor() -> None:
    source = MESSAGE.read_text(encoding="utf-8")
    if "pub(crate) max_concurrent_turns: Option<NonZeroUsize>" in source:
        return
    source = replace_once(
        source,
        "    pub(crate) turn_queue_capacity: Option<NonZeroUsize>,\n",
        "    pub(crate) turn_queue_capacity: Option<NonZeroUsize>,\n"
        "    pub(crate) max_concurrent_turns: Option<NonZeroUsize>,\n",
        "MessageProcessorArgs field",
    )
    source = replace_once(
        source,
        "            turn_queue_capacity,\n            hepta_cognitive_runtime,\n",
        "            turn_queue_capacity,\n"
        "            max_concurrent_turns,\n"
        "            hepta_cognitive_runtime,\n",
        "MessageProcessorArgs destructure",
    )
    source = replace_once(
        source,
        "        let thread_watch_manager =\n"
        "            crate::thread_status::ThreadWatchManager::new_with_outgoing(outgoing.clone());\n",
        "        let thread_watch_manager =\n"
        "            crate::thread_status::ThreadWatchManager::new_with_outgoing_and_turn_limit(\n"
        "                outgoing.clone(),\n"
        "                max_concurrent_turns,\n"
        "            );\n",
        "ThreadWatchManager construction",
    )
    MESSAGE.write_text(source, encoding="utf-8")


def patch_thread_status() -> None:
    source = THREAD_STATUS.read_text(encoding="utf-8")
    if "try_reserve_new_turn" in source:
        return
    source = replace_once(
        source,
        "use std::collections::HashMap;\nuse std::sync::Arc;\n",
        "use std::collections::HashMap;\n"
        "use std::num::NonZeroUsize;\n"
        "use std::sync::Arc;\n",
        "ThreadWatchManager imports",
    )
    source = replace_once(
        source,
        "use tokio::sync::Mutex;\n",
        "use tokio::sync::Mutex;\n"
        "use tokio::sync::OwnedSemaphorePermit;\n"
        "use tokio::sync::Semaphore;\n",
        "ThreadWatchManager semaphore imports",
    )
    source = replace_once(
        source,
        "    running_turn_count_tx: watch::Sender<usize>,\n",
        "    running_turn_count_tx: watch::Sender<usize>,\n"
        "    turn_capacity: Option<Arc<Semaphore>>,\n"
        "    max_concurrent_turns: Option<NonZeroUsize>,\n",
        "ThreadWatchManager fields",
    )
    old_constructors = """    pub(crate) fn new() -> Self {
        let (running_turn_count_tx, _running_turn_count_rx) = watch::channel(0);
        Self {
            state: Arc::new(Mutex::new(ThreadWatchState::default())),
            outgoing: None,
            running_turn_count_tx,
        }
    }

    pub(crate) fn new_with_outgoing(outgoing: Arc<OutgoingMessageSender>) -> Self {
        let (running_turn_count_tx, _running_turn_count_rx) = watch::channel(0);
        Self {
            state: Arc::new(Mutex::new(ThreadWatchState::default())),
            outgoing: Some(outgoing),
            running_turn_count_tx,
        }
    }
"""
    new_constructors = """    pub(crate) fn new() -> Self {
        Self::new_with_options(None, None)
    }

    pub(crate) fn new_with_outgoing(outgoing: Arc<OutgoingMessageSender>) -> Self {
        Self::new_with_options(Some(outgoing), None)
    }

    pub(crate) fn new_with_outgoing_and_turn_limit(
        outgoing: Arc<OutgoingMessageSender>,
        max_concurrent_turns: Option<NonZeroUsize>,
    ) -> Self {
        Self::new_with_options(Some(outgoing), max_concurrent_turns)
    }

    fn new_with_options(
        outgoing: Option<Arc<OutgoingMessageSender>>,
        max_concurrent_turns: Option<NonZeroUsize>,
    ) -> Self {
        let (running_turn_count_tx, _running_turn_count_rx) = watch::channel(0);
        let turn_capacity = max_concurrent_turns
            .map(|limit| Arc::new(Semaphore::new(limit.get())));
        Self {
            state: Arc::new(Mutex::new(ThreadWatchState::default())),
            outgoing,
            running_turn_count_tx,
            turn_capacity,
            max_concurrent_turns,
        }
    }
"""
    source = replace_once(
        source,
        old_constructors,
        new_constructors,
        "ThreadWatchManager constructors",
    )
    source = replace_once(
        source,
        "    pub(crate) fn subscribe_running_turn_count(&self) -> watch::Receiver<usize> {\n"
        "        self.running_turn_count_tx.subscribe()\n"
        "    }\n",
        "    pub(crate) fn subscribe_running_turn_count(&self) -> watch::Receiver<usize> {\n"
        "        self.running_turn_count_tx.subscribe()\n"
        "    }\n\n"
        "    pub(crate) async fn try_reserve_new_turn(\n"
        "        &self,\n"
        "        thread_id: &str,\n"
        "        core_reports_running: bool,\n"
        "    ) -> Result<Option<OwnedSemaphorePermit>, usize> {\n"
        "        let Some(limit) = self.max_concurrent_turns else {\n"
        "            return Ok(None);\n"
        "        };\n"
        "        let runtime_reports_running = self\n"
        "            .state\n"
        "            .lock()\n"
        "            .await\n"
        "            .runtime_by_thread_id\n"
        "            .get(thread_id)\n"
        "            .is_some_and(|runtime| runtime.running);\n"
        "        if core_reports_running || runtime_reports_running {\n"
        "            return Ok(None);\n"
        "        }\n"
        "        let Some(capacity) = self.turn_capacity.as_ref() else {\n"
        "            return Err(limit.get());\n"
        "        };\n"
        "        capacity\n"
        "            .clone()\n"
        "            .try_acquire_owned()\n"
        "            .map(Some)\n"
        "            .map_err(|_| limit.get())\n"
        "    }\n\n"
        "    pub(crate) async fn commit_turn_permit(\n"
        "        &self,\n"
        "        thread_id: &str,\n"
        "        permit: OwnedSemaphorePermit,\n"
        "    ) {\n"
        "        let mut state = self.state.lock().await;\n"
        "        let running = state\n"
        "            .runtime_by_thread_id\n"
        "            .get(thread_id)\n"
        "            .is_some_and(|runtime| runtime.running);\n"
        "        if running {\n"
        "            state\n"
        "                .turn_permit_by_thread_id\n"
        "                .insert(thread_id.to_string(), permit);\n"
        "        }\n"
        "    }\n",
        "ThreadWatchManager reservation methods",
    )
    source = replace_once(
        source,
        "struct ThreadWatchState {\n"
        "    runtime_by_thread_id: HashMap<String, RuntimeFacts>,\n"
        "    status_watcher_by_thread_id: HashMap<String, watch::Sender<ThreadStatus>>,\n"
        "}\n",
        "struct ThreadWatchState {\n"
        "    runtime_by_thread_id: HashMap<String, RuntimeFacts>,\n"
        "    status_watcher_by_thread_id: HashMap<String, watch::Sender<ThreadStatus>>,\n"
        "    turn_permit_by_thread_id: HashMap<String, OwnedSemaphorePermit>,\n"
        "}\n",
        "ThreadWatchState permit map",
    )
    source = replace_once(
        source,
        "        self.runtime_by_thread_id.remove(thread_id);\n"
        "        self.update_status_watcher(thread_id, &ThreadStatus::NotLoaded);\n",
        "        self.runtime_by_thread_id.remove(thread_id);\n"
        "        self.turn_permit_by_thread_id.remove(thread_id);\n"
        "        self.update_status_watcher(thread_id, &ThreadStatus::NotLoaded);\n",
        "ThreadWatchState remove release",
    )
    source = replace_once(
        source,
        "        let runtime = self\n"
        "            .runtime_by_thread_id\n"
        "            .entry(thread_id.to_string())\n"
        "            .or_default();\n"
        "        runtime.is_loaded = true;\n"
        "        mutate(runtime);\n"
        "        self.update_status_watcher_for_thread(thread_id);\n",
        "        let running = {\n"
        "            let runtime = self\n"
        "                .runtime_by_thread_id\n"
        "                .entry(thread_id.to_string())\n"
        "                .or_default();\n"
        "            runtime.is_loaded = true;\n"
        "            mutate(runtime);\n"
        "            runtime.running\n"
        "        };\n"
        "        if !running {\n"
        "            self.turn_permit_by_thread_id.remove(thread_id);\n"
        "        }\n"
        "        self.update_status_watcher_for_thread(thread_id);\n",
        "ThreadWatchState terminal release",
    )
    test_marker = """    #[tokio::test]
    async fn loaded_status_defaults_to_not_loaded_for_untracked_threads() {
"""
    tests = """    #[tokio::test]
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

""" + test_marker
    source = replace_once(source, test_marker, tests, "ThreadWatchManager tests")
    THREAD_STATUS.write_text(source, encoding="utf-8")


def patch_turn_processor() -> None:
    source = TURN.read_text(encoding="utf-8")
    if "try_reserve_new_turn" in source:
        return
    marker = """        let turn_input_request = TurnInputRequest::new(TurnInput::UserInput {
            content: mapped_items,
            client_id: client_user_message_id,
        })
        .with_thread_settings(thread_settings)
        .on_start(TurnStartOptions {
            final_output_json_schema: params.output_schema,
            ..Default::default()
        })
        .with_additional_context(additional_context)
        .with_responses_metadata(params.responsesapi_client_metadata)
        .with_trace(self.request_trace_context(&request_id).await);

        // Eligible memory startup uses Core's exact admission result. A steered,
"""
    replacement = """        let turn_input_request = TurnInputRequest::new(TurnInput::UserInput {
            content: mapped_items,
            client_id: client_user_message_id,
        })
        .with_thread_settings(thread_settings)
        .on_start(TurnStartOptions {
            final_output_json_schema: params.output_schema,
            ..Default::default()
        })
        .with_additional_context(additional_context)
        .with_responses_metadata(params.responsesapi_client_metadata)
        .with_trace(self.request_trace_context(&request_id).await);

        let thread_id_text = thread_id.to_string();
        let core_reports_running = matches!(thread.agent_status().await, AgentStatus::Running);
        let mut turn_capacity_permit = match self
            .thread_watch_manager
            .try_reserve_new_turn(&thread_id_text, core_reports_running)
            .await
        {
            Ok(permit) => permit,
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
        };

        // Eligible memory startup uses Core's exact admission result. A steered,
"""
    source = replace_once(source, marker, replacement, "turn capacity reservation")
    source = replace_once(
        source,
        "                } => {\n"
        "                    let config = thread.config().await;\n",
        "                } => {\n"
        "                    if let Some(permit) = turn_capacity_permit.take() {\n"
        "                        self.thread_watch_manager\n"
        "                            .commit_turn_permit(&thread_id_text, permit)\n"
        "                            .await;\n"
        "                    }\n"
        "                    let config = thread.config().await;\n",
        "memory-started permit commit",
    )
    source = replace_once(
        source,
        "            match submission {\n"
        "                TurnInputSubmission::Started { turn_id }\n"
        "                | TurnInputSubmission::Steered { turn_id } => turn_id,\n",
        "            match submission {\n"
        "                TurnInputSubmission::Started { turn_id } => {\n"
        "                    if let Some(permit) = turn_capacity_permit.take() {\n"
        "                        self.thread_watch_manager\n"
        "                            .commit_turn_permit(&thread_id_text, permit)\n"
        "                            .await;\n"
        "                    }\n"
        "                    turn_id\n"
        "                }\n"
        "                TurnInputSubmission::Steered { turn_id } => turn_id,\n",
        "ordinary-started permit commit",
    )
    TURN.write_text(source, encoding="utf-8")


def patch_agentd() -> None:
    source = AGENTD_APP.read_text(encoding="utf-8")
    if "max_concurrent_turns: Some(max_concurrent_turns)" in source:
        return
    source = replace_once(
        source,
        "    let turn_queue_capacity = NonZeroUsize::new(turn_queue_capacity).ok_or_else(|| {\n"
        "        std::io::Error::other(\"agent manifest contains a zero turn queue capacity\")\n"
        "    })?;\n",
        "    let turn_queue_capacity = NonZeroUsize::new(turn_queue_capacity).ok_or_else(|| {\n"
        "        std::io::Error::other(\"agent manifest contains a zero turn queue capacity\")\n"
        "    })?;\n"
        "    let max_concurrent_turns = usize::try_from(identity.resources.max_concurrent_turns)\n"
        "        .map_err(|_| {\n"
        "            std::io::Error::other(\"max concurrent turns does not fit this platform\")\n"
        "        })?;\n"
        "    let max_concurrent_turns = NonZeroUsize::new(max_concurrent_turns).ok_or_else(|| {\n"
        "        std::io::Error::other(\"agent manifest contains zero max concurrent turns\")\n"
        "    })?;\n",
        "Agentd concurrent turn conversion",
    )
    source = replace_once(
        source,
        "        turn_queue_capacity: Some(turn_queue_capacity),\n",
        "        turn_queue_capacity: Some(turn_queue_capacity),\n"
        "        max_concurrent_turns: Some(max_concurrent_turns),\n",
        "Agentd runtime option",
    )
    source = replace_once(
        source,
        "        assert_eq!(\n"
        "            Some(37),\n"
        "            options.turn_queue_capacity.map(std::num::NonZeroUsize::get)\n"
        "        );\n",
        "        assert_eq!(\n"
        "            Some(37),\n"
        "            options.turn_queue_capacity.map(std::num::NonZeroUsize::get)\n"
        "        );\n"
        "        assert_eq!(\n"
        "            Some(identity.resources.max_concurrent_turns as usize),\n"
        "            options\n"
        "                .max_concurrent_turns\n"
        "                .map(std::num::NonZeroUsize::get)\n"
        "        );\n",
        "Agentd runtime option test",
    )
    AGENTD_APP.write_text(source, encoding="utf-8")


def main() -> int:
    for path in (APP_LIB, MESSAGE, TURN, THREAD_STATUS, AGENTD_APP):
        if not path.is_file():
            fail(f"required file is missing: {path.relative_to(ROOT)}")
    patch_app_lib()
    patch_message_processor()
    patch_thread_status()
    patch_turn_processor()
    patch_agentd()
    print("PASS_HEPTA_MAX_CONCURRENT_TURNS_P1_SOURCE")
    return 0


if __name__ == "__main__":
    sys.exit(main())
