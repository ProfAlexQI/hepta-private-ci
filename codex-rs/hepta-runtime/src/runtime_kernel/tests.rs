use super::ApprovalRequirement;
use super::DoctorStatus;
use super::MergeOptions;
use super::ProviderTransportKind;
use super::RollbackGroupAttempt;
use super::RollbackGroupAttemptStatus;
use super::RuntimeKernel;
use super::ToolRegistry;
use super::WriteGroupLock;
use super::WriteTargetLock;
use super::WriteTransactionEntry;
use super::WriteTransactionGroup;
use super::current_unix_ms;
use super::extract_explicit_exec_tool_call;
use super::extract_explicit_process_tool_call;
use super::looks_like_assistant_identity_intent;
use super::looks_like_model_identity_intent;
use super::merge_runtime_config_value;
use super::native_pre_model_tool_call;
use super::preview_backup_path_from_ts;
use super::preview_transaction_checkpoint_path;
use super::render_native_tool_result_reply;
use super::should_offer_model_tools_for_turn;
use hepta_core::CorrelationId;
use hepta_core::EventKind;
use hepta_core::ExecutionProfile;
use hepta_core::FilesystemScope;
use hepta_core::IntuitionFeedbackOutcome;
use hepta_core::MemoryRecord;
use hepta_core::MemoryScope;
use hepta_core::MemoryStore;
use hepta_core::MessageRole;
use hepta_core::ModelMessage;
use hepta_core::ModelRef;
use hepta_core::ModelRequest;
use hepta_core::ModelToolSpec;
use hepta_core::SessionId;
use hepta_core::ThinkingLevel;
use hepta_core::ToolCallRequest;
use hepta_core::ToolContext;
use hepta_core::WritePathScope;
use hepta_intelligence::TopicAwareModelFeedbackOutcome;
use serde_json::Value;
use serde_json::json;
use std::fs;
use std::path::PathBuf;

mod tempfile {
    pub(super) fn tempdir() -> std::io::Result<::tempfile::TempDir> {
        let directory = ::tempfile::tempdir()?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            std::fs::set_permissions(directory.path(), std::fs::Permissions::from_mode(0o700))?;
        }
        Ok(directory)
    }
}

fn extract_json_string_field(json_text: &str, field: &str) -> Option<String> {
    serde_json::from_str::<Value>(json_text)
        .ok()?
        .get(field)?
        .as_str()
        .map(|value| value.to_string())
}

fn architecture_foundation_read_intent() -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/decisions/ADR-0001-architecture-foundation.md")
        .canonicalize()
        .expect("architecture foundation ADR should resolve inside the workspace");
    format!("read:{}", path.display())
}

fn test_artifact_path(file_name: impl AsRef<std::path::Path>) -> PathBuf {
    crate::tool_workspace_root_path()
        .join("artifacts")
        .join(file_name)
}

fn provider_test_context(session: &str, correlation: &str) -> ToolContext {
    let attempt_id = uuid::Uuid::new_v4().to_string();
    ToolContext {
        session_id: Some(SessionId(session.into())),
        correlation_id: Some(CorrelationId(correlation.into())),
        idempotency_key: Some(format!(
            "hepta-execution:{attempt_id}:sha256:{}",
            "a".repeat(64)
        )),
        execution_attempt_id: Some(attempt_id),
    }
}

fn selected_context_recall_block(rendered: &str) -> Option<&str> {
    let (_, rest) = rendered.split_once("<selected_context_recall>")?;
    let (block, _) = rest.split_once("</selected_context_recall>")?;
    Some(block)
}

fn write_fake_workspace_backup(logical_path: &str, ts: u64, content: &str) -> PathBuf {
    let backup_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("artifacts/backups/write_file/workspace")
        .join(format!("{}.hepta-bak-{}", logical_path, ts));
    fs::create_dir_all(backup_path.parent().expect("backup parent should exist"))
        .expect("backup parent should be creatable");
    fs::write(&backup_path, content).expect("backup file should be writable");
    backup_path
}

#[path = "tests/part_01.rs"]
mod part_01;
#[path = "tests/part_02.rs"]
mod part_02;
#[path = "tests/part_03.rs"]
mod part_03;
