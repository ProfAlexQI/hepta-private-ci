#![allow(warnings, clippy::all)]

use super::*;
use crate::list::parse_cursor;
use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Timelike;
use chrono::Utc;
use codex_protocol::ThreadId;
use codex_protocol::protocol::RolloutItem;
use codex_protocol::protocol::RolloutLine;
use codex_protocol::protocol::SessionMeta;
use codex_protocol::protocol::SessionMetaLine;
use codex_protocol::protocol::ThreadHistoryMode;
use pretty_assertions::assert_eq;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tempfile::TempDir;

#[test]
fn cursor_to_anchor_normalizes_timestamp_format() {
    let ts_str = "2026-01-27T12-34-56";
    let cursor = parse_cursor(ts_str).expect("cursor should parse");
    let anchor = cursor_to_anchor(Some(&cursor)).expect("anchor should parse");

    let naive =
        NaiveDateTime::parse_from_str(ts_str, "%Y-%m-%dT%H-%M-%S").expect("ts should parse");
    let expected_ts = DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc)
        .with_nanosecond(0)
        .expect("nanosecond");

    assert_eq!(anchor.ts, expected_ts);
}

#[tokio::test]
async fn try_init_waits_for_concurrent_startup_backfill() -> anyhow::Result<()> {
    let home = TempDir::new().expect("temp dir");
    let runtime =
        codex_state::StateRuntime::init(home.path().to_path_buf(), "test-provider".to_string())
            .await?;
    let claimed = runtime.try_claim_backfill(/*lease_seconds*/ 60).await?;
    assert!(claimed);
    let runtime_for_completion = runtime.clone();
    let complete_backfill = tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(25)).await;
        runtime_for_completion
            .mark_backfill_complete(/*last_watermark*/ None)
            .await
    });

    let initialized = try_init_with_roots_and_backfill_lease(
        home.path().to_path_buf(),
        home.path().to_path_buf(),
        "test-provider".to_string(),
        /*backfill_lease_seconds*/ 60,
    )
    .await?;
    complete_backfill.await??;
    assert_eq!(
        initialized.get_backfill_state().await?.status,
        codex_state::BackfillStatus::Complete
    );

    Ok(())
}

#[tokio::test]
async fn try_init_times_out_waiting_for_stuck_startup_backfill() -> anyhow::Result<()> {
    let home = TempDir::new().expect("temp dir");
    let runtime =
        codex_state::StateRuntime::init(home.path().to_path_buf(), "test-provider".to_string())
            .await?;
    let claimed = runtime.try_claim_backfill(/*lease_seconds*/ 60).await?;
    assert!(claimed);

    let result = try_init_with_roots_and_backfill_lease(
        home.path().to_path_buf(),
        home.path().to_path_buf(),
        "test-provider".to_string(),
        /*backfill_lease_seconds*/ 60,
    )
    .await;
    let err = match result {
        Ok(_) => panic!("state db init should not wait forever for incomplete backfill"),
        Err(err) => err,
    };
    assert!(
        err.to_string()
            .contains("timed out waiting for state db backfill"),
        "unexpected error: {err}"
    );

    Ok(())
}

#[tokio::test]
async fn reconcile_rollout_respects_memory_mode_ownership_matrix() -> anyhow::Result<()> {
    let home = TempDir::new().expect("temp dir");
    let runtime =
        codex_state::StateRuntime::init(home.path().to_path_buf(), "test-provider".to_string())
            .await?;
    let cases = [
        (
            "paginated explicit enabled",
            ThreadHistoryMode::Paginated,
            Some("enabled"),
            "disabled",
        ),
        (
            "paginated missing mode",
            ThreadHistoryMode::Paginated,
            None,
            "disabled",
        ),
        (
            "legacy missing mode",
            ThreadHistoryMode::Legacy,
            None,
            "enabled",
        ),
    ];

    for (label, history_mode, rollout_mode, expected_after_reconcile) in cases {
        let thread_id = ThreadId::new();
        let rollout_path =
            write_rollout_with_memory_mode(home.path(), thread_id, history_mode, rollout_mode)?;

        reconcile_test_rollout(runtime.as_ref(), rollout_path.as_path()).await;
        assert_eq!(
            runtime.get_thread_memory_mode(thread_id).await?.as_deref(),
            Some("enabled"),
            "{label}: a missing SQLite row uses the rollout/default seed"
        );
        assert!(
            runtime
                .set_thread_memory_mode(thread_id, "disabled")
                .await?
        );

        reconcile_test_rollout(runtime.as_ref(), rollout_path.as_path()).await;
        assert_eq!(
            runtime.get_thread_memory_mode(thread_id).await?.as_deref(),
            Some(expected_after_reconcile),
            "{label}: unexpected ownership after reconciliation"
        );
    }

    Ok(())
}

async fn reconcile_test_rollout(runtime: &codex_state::StateRuntime, rollout_path: &Path) {
    reconcile_rollout(
        Some(runtime),
        rollout_path,
        "test-provider",
        /*builder*/ None,
        &[],
        /*archived_only*/ None,
        /*new_thread_memory_mode*/ None,
    )
    .await;
}

fn write_rollout_with_memory_mode(
    home: &Path,
    thread_id: ThreadId,
    history_mode: ThreadHistoryMode,
    memory_mode: Option<&str>,
) -> anyhow::Result<std::path::PathBuf> {
    let dir = home.join("sessions/2026/07/21");
    std::fs::create_dir_all(dir.as_path())?;
    let rollout_path = dir.join(format!("rollout-2026-07-21T00-00-00-{thread_id}.jsonl"));
    let line = RolloutLine {
        timestamp: "2026-07-21T00:00:00Z".to_string(),
        ordinal: None,
        item: RolloutItem::SessionMeta(SessionMetaLine {
            meta: SessionMeta {
                id: thread_id,
                timestamp: "2026-07-21T00:00:00Z".to_string(),
                cwd: home.to_path_buf(),
                model_provider: Some("test-provider".to_string()),
                memory_mode: memory_mode.map(str::to_string),
                history_mode,
                ..Default::default()
            },
            git: None,
        }),
    };
    let mut file = File::create(&rollout_path)?;
    writeln!(file, "{}", serde_json::to_string(&line)?)?;
    Ok(rollout_path)
}
