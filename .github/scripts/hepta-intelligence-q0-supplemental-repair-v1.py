#!/usr/bin/env python3
"""Apply the bounded post-reconstruction Q0 source repairs.

Every edit is exact-count and fail-closed.  This script runs only inside the
Q0 qualification worktree; it never writes the branch or grants runtime
authority.
"""

from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(".")


def load(path: str) -> tuple[Path, str]:
    file_path = ROOT / path
    return file_path, file_path.read_text(encoding="utf-8")


def save(path: Path, text: str) -> None:
    path.write_text(text, encoding="utf-8")


def replace_exact(path: str, old: str, new: str, *, expected: int = 1) -> None:
    file_path, text = load(path)
    count = text.count(old)
    if count != expected:
        raise AssertionError(
            f"{path}: expected {expected} copies, found {count}: {old[:120]!r}"
        )
    save(file_path, text.replace(old, new, expected))


def insert_expect_before_function(path: str, name: str, lint: str, reason: str) -> None:
    file_path, text = load(path)
    pattern = re.compile(
        rf"(?m)^(?P<indent>[ \t]*)(?P<signature>"
        rf"(?:(?:pub(?:\([^\)]*\))?)[ \t]+)?"
        rf"(?:async[ \t]+)?fn[ \t]+{re.escape(name)}"
        rf"(?:<[^\n]*?>)?[ \t]*\()"
    )
    matches = list(pattern.finditer(text))
    if len(matches) != 1:
        raise AssertionError(f"{path}: expected one function {name}, found {len(matches)}")
    match = matches[0]
    indent = match.group("indent")
    attribute = (
        f"{indent}#[expect(\n"
        f"{indent}    clippy::{lint},\n"
        f"{indent}    reason = \"{reason}\"\n"
        f"{indent})]\n"
    )
    save(file_path, text[: match.start()] + attribute + text[match.start() :])


# ARM64 can spend more than one wall-clock second compiling before the first
# append.  Keep the test's post-TTL assertion, but wait against the persisted
# expiry rather than racing a one-second lease and a fixed sleep.
replace_exact(
    "codex-rs/hepta-memory/src/h7_trajectory_store_tests.rs",
    "    let (temp, store, lease, executor, binding) = prepared_with_ttl(1).await;\n"
    "    let trajectory_id = \"trajectory:h7-trajectory-expiring\";",
    "    let (temp, store, lease, executor, binding) = prepared_with_ttl(10).await;\n"
    "    let trajectory_id = \"trajectory:h7-trajectory-expiring\";",
)
replace_exact(
    "codex-rs/hepta-memory/src/h7_trajectory_store_tests.rs",
    "    drop(lease);\n"
    "    drop(store);\n"
    "    tokio::time::sleep(Duration::from_secs(2)).await;\n\n"
    "    let owner = AgentId::parse(\"00000000-0000-4000-8000-000000000972\")",
    "    drop(lease);\n"
    "    drop(store);\n"
    "    let now = SystemTime::now()\n"
    "        .duration_since(UNIX_EPOCH)\n"
    "        .expect(\"clock after epoch\")\n"
    "        .as_secs();\n"
    "    if now <= expiry {\n"
    "        tokio::time::sleep(Duration::from_secs(expiry - now + 1)).await;\n"
    "    }\n\n"
    "    let owner = AgentId::parse(\"00000000-0000-4000-8000-000000000972\")",
)

# The queue worker and explicit start share one bounded recovery window.  A
# terminalizing session is transient while the durable row remains pending;
# all other errors continue to fail closed.
replace_exact(
    "codex-rs/hepta-agentd/tests/two_agent_turn_recovery.rs",
    "                        || error\n"
    "                            .to_string()\n"
    "                            .contains(\"malformed rollout turn boundary\") =>",
    "                        || error\n"
    "                            .to_string()\n"
    "                            .contains(\"malformed rollout turn boundary\")\n"
    "                        || error.to_string().contains(\n"
    "                            \"turn start is fenced while the session is terminalizing or shutting down\",\n"
    "                        ) =>",
)

# Frozen witness constructors intentionally retain explicit persisted fields.
insert_expect_before_function(
    "codex-rs/ext/hepta-memory/src/extension.rs",
    "install_with_turn_writer",
    "too_many_arguments",
    "the qualification installer keeps each independent authority gate explicit",
)
replace_exact(
    "codex-rs/ext/hepta-memory/src/extension.rs",
    "    if qualification_writer_profile {\n"
    "        if let Some(host) = qualification_turn_writer {\n"
    "            builder.turn_lifecycle_contributor(Arc::new(\n"
    "                QualificationTurnLifecycleContributor::with_host(host),\n"
    "            ));\n"
    "        }\n"
    "    }",
    "    if qualification_writer_profile && let Some(host) = qualification_turn_writer {\n"
    "        builder.turn_lifecycle_contributor(Arc::new(\n"
    "            QualificationTurnLifecycleContributor::with_host(host),\n"
    "        ));\n"
    "    }",
)
replace_exact(
    "codex-rs/ext/hepta-memory/src/cognitive/grounding_v3.rs",
    ".try_fold(0usize, |total, count| total.checked_add(count))",
    ".try_fold(0usize, usize::checked_add)",
)
replace_exact(
    "codex-rs/ext/hepta-memory/src/local_lifecycle.rs",
    "    fn state<'a>(&self, turn_store: &'a ExtensionData) -> Arc<Mutex<TurnLeaseState>> {",
    "    fn state(&self, turn_store: &ExtensionData) -> Arc<Mutex<TurnLeaseState>> {",
)
insert_expect_before_function(
    "codex-rs/ext/hepta-memory/src/local_replay.rs",
    "replay_binding_digest",
    "too_many_arguments",
    "the replay digest commits every persisted fence and negative-authority field explicitly",
)
insert_expect_before_function(
    "codex-rs/ext/hepta-memory/src/local_runtime.rs",
    "binding_digest",
    "too_many_arguments",
    "the runtime digest commits every persisted lease binding field explicitly",
)
replace_exact(
    "codex-rs/ext/hepta-memory/src/local_turn_writer.rs",
    "pub struct QualificationTurnLifecycleContributor {\n    host: Option<QualificationTurnWriterHost>,\n}\n\nimpl Default for QualificationTurnLifecycleContributor {\n    fn default() -> Self {\n        Self { host: None }\n    }\n}",
    "#[derive(Default)]\n"
    "pub struct QualificationTurnLifecycleContributor {\n"
    "    host: Option<QualificationTurnWriterHost>,\n"
    "}",
)
replace_exact(
    "codex-rs/ext/hepta-memory/src/local_turn_writer.rs",
    "    fn state<'a>(&self, turn_store: &'a ExtensionData) -> std::sync::Arc<Mutex<TurnWriterState>> {",
    "    fn state(&self, turn_store: &ExtensionData) -> std::sync::Arc<Mutex<TurnWriterState>> {",
)
# The nested form mirrors the recovery proof: first a trajectory, then its
# terminal tail.  Preserve that shape and document the deliberate boundary.
replace_exact(
    "codex-rs/ext/hepta-memory/src/local_turn_writer.rs",
    "        if let Some(trajectory) = recovery.trajectory {",
    "        #[expect(\n"
    "            clippy::collapsible_if,\n"
    "            reason = \"the recovery proof keeps trajectory and terminal-tail checks distinct\"\n"
    "        )]\n"
    "        if let Some(trajectory) = recovery.trajectory {",
    expected=2,
)
insert_expect_before_function(
    "codex-rs/ext/hepta-memory/src/local_witness.rs",
    "with_policy",
    "too_many_arguments",
    "the host witness input keeps every lease and checkpoint binding explicit",
)

# Move compile-time negative-authority assertions into const blocks.
replace_exact(
    "codex-rs/ext/hepta-memory/src/extension_tests.rs",
    "    assert!(!crate::LOCAL_REHYDRATION_REPLAY_LIFECYCLE_REGISTERED);",
    "    const {\n"
    "        assert!(!crate::LOCAL_REHYDRATION_REPLAY_LIFECYCLE_REGISTERED);\n"
    "    }",
)
replace_exact(
    "codex-rs/ext/hepta-memory/src/cognitive/grounding_v3.rs",
    "        assert!(!GROUNDED_TOOL_V3_REGISTERED);\n"
    "        assert!(!GROUNDED_TOOL_V3_PRODUCTION_AUTHORITY);",
    "        const {\n"
    "            assert!(!GROUNDED_TOOL_V3_REGISTERED);\n"
    "            assert!(!GROUNDED_TOOL_V3_PRODUCTION_AUTHORITY);\n"
    "        }",
)
replace_exact(
    "codex-rs/ext/hepta-memory/src/local_lifecycle.rs",
    "        let guard = restarted_state\n"
    "            .lock()\n"
    "            .unwrap_or_else(PoisonError::into_inner);\n"
    "        assert!(guard.active.is_none());\n"
    "        assert!(guard.terminal_started);\n"
    "        drop(guard);\n"
    "        assert_eq!(\n"
    "            active.lease.snapshot_counts().await.expect(\"counts\"),",
    "        {\n"
    "            let guard = restarted_state\n"
    "                .lock()\n"
    "                .unwrap_or_else(PoisonError::into_inner);\n"
    "            assert!(guard.active.is_none());\n"
    "            assert!(guard.terminal_started);\n"
    "        }\n"
    "        assert_eq!(\n"
    "            active.lease.snapshot_counts().await.expect(\"counts\"),",
)
replace_exact(
    "codex-rs/ext/hepta-memory/src/local_replay.rs",
    "        assert!(!LOCAL_REHYDRATION_REPLAY_LIFECYCLE_REGISTERED);",
    "        const {\n"
    "            assert!(!LOCAL_REHYDRATION_REPLAY_LIFECYCLE_REGISTERED);\n"
    "        }",
)
replace_exact(
    "codex-rs/ext/hepta-memory/src/local_replay.rs",
    "        let mut replay_tampered = replay.clone();",
    "        let mut replay_tampered = replay;",
)
replace_exact(
    "codex-rs/ext/hepta-memory/src/local_turn_writer.rs",
    "            !writer_input\n"
    "                .lease\n"
    "                .admit(\n"
    "                    OCCURRENCE,\n"
    "                    TURN_START_TOPIC,\n"
    "                    r#\"{\\\"candidate_id\\\":\\\"different\\\"}\"#,\n"
    "                )\n"
    "                .await\n"
    "                .is_ok(),",
    "            writer_input\n"
    "                .lease\n"
    "                .admit(\n"
    "                    OCCURRENCE,\n"
    "                    TURN_START_TOPIC,\n"
    "                    r#\"{\\\"candidate_id\\\":\\\"different\\\"}\"#,\n"
    "                )\n"
    "                .await\n"
    "                .is_err(),",
)
replace_exact(
    "codex-rs/ext/hepta-memory/src/local_turn_writer.rs",
    "        assert!(!QUALIFICATION_TURN_WRITER_EXTERNAL_EFFECTS);\n"
    "        assert!(!QUALIFICATION_TURN_WRITER_KG_WRITE_AUTHORITY);\n"
    "        assert!(!QUALIFICATION_TURN_WRITER_PRODUCTION_CALLER);",
    "        const {\n"
    "            assert!(!QUALIFICATION_TURN_WRITER_EXTERNAL_EFFECTS);\n"
    "            assert!(!QUALIFICATION_TURN_WRITER_KG_WRITE_AUTHORITY);\n"
    "            assert!(!QUALIFICATION_TURN_WRITER_PRODUCTION_CALLER);\n"
    "        }",
)

# Agent protocol and thread-history Clippy closures are fail-closed rather
# than panic-based; the qualification path never converts absence into power.
replace_exact(
    "codex-rs/hepta-agent-protocol/src/lib.rs",
    "impl HostTurnAuthorityBinding {\n    pub fn new(",
    "impl HostTurnAuthorityBinding {\n"
    "    #[expect(\n"
    "        clippy::too_many_arguments,\n"
    "        reason = \"the transport commits every persisted authority-binding field explicitly\"\n"
    "    )]\n"
    "    pub fn new(",
)
replace_exact(
    "codex-rs/app-server-protocol/src/protocol/thread_history.rs",
    "                let (fingerprint, replay) = binding.expect(\"binding checked above\");\n"
    "                state.request_fingerprint_sha256 = Some(fingerprint);\n"
    "                state.replay = Some(replay);",
    "                let Some((fingerprint, replay)) = binding else {\n"
    "                    return;\n"
    "                };\n"
    "                state.request_fingerprint_sha256 = Some(fingerprint);\n"
    "                state.replay = Some(replay);",
    expected=2,
)
replace_exact(
    "codex-rs/app-server-protocol/src/protocol/thread_history.rs",
    "            let turn = self\n"
    "                .turns\n"
    "                .pop()\n"
    "                .expect(\"logical tail must exist after strict last-turn check\");",
    "            let Some(turn) = self.turns.pop() else {\n"
    "                return;\n"
    "            };",
)

print("PASS_Q0_SUPPLEMENTAL_REPAIR_V1")
