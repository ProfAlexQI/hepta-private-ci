use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use codex_core_skills::SkillLoadOutcome;
use codex_core_skills::loader::SkillRoot;
use codex_core_skills::loader::SkillRootFileSystem;
use codex_core_skills::loader::load_skills_from_roots;
use codex_exec_server::CopyOptions;
use codex_exec_server::CreateDirectoryOptions;
use codex_exec_server::ExecutorFileSystem;
use codex_exec_server::FileMetadata;
use codex_exec_server::FileSystemResult;
use codex_exec_server::FileSystemSandboxContext;
use codex_exec_server::LOCAL_FS;
use codex_exec_server::ReadDirectoryEntry;
use codex_exec_server::RemoveOptions;
use codex_extension_api::ExtensionData;
use codex_extension_api::FunctionCallError;
use codex_extension_api::ToolCall;
use codex_extension_api::ToolContributor;
use codex_extension_api::ToolExecutor;
use codex_protocol::protocol::SkillScope;
use codex_tools::ToolPayload;
use codex_utils_absolute_path::AbsolutePathBuf;
use codex_utils_plugins::SkillDiscoveryMode;
use serde_json::Value;
use serde_json::json;

use super::ExecutorSkillsExtension;
use super::attach_step_authority;

struct LegacyReadOnlyFileSystem {
    inner: Arc<dyn ExecutorFileSystem>,
    read_calls: Arc<AtomicUsize>,
}

#[async_trait::async_trait]
impl ExecutorFileSystem for LegacyReadOnlyFileSystem {
    async fn read_file(
        &self,
        path: &AbsolutePathBuf,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<Vec<u8>> {
        self.read_calls.fetch_add(1, Ordering::SeqCst);
        self.inner.read_file(path, sandbox).await
    }

    async fn write_file(
        &self,
        path: &AbsolutePathBuf,
        contents: Vec<u8>,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<()> {
        self.inner.write_file(path, contents, sandbox).await
    }

    async fn create_directory(
        &self,
        path: &AbsolutePathBuf,
        options: CreateDirectoryOptions,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<()> {
        self.inner.create_directory(path, options, sandbox).await
    }

    async fn get_metadata(
        &self,
        path: &AbsolutePathBuf,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<FileMetadata> {
        self.inner.get_metadata(path, sandbox).await
    }

    async fn read_directory(
        &self,
        path: &AbsolutePathBuf,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<Vec<ReadDirectoryEntry>> {
        self.inner.read_directory(path, sandbox).await
    }

    async fn remove(
        &self,
        path: &AbsolutePathBuf,
        options: RemoveOptions,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<()> {
        self.inner.remove(path, options, sandbox).await
    }

    async fn copy(
        &self,
        source_path: &AbsolutePathBuf,
        destination_path: &AbsolutePathBuf,
        options: CopyOptions,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<()> {
        self.inner
            .copy(source_path, destination_path, options, sandbox)
            .await
    }
}

fn stores() -> (ExtensionData, ExtensionData, ExtensionData) {
    (
        ExtensionData::new("session"),
        ExtensionData::new("thread"),
        ExtensionData::new("step"),
    )
}

async fn outcome_with_skills(
    count: usize,
    body_bytes: usize,
) -> (tempfile::TempDir, SkillLoadOutcome) {
    let temp = tempfile::tempdir().expect("temporary skill root");
    for index in 0..count {
        let skill_dir = temp.path().join(format!("skill-{index:02}"));
        std::fs::create_dir_all(&skill_dir).expect("create skill directory");
        let body = "x".repeat(body_bytes);
        std::fs::write(
            skill_dir.join("SKILL.md"),
            format!(
                "---\nname: skill-{index:02}\ndescription: executor skill {index}\n---\n\n{body}"
            ),
        )
        .expect("write skill");
        std::fs::write(skill_dir.join("reference.md"), format!("reference-{index}"))
            .expect("write resource");
        std::fs::write(temp.path().join("outside.txt"), "outside").expect("write outside fixture");
    }
    let root = AbsolutePathBuf::from_absolute_path_checked(temp.path()).expect("absolute root");
    let outcome = load_skills_from_roots([SkillRoot {
        path: root,
        scope: SkillScope::User,
        file_system: SkillRootFileSystem::Executor(Arc::clone(&LOCAL_FS)),
        plugin_id: None,
        plugin_namespace: None,
        plugin_root: None,
        discovery_mode: SkillDiscoveryMode::Recursive,
    }])
    .await;
    assert!(outcome.errors.is_empty(), "{:?}", outcome.errors);
    (temp, outcome)
}

fn call(arguments: Value, tool: &dyn ToolExecutor<ToolCall>) -> ToolCall {
    ToolCall {
        call_id: "call-1".into(),
        tool_name: tool.tool_name(),
        payload: ToolPayload::Function {
            arguments: arguments.to_string(),
        },
    }
}

async fn invoke(
    tool: &dyn ToolExecutor<ToolCall>,
    arguments: Value,
) -> Result<Value, FunctionCallError> {
    let call = call(arguments, tool);
    let payload = call.payload.clone();
    tool.handle(call)
        .await
        .map(|output| output.code_mode_result(&payload))
}

fn named<'a>(
    tools: &'a [Arc<dyn ToolExecutor<ToolCall>>],
    name: &str,
) -> &'a dyn ToolExecutor<ToolCall> {
    tools
        .iter()
        .find(|tool| tool.tool_name().name == name)
        .map(AsRef::as_ref)
        .expect("tool should exist")
}

#[tokio::test]
async fn zero_authority_contributes_no_skill_tools() {
    let extension = ExecutorSkillsExtension;
    let (session, thread, step) = stores();
    assert!(
        extension
            .tools_for_step(&session, &thread, &step)
            .is_empty()
    );
    attach_step_authority(&step, &SkillLoadOutcome::default(), "step-1").await;
    assert!(
        extension
            .tools_for_step(&session, &thread, &step)
            .is_empty()
    );
}

#[tokio::test]
async fn host_local_skills_never_gain_executor_authority() {
    let temp = tempfile::tempdir().expect("temporary host skill root");
    let skill_dir = temp.path().join("host-skill");
    std::fs::create_dir_all(&skill_dir).expect("create host skill directory");
    std::fs::write(
        skill_dir.join("SKILL.md"),
        "---\nname: host-skill\ndescription: host-only skill\n---\n",
    )
    .expect("write host skill");
    let outcome = load_skills_from_roots([SkillRoot {
        path: AbsolutePathBuf::from_absolute_path_checked(temp.path()).expect("absolute root"),
        scope: SkillScope::User,
        file_system: SkillRootFileSystem::Local,
        plugin_id: None,
        plugin_namespace: None,
        plugin_root: None,
        discovery_mode: SkillDiscoveryMode::Recursive,
    }])
    .await;
    assert_eq!(outcome.skills.len(), 1);
    assert!(!outcome.is_executor_skill(&outcome.skills[0]));

    let extension = ExecutorSkillsExtension;
    let (session, thread, step) = stores();
    attach_step_authority(&step, &outcome, "step-host").await;
    assert!(
        extension
            .tools_for_step(&session, &thread, &step)
            .is_empty(),
        "host-local skills must not be relabeled as executor authority"
    );
}

#[tokio::test]
async fn list_and_read_require_exact_step_authority_and_package() {
    let (_temp, outcome) = outcome_with_skills(1, 8).await;
    let extension = ExecutorSkillsExtension;
    let (session, thread, step) = stores();
    attach_step_authority(&step, &outcome, "step-1").await;
    let tools = extension.tools_for_step(&session, &thread, &step);
    assert_eq!(tools.len(), 2);

    let listed = invoke(
        named(&tools, "list"),
        json!({"authority": {"kind": "executor"}}),
    )
    .await
    .expect("list");
    let first = &listed["skills"][0];
    assert_eq!(first["authority"]["kind"], "executor");
    assert!(first["package"].as_str().unwrap().starts_with("skill://"));
    assert!(
        first["main_resource_digest"]
            .as_str()
            .is_some_and(|digest| digest.starts_with("sha256:"))
    );

    let read = invoke(
        named(&tools, "read"),
        json!({
            "authority": first["authority"],
            "package": first["package"],
            "resource": first["main_resource"],
        }),
    )
    .await
    .expect("read");
    assert!(
        read["contents"]
            .as_str()
            .unwrap()
            .contains("executor skill")
    );
    assert_eq!(read["content_digest"], first["main_resource_digest"]);

    let denied = invoke(
        named(&tools, "read"),
        json!({
            "authority": {"kind": "executor", "id": "guessed"},
            "package": first["package"],
            "resource": first["main_resource"],
        }),
    )
    .await;
    assert!(matches!(
        denied,
        Err(FunctionCallError::RespondToModel(message))
            if message.contains("not available")
    ));
}

#[tokio::test]
async fn listed_main_resource_is_an_immutable_content_digest_snapshot() {
    let (temp, outcome) = outcome_with_skills(1, 8).await;
    let extension = ExecutorSkillsExtension;
    let (session, thread, step) = stores();
    attach_step_authority(&step, &outcome, "step-1").await;
    let tools = extension.tools_for_step(&session, &thread, &step);
    let listed = invoke(
        named(&tools, "list"),
        json!({"authority": {"kind": "executor"}}),
    )
    .await
    .expect("list");
    let first = &listed["skills"][0];

    std::fs::write(
        temp.path().join("skill-00/SKILL.md"),
        "---\nname: skill-00\ndescription: replaced after discovery\n---\n\nreplacement",
    )
    .expect("replace ordinary skill file after authority attachment");
    let read = invoke(
        named(&tools, "read"),
        json!({
            "authority": first["authority"],
            "package": first["package"],
            "resource": first["main_resource"],
        }),
    )
    .await
    .expect("read frozen main resource");

    assert!(
        read["contents"]
            .as_str()
            .is_some_and(|contents| contents.contains("executor skill 0"))
    );
    assert!(
        read["contents"]
            .as_str()
            .is_some_and(|contents| !contents.contains("replacement"))
    );
    assert_eq!(read["content_digest"], first["main_resource_digest"]);
}

#[tokio::test]
async fn resource_reads_are_confined_beneath_the_exact_package() {
    let (_temp, outcome) = outcome_with_skills(1, 8).await;
    let extension = ExecutorSkillsExtension;
    let (session, thread, step) = stores();
    attach_step_authority(&step, &outcome, "step-1").await;
    let tools = extension.tools_for_step(&session, &thread, &step);
    let listed = invoke(
        named(&tools, "list"),
        json!({"authority": {"kind": "executor"}}),
    )
    .await
    .expect("list");
    let first = &listed["skills"][0];
    let package = first["package"].as_str().unwrap();

    let reference = invoke(
        named(&tools, "read"),
        json!({
            "authority": first["authority"],
            "package": package,
            "resource": format!("{package}/reference.md"),
        }),
    )
    .await
    .expect("read confined reference");
    assert_eq!(reference["contents"], "reference-0");

    for resource in [
        format!("{package}/../outside.txt"),
        format!("{package}/nested/../../outside.txt"),
        "skill://other/resource".to_string(),
    ] {
        let denied = invoke(
            named(&tools, "read"),
            json!({
                "authority": first["authority"],
                "package": package,
                "resource": resource,
            }),
        )
        .await;
        assert!(matches!(denied, Err(FunctionCallError::RespondToModel(_))));
    }
}

#[tokio::test]
async fn rotated_step_revokes_previously_captured_tools() {
    let (_temp, outcome) = outcome_with_skills(1, 8).await;
    let extension = ExecutorSkillsExtension;
    let (session, thread, step) = stores();
    attach_step_authority(&step, &outcome, "step-1").await;
    let stale_tools = extension.tools_for_step(&session, &thread, &step);
    attach_step_authority(&step, &outcome, "step-2").await;

    let result = invoke(
        named(&stale_tools, "list"),
        json!({"authority": {"kind": "executor"}}),
    )
    .await;
    assert!(matches!(
        result,
        Err(FunctionCallError::RespondToModel(message))
            if message.contains("capability is stale")
    ));
}

#[tokio::test]
async fn reattaching_the_same_step_rotates_the_unguessable_capability() {
    let (_temp, outcome) = outcome_with_skills(1, 8).await;
    let extension = ExecutorSkillsExtension;
    let (session, thread, step) = stores();
    attach_step_authority(&step, &outcome, "step-1").await;
    let stale_tools = extension.tools_for_step(&session, &thread, &step);
    attach_step_authority(&step, &outcome, "step-1").await;

    let result = invoke(
        named(&stale_tools, "list"),
        json!({"authority": {"kind": "executor"}}),
    )
    .await;
    assert!(matches!(
        result,
        Err(FunctionCallError::RespondToModel(message))
            if message.contains("capability is stale")
    ));
}

#[cfg(unix)]
#[tokio::test]
async fn symlinked_package_components_fail_closed() {
    let (temp, outcome) = outcome_with_skills(1, 8).await;
    let skill_dir = temp.path().join("skill-00");
    std::os::unix::fs::symlink(temp.path(), skill_dir.join("escape"))
        .expect("create symlink escape");
    let extension = ExecutorSkillsExtension;
    let (session, thread, step) = stores();
    attach_step_authority(&step, &outcome, "step-1").await;
    let tools = extension.tools_for_step(&session, &thread, &step);
    let listed = invoke(
        named(&tools, "list"),
        json!({"authority": {"kind": "executor"}}),
    )
    .await
    .expect("list");
    let first = &listed["skills"][0];
    let package = first["package"].as_str().unwrap();
    let denied = invoke(
        named(&tools, "read"),
        json!({
            "authority": first["authority"],
            "package": package,
            "resource": format!("{package}/escape/outside.txt"),
        }),
    )
    .await;
    assert!(matches!(denied, Err(FunctionCallError::RespondToModel(_))));
}

#[cfg(unix)]
#[tokio::test]
async fn symlinked_package_root_fails_closed_after_discovery() {
    let (temp, outcome) = outcome_with_skills(1, 8).await;
    let original = temp.path().join("skill-00");
    let relocated = temp.path().join("relocated-skill");
    std::fs::rename(&original, &relocated).expect("relocate discovered package");
    std::os::unix::fs::symlink(&relocated, &original).expect("replace package with symlink");
    let extension = ExecutorSkillsExtension;
    let (session, thread, step) = stores();
    attach_step_authority(&step, &outcome, "step-1").await;
    let tools = extension.tools_for_step(&session, &thread, &step);
    let listed = invoke(
        named(&tools, "list"),
        json!({"authority": {"kind": "executor"}}),
    )
    .await
    .expect("list");
    assert_eq!(tools.len(), 1);
    assert!(listed["skills"].as_array().is_some_and(Vec::is_empty));
    assert!(
        listed["warnings"]
            .as_array()
            .is_some_and(|warnings| !warnings.is_empty())
    );
}

#[cfg(unix)]
#[tokio::test]
async fn symlinked_final_resource_fails_closed_after_discovery() {
    let (temp, outcome) = outcome_with_skills(1, 8).await;
    let resource = temp.path().join("skill-00/reference.md");
    let relocated = temp.path().join("skill-00/relocated.md");
    std::fs::rename(&resource, &relocated).expect("relocate resource");
    std::os::unix::fs::symlink("relocated.md", &resource).expect("replace resource with symlink");
    let extension = ExecutorSkillsExtension;
    let (session, thread, step) = stores();
    attach_step_authority(&step, &outcome, "step-1").await;
    let tools = extension.tools_for_step(&session, &thread, &step);
    let listed = invoke(
        named(&tools, "list"),
        json!({"authority": {"kind": "executor"}}),
    )
    .await
    .expect("list");
    let first = &listed["skills"][0];
    let package = first["package"].as_str().expect("package");

    let denied = invoke(
        named(&tools, "read"),
        json!({
            "authority": first["authority"],
            "package": package,
            "resource": format!("{package}/reference.md"),
        }),
    )
    .await;

    assert!(matches!(denied, Err(FunctionCallError::RespondToModel(_))));
}

#[tokio::test]
async fn oversize_resource_is_rejected_at_the_one_mib_bound() {
    let (_temp, outcome) = outcome_with_skills(1, 1024 * 1024).await;
    let extension = ExecutorSkillsExtension;
    let (session, thread, step) = stores();
    attach_step_authority(&step, &outcome, "step-1").await;
    let tools = extension.tools_for_step(&session, &thread, &step);
    let listed = invoke(
        named(&tools, "list"),
        json!({"authority": {"kind": "executor"}}),
    )
    .await
    .expect("list");
    assert_eq!(tools.len(), 1);
    assert!(listed["skills"].as_array().is_some_and(Vec::is_empty));
    assert!(
        listed["warnings"]
            .as_array()
            .is_some_and(|warnings| !warnings.is_empty())
    );
}

#[tokio::test]
async fn unsupported_executor_fails_closed_without_legacy_read_fallback() {
    let temp = tempfile::tempdir().expect("temporary skill root");
    let skill_dir = temp.path().join("legacy-skill");
    std::fs::create_dir_all(&skill_dir).expect("create skill directory");
    std::fs::write(
        skill_dir.join("SKILL.md"),
        "---\nname: legacy-skill\ndescription: legacy executor\n---\n",
    )
    .expect("write skill");
    let read_calls = Arc::new(AtomicUsize::new(0));
    let legacy_file_system: Arc<dyn ExecutorFileSystem> = Arc::new(LegacyReadOnlyFileSystem {
        inner: Arc::clone(&LOCAL_FS),
        read_calls: Arc::clone(&read_calls),
    });
    let outcome = load_skills_from_roots([SkillRoot {
        path: AbsolutePathBuf::from_absolute_path_checked(temp.path()).expect("absolute root"),
        scope: SkillScope::User,
        file_system: SkillRootFileSystem::Executor(legacy_file_system),
        plugin_id: None,
        plugin_namespace: None,
        plugin_root: None,
        discovery_mode: SkillDiscoveryMode::Recursive,
    }])
    .await;
    assert!(outcome.errors.is_empty(), "{:?}", outcome.errors);
    assert_eq!(outcome.skills.len(), 1);
    let reads_after_discovery = read_calls.load(Ordering::SeqCst);

    let extension = ExecutorSkillsExtension;
    let (session, thread, step) = stores();
    attach_step_authority(&step, &outcome, "step-1").await;
    let tools = extension.tools_for_step(&session, &thread, &step);
    let listed = invoke(
        named(&tools, "list"),
        json!({"authority": {"kind": "executor"}}),
    )
    .await
    .expect("list");
    assert_eq!(tools.len(), 1);
    assert!(listed["skills"].as_array().is_some_and(Vec::is_empty));
    assert!(
        listed["warnings"]
            .as_array()
            .is_some_and(|warnings| warnings.iter().any(|warning| {
                warning
                    .as_str()
                    .is_some_and(|warning| warning.contains("atomic bounded-read authority"))
            }))
    );
    assert_eq!(
        read_calls.load(Ordering::SeqCst),
        reads_after_discovery,
        "authority attachment must never fall back to the legacy unbounded read API"
    );
}

#[tokio::test]
async fn pagination_cursors_are_bounded_and_reject_stale_fingerprints() {
    let (_temp, outcome) = outcome_with_skills(21, 8).await;
    let extension = ExecutorSkillsExtension;
    let (session, thread, step) = stores();
    attach_step_authority(&step, &outcome, "step-1").await;
    let tools = extension.tools_for_step(&session, &thread, &step);
    let list = named(&tools, "list");
    let first = invoke(list, json!({"authority": {"kind": "executor"}}))
        .await
        .expect("first page");
    assert_eq!(first["skills"].as_array().unwrap().len(), 20);
    let cursor = first["next_cursor"].as_str().expect("next cursor");
    let second = invoke(
        list,
        json!({"authority": {"kind": "executor"}, "cursor": cursor}),
    )
    .await
    .expect("second page");
    assert_eq!(second["skills"].as_array().unwrap().len(), 1);

    let stale = invoke(
        list,
        json!({"authority": {"kind": "executor"}, "cursor": "0000:20"}),
    )
    .await;
    assert!(matches!(
        stale,
        Err(FunctionCallError::RespondToModel(message))
            if message.contains("cursor is stale")
    ));
}

#[tokio::test]
async fn read_pages_stay_under_the_response_limit() {
    let (_temp, outcome) = outcome_with_skills(1, 700_000).await;
    let extension = ExecutorSkillsExtension;
    let (session, thread, step) = stores();
    attach_step_authority(&step, &outcome, "step-1").await;
    let tools = extension.tools_for_step(&session, &thread, &step);
    let listed = invoke(
        named(&tools, "list"),
        json!({"authority": {"kind": "executor"}}),
    )
    .await
    .expect("list");
    let first = &listed["skills"][0];
    let read = named(&tools, "read");
    let first_page = invoke(
        read,
        json!({
            "authority": first["authority"],
            "package": first["package"],
            "resource": first["main_resource"],
        }),
    )
    .await
    .expect("first page");
    assert!(serde_json::to_vec(&first_page).unwrap().len() <= 512 * 1024);
    let cursor = first_page["next_cursor"].as_str().expect("read cursor");
    let second_page = invoke(
        read,
        json!({
            "authority": first["authority"],
            "package": first["package"],
            "resource": first["main_resource"],
            "cursor": cursor,
        }),
    )
    .await
    .expect("second page");
    assert!(serde_json::to_vec(&second_page).unwrap().len() <= 512 * 1024);
}
