use super::*;
use crate::items::FileChangeItem;
use crate::items::ImageGenerationItem;
use crate::items::McpToolCallItem;
use crate::items::McpToolCallStatus;
use crate::items::UserMessageItem;
use crate::items::WebSearchItem;
use crate::mcp::CallToolResult;
use crate::permissions::FileSystemAccessMode;
use crate::permissions::FileSystemPath;
use crate::permissions::FileSystemSandboxEntry;
use crate::permissions::FileSystemSandboxPolicy;
use crate::permissions::FileSystemSpecialPath;
use crate::permissions::NetworkSandboxPolicy;
use anyhow::Result;
use codex_utils_absolute_path::AbsolutePathBuf;
use codex_utils_absolute_path::test_support::PathBufExt;
use codex_utils_absolute_path::test_support::test_path_buf;
use pretty_assertions::assert_eq;
use serde_json::json;
use std::path::PathBuf;
use tempfile::NamedTempFile;
use tempfile::TempDir;

#[test]
fn thread_history_mode_has_stable_wire_values_and_legacy_default() -> Result<()> {
    assert_eq!(ThreadHistoryMode::default(), ThreadHistoryMode::Legacy);
    assert_eq!(ThreadHistoryMode::Legacy.as_str(), "legacy");
    assert_eq!(ThreadHistoryMode::Paginated.as_str(), "paginated");
    assert_eq!(
        serde_json::to_string(&ThreadHistoryMode::Paginated)?,
        r#""paginated""#
    );
    assert_eq!(
        serde_json::from_str::<ThreadHistoryMode>(r#""legacy""#)?,
        ThreadHistoryMode::Legacy
    );
    assert_eq!(
        ThreadHistoryMode::from_str("paginated"),
        Ok(ThreadHistoryMode::Paginated)
    );
    assert!(ThreadHistoryMode::from_str("future").is_err());
    Ok(())
}

#[test]
fn session_meta_defaults_to_legacy_and_rejects_unknown_history_mode() -> Result<()> {
    let meta: SessionMeta = serde_json::from_value(json!({
        "id": "00000000-0000-0000-0000-000000000001",
        "timestamp": "2026-01-01T00:00:00Z",
        "cwd": "/tmp",
        "originator": "hepta",
        "cli_version": "0.0.0",
        "model_provider": null,
        "base_instructions": null
    }))?;

    assert_eq!(meta.history_mode, ThreadHistoryMode::Legacy);
    let mut serialized = serde_json::to_value(meta)?;
    assert_eq!(serialized["history_mode"], json!("legacy"));
    serialized["history_mode"] = json!("future");
    assert!(serde_json::from_value::<SessionMeta>(serialized).is_err());
    Ok(())
}

#[test]
fn resumed_history_uses_canonical_persisted_history_mode() -> Result<()> {
    let thread_id = ThreadId::from_string("00000000-0000-0000-0000-000000000001")?;
    let session_meta = RolloutItem::SessionMeta(SessionMetaLine {
        meta: SessionMeta {
            id: thread_id,
            history_mode: ThreadHistoryMode::Paginated,
            ..SessionMeta::default()
        },
        git: None,
    });
    let resumed = InitialHistory::Resumed(ResumedHistory {
        conversation_id: thread_id,
        history: vec![session_meta.clone()],
        rollout_path: None,
    });

    assert_eq!(
        resumed.get_history_mode(ThreadHistoryMode::Legacy),
        ThreadHistoryMode::Paginated
    );
    assert_eq!(
        InitialHistory::Forked(vec![session_meta]).get_history_mode(ThreadHistoryMode::Legacy),
        ThreadHistoryMode::Legacy
    );
    assert_eq!(
        InitialHistory::New.get_history_mode(ThreadHistoryMode::Paginated),
        ThreadHistoryMode::Paginated
    );
    Ok(())
}

#[test]
fn strip_user_message_context_accepts_current_marker() {
    let message = "context\n## My request for Hepta:\n  optimize this\n";

    assert_eq!(strip_user_message_context(message), "optimize this");
}

#[test]
fn strip_user_message_context_accepts_legacy_marker() {
    let message = "context\n## My request for Codex:\n  optimize this\n";

    assert_eq!(strip_user_message_context(message), "optimize this");
}

fn sorted_writable_roots(roots: Vec<WritableRoot>) -> Vec<(PathBuf, Vec<PathBuf>)> {
    let mut sorted_roots: Vec<(PathBuf, Vec<PathBuf>)> = roots
        .into_iter()
        .map(|root| {
            let mut read_only_subpaths: Vec<PathBuf> = root
                .read_only_subpaths
                .into_iter()
                .map(|path| path.to_path_buf())
                .collect();
            read_only_subpaths.sort();
            (root.root.to_path_buf(), read_only_subpaths)
        })
        .collect();
    sorted_roots.sort_by(|left, right| left.0.cmp(&right.0));
    sorted_roots
}

fn sandbox_policy_allows_read(policy: &SandboxPolicy, _path: &Path, _cwd: &Path) -> bool {
    policy.has_full_disk_read_access()
}

fn sandbox_policy_allows_write(policy: &SandboxPolicy, path: &Path, cwd: &Path) -> bool {
    if policy.has_full_disk_write_access() {
        return true;
    }

    policy
        .get_writable_roots_with_cwd(cwd)
        .iter()
        .any(|root| root.is_path_writable(path))
}

#[test]
fn session_source_from_startup_arg_maps_known_values() {
    assert_eq!(
        SessionSource::from_startup_arg("vscode").unwrap(),
        SessionSource::VSCode
    );
    assert_eq!(
        SessionSource::from_startup_arg("app-server").unwrap(),
        SessionSource::Mcp
    );
}

#[test]
fn inter_agent_communication_response_input_item_preserves_commentary_phase() {
    let communication = InterAgentCommunication {
        author: AgentPath::root(),
        recipient: AgentPath::root().join("reviewer").expect("recipient path"),
        other_recipients: vec![AgentPath::root().join("worker").expect("recipient path")],
        content: "review the diff".to_string(),
        encrypted_content: None,
        trigger_turn: true,
    };

    assert_eq!(
        communication.to_response_input_item(),
        ResponseInputItem::Message {
            role: "assistant".to_string(),
            content: vec![ContentItem::OutputText {
                text: serde_json::to_string(&communication).expect("serialize communication"),
            }],
            phase: Some(MessagePhase::Commentary),
        }
    );
}

#[test]
fn session_source_from_startup_arg_normalizes_custom_values() {
    assert_eq!(
        SessionSource::from_startup_arg("atlas").unwrap(),
        SessionSource::Custom("atlas".to_string())
    );
    assert_eq!(
        SessionSource::from_startup_arg(" Atlas ").unwrap(),
        SessionSource::Custom("atlas".to_string())
    );
}

#[test]
fn session_source_restriction_product_defaults_non_subagent_sources_to_hepta() {
    assert_eq!(
        SessionSource::Cli.restriction_product(),
        Some(Product::Hepta)
    );
    assert_eq!(
        SessionSource::VSCode.restriction_product(),
        Some(Product::Hepta)
    );
    assert_eq!(
        SessionSource::Exec.restriction_product(),
        Some(Product::Hepta)
    );
    assert_eq!(
        SessionSource::Mcp.restriction_product(),
        Some(Product::Hepta)
    );
    assert_eq!(
        SessionSource::Unknown.restriction_product(),
        Some(Product::Hepta)
    );
}

#[test]
fn session_source_restriction_product_does_not_guess_subagent_products() {
    assert_eq!(
        SessionSource::SubAgent(SubAgentSource::Review).restriction_product(),
        None
    );
    assert_eq!(
        SessionSource::Internal(InternalSessionSource::MemoryConsolidation).restriction_product(),
        None
    );
}

#[test]
fn session_source_restriction_product_maps_custom_sources_to_products() {
    assert_eq!(
        SessionSource::Custom("chatgpt".to_string()).restriction_product(),
        Some(Product::Chatgpt)
    );
    assert_eq!(
        SessionSource::Custom("ATLAS".to_string()).restriction_product(),
        Some(Product::Atlas)
    );
    assert_eq!(
        SessionSource::Custom("codex".to_string()).restriction_product(),
        Some(Product::Codex)
    );
    assert_eq!(
        SessionSource::Custom("hepta".to_string()).restriction_product(),
        Some(Product::Hepta)
    );
    assert_eq!(
        SessionSource::Custom("atlas-dev".to_string()).restriction_product(),
        None
    );
}

#[test]
fn session_source_matches_product_restriction() {
    assert!(
        SessionSource::Custom("chatgpt".to_string())
            .matches_product_restriction(&[Product::Chatgpt])
    );
    assert!(
        !SessionSource::Custom("chatgpt".to_string())
            .matches_product_restriction(&[Product::Codex])
    );
    assert!(SessionSource::VSCode.matches_product_restriction(&[Product::Codex]));
    assert!(SessionSource::VSCode.matches_product_restriction(&[Product::Hepta]));
    assert!(
        !SessionSource::Custom("codex".to_string()).matches_product_restriction(&[Product::Hepta])
    );
    assert!(
        !SessionSource::Custom("atlas-dev".to_string())
            .matches_product_restriction(&[Product::Atlas])
    );
    assert!(SessionSource::Custom("atlas-dev".to_string()).matches_product_restriction(&[]));
}

#[test]
fn hepta_product_uses_codex_platform_compatibility() {
    assert_eq!(Product::Hepta.to_app_platform(), "codex");
    assert!(Product::Hepta.matches_product_restriction(&[Product::Codex]));
    assert!(Product::Hepta.matches_product_restriction(&[Product::Hepta]));
    assert!(!Product::Codex.matches_product_restriction(&[Product::Hepta]));
}

fn sandbox_policy_probe_paths(policy: &SandboxPolicy, cwd: &Path) -> Vec<PathBuf> {
    let mut paths = vec![cwd.to_path_buf()];
    for root in policy.get_writable_roots_with_cwd(cwd) {
        paths.push(root.root.to_path_buf());
        paths.extend(
            root.read_only_subpaths
                .into_iter()
                .map(|path| path.to_path_buf()),
        );
    }
    paths.sort();
    paths.dedup();
    paths
}

fn assert_same_sandbox_policy_semantics(
    expected: &SandboxPolicy,
    actual: &SandboxPolicy,
    cwd: &Path,
) {
    assert_eq!(
        actual.has_full_disk_read_access(),
        expected.has_full_disk_read_access()
    );
    assert_eq!(
        actual.has_full_disk_write_access(),
        expected.has_full_disk_write_access()
    );
    assert_eq!(
        actual.has_full_network_access(),
        expected.has_full_network_access()
    );
    let mut probe_paths = sandbox_policy_probe_paths(expected, cwd);
    probe_paths.extend(sandbox_policy_probe_paths(actual, cwd));
    probe_paths.sort();
    probe_paths.dedup();

    for path in probe_paths {
        assert_eq!(
            sandbox_policy_allows_read(actual, &path, cwd),
            sandbox_policy_allows_read(expected, &path, cwd),
            "read access mismatch for {}",
            path.display()
        );
        assert_eq!(
            sandbox_policy_allows_write(actual, &path, cwd),
            sandbox_policy_allows_write(expected, &path, cwd),
            "write access mismatch for {}",
            path.display()
        );
    }
}

#[test]
fn external_sandbox_reports_full_access_flags() {
    let restricted = SandboxPolicy::ExternalSandbox {
        network_access: NetworkAccess::Restricted,
    };
    assert!(restricted.has_full_disk_write_access());
    assert!(!restricted.has_full_network_access());

    let enabled = SandboxPolicy::ExternalSandbox {
        network_access: NetworkAccess::Enabled,
    };
    assert!(enabled.has_full_disk_write_access());
    assert!(enabled.has_full_network_access());
}

#[test]
fn read_only_reports_network_access_flags() {
    let restricted = SandboxPolicy::new_read_only_policy();
    assert!(!restricted.has_full_network_access());

    let enabled = SandboxPolicy::ReadOnly {
        network_access: true,
    };
    assert!(enabled.has_full_network_access());
}

#[test]
fn granular_approval_config_mcp_elicitation_flag_is_field_driven() {
    assert!(
        GranularApprovalConfig {
            sandbox_approval: false,
            rules: false,
            skill_approval: false,
            request_permissions: false,
            mcp_elicitations: true,
        }
        .allows_mcp_elicitations()
    );
    assert!(
        !GranularApprovalConfig {
            sandbox_approval: false,
            rules: false,
            skill_approval: false,
            request_permissions: false,
            mcp_elicitations: false,
        }
        .allows_mcp_elicitations()
    );
}

#[test]
fn granular_approval_config_skill_approval_flag_is_field_driven() {
    assert!(
        GranularApprovalConfig {
            sandbox_approval: false,
            rules: false,
            skill_approval: true,
            request_permissions: false,
            mcp_elicitations: false,
        }
        .allows_skill_approval()
    );
    assert!(
        !GranularApprovalConfig {
            sandbox_approval: false,
            rules: false,
            skill_approval: false,
            request_permissions: false,
            mcp_elicitations: false,
        }
        .allows_skill_approval()
    );
}

#[test]
fn granular_approval_config_request_permissions_flag_is_field_driven() {
    assert!(
        GranularApprovalConfig {
            sandbox_approval: false,
            rules: false,
            skill_approval: false,
            request_permissions: true,
            mcp_elicitations: false,
        }
        .allows_request_permissions()
    );
    assert!(
        !GranularApprovalConfig {
            sandbox_approval: false,
            rules: false,
            skill_approval: false,
            request_permissions: false,
            mcp_elicitations: false,
        }
        .allows_request_permissions()
    );
}

#[test]
fn granular_approval_config_defaults_missing_optional_flags_to_false() {
    let decoded = serde_json::from_value::<GranularApprovalConfig>(serde_json::json!({
        "sandbox_approval": true,
        "rules": false,
        "mcp_elicitations": true,
    }))
    .expect("granular approval config should deserialize");

    assert_eq!(
        decoded,
        GranularApprovalConfig {
            sandbox_approval: true,
            rules: false,
            skill_approval: false,
            request_permissions: false,
            mcp_elicitations: true,
        }
    );
}

#[test]
fn restricted_file_system_policy_reports_full_access_from_root_entries() {
    let read_only = FileSystemSandboxPolicy::restricted(vec![FileSystemSandboxEntry {
        path: FileSystemPath::Special {
            value: FileSystemSpecialPath::Root,
        },
        access: FileSystemAccessMode::Read,
    }]);
    assert!(read_only.has_full_disk_read_access());
    assert!(!read_only.has_full_disk_write_access());
    assert!(!read_only.include_platform_defaults());

    let writable = FileSystemSandboxPolicy::restricted(vec![FileSystemSandboxEntry {
        path: FileSystemPath::Special {
            value: FileSystemSpecialPath::Root,
        },
        access: FileSystemAccessMode::Write,
    }]);
    assert!(writable.has_full_disk_read_access());
    assert!(writable.has_full_disk_write_access());
}

#[test]
fn restricted_file_system_policy_treats_root_with_carveouts_as_scoped_access() {
    let cwd = TempDir::new().expect("tempdir");
    let canonical_cwd = codex_utils_absolute_path::canonicalize_preserving_symlinks(cwd.path())
        .expect("canonicalize cwd");
    let root = AbsolutePathBuf::from_absolute_path(&canonical_cwd)
        .expect("absolute canonical tempdir")
        .as_path()
        .ancestors()
        .last()
        .and_then(|path| AbsolutePathBuf::from_absolute_path(path).ok())
        .expect("filesystem root");
    let blocked = AbsolutePathBuf::resolve_path_against_base("blocked", cwd.path());
    let expected_blocked = AbsolutePathBuf::from_absolute_path(
        codex_utils_absolute_path::canonicalize_preserving_symlinks(cwd.path())
            .expect("canonicalize cwd")
            .join("blocked"),
    )
    .expect("canonical blocked");
    let policy = FileSystemSandboxPolicy::restricted(vec![
        FileSystemSandboxEntry {
            path: FileSystemPath::Special {
                value: FileSystemSpecialPath::Root,
            },
            access: FileSystemAccessMode::Write,
        },
        FileSystemSandboxEntry {
            path: FileSystemPath::Path { path: blocked },
            access: FileSystemAccessMode::None,
        },
    ]);

    assert!(!policy.has_full_disk_read_access());
    assert!(!policy.has_full_disk_write_access());
    assert_eq!(
        policy.get_readable_roots_with_cwd(cwd.path()),
        vec![root.clone()]
    );
    assert_eq!(
        policy.get_unreadable_roots_with_cwd(cwd.path()),
        vec![expected_blocked.clone()]
    );

    let writable_roots = policy.get_writable_roots_with_cwd(cwd.path());
    assert_eq!(writable_roots.len(), 1);
    assert_eq!(writable_roots[0].root, root);
    assert!(
        writable_roots[0]
            .read_only_subpaths
            .iter()
            .any(|path| path.as_path() == expected_blocked.as_path())
    );
}

#[test]
fn restricted_file_system_policy_derives_effective_paths() {
    let cwd = TempDir::new().expect("tempdir");
    std::fs::create_dir_all(cwd.path().join(".agents")).expect("create .agents");
    std::fs::create_dir_all(cwd.path().join(".codex")).expect("create .codex");
    let canonical_cwd = codex_utils_absolute_path::canonicalize_preserving_symlinks(cwd.path())
        .expect("canonicalize cwd");
    let cwd_absolute =
        AbsolutePathBuf::from_absolute_path(&canonical_cwd).expect("absolute tempdir");
    let secret = AbsolutePathBuf::resolve_path_against_base("secret", cwd.path());
    let expected_secret = AbsolutePathBuf::from_absolute_path(canonical_cwd.join("secret"))
        .expect("canonical secret");
    let expected_agents = AbsolutePathBuf::from_absolute_path(canonical_cwd.join(".agents"))
        .expect("canonical .agents");
    let expected_codex = AbsolutePathBuf::from_absolute_path(canonical_cwd.join(".codex"))
        .expect("canonical .codex");
    let policy = FileSystemSandboxPolicy::restricted(vec![
        FileSystemSandboxEntry {
            path: FileSystemPath::Special {
                value: FileSystemSpecialPath::Minimal,
            },
            access: FileSystemAccessMode::Read,
        },
        FileSystemSandboxEntry {
            path: FileSystemPath::Special {
                value: FileSystemSpecialPath::project_roots(/*subpath*/ None),
            },
            access: FileSystemAccessMode::Write,
        },
        FileSystemSandboxEntry {
            path: FileSystemPath::Path { path: secret },
            access: FileSystemAccessMode::None,
        },
    ]);

    assert!(!policy.has_full_disk_read_access());
    assert!(!policy.has_full_disk_write_access());
    assert!(policy.include_platform_defaults());
    assert_eq!(
        policy.get_readable_roots_with_cwd(cwd.path()),
        vec![cwd_absolute.clone()]
    );
    assert_eq!(
        policy.get_unreadable_roots_with_cwd(cwd.path()),
        vec![expected_secret.clone()]
    );

    let writable_roots = policy.get_writable_roots_with_cwd(cwd.path());
    assert_eq!(writable_roots.len(), 1);
    assert_eq!(writable_roots[0].root, cwd_absolute);
    assert!(
        writable_roots[0]
            .read_only_subpaths
            .iter()
            .any(|path| path.as_path() == expected_secret.as_path())
    );
    assert!(
        writable_roots[0]
            .read_only_subpaths
            .iter()
            .any(|path| path.as_path() == expected_agents.as_path())
    );
    assert!(
        writable_roots[0]
            .read_only_subpaths
            .iter()
            .any(|path| path.as_path() == expected_codex.as_path())
    );
}

#[test]
fn restricted_file_system_policy_treats_read_entries_as_read_only_subpaths() {
    let cwd = TempDir::new().expect("tempdir");
    let canonical_cwd = codex_utils_absolute_path::canonicalize_preserving_symlinks(cwd.path())
        .expect("canonicalize cwd");
    let docs = AbsolutePathBuf::resolve_path_against_base("docs", cwd.path());
    let docs_public = AbsolutePathBuf::resolve_path_against_base("docs/public", cwd.path());
    let expected_docs =
        AbsolutePathBuf::from_absolute_path(canonical_cwd.join("docs")).expect("canonical docs");
    let expected_docs_public =
        AbsolutePathBuf::from_absolute_path(canonical_cwd.join("docs/public"))
            .expect("canonical docs/public");
    let expected_dot_codex = AbsolutePathBuf::from_absolute_path(canonical_cwd.join(".codex"))
        .expect("canonical .codex");
    let policy = FileSystemSandboxPolicy::restricted(vec![
        FileSystemSandboxEntry {
            path: FileSystemPath::Special {
                value: FileSystemSpecialPath::project_roots(/*subpath*/ None),
            },
            access: FileSystemAccessMode::Write,
        },
        FileSystemSandboxEntry {
            path: FileSystemPath::Path { path: docs },
            access: FileSystemAccessMode::Read,
        },
        FileSystemSandboxEntry {
            path: FileSystemPath::Path { path: docs_public },
            access: FileSystemAccessMode::Write,
        },
    ]);

    assert!(!policy.has_full_disk_write_access());
    assert_eq!(
        sorted_writable_roots(policy.get_writable_roots_with_cwd(cwd.path())),
        vec![
            (
                canonical_cwd,
                vec![
                    expected_dot_codex.to_path_buf(),
                    expected_docs.to_path_buf()
                ],
            ),
            (expected_docs_public.to_path_buf(), Vec::new()),
        ]
    );
}

#[test]
fn file_system_policy_rejects_legacy_bridge_for_non_workspace_writes() {
    let cwd = if cfg!(windows) {
        Path::new(r"C:\workspace")
    } else {
        Path::new("/tmp/workspace")
    };
    let external_write_path = if cfg!(windows) {
        AbsolutePathBuf::from_absolute_path(r"C:\temp").expect("absolute windows temp path")
    } else {
        AbsolutePathBuf::from_absolute_path("/tmp").expect("absolute tmp path")
    };
    let policy = FileSystemSandboxPolicy::restricted(vec![FileSystemSandboxEntry {
        path: FileSystemPath::Path {
            path: external_write_path,
        },
        access: FileSystemAccessMode::Write,
    }]);

    let err = policy
        .to_legacy_sandbox_policy(NetworkSandboxPolicy::Restricted, cwd)
        .expect_err("non-workspace writes should be rejected");

    assert!(
        err.to_string()
            .contains("filesystem writes outside the workspace root"),
        "{err}"
    );
}

#[test]
fn legacy_sandbox_policy_semantics_survive_split_bridge() {
    let cwd = TempDir::new().expect("tempdir");
    let writable_root = AbsolutePathBuf::resolve_path_against_base("writable", cwd.path());
    let policies = [
        SandboxPolicy::DangerFullAccess,
        SandboxPolicy::ExternalSandbox {
            network_access: NetworkAccess::Restricted,
        },
        SandboxPolicy::ExternalSandbox {
            network_access: NetworkAccess::Enabled,
        },
        SandboxPolicy::ReadOnly {
            network_access: false,
        },
        SandboxPolicy::WorkspaceWrite {
            writable_roots: vec![],
            network_access: false,
            exclude_tmpdir_env_var: true,
            exclude_slash_tmp: true,
        },
        SandboxPolicy::WorkspaceWrite {
            writable_roots: vec![writable_root],
            network_access: true,
            exclude_tmpdir_env_var: false,
            exclude_slash_tmp: true,
        },
    ];

    for expected in policies {
        let actual =
            FileSystemSandboxPolicy::from_legacy_sandbox_policy_for_cwd(&expected, cwd.path())
                .to_legacy_sandbox_policy(NetworkSandboxPolicy::from(&expected), cwd.path())
                .expect("legacy bridge should preserve legacy policy semantics");

        assert_same_sandbox_policy_semantics(&expected, &actual, cwd.path());
    }
}

#[test]
fn item_started_event_from_web_search_emits_begin_event() {
    let event = ItemStartedEvent {
        thread_id: ThreadId::new(),
        turn_id: "turn-1".into(),
        item: TurnItem::WebSearch(WebSearchItem {
            id: "search-1".into(),
            query: "find docs".into(),
            action: WebSearchAction::Search {
                query: Some("find docs".into()),
                queries: None,
            },
        }),
        started_at_ms: 0,
    };

    let legacy_events = event.as_legacy_events(/*show_raw_agent_reasoning*/ false);
    assert_eq!(legacy_events.len(), 1);
    match &legacy_events[0] {
        EventMsg::WebSearchBegin(event) => assert_eq!(event.call_id, "search-1"),
        _ => panic!("expected WebSearchBegin event"),
    }
}

#[test]
fn item_started_event_from_non_web_search_emits_no_legacy_events() {
    let event = ItemStartedEvent {
        thread_id: ThreadId::new(),
        turn_id: "turn-1".into(),
        item: TurnItem::UserMessage(UserMessageItem::new(&[])),
        started_at_ms: 0,
    };

    assert!(
        event
            .as_legacy_events(/*show_raw_agent_reasoning*/ false)
            .is_empty()
    );
}

#[test]
fn item_started_event_from_image_generation_emits_begin_event() {
    let event = ItemStartedEvent {
        thread_id: ThreadId::new(),
        turn_id: "turn-1".into(),
        item: TurnItem::ImageGeneration(ImageGenerationItem {
            id: "ig-1".into(),
            status: "in_progress".into(),
            revised_prompt: None,
            result: String::new(),
            saved_path: None,
        }),
        started_at_ms: 0,
    };

    let legacy_events = event.as_legacy_events(/*show_raw_agent_reasoning*/ false);
    assert_eq!(legacy_events.len(), 1);
    match &legacy_events[0] {
        EventMsg::ImageGenerationBegin(event) => assert_eq!(event.call_id, "ig-1"),
        _ => panic!("expected ImageGenerationBegin event"),
    }
}

#[test]
fn item_started_event_from_file_change_emits_patch_begin_event() {
    let event = ItemStartedEvent {
        thread_id: ThreadId::new(),
        turn_id: "turn-1".into(),
        started_at_ms: 0,
        item: TurnItem::FileChange(FileChangeItem {
            id: "patch-1".into(),
            changes: [(
                PathBuf::from("new.txt"),
                FileChange::Add {
                    content: "hello".into(),
                },
            )]
            .into_iter()
            .collect(),
            status: None,
            auto_approved: Some(true),
            stdout: None,
            stderr: None,
        }),
    };

    let legacy_events = event.as_legacy_events(/*show_raw_agent_reasoning*/ false);
    assert_eq!(legacy_events.len(), 1);
    match &legacy_events[0] {
        EventMsg::PatchApplyBegin(event) => {
            assert_eq!(event.call_id, "patch-1");
            assert_eq!(event.turn_id, "turn-1");
            assert!(event.auto_approved);
            assert!(event.changes.contains_key(&PathBuf::from("new.txt")));
        }
        _ => panic!("expected PatchApplyBegin event"),
    }
}

#[test]
fn item_started_event_from_mcp_tool_call_emits_begin_event() {
    let event = ItemStartedEvent {
        thread_id: ThreadId::new(),
        turn_id: "turn-1".into(),
        started_at_ms: 0,
        item: TurnItem::McpToolCall(McpToolCallItem {
            id: "mcp-1".into(),
            server: "server".into(),
            tool: "tool".into(),
            arguments: json!({"arg": "value"}),
            mcp_app_resource_uri: Some("app://connector".into()),
            read_only_hint: None,
            status: McpToolCallStatus::InProgress,
            result: None,
            error: None,
            duration: None,
        }),
    };

    let legacy_events = event.as_legacy_events(/*show_raw_agent_reasoning*/ false);
    assert_eq!(legacy_events.len(), 1);
    match &legacy_events[0] {
        EventMsg::McpToolCallBegin(event) => {
            assert_eq!(event.call_id, "mcp-1");
            assert_eq!(event.invocation.server, "server");
            assert_eq!(event.invocation.tool, "tool");
            assert_eq!(
                event.mcp_app_resource_uri.as_deref(),
                Some("app://connector")
            );
        }
        _ => panic!("expected McpToolCallBegin event"),
    }
}

#[test]
fn item_completed_event_from_image_generation_emits_end_event() {
    let event = ItemCompletedEvent {
        thread_id: ThreadId::new(),
        turn_id: "turn-1".into(),
        item: TurnItem::ImageGeneration(ImageGenerationItem {
            id: "ig-1".into(),
            status: "completed".into(),
            revised_prompt: Some("A tiny blue square".into()),
            result: "Zm9v".into(),
            saved_path: Some(test_path_buf("/tmp/ig-1.png").abs()),
        }),
        started_at_ms: Some(0),
        completed_at_ms: 0,
    };

    let legacy_events = event.as_legacy_events(/*show_raw_agent_reasoning*/ false);
    assert_eq!(legacy_events.len(), 1);
    match &legacy_events[0] {
        EventMsg::ImageGenerationEnd(event) => {
            assert_eq!(event.call_id, "ig-1");
            assert_eq!(event.status, "completed");
            assert_eq!(event.revised_prompt.as_deref(), Some("A tiny blue square"));
            assert_eq!(event.result, "Zm9v");
            assert_eq!(
                event.saved_path.as_ref().map(AbsolutePathBuf::as_path),
                Some(test_path_buf("/tmp/ig-1.png").as_path())
            );
        }
        _ => panic!("expected ImageGenerationEnd event"),
    }
}

#[test]
fn item_completed_event_from_file_change_emits_patch_end_event() {
    let event = ItemCompletedEvent {
        thread_id: ThreadId::new(),
        turn_id: "turn-1".into(),
        started_at_ms: Some(0),
        completed_at_ms: 0,
        item: TurnItem::FileChange(FileChangeItem {
            id: "patch-1".into(),
            changes: [(
                PathBuf::from("new.txt"),
                FileChange::Add {
                    content: "hello".into(),
                },
            )]
            .into_iter()
            .collect(),
            status: Some(PatchApplyStatus::Completed),
            auto_approved: None,
            stdout: Some("Done!".into()),
            stderr: Some(String::new()),
        }),
    };

    let legacy_events = event.as_legacy_events(/*show_raw_agent_reasoning*/ false);
    assert_eq!(legacy_events.len(), 1);
    match &legacy_events[0] {
        EventMsg::PatchApplyEnd(event) => {
            assert_eq!(event.call_id, "patch-1");
            assert_eq!(event.turn_id, "turn-1");
            assert_eq!(event.stdout, "Done!");
            assert!(event.success);
            assert_eq!(event.status, PatchApplyStatus::Completed);
            assert!(event.changes.contains_key(&PathBuf::from("new.txt")));
        }
        _ => panic!("expected PatchApplyEnd event"),
    }
}

#[test]
fn item_completed_event_from_mcp_tool_call_emits_end_event() {
    let event = ItemCompletedEvent {
        thread_id: ThreadId::new(),
        turn_id: "turn-1".into(),
        started_at_ms: Some(0),
        completed_at_ms: 0,
        item: TurnItem::McpToolCall(McpToolCallItem {
            id: "mcp-1".into(),
            server: "server".into(),
            tool: "tool".into(),
            arguments: json!({"arg": "value"}),
            mcp_app_resource_uri: Some("app://connector".into()),
            read_only_hint: None,
            status: McpToolCallStatus::Completed,
            result: Some(CallToolResult {
                content: vec![json!({"type": "text", "text": "ok"})],
                structured_content: None,
                is_error: Some(false),
                meta: None,
            }),
            error: None,
            duration: Some(Duration::from_millis(42)),
        }),
    };

    let legacy_events = event.as_legacy_events(/*show_raw_agent_reasoning*/ false);
    assert_eq!(legacy_events.len(), 1);
    match &legacy_events[0] {
        EventMsg::McpToolCallEnd(event) => {
            assert_eq!(event.call_id, "mcp-1");
            assert_eq!(event.invocation.server, "server");
            assert_eq!(event.invocation.tool, "tool");
            assert_eq!(
                event.mcp_app_resource_uri.as_deref(),
                Some("app://connector")
            );
            assert_eq!(event.duration, Duration::from_millis(42));
            assert!(event.is_success());
        }
        _ => panic!("expected McpToolCallEnd event"),
    }
}

#[test]
fn item_started_event_requires_started_at_ms() {
    let mut value = serde_json::to_value(ItemStartedEvent {
        thread_id: ThreadId::new(),
        turn_id: "turn-1".into(),
        item: TurnItem::UserMessage(UserMessageItem::new(&[])),
        started_at_ms: 123,
    })
    .unwrap();
    value.as_object_mut().unwrap().remove("started_at_ms");

    assert!(serde_json::from_value::<ItemStartedEvent>(value).is_err());
}

#[test]
fn item_completed_event_defaults_missing_completed_at_ms() {
    let mut value = serde_json::to_value(ItemCompletedEvent {
        thread_id: ThreadId::new(),
        turn_id: "turn-1".into(),
        item: TurnItem::UserMessage(UserMessageItem::new(&[])),
        started_at_ms: None,
        completed_at_ms: 123,
    })
    .unwrap();
    value.as_object_mut().unwrap().remove("completed_at_ms");

    let event = serde_json::from_value::<ItemCompletedEvent>(value).unwrap();
    assert_eq!(event.started_at_ms, None);
    assert_eq!(event.completed_at_ms, 0);
}
#[test]
fn rollback_failed_error_does_not_affect_turn_status() {
    let event = ErrorEvent {
        message: "rollback failed".into(),
        codex_error_info: Some(CodexErrorInfo::ThreadRollbackFailed),
    };
    assert!(!event.affects_turn_status());
}

#[test]
fn active_turn_not_steerable_error_does_not_affect_turn_status() {
    let event = ErrorEvent {
        message: "cannot steer a review turn".into(),
        codex_error_info: Some(CodexErrorInfo::ActiveTurnNotSteerable {
            turn_kind: NonSteerableTurnKind::Review,
        }),
    };
    assert!(!event.affects_turn_status());
}

#[test]
fn generic_error_affects_turn_status() {
    let event = ErrorEvent {
        message: "generic".into(),
        codex_error_info: Some(CodexErrorInfo::Other),
    };
    assert!(event.affects_turn_status());
}

#[test]
fn conversation_op_serializes_as_unnested_variants() {
    let audio = Op::RealtimeConversationAudio(ConversationAudioParams {
        frame: RealtimeAudioFrame {
            data: "AQID".to_string(),
            sample_rate: 24_000,
            num_channels: 1,
            samples_per_channel: Some(480),
            item_id: None,
        },
    });
    let start = Op::RealtimeConversationStart(ConversationStartParams {
        output_modality: RealtimeOutputModality::Audio,
        prompt: Some(Some("be helpful".to_string())),
        realtime_session_id: Some("conv_1".to_string()),
        transport: None,
        voice: None,
    });
    let webrtc_start = Op::RealtimeConversationStart(ConversationStartParams {
        output_modality: RealtimeOutputModality::Audio,
        prompt: Some(Some("be helpful".to_string())),
        realtime_session_id: Some("conv_1".to_string()),
        transport: Some(ConversationStartTransport::Webrtc {
            sdp: "v=offer\r\n".to_string(),
        }),
        voice: Some(RealtimeVoice::Cove),
    });
    let text = Op::RealtimeConversationText(ConversationTextParams {
        text: "hello".to_string(),
    });
    let close = Op::RealtimeConversationClose;
    let default_prompt_start = Op::RealtimeConversationStart(ConversationStartParams {
        output_modality: RealtimeOutputModality::Audio,
        prompt: None,
        realtime_session_id: None,
        transport: None,
        voice: None,
    });
    let null_prompt_start = Op::RealtimeConversationStart(ConversationStartParams {
        output_modality: RealtimeOutputModality::Audio,
        prompt: Some(None),
        realtime_session_id: None,
        transport: None,
        voice: None,
    });
    let list_voices = Op::RealtimeConversationListVoices;

    assert_eq!(
        serde_json::to_value(&start).unwrap(),
        json!({
            "type": "realtime_conversation_start",
            "output_modality": "audio",
            "prompt": "be helpful",
            "realtime_session_id": "conv_1"
        })
    );
    assert_eq!(
        serde_json::to_value(&default_prompt_start).unwrap(),
        json!({
            "type": "realtime_conversation_start",
            "output_modality": "audio"
        })
    );
    assert_eq!(
        serde_json::to_value(&null_prompt_start).unwrap(),
        json!({
            "type": "realtime_conversation_start",
            "output_modality": "audio",
            "prompt": null
        })
    );
    assert_eq!(
        serde_json::from_value::<Op>(json!({
            "type": "realtime_conversation_start",
            "output_modality": "audio"
        }))
        .unwrap(),
        default_prompt_start
    );
    assert_eq!(
        serde_json::from_value::<Op>(json!({
            "type": "realtime_conversation_start",
            "output_modality": "audio",
            "prompt": null
        }))
        .unwrap(),
        null_prompt_start
    );
    assert_eq!(
        serde_json::to_value(&audio).unwrap(),
        json!({
            "type": "realtime_conversation_audio",
            "frame": {
                "data": "AQID",
                "sample_rate": 24000,
                "num_channels": 1,
                "samples_per_channel": 480
            }
        })
    );
    assert_eq!(
        serde_json::from_value::<Op>(serde_json::to_value(&text).unwrap()).unwrap(),
        text
    );
    assert_eq!(
        serde_json::to_value(&close).unwrap(),
        json!({
            "type": "realtime_conversation_close"
        })
    );
    assert_eq!(
        serde_json::from_value::<Op>(serde_json::to_value(&close).unwrap()).unwrap(),
        close
    );
    assert_eq!(
        serde_json::to_value(&list_voices).unwrap(),
        json!({
            "type": "realtime_conversation_list_voices"
        })
    );
    assert_eq!(
        serde_json::from_value::<Op>(serde_json::to_value(&list_voices).unwrap()).unwrap(),
        list_voices
    );
    assert_eq!(
        serde_json::to_value(&webrtc_start).unwrap(),
        json!({
            "type": "realtime_conversation_start",
            "output_modality": "audio",
            "prompt": "be helpful",
            "realtime_session_id": "conv_1",
            "transport": {
                "type": "webrtc",
                "sdp": "v=offer\r\n"
            },
            "voice": "cove"
        })
    );
}

#[test]
fn realtime_conversation_started_event_uses_realtime_session_id() {
    let event = RealtimeConversationStartedEvent {
        realtime_session_id: Some("conv_1".to_string()),
        version: RealtimeConversationVersion::V2,
    };

    assert_eq!(
        serde_json::to_value(&event).unwrap(),
        json!({
            "realtime_session_id": "conv_1",
            "version": "v2"
        })
    );
}

#[test]
fn realtime_voice_list_is_stable() {
    assert_eq!(
        RealtimeVoicesList::builtin(),
        RealtimeVoicesList {
            v1: vec![
                RealtimeVoice::Juniper,
                RealtimeVoice::Maple,
                RealtimeVoice::Spruce,
                RealtimeVoice::Ember,
                RealtimeVoice::Vale,
                RealtimeVoice::Breeze,
                RealtimeVoice::Arbor,
                RealtimeVoice::Sol,
                RealtimeVoice::Cove,
            ],
            v2: vec![
                RealtimeVoice::Alloy,
                RealtimeVoice::Ash,
                RealtimeVoice::Ballad,
                RealtimeVoice::Coral,
                RealtimeVoice::Echo,
                RealtimeVoice::Sage,
                RealtimeVoice::Shimmer,
                RealtimeVoice::Verse,
                RealtimeVoice::Marin,
                RealtimeVoice::Cedar,
            ],
            default_v1: RealtimeVoice::Cove,
            default_v2: RealtimeVoice::Marin,
        }
    );
}

#[test]
fn user_input_serialization_omits_final_output_json_schema_when_none() -> Result<()> {
    let op = Op::UserInput {
        environments: None,
        items: Vec::new(),
        final_output_json_schema: None,
        responsesapi_client_metadata: None,
    };

    let json_op = serde_json::to_value(op)?;
    assert_eq!(json_op, json!({ "type": "user_input", "items": [] }));

    Ok(())
}

#[test]
fn user_input_deserializes_without_final_output_json_schema_field() -> Result<()> {
    let op: Op = serde_json::from_value(json!({ "type": "user_input", "items": [] }))?;

    assert_eq!(
        op,
        Op::UserInput {
            environments: None,
            items: Vec::new(),
            final_output_json_schema: None,
            responsesapi_client_metadata: None,
        }
    );

    Ok(())
}

#[test]
fn user_input_serialization_includes_final_output_json_schema_when_some() -> Result<()> {
    let schema = json!({
        "type": "object",
        "properties": {
            "answer": { "type": "string" }
        },
        "required": ["answer"],
        "additionalProperties": false
    });
    let op = Op::UserInput {
        environments: None,
        items: Vec::new(),
        final_output_json_schema: Some(schema.clone()),
        responsesapi_client_metadata: None,
    };

    let json_op = serde_json::to_value(op)?;
    assert_eq!(
        json_op,
        json!({
            "type": "user_input",
            "items": [],
            "final_output_json_schema": schema,
        })
    );

    Ok(())
}

#[test]
fn user_input_with_responsesapi_client_metadata_round_trips() -> Result<()> {
    let op = Op::UserInput {
        environments: None,
        items: Vec::new(),
        final_output_json_schema: None,
        responsesapi_client_metadata: Some(HashMap::from([(
            "fiber_run_id".to_string(),
            "fiber-123".to_string(),
        )])),
    };

    let json_op = serde_json::to_value(&op)?;
    assert_eq!(
        json_op,
        json!({
            "type": "user_input",
            "items": [],
            "responsesapi_client_metadata": {
                "fiber_run_id": "fiber-123",
            }
        })
    );
    assert_eq!(serde_json::from_value::<Op>(json_op)?, op);

    Ok(())
}

#[test]
fn user_input_with_turn_context_deserializes_without_selected_snippet_handoff() -> Result<()> {
    let op: Op = serde_json::from_value(json!({
        "type": "user_input_with_turn_context",
        "items": []
    }))?;

    let Op::UserInputWithTurnContext {
        context_recall_selected_snippets,
        ..
    } = op
    else {
        panic!("expected user_input_with_turn_context");
    };
    assert_eq!(context_recall_selected_snippets, None);

    Ok(())
}

#[test]
fn user_input_with_turn_context_serializes_selected_snippet_handoff() -> Result<()> {
    let selected_snippets = test_selected_snippet_envelope();
    let op = Op::UserInputWithTurnContext {
        environments: None,
        items: Vec::new(),
        final_output_json_schema: None,
        responsesapi_client_metadata: None,
        context_recall_selected_snippets: Some(selected_snippets.clone()),
        cwd: None,
        workspace_roots: None,
        profile_workspace_roots: None,
        approval_policy: None,
        approvals_reviewer: None,
        sandbox_policy: None,
        permission_profile: None,
        active_permission_profile: None,
        windows_sandbox_level: None,
        model: None,
        effort: None,
        summary: None,
        service_tier: None,
        collaboration_mode: None,
        personality: None,
    };

    let json_op = serde_json::to_value(&op)?;
    assert_eq!(
        json_op["context_recall_selected_snippets"]["selected_snippet_count"],
        1
    );
    assert_eq!(
        json_op["context_recall_selected_snippets"]["snippets"][0]["text"],
        "[redacted-query] bounded memory"
    );
    assert!(
        json_op["context_recall_selected_snippets"]["snippets"][0]
            .get("source_id")
            .is_none()
    );
    assert_eq!(serde_json::from_value::<Op>(json_op)?, op);
    assert!(selected_snippets.has_shadow_integrity());

    Ok(())
}

#[test]
fn user_input_text_serializes_empty_text_elements() -> Result<()> {
    let input = UserInput::Text {
        text: "hello".to_string(),
        text_elements: Vec::new(),
    };

    let json_input = serde_json::to_value(input)?;
    assert_eq!(
        json_input,
        json!({
            "type": "text",
            "text": "hello",
            "text_elements": [],
        })
    );

    Ok(())
}

#[test]
fn user_message_event_serializes_empty_metadata_vectors() -> Result<()> {
    let event = UserMessageEvent {
        message: "hello".to_string(),
        images: None,
        local_images: Vec::new(),
        text_elements: Vec::new(),
        ..Default::default()
    };

    let json_event = serde_json::to_value(event)?;
    assert_eq!(
        json_event,
        json!({
            "message": "hello",
            "local_images": [],
            "text_elements": [],
        })
    );

    Ok(())
}

#[test]
fn user_message_event_deserializes_without_image_detail_fields() -> Result<()> {
    let event: UserMessageEvent = serde_json::from_value(json!({
        "message": "hello",
        "images": ["https://example.com/image.png"],
        "local_images": ["/tmp/local.png"],
        "text_elements": [],
    }))?;

    assert_eq!(event.message, "hello");
    assert_eq!(
        event.images,
        Some(vec!["https://example.com/image.png".to_string()])
    );
    assert_eq!(event.image_details, Vec::<Option<ImageDetail>>::new());
    assert_eq!(event.local_images, vec![PathBuf::from("/tmp/local.png")]);
    assert_eq!(event.local_image_details, Vec::<Option<ImageDetail>>::new());
    assert_eq!(event.text_elements, Vec::new());

    Ok(())
}

#[test]
fn user_message_item_legacy_event_preserves_image_details() {
    let local_path = PathBuf::from("/tmp/local.png");
    let item = UserMessageItem::new(&[
        crate::user_input::UserInput::Image {
            image_url: "https://example.com/first.png".to_string(),
            detail: Some(ImageDetail::Original),
        },
        crate::user_input::UserInput::Image {
            image_url: "https://example.com/second.png".to_string(),
            detail: None,
        },
        crate::user_input::UserInput::LocalImage {
            path: local_path.clone(),
            detail: Some(ImageDetail::Original),
        },
    ]);

    let EventMsg::UserMessage(event) = item.as_legacy_event() else {
        panic!("expected user message event");
    };

    assert_eq!(
        event.images,
        Some(vec![
            "https://example.com/first.png".to_string(),
            "https://example.com/second.png".to_string(),
        ])
    );
    assert_eq!(event.image_details, vec![Some(ImageDetail::Original)]);
    assert_eq!(event.local_images, vec![local_path]);
    assert_eq!(event.local_image_details, vec![Some(ImageDetail::Original)]);
}

#[test]
fn turn_aborted_event_deserializes_without_turn_id() -> Result<()> {
    let event: EventMsg = serde_json::from_value(json!({
        "type": "turn_aborted",
        "reason": "interrupted",
    }))?;

    match event {
        EventMsg::TurnAborted(TurnAbortedEvent {
            started_at: None,
            turn_id,
            reason,
            ..
        }) => {
            assert_eq!(turn_id, None);
            assert_eq!(reason, TurnAbortReason::Interrupted);
        }
        _ => panic!("expected turn_aborted event"),
    }

    Ok(())
}

#[test]
fn turn_context_item_deserializes_without_network() -> Result<()> {
    let item: TurnContextItem = serde_json::from_value(json!({
        "cwd": test_path_buf("/tmp"),
        "approval_policy": "never",
        "sandbox_policy": { "type": "danger-full-access" },
        "model": "gpt-5",
        "summary": "auto",
    }))?;

    assert_eq!(item.trace_id, None);
    assert_eq!(item.network, None);
    assert_eq!(item.file_system_sandbox_policy, None);
    assert_eq!(item.context_manifest, None);
    Ok(())
}

#[test]
fn turn_context_item_serializes_network_when_present() -> Result<()> {
    let item = TurnContextItem {
        turn_id: None,
        trace_id: None,
        cwd: test_path_buf("/tmp"),
        current_date: None,
        timezone: None,
        approval_policy: AskForApproval::Never,
        sandbox_policy: SandboxPolicy::DangerFullAccess,
        permission_profile: None,
        network: Some(TurnContextNetworkItem {
            allowed_domains: vec!["api.example.com".to_string()],
            denied_domains: vec!["blocked.example.com".to_string()],
        }),
        file_system_sandbox_policy: Some(FileSystemSandboxPolicy::restricted(vec![
            FileSystemSandboxEntry {
                path: FileSystemPath::GlobPattern {
                    pattern: "/tmp/private/**/*.txt".to_string(),
                },
                access: FileSystemAccessMode::None,
            },
        ])),
        model: "gpt-5".to_string(),
        personality: None,
        collaboration_mode: None,
        realtime_active: None,
        effort: None,
        summary: ReasoningSummaryConfig::Auto,
        user_instructions: None,
        developer_instructions: None,
        final_output_json_schema: None,
        truncation_policy: None,
        context_manifest: None,
    };

    let value = serde_json::to_value(item)?;
    assert_eq!(
        value["network"],
        json!({
            "allowed_domains": ["api.example.com"],
            "denied_domains": ["blocked.example.com"],
        })
    );
    assert_eq!(
        value["file_system_sandbox_policy"],
        json!({
            "kind": "restricted",
            "entries": [{
                "path": {
                    "type": "glob_pattern",
                    "pattern": "/tmp/private/**/*.txt"
                },
                "access": "none"
            }]
        })
    );
    Ok(())
}

#[test]
fn turn_context_manifest_entry_tier_is_backward_compatible() -> Result<()> {
    let legacy_entry: TurnContextManifestEntry = serde_json::from_value(json!({
        "role": "developer",
        "source": "initial_context:permissions:0",
        "replay_key": "initial_context:permissions:0:0123456789abcdef",
        "text_hash": "0123456789abcdef",
        "estimated_tokens": 3,
    }))?;
    assert_eq!(legacy_entry.tier, TurnContextTier::Unknown);
    assert!(serde_json::to_value(&legacy_entry)?.get("tier").is_none());

    let mut legacy_manifest = TurnContextManifestItem {
        version: TURN_CONTEXT_MANIFEST_VERSION,
        estimated_tokens: 3,
        ledger_hash: None,
        budget_tokens: None,
        omitted_entries: 0,
        omitted_sources: Vec::new(),
        truncated: false,
        decision_ledger_hash: None,
        decision_ledger: Vec::new(),
        recall_selection: None,
        recall_selected_snippets: None,
        memory_taxonomy: Vec::new(),
        memory_formation_receipts: Vec::new(),
        memory_temporal_facts: Vec::new(),
        compression_candidates: Vec::new(),
        adaptive_budget_allocations: Vec::new(),
        compression_stages: Vec::new(),
        entries: vec![legacy_entry],
    }
    .with_refreshed_ledger_hash();
    assert!(legacy_manifest.has_replay_integrity());
    let legacy_hash = legacy_manifest
        .ledger_hash
        .clone()
        .expect("legacy hash should be materialized");

    legacy_manifest.entries[0].tier = TurnContextTier::System;
    legacy_manifest.refresh_ledger_hash();
    assert!(legacy_manifest.has_replay_integrity());
    assert_ne!(
        legacy_manifest.ledger_hash.as_deref(),
        Some(legacy_hash.as_str())
    );
    assert_eq!(
        serde_json::to_value(&legacy_manifest.entries[0])?["tier"],
        "system"
    );
    assert!(
        serde_json::to_value(&legacy_manifest)?
            .get("compression_candidates")
            .is_none()
    );
    assert!(
        serde_json::to_value(&legacy_manifest)?
            .get("compression_stages")
            .is_none()
    );

    Ok(())
}

#[test]
fn turn_context_manifest_hashes_are_replay_hashes_not_trust_digests() {
    let replay_hash = stable_turn_context_manifest_replay_hash("text:payload-light\n");
    assert_eq!(replay_hash.len(), 16);
    assert!(is_stable_manifest_replay_hash(&replay_hash));
    assert_eq!(
        stable_turn_context_manifest_text_hash("text:payload-light\n"),
        replay_hash
    );

    let sha256_shaped_digest = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    assert_eq!(sha256_shaped_digest.len(), 64);
    assert!(!is_stable_manifest_replay_hash(sha256_shaped_digest));
}

#[test]
fn turn_context_decision_entry_constructors_preserve_legacy_wire_strings() {
    let entries = vec![
        TurnContextDecisionEntry::included(
            "turn_context:developer:permissions:0",
            "always_include_safety_policy",
            Some("aaaaaaaaaaaaaaaa".to_string()),
        ),
        TurnContextDecisionEntry::policy(
            "turn_context:assembly_policy",
            "source_aware_omission",
            "budget_exceeded",
            None,
        ),
        TurnContextDecisionEntry::candidate_omit(
            "turn_context:developer:available_plugins:0:2",
            "available_plugins",
            20,
            11,
            None,
        ),
        TurnContextDecisionEntry::candidate_truncate(
            "turn_context:developer:selected_context_recall:0",
            "selected_context_recall",
            4,
            13,
            None,
        ),
        TurnContextDecisionEntry::omitted("turn_context:developer:apps:0:3", "apps", 30, 9, None),
        TurnContextDecisionEntry::truncated(
            "turn_context:developer:selected_context_recall:0",
            "selected_context_recall",
            24,
            3,
            None,
        ),
    ];
    let expected = [
        "included:always_include_safety_policy",
        "policy:source_aware_omission:budget_exceeded",
        "candidate_omit:available_plugins:priority:20:tokens:11",
        "candidate_truncate:selected_context_recall:remaining_over_budget:4:tokens:13",
        "omitted:apps:priority:30:tokens:9",
        "truncated:selected_context_recall:original_tokens:24:tokens:3",
    ];

    for (entry, expected_decision) in entries.iter().zip(expected) {
        assert_eq!(entry.decision, expected_decision);
        let kind = entry.kind();
        assert_eq!(
            kind.schema_version(),
            Some(TURN_CONTEXT_DECISION_SCHEMA_VERSION)
        );
        assert_eq!(kind.to_legacy_decision_string(), expected_decision);
    }
    assert_eq!(entries[0].reason_hash.as_deref(), Some("aaaaaaaaaaaaaaaa"));
    assert!(entries[3].kind().is_candidate_truncation());

    let summary = summarize_turn_context_decision_ledger(&entries);
    assert_eq!(summary.schema_version, TURN_CONTEXT_DECISION_SCHEMA_VERSION);
    assert_eq!(summary.known_count(), 6);
    assert_eq!(summary.included_count, 1);
    assert_eq!(summary.policy_count, 1);
    assert_eq!(summary.candidate_omit_count, 1);
    assert_eq!(summary.candidate_truncate_count, 1);
    assert_eq!(summary.omitted_count, 1);
    assert_eq!(summary.truncated_count, 1);
    assert_eq!(summary.unknown_count, 0);

    let unknown = TurnContextDecisionKind::Unknown {
        raw: "legacy:custom".to_string(),
    };
    assert_eq!(unknown.schema_version(), None);
    assert_eq!(unknown.to_legacy_decision_string(), "legacy:custom");

    let unknown_entry = TurnContextDecisionEntry::from_kind("turn_context:legacy", unknown, None);
    let mixed_summary =
        summarize_turn_context_decision_ledger(&[entries[0].clone(), unknown_entry.clone()]);
    assert_eq!(
        mixed_summary.schema_version,
        TURN_CONTEXT_DECISION_SCHEMA_VERSION
    );
    assert_eq!(mixed_summary.known_count(), 1);
    assert_eq!(mixed_summary.unknown_count, 1);

    let unknown_only_summary = summarize_turn_context_decision_ledger(&[unknown_entry]);
    assert_eq!(unknown_only_summary.schema_version, 0);
    assert_eq!(unknown_only_summary.known_count(), 0);
    assert_eq!(unknown_only_summary.unknown_count, 1);
}

#[test]
fn turn_context_manifest_compression_candidates_are_payload_light_and_hashed() -> Result<()> {
    let mut manifest = TurnContextManifestItem {
        version: TURN_CONTEXT_MANIFEST_VERSION,
        estimated_tokens: 12,
        ledger_hash: None,
        budget_tokens: Some(8),
        omitted_entries: 0,
        omitted_sources: Vec::new(),
        truncated: false,
        decision_ledger_hash: None,
        decision_ledger: Vec::new(),
        recall_selection: None,
        recall_selected_snippets: None,
        memory_taxonomy: Vec::new(),
        memory_formation_receipts: Vec::new(),
        memory_temporal_facts: Vec::new(),
        compression_candidates: vec![
            TurnContextCompressionCandidate {
                kind: TurnContextCompressionStageKind::Summary,
                tier: TurnContextTier::RetrievedSnippets,
                source_id: "selected_context_recall".into(),
                input_tokens: 40,
                estimated_output_tokens: 12,
                affected_entries: 1,
                not_executed_reason: TurnContextCompressionCandidateReason::BudgetPressureDryRun,
            },
            TurnContextCompressionCandidate {
                kind: TurnContextCompressionStageKind::Prune,
                tier: TurnContextTier::Tool,
                source_id: "extension_developer_capabilities".into(),
                input_tokens: 12,
                estimated_output_tokens: 6,
                affected_entries: 1,
                not_executed_reason: TurnContextCompressionCandidateReason::BudgetPressureDryRun,
            },
        ],
        adaptive_budget_allocations: Vec::new(),
        compression_stages: Vec::new(),
        entries: vec![TurnContextManifestEntry {
            role: "developer".into(),
            tier: TurnContextTier::RetrievedSnippets,
            source: "turn_context:developer:selected_context_recall:0".into(),
            replay_key: "turn_context:developer:selected_context_recall:0:0123456789abcdef".into(),
            text_hash: "0123456789abcdef".into(),
            estimated_tokens: 12,
        }],
    }
    .with_refreshed_ledger_hash();

    let value = serde_json::to_value(&manifest)?;

    assert_eq!(
        manifest.compression_candidates[0].schema_version(),
        Some(TURN_CONTEXT_COMPRESSION_CANDIDATE_SCHEMA_VERSION)
    );
    assert_eq!(value["compression_candidates"][0]["kind"], "summary");
    assert_eq!(
        value["compression_candidates"][0]["tier"],
        "retrieved_snippets"
    );
    assert_eq!(
        value["compression_candidates"][0]["source_id"],
        "selected_context_recall"
    );
    assert_eq!(value["compression_candidates"][0]["input_tokens"], 40);
    assert_eq!(
        value["compression_candidates"][0]["estimated_output_tokens"],
        12
    );
    assert_eq!(
        value["compression_candidates"][0]["not_executed_reason"],
        "budget_pressure_dry_run"
    );
    assert!(value["compression_candidates"][0].get("source").is_none());
    assert!(value["compression_candidates"][0].get("text").is_none());
    assert!(value["compression_candidates"][0].get("query").is_none());
    assert!(manifest.compression_candidates_have_integrity());
    assert!(manifest.has_replay_integrity());

    let original_ledger_hash = manifest
        .ledger_hash
        .clone()
        .expect("ledger hash should be materialized");
    manifest.compression_candidates[0].estimated_output_tokens = 11;
    assert!(!manifest.ledger_hash_matches_manifest());
    manifest.refresh_ledger_hash();
    assert_ne!(
        manifest.ledger_hash.as_deref(),
        Some(original_ledger_hash.as_str())
    );
    assert!(manifest.has_replay_integrity());

    manifest.compression_candidates[0].estimated_output_tokens = 41;
    manifest.refresh_ledger_hash();
    assert!(!manifest.compression_candidates_have_integrity());
    assert!(!manifest.has_replay_integrity());

    manifest.compression_candidates[0].estimated_output_tokens = 11;
    manifest.compression_candidates[1].source_id = "turn_context:developer:raw:0".into();
    manifest.refresh_ledger_hash();
    assert!(!manifest.compression_candidates_have_integrity());
    assert!(!manifest.has_replay_integrity());

    manifest.compression_candidates[1].source_id = "extension_developer_capabilities".into();
    manifest.compression_candidates[1].not_executed_reason =
        TurnContextCompressionCandidateReason::Unknown;
    manifest.refresh_ledger_hash();
    assert!(!manifest.compression_candidates_have_integrity());
    assert!(!manifest.has_replay_integrity());

    Ok(())
}

#[test]
fn turn_context_manifest_adaptive_budget_allocations_are_payload_light_and_hashed() -> Result<()> {
    let mut manifest = TurnContextManifestItem {
        version: TURN_CONTEXT_MANIFEST_VERSION,
        estimated_tokens: 52,
        ledger_hash: None,
        budget_tokens: Some(24),
        omitted_entries: 0,
        omitted_sources: Vec::new(),
        truncated: false,
        decision_ledger_hash: None,
        decision_ledger: Vec::new(),
        recall_selection: None,
        recall_selected_snippets: None,
        memory_taxonomy: Vec::new(),
        memory_formation_receipts: Vec::new(),
        memory_temporal_facts: Vec::new(),
        compression_candidates: Vec::new(),
        adaptive_budget_allocations: vec![
            TurnContextAdaptiveBudgetAllocation {
                tier: TurnContextTier::RetrievedSnippets,
                source_id: "selected_context_recall".into(),
                budget_class: "bounded_recall".into(),
                input_tokens: 40,
                reserve_tokens: 16,
                proposed_budget_tokens: 16,
                overflow_tokens: 24,
                omit_priority: Some(50),
                compression_kind: Some(TurnContextCompressionStageKind::Summary),
                estimated_compressed_tokens: Some(16),
                current_heuristic_action: TurnContextBudgetAllocationAction::Drop,
                proposed_action: TurnContextBudgetAllocationAction::Compress,
                would_drop: false,
                would_compress: true,
            },
            TurnContextAdaptiveBudgetAllocation {
                tier: TurnContextTier::Tool,
                source_id: "available_plugins".into(),
                budget_class: "tool_inventory".into(),
                input_tokens: 12,
                reserve_tokens: 9,
                proposed_budget_tokens: 8,
                overflow_tokens: 4,
                omit_priority: Some(20),
                compression_kind: Some(TurnContextCompressionStageKind::Defragment),
                estimated_compressed_tokens: Some(9),
                current_heuristic_action: TurnContextBudgetAllocationAction::Drop,
                proposed_action: TurnContextBudgetAllocationAction::Compress,
                would_drop: false,
                would_compress: true,
            },
        ],
        compression_stages: Vec::new(),
        entries: vec![TurnContextManifestEntry {
            role: "developer".into(),
            tier: TurnContextTier::RetrievedSnippets,
            source: "turn_context:developer:selected_context_recall:0".into(),
            replay_key: "turn_context:developer:selected_context_recall:0:0123456789abcdef".into(),
            text_hash: "0123456789abcdef".into(),
            estimated_tokens: 40,
        }],
    }
    .with_refreshed_ledger_hash();

    let value = serde_json::to_value(&manifest)?;

    assert_eq!(
        manifest.adaptive_budget_allocations[0].schema_version(),
        Some(TURN_CONTEXT_ADAPTIVE_BUDGET_ALLOCATION_SCHEMA_VERSION)
    );
    assert_eq!(
        value["adaptive_budget_allocations"][0]["source_id"],
        "selected_context_recall"
    );
    assert_eq!(
        value["adaptive_budget_allocations"][0]["budget_class"],
        "bounded_recall"
    );
    assert_eq!(
        value["adaptive_budget_allocations"][0]["compression_kind"],
        "summary"
    );
    assert_eq!(
        value["adaptive_budget_allocations"][0]["proposed_action"],
        "compress"
    );
    assert_eq!(
        value["adaptive_budget_allocations"][0]["would_compress"],
        true
    );
    assert!(
        value["adaptive_budget_allocations"][0]
            .get("source")
            .is_none()
    );
    assert!(
        value["adaptive_budget_allocations"][0]
            .get("text")
            .is_none()
    );
    assert!(
        value["adaptive_budget_allocations"][0]
            .get("query")
            .is_none()
    );
    assert!(manifest.adaptive_budget_allocations_have_integrity());
    assert!(manifest.has_replay_integrity());

    let original_ledger_hash = manifest
        .ledger_hash
        .clone()
        .expect("ledger hash should be materialized");
    manifest.adaptive_budget_allocations[0].proposed_budget_tokens = 15;
    assert!(!manifest.ledger_hash_matches_manifest());
    manifest.adaptive_budget_allocations[0].overflow_tokens = 25;
    manifest.refresh_ledger_hash();
    assert_ne!(
        manifest.ledger_hash.as_deref(),
        Some(original_ledger_hash.as_str())
    );
    assert!(manifest.has_replay_integrity());

    manifest.adaptive_budget_allocations[0].overflow_tokens = 24;
    manifest.refresh_ledger_hash();
    assert!(!manifest.adaptive_budget_allocations_have_integrity());
    assert!(!manifest.has_replay_integrity());

    manifest.adaptive_budget_allocations[0].overflow_tokens = 25;
    manifest.adaptive_budget_allocations[1].source_id = "turn_context:developer:raw:0".into();
    manifest.refresh_ledger_hash();
    assert!(!manifest.adaptive_budget_allocations_have_integrity());
    assert!(!manifest.has_replay_integrity());

    Ok(())
}

#[test]
fn turn_context_manifest_memory_taxonomy_is_payload_light_and_hashed() -> Result<()> {
    let mut manifest = TurnContextManifestItem {
        version: TURN_CONTEXT_MANIFEST_VERSION,
        estimated_tokens: 34,
        ledger_hash: None,
        budget_tokens: Some(24),
        omitted_entries: 0,
        omitted_sources: Vec::new(),
        truncated: false,
        decision_ledger_hash: None,
        decision_ledger: Vec::new(),
        recall_selection: None,
        recall_selected_snippets: None,
        memory_taxonomy: vec![
            TurnContextMemoryTaxonomyBucket {
                class: TurnContextMemoryTaxonomyClass::Semantic,
                source_count: 1,
                returned_count: 2,
                available_count: 3,
                omitted_count: 1,
                provenance_span_count: 0,
            },
            TurnContextMemoryTaxonomyBucket {
                class: TurnContextMemoryTaxonomyClass::Episodic,
                source_count: 1,
                returned_count: 1,
                available_count: 1,
                omitted_count: 0,
                provenance_span_count: 0,
            },
            TurnContextMemoryTaxonomyBucket {
                class: TurnContextMemoryTaxonomyClass::Control,
                source_count: 1,
                returned_count: 0,
                available_count: 2,
                omitted_count: 2,
                provenance_span_count: 0,
            },
            TurnContextMemoryTaxonomyBucket {
                class: TurnContextMemoryTaxonomyClass::Transcript,
                source_count: 2,
                returned_count: 3,
                available_count: 5,
                omitted_count: 2,
                provenance_span_count: 2,
            },
        ],
        memory_formation_receipts: Vec::new(),
        memory_temporal_facts: Vec::new(),
        compression_candidates: Vec::new(),
        adaptive_budget_allocations: Vec::new(),
        compression_stages: Vec::new(),
        entries: vec![TurnContextManifestEntry {
            role: "developer".into(),
            tier: TurnContextTier::RetrievedSnippets,
            source: "turn_context:developer:selected_context_recall:0".into(),
            replay_key: "turn_context:developer:selected_context_recall:0:0123456789abcdef".into(),
            text_hash: "0123456789abcdef".into(),
            estimated_tokens: 34,
        }],
    }
    .with_refreshed_ledger_hash();

    let value = serde_json::to_value(&manifest)?;

    assert_eq!(
        manifest.memory_taxonomy[0].schema_version(),
        Some(TURN_CONTEXT_MEMORY_TAXONOMY_SCHEMA_VERSION)
    );
    assert_eq!(value["memory_taxonomy"][0]["class"], "semantic");
    assert_eq!(value["memory_taxonomy"][0]["source_count"], 1);
    assert_eq!(value["memory_taxonomy"][0]["returned_count"], 2);
    assert_eq!(value["memory_taxonomy"][0]["available_count"], 3);
    assert_eq!(value["memory_taxonomy"][0]["omitted_count"], 1);
    assert_eq!(value["memory_taxonomy"][3]["class"], "transcript");
    assert_eq!(value["memory_taxonomy"][3]["provenance_span_count"], 2);
    assert!(value["memory_taxonomy"][0].get("source_id").is_none());
    assert!(value["memory_taxonomy"][0].get("memory_id").is_none());
    assert!(value["memory_taxonomy"][0].get("text").is_none());
    assert!(value["memory_taxonomy"][0].get("query").is_none());
    assert!(manifest.memory_taxonomy_has_integrity());
    assert!(manifest.has_replay_integrity());

    let original_ledger_hash = manifest
        .ledger_hash
        .clone()
        .expect("ledger hash should be materialized");
    manifest.memory_taxonomy[0].returned_count = 1;
    assert!(!manifest.ledger_hash_matches_manifest());
    manifest.memory_taxonomy[0].omitted_count = 2;
    manifest.refresh_ledger_hash();
    assert_ne!(
        manifest.ledger_hash.as_deref(),
        Some(original_ledger_hash.as_str())
    );
    assert!(manifest.has_replay_integrity());

    manifest.memory_taxonomy[0].omitted_count = 1;
    manifest.refresh_ledger_hash();
    assert!(!manifest.memory_taxonomy_has_integrity());
    assert!(!manifest.has_replay_integrity());

    manifest.memory_taxonomy[0].omitted_count = 2;
    manifest.memory_taxonomy[1].class = TurnContextMemoryTaxonomyClass::Unknown;
    manifest.refresh_ledger_hash();
    assert!(!manifest.memory_taxonomy_has_integrity());
    assert!(!manifest.has_replay_integrity());

    Ok(())
}

#[test]
fn turn_context_manifest_memory_formation_receipts_are_payload_light_and_hashed() -> Result<()> {
    let mut manifest = TurnContextManifestItem {
        version: TURN_CONTEXT_MANIFEST_VERSION,
        estimated_tokens: 21,
        ledger_hash: None,
        budget_tokens: Some(30),
        omitted_entries: 0,
        omitted_sources: Vec::new(),
        truncated: false,
        decision_ledger_hash: None,
        decision_ledger: Vec::new(),
        recall_selection: None,
        recall_selected_snippets: None,
        memory_taxonomy: Vec::new(),
        memory_formation_receipts: vec![
            TurnContextMemoryFormationReceipt {
                candidate_type: TurnContextMemoryFormationCandidateType::Fact,
                transcript_span_count: 2,
                provenance_span_count: 2,
                confidence_basis_points: 6400,
                idempotency_key_hash: "0123456789abcdef".into(),
                privacy_class: "user_private".into(),
                queued_for_background: true,
                production_write: false,
            },
            TurnContextMemoryFormationReceipt {
                candidate_type: TurnContextMemoryFormationCandidateType::Summary,
                transcript_span_count: 2,
                provenance_span_count: 1,
                confidence_basis_points: 7000,
                idempotency_key_hash: "fedcba9876543210".into(),
                privacy_class: "user_private".into(),
                queued_for_background: true,
                production_write: false,
            },
        ],
        memory_temporal_facts: Vec::new(),
        compression_candidates: Vec::new(),
        adaptive_budget_allocations: Vec::new(),
        compression_stages: Vec::new(),
        entries: vec![TurnContextManifestEntry {
            role: "developer".into(),
            tier: TurnContextTier::RetrievedSnippets,
            source: "turn_context:developer:selected_context_recall:0".into(),
            replay_key: "turn_context:developer:selected_context_recall:0:0123456789abcdef".into(),
            text_hash: "0123456789abcdef".into(),
            estimated_tokens: 21,
        }],
    }
    .with_refreshed_ledger_hash();

    let value = serde_json::to_value(&manifest)?;

    assert_eq!(
        manifest.memory_formation_receipts[0].schema_version(),
        Some(TURN_CONTEXT_MEMORY_FORMATION_RECEIPT_SCHEMA_VERSION)
    );
    assert_eq!(
        value["memory_formation_receipts"][0]["candidate_type"],
        "fact"
    );
    assert_eq!(
        value["memory_formation_receipts"][0]["transcript_span_count"],
        2
    );
    assert_eq!(
        value["memory_formation_receipts"][0]["provenance_span_count"],
        2
    );
    assert_eq!(
        value["memory_formation_receipts"][0]["confidence_basis_points"],
        6400
    );
    assert_eq!(
        value["memory_formation_receipts"][0]["idempotency_key_hash"],
        "0123456789abcdef"
    );
    assert_eq!(
        value["memory_formation_receipts"][0]["privacy_class"],
        "user_private"
    );
    assert_eq!(
        value["memory_formation_receipts"][0]["queued_for_background"],
        true
    );
    assert!(
        value["memory_formation_receipts"][0]
            .get("production_write")
            .is_none()
    );
    assert!(
        value["memory_formation_receipts"][0]
            .get("transcript_text")
            .is_none()
    );
    assert!(
        value["memory_formation_receipts"][0]
            .get("memory_id")
            .is_none()
    );
    assert!(
        value["memory_formation_receipts"][0]
            .get("source_id")
            .is_none()
    );
    assert!(value["memory_formation_receipts"][0].get("query").is_none());
    assert!(manifest.memory_formation_receipts_have_integrity());
    assert!(manifest.has_replay_integrity());

    let original_ledger_hash = manifest
        .ledger_hash
        .clone()
        .expect("ledger hash should be materialized");
    manifest.memory_formation_receipts[0].confidence_basis_points = 6100;
    assert!(!manifest.ledger_hash_matches_manifest());
    manifest.refresh_ledger_hash();
    assert_ne!(
        manifest.ledger_hash.as_deref(),
        Some(original_ledger_hash.as_str())
    );
    assert!(manifest.has_replay_integrity());

    manifest.memory_formation_receipts[0].production_write = true;
    manifest.refresh_ledger_hash();
    assert!(!manifest.memory_formation_receipts_have_integrity());
    assert!(!manifest.has_replay_integrity());

    manifest.memory_formation_receipts[0].production_write = false;
    manifest.memory_formation_receipts[1].candidate_type =
        TurnContextMemoryFormationCandidateType::Unknown;
    manifest.refresh_ledger_hash();
    assert!(!manifest.memory_formation_receipts_have_integrity());
    assert!(!manifest.has_replay_integrity());

    manifest.memory_formation_receipts[1].candidate_type =
        TurnContextMemoryFormationCandidateType::Summary;
    manifest.memory_formation_receipts[1].idempotency_key_hash = "raw-key".into();
    manifest.refresh_ledger_hash();
    assert!(!manifest.memory_formation_receipts_have_integrity());
    assert!(!manifest.has_replay_integrity());

    Ok(())
}

#[test]
fn turn_context_manifest_memory_temporal_facts_are_payload_light_and_hashed() -> Result<()> {
    let mut manifest = TurnContextManifestItem {
        version: TURN_CONTEXT_MANIFEST_VERSION,
        estimated_tokens: 21,
        ledger_hash: None,
        budget_tokens: Some(24),
        omitted_entries: 0,
        omitted_sources: Vec::new(),
        truncated: false,
        decision_ledger_hash: None,
        decision_ledger: Vec::new(),
        recall_selection: None,
        recall_selected_snippets: None,
        memory_taxonomy: Vec::new(),
        memory_formation_receipts: Vec::new(),
        memory_temporal_facts: vec![
            TurnContextMemoryTemporalFact {
                fact_type: TurnContextMemoryTemporalFactType::Attribute,
                entity_hash: "0123456789abcdef".into(),
                provenance_span_count: 2,
                valid_from_sequence: 8,
                invalid_at_sequence: None,
                confidence_basis_points: 6200,
                supersedes_fact_hash: None,
                privacy_class: "user_private".into(),
                dry_run_only: true,
                production_write: false,
            },
            TurnContextMemoryTemporalFact {
                fact_type: TurnContextMemoryTemporalFactType::Summary,
                entity_hash: "fedcba9876543210".into(),
                provenance_span_count: 1,
                valid_from_sequence: 9,
                invalid_at_sequence: Some(12),
                confidence_basis_points: 7000,
                supersedes_fact_hash: Some("aaaaaaaaaaaaaaaa".into()),
                privacy_class: "user_private".into(),
                dry_run_only: true,
                production_write: false,
            },
        ],
        compression_candidates: Vec::new(),
        adaptive_budget_allocations: Vec::new(),
        compression_stages: Vec::new(),
        entries: vec![TurnContextManifestEntry {
            role: "developer".into(),
            tier: TurnContextTier::RetrievedSnippets,
            source: "turn_context:developer:selected_context_recall:0".into(),
            replay_key: "turn_context:developer:selected_context_recall:0:0123456789abcdef".into(),
            text_hash: "0123456789abcdef".into(),
            estimated_tokens: 21,
        }],
    }
    .with_refreshed_ledger_hash();

    let value = serde_json::to_value(&manifest)?;

    assert_eq!(
        manifest.memory_temporal_facts[0].schema_version(),
        Some(TURN_CONTEXT_MEMORY_TEMPORAL_FACT_SCHEMA_VERSION)
    );
    assert_eq!(value["memory_temporal_facts"][0]["fact_type"], "attribute");
    assert_eq!(
        value["memory_temporal_facts"][0]["entity_hash"],
        "0123456789abcdef"
    );
    assert_eq!(
        value["memory_temporal_facts"][0]["provenance_span_count"],
        2
    );
    assert_eq!(value["memory_temporal_facts"][0]["valid_from_sequence"], 8);
    assert_eq!(
        value["memory_temporal_facts"][0]["confidence_basis_points"],
        6200
    );
    assert_eq!(
        value["memory_temporal_facts"][1]["supersedes_fact_hash"],
        "aaaaaaaaaaaaaaaa"
    );
    assert!(value["memory_temporal_facts"][0].get("fact_text").is_none());
    assert!(
        value["memory_temporal_facts"][0]
            .get("transcript_text")
            .is_none()
    );
    assert!(
        value["memory_temporal_facts"][0]
            .get("memory_text")
            .is_none()
    );
    assert!(value["memory_temporal_facts"][0].get("source_id").is_none());
    assert!(value["memory_temporal_facts"][0].get("memory_id").is_none());
    assert!(value["memory_temporal_facts"][0].get("query").is_none());
    assert!(manifest.memory_temporal_facts_have_integrity());
    assert!(manifest.has_replay_integrity());

    let original_ledger_hash = manifest
        .ledger_hash
        .clone()
        .expect("ledger hash should be materialized");
    manifest.memory_temporal_facts[0].confidence_basis_points = 6100;
    assert!(!manifest.ledger_hash_matches_manifest());
    manifest.refresh_ledger_hash();
    assert_ne!(
        manifest.ledger_hash.as_deref(),
        Some(original_ledger_hash.as_str())
    );
    assert!(manifest.has_replay_integrity());

    manifest.memory_temporal_facts[0].production_write = true;
    manifest.refresh_ledger_hash();
    assert!(!manifest.memory_temporal_facts_have_integrity());
    assert!(!manifest.has_replay_integrity());

    manifest.memory_temporal_facts[0].production_write = false;
    manifest.memory_temporal_facts[1].fact_type = TurnContextMemoryTemporalFactType::Unknown;
    manifest.refresh_ledger_hash();
    assert!(!manifest.memory_temporal_facts_have_integrity());
    assert!(!manifest.has_replay_integrity());

    manifest.memory_temporal_facts[1].fact_type = TurnContextMemoryTemporalFactType::Summary;
    manifest.memory_temporal_facts[1].supersedes_fact_hash = Some("raw-fact-id".into());
    manifest.refresh_ledger_hash();
    assert!(!manifest.memory_temporal_facts_have_integrity());
    assert!(!manifest.has_replay_integrity());

    Ok(())
}

#[test]
fn turn_context_manifest_compression_stages_are_payload_light_and_hashed() -> Result<()> {
    assert_eq!(
        TurnContextCompressionStageKind::Summary.schema_version(),
        Some(TURN_CONTEXT_COMPRESSION_STAGE_SCHEMA_VERSION)
    );
    assert_eq!(
        TurnContextCompressionStageKind::Unknown.schema_version(),
        None
    );

    let mut manifest = TurnContextManifestItem {
        version: TURN_CONTEXT_MANIFEST_VERSION,
        estimated_tokens: 12,
        ledger_hash: None,
        budget_tokens: Some(16),
        omitted_entries: 0,
        omitted_sources: Vec::new(),
        truncated: false,
        decision_ledger_hash: None,
        decision_ledger: Vec::new(),
        recall_selection: None,
        recall_selected_snippets: None,
        memory_taxonomy: Vec::new(),
        memory_formation_receipts: Vec::new(),
        memory_temporal_facts: Vec::new(),
        compression_candidates: Vec::new(),
        adaptive_budget_allocations: Vec::new(),
        compression_stages: vec![
            TurnContextCompressionStage {
                kind: TurnContextCompressionStageKind::Summary,
                input_tokens: 40,
                output_tokens: 12,
                affected_entries: 2,
                loss_check_status: Some(TurnContextCompressionLossCheckStatus::MarkerBoundaryOnly),
                rollback_source_text_hash: Some("aaaaaaaaaaaaaaaa".into()),
                protected_tier_invariant: Some(
                    TurnContextCompressionProtectedTierInvariant::Preserved,
                ),
            },
            TurnContextCompressionStage {
                kind: TurnContextCompressionStageKind::Defragment,
                input_tokens: 12,
                output_tokens: 10,
                affected_entries: 1,
                loss_check_status: Some(TurnContextCompressionLossCheckStatus::MarkerBoundaryOnly),
                rollback_source_text_hash: Some("bbbbbbbbbbbbbbbb".into()),
                protected_tier_invariant: Some(
                    TurnContextCompressionProtectedTierInvariant::Preserved,
                ),
            },
        ],
        entries: vec![TurnContextManifestEntry {
            role: "developer".into(),
            tier: TurnContextTier::Summary,
            source: "turn_context:developer:summary:0".into(),
            replay_key: "turn_context:developer:summary:0:0123456789abcdef".into(),
            text_hash: "0123456789abcdef".into(),
            estimated_tokens: 12,
        }],
    }
    .with_refreshed_ledger_hash();

    let value = serde_json::to_value(&manifest)?;

    assert_eq!(value["compression_stages"][0]["kind"], "summary");
    assert_eq!(value["compression_stages"][0]["input_tokens"], 40);
    assert_eq!(value["compression_stages"][0]["output_tokens"], 12);
    assert_eq!(value["compression_stages"][0]["affected_entries"], 2);
    assert_eq!(
        value["compression_stages"][0]["loss_check_status"],
        "marker_boundary_only"
    );
    assert_eq!(
        value["compression_stages"][0]["rollback_source_text_hash"],
        "aaaaaaaaaaaaaaaa"
    );
    assert_eq!(
        value["compression_stages"][0]["protected_tier_invariant"],
        "preserved"
    );
    assert_eq!(value["compression_stages"][1]["kind"], "defragment");
    assert!(value["compression_stages"][0].get("source").is_none());
    assert!(value["compression_stages"][0].get("text").is_none());
    assert!(value["compression_stages"][0].get("query").is_none());
    assert!(manifest.compression_stages_have_integrity());
    assert!(manifest.has_replay_integrity());

    let original_ledger_hash = manifest
        .ledger_hash
        .clone()
        .expect("ledger hash should be materialized");
    manifest.compression_stages[0].loss_check_status =
        Some(TurnContextCompressionLossCheckStatus::SemanticLossCheckPassed);
    assert!(!manifest.ledger_hash_matches_manifest());
    manifest.refresh_ledger_hash();
    assert_ne!(
        manifest.ledger_hash.as_deref(),
        Some(original_ledger_hash.as_str())
    );
    assert!(manifest.has_replay_integrity());

    manifest.compression_stages[0].output_tokens = 11;
    assert!(!manifest.ledger_hash_matches_manifest());
    manifest.refresh_ledger_hash();
    assert!(manifest.has_replay_integrity());

    manifest.compression_stages[0].output_tokens = 41;
    manifest.refresh_ledger_hash();
    assert!(!manifest.compression_stages_have_integrity());
    assert!(!manifest.has_replay_integrity());

    manifest.compression_stages[0].output_tokens = 11;
    manifest.compression_stages[1].rollback_source_text_hash = Some("not-a-hash".into());
    manifest.refresh_ledger_hash();
    assert!(!manifest.compression_stages_have_integrity());
    assert!(!manifest.has_replay_integrity());

    manifest.compression_stages[1].rollback_source_text_hash = Some("bbbbbbbbbbbbbbbb".into());
    manifest.compression_stages[1].kind = TurnContextCompressionStageKind::Unknown;
    manifest.refresh_ledger_hash();
    assert!(!manifest.compression_stages_have_integrity());
    assert!(!manifest.has_replay_integrity());

    Ok(())
}

#[test]
fn turn_context_manifest_recall_selection_serializes_payload_light_rollup() -> Result<()> {
    let mut manifest = TurnContextManifestItem {
        version: TURN_CONTEXT_MANIFEST_VERSION,
        estimated_tokens: 3,
        ledger_hash: None,
        budget_tokens: Some(4),
        omitted_entries: 0,
        omitted_sources: Vec::new(),
        truncated: false,
        decision_ledger_hash: None,
        decision_ledger: Vec::new(),
        recall_selection: Some(TurnContextRecallSelectionSummary {
            returned_source_count: 4,
            selected_source_count: 3,
            ranked_source_count: 3,
            returned_unselected_source_count: 1,
            source_diversity_met: true,
            source_diversity_target: 3,
            max_per_source: 2,
            ranked_item_count: 3,
            omitted_by_budget_count: 1,
            memory_control_omitted_count: 2,
            low_trust_ranked_item_count: 1,
            low_recency_ranked_item_count: 2,
        }),
        recall_selected_snippets: Some(TurnContextRecallSelectedSnippetEnvelope {
            version: TURN_CONTEXT_RECALL_SELECTED_SNIPPET_ENVELOPE_VERSION,
            max_snippets: 4,
            max_snippet_chars: 120,
            selected_snippet_count: 1,
            omitted_snippet_count: 2,
            redacted_snippet_count: 1,
            truncated_snippet_count: 0,
            snippets: vec![TurnContextRecallSelectedSnippet {
                snippet_hash: "fedcba9876543210".into(),
                text: "[redacted-query] bounded memory".into(),
                estimated_tokens: 8,
                redacted: true,
                truncated: false,
            }],
            safety: TurnContextRecallSelectedSnippetSafety {
                ready_for_shadow_handoff: true,
                bounded: true,
                origin_identifiers_exposed: false,
                raw_ranked_payload_exposed: false,
                rank_explanation_exposed: false,
                control_marker_exposed: false,
                query_payload_exposed: false,
                per_origin_list_exposed: false,
            },
        }),
        memory_taxonomy: Vec::new(),
        memory_formation_receipts: Vec::new(),
        memory_temporal_facts: Vec::new(),
        compression_candidates: Vec::new(),
        adaptive_budget_allocations: Vec::new(),
        compression_stages: Vec::new(),
        entries: vec![TurnContextManifestEntry {
            role: "developer".into(),
            tier: TurnContextTier::System,
            source: "initial_context:permissions:0".into(),
            replay_key: "initial_context:permissions:0:0123456789abcdef".into(),
            text_hash: "0123456789abcdef".into(),
            estimated_tokens: 3,
        }],
    }
    .with_refreshed_ledger_hash();

    let value = serde_json::to_value(&manifest)?;

    assert_eq!(value["recall_selection"]["returned_source_count"], 4);
    assert_eq!(value["recall_selection"]["selected_source_count"], 3);
    assert_eq!(value["recall_selection"]["ranked_source_count"], 3);
    assert_eq!(
        value["recall_selection"]["returned_unselected_source_count"],
        1
    );
    assert_eq!(value["recall_selection"]["source_diversity_met"], true);
    assert_eq!(value["recall_selection"]["source_diversity_target"], 3);
    assert_eq!(value["recall_selection"]["max_per_source"], 2);
    assert_eq!(value["recall_selection"]["ranked_item_count"], 3);
    assert_eq!(value["recall_selection"]["omitted_by_budget_count"], 1);
    assert_eq!(value["recall_selection"]["memory_control_omitted_count"], 2);
    assert_eq!(value["recall_selection"]["low_trust_ranked_item_count"], 1);
    assert_eq!(
        value["recall_selection"]["low_recency_ranked_item_count"],
        2
    );
    assert!(value["recall_selection"].get("source_id").is_none());
    assert!(value["recall_selection"].get("summary").is_none());
    assert_eq!(
        value["recall_selected_snippets"]["selected_snippet_count"],
        1
    );
    assert_eq!(
        value["recall_selected_snippets"]["snippets"][0]["text"],
        "[redacted-query] bounded memory"
    );
    assert!(value["recall_selected_snippets"].get("source_id").is_none());
    assert!(
        value["recall_selected_snippets"]["snippets"][0]
            .get("source_memory_ids")
            .is_none()
    );
    assert!(
        manifest
            .recall_selection
            .as_ref()
            .expect("recall selection")
            .returned_unselected_source_count_matches()
    );
    assert!(manifest.recall_selection_has_integrity());
    assert!(manifest.recall_selected_snippets_have_integrity());
    assert!(manifest.has_replay_integrity());

    let original_ledger_hash = manifest
        .ledger_hash
        .clone()
        .expect("ledger hash should be materialized");
    manifest
        .recall_selection
        .as_mut()
        .expect("recall selection")
        .omitted_by_budget_count = 0;
    assert!(!manifest.ledger_hash_matches_manifest());
    manifest.refresh_ledger_hash();
    assert_ne!(
        manifest.ledger_hash.as_deref(),
        Some(original_ledger_hash.as_str())
    );
    assert!(manifest.has_replay_integrity());

    manifest
        .recall_selection
        .as_mut()
        .expect("recall selection")
        .ranked_item_count = 2;
    manifest.refresh_ledger_hash();
    assert!(!manifest.recall_selection_has_integrity());
    assert!(!manifest.has_replay_integrity());
    manifest
        .recall_selection
        .as_mut()
        .expect("recall selection")
        .ranked_item_count = 3;
    manifest.refresh_ledger_hash();
    assert!(manifest.recall_selection_has_integrity());
    assert!(manifest.has_replay_integrity());

    {
        let recall_selection = manifest
            .recall_selection
            .as_mut()
            .expect("recall selection");
        recall_selection.ranked_source_count = 0;
        recall_selection.low_trust_ranked_item_count = 0;
        recall_selection.low_recency_ranked_item_count = 0;
    }
    manifest.refresh_ledger_hash();
    assert!(!manifest.recall_selection_has_integrity());
    assert!(!manifest.has_replay_integrity());
    manifest
        .recall_selection
        .as_mut()
        .expect("recall selection")
        .ranked_item_count = 0;
    manifest.refresh_ledger_hash();
    assert!(manifest.recall_selection_has_integrity());
    assert!(manifest.has_replay_integrity());

    {
        let recall_selection = manifest
            .recall_selection
            .as_mut()
            .expect("recall selection");
        recall_selection.ranked_source_count = 3;
        recall_selection.ranked_item_count = 3;
        recall_selection.low_trust_ranked_item_count = 1;
        recall_selection.low_recency_ranked_item_count = 2;
        recall_selection.source_diversity_met = false;
    }
    manifest.refresh_ledger_hash();
    assert!(!manifest.recall_selection_has_integrity());
    assert!(!manifest.has_replay_integrity());
    {
        let recall_selection = manifest
            .recall_selection
            .as_mut()
            .expect("recall selection");
        recall_selection.source_diversity_target = 0;
    }
    manifest.refresh_ledger_hash();
    assert!(manifest.recall_selection_has_integrity());
    assert!(manifest.has_replay_integrity());

    manifest
        .recall_selected_snippets
        .as_mut()
        .expect("recall selected snippets")
        .selected_snippet_count = 2;
    manifest.refresh_ledger_hash();
    assert!(!manifest.recall_selected_snippets_have_integrity());
    assert!(!manifest.has_replay_integrity());
    manifest
        .recall_selected_snippets
        .as_mut()
        .expect("recall selected snippets")
        .selected_snippet_count = 1;
    manifest.refresh_ledger_hash();
    assert!(manifest.recall_selected_snippets_have_integrity());
    assert!(manifest.has_replay_integrity());

    manifest
        .recall_selected_snippets
        .as_mut()
        .expect("recall selected snippets")
        .safety
        .query_payload_exposed = true;
    manifest.refresh_ledger_hash();
    assert!(!manifest.recall_selected_snippets_have_integrity());
    assert!(!manifest.has_replay_integrity());
    manifest
        .recall_selected_snippets
        .as_mut()
        .expect("recall selected snippets")
        .safety
        .query_payload_exposed = false;
    manifest.refresh_ledger_hash();
    assert!(manifest.recall_selected_snippets_have_integrity());
    assert!(manifest.has_replay_integrity());

    Ok(())
}

#[test]
fn turn_context_manifest_selected_snippets_serializes_shadow_envelope() -> Result<()> {
    let mut manifest = TurnContextManifestItem {
        version: TURN_CONTEXT_MANIFEST_VERSION,
        estimated_tokens: 3,
        ledger_hash: None,
        budget_tokens: Some(4),
        omitted_entries: 0,
        omitted_sources: Vec::new(),
        truncated: false,
        decision_ledger_hash: None,
        decision_ledger: Vec::new(),
        recall_selection: None,
        recall_selected_snippets: Some(TurnContextRecallSelectedSnippetEnvelope {
            version: TURN_CONTEXT_RECALL_SELECTED_SNIPPET_ENVELOPE_VERSION,
            max_snippets: 4,
            max_snippet_chars: 120,
            selected_snippet_count: 1,
            omitted_snippet_count: 2,
            redacted_snippet_count: 1,
            truncated_snippet_count: 0,
            snippets: vec![TurnContextRecallSelectedSnippet {
                snippet_hash: "fedcba9876543210".into(),
                text: "[redacted-query] bounded memory".into(),
                estimated_tokens: 8,
                redacted: true,
                truncated: false,
            }],
            safety: TurnContextRecallSelectedSnippetSafety {
                ready_for_shadow_handoff: true,
                bounded: true,
                origin_identifiers_exposed: false,
                raw_ranked_payload_exposed: false,
                rank_explanation_exposed: false,
                control_marker_exposed: false,
                query_payload_exposed: false,
                per_origin_list_exposed: false,
            },
        }),
        memory_taxonomy: Vec::new(),
        memory_formation_receipts: Vec::new(),
        memory_temporal_facts: Vec::new(),
        compression_candidates: Vec::new(),
        adaptive_budget_allocations: Vec::new(),
        compression_stages: Vec::new(),
        entries: vec![TurnContextManifestEntry {
            role: "developer".into(),
            tier: TurnContextTier::System,
            source: "initial_context:permissions:0".into(),
            replay_key: "initial_context:permissions:0:0123456789abcdef".into(),
            text_hash: "0123456789abcdef".into(),
            estimated_tokens: 3,
        }],
    }
    .with_refreshed_ledger_hash();

    let value = serde_json::to_value(&manifest)?;

    assert_eq!(
        value["recall_selected_snippets"]["selected_snippet_count"],
        1
    );
    assert_eq!(
        value["recall_selected_snippets"]["snippets"][0]["text"],
        "[redacted-query] bounded memory"
    );
    assert!(
        value["recall_selected_snippets"]["snippets"][0]
            .get("source_id")
            .is_none()
    );
    assert!(manifest.recall_selected_snippets_have_integrity());
    assert!(manifest.has_replay_integrity());

    let original_ledger_hash = manifest
        .ledger_hash
        .clone()
        .expect("ledger hash should be materialized");
    manifest
        .recall_selected_snippets
        .as_mut()
        .expect("recall selected snippets")
        .snippets[0]
        .text = "[redacted-query] changed bounded memory".into();
    assert!(!manifest.ledger_hash_matches_manifest());
    manifest.refresh_ledger_hash();
    assert_ne!(
        manifest.ledger_hash.as_deref(),
        Some(original_ledger_hash.as_str())
    );
    assert!(manifest.has_replay_integrity());

    manifest
        .recall_selected_snippets
        .as_mut()
        .expect("recall selected snippets")
        .snippets[0]
        .text = "[hepta-memory:tombstone] leaked control marker".into();
    manifest.refresh_ledger_hash();
    assert!(!manifest.recall_selected_snippets_have_integrity());
    assert!(!manifest.has_replay_integrity());

    Ok(())
}

/// Serialize Event to verify that its JSON representation has the expected
/// amount of nesting.
#[test]
fn serialize_event() -> Result<()> {
    let session_id = SessionId::from_string("67e55044-10b1-426f-9247-bb680e5fe0c7")?;
    let thread_id = ThreadId::from_string("67e55044-10b1-426f-9247-bb680e5fe0c8")?;
    let rollout_file = NamedTempFile::new()?;
    let permission_profile = PermissionProfile::read_only();
    let event = Event {
        id: "1234".to_string(),
        msg: EventMsg::SessionConfigured(SessionConfiguredEvent {
            session_id,
            thread_id,
            forked_from_id: None,
            thread_source: None,
            thread_name: None,
            model: "codex-mini-latest".to_string(),
            model_provider_id: "openai".to_string(),
            service_tier: None,
            approval_policy: AskForApproval::Never,
            approvals_reviewer: ApprovalsReviewer::User,
            permission_profile: permission_profile.clone(),
            active_permission_profile: None,
            cwd: test_path_buf("/home/user/project").abs(),
            reasoning_effort: Some(ReasoningEffortConfig::default()),
            initial_messages: None,
            network_proxy: None,
            rollout_path: Some(rollout_file.path().to_path_buf()),
        }),
    };

    let expected = json!({
        "id": "1234",
        "msg": {
            "type": "session_configured",
            "session_id": "67e55044-10b1-426f-9247-bb680e5fe0c7",
            "thread_id": "67e55044-10b1-426f-9247-bb680e5fe0c8",
            "model": "codex-mini-latest",
            "model_provider_id": "openai",
            "approval_policy": "never",
            "approvals_reviewer": "user",
            "permission_profile": permission_profile,
            "cwd": test_path_buf("/home/user/project"),
            "reasoning_effort": "medium",
            "rollout_path": format!("{}", rollout_file.path().display()),
        }
    });
    assert_eq!(expected, serde_json::to_value(&event)?);
    Ok(())
}

#[test]
fn deserialize_legacy_session_configured_event_uses_sandbox_policy() -> Result<()> {
    let cwd = test_path_buf("/home/user/project");
    let value = json!({
        "session_id": "67e55044-10b1-426f-9247-bb680e5fe0c8",
        "model": "codex-mini-latest",
        "model_provider_id": "openai",
        "approval_policy": "never",
        "approvals_reviewer": "user",
        "sandbox_policy": {
            "type": "read-only"
        },
        "cwd": cwd,
    });

    let event: SessionConfiguredEvent = serde_json::from_value(value)?;
    assert_eq!(event.permission_profile, PermissionProfile::read_only());
    Ok(())
}

#[test]
fn vec_u8_as_base64_serialization_and_deserialization() -> Result<()> {
    let event = ExecCommandOutputDeltaEvent {
        call_id: "call21".to_string(),
        stream: ExecOutputStream::Stdout,
        chunk: vec![1, 2, 3, 4, 5],
    };
    let serialized = serde_json::to_string(&event)?;
    assert_eq!(
        r#"{"call_id":"call21","stream":"stdout","chunk":"AQIDBAU="}"#,
        serialized,
    );

    let deserialized: ExecCommandOutputDeltaEvent = serde_json::from_str(&serialized)?;
    assert_eq!(deserialized, event);
    Ok(())
}

#[test]
fn serialize_mcp_startup_update_event() -> Result<()> {
    let event = Event {
        id: "init".to_string(),
        msg: EventMsg::McpStartupUpdate(McpStartupUpdateEvent {
            server: "srv".to_string(),
            status: McpStartupStatus::Failed {
                error: "boom".to_string(),
            },
        }),
    };

    let value = serde_json::to_value(&event)?;
    assert_eq!(value["msg"]["type"], "mcp_startup_update");
    assert_eq!(value["msg"]["server"], "srv");
    assert_eq!(value["msg"]["status"]["state"], "failed");
    assert_eq!(value["msg"]["status"]["error"], "boom");
    Ok(())
}

#[test]
fn serialize_mcp_startup_complete_event() -> Result<()> {
    let event = Event {
        id: "init".to_string(),
        msg: EventMsg::McpStartupComplete(McpStartupCompleteEvent {
            ready: vec!["a".to_string()],
            failed: vec![McpStartupFailure {
                server: "b".to_string(),
                error: "bad".to_string(),
            }],
            cancelled: vec!["c".to_string()],
        }),
    };

    let value = serde_json::to_value(&event)?;
    assert_eq!(value["msg"]["type"], "mcp_startup_complete");
    assert_eq!(value["msg"]["ready"][0], "a");
    assert_eq!(value["msg"]["failed"][0]["server"], "b");
    assert_eq!(value["msg"]["failed"][0]["error"], "bad");
    assert_eq!(value["msg"]["cancelled"][0], "c");
    Ok(())
}

#[test]
fn token_usage_info_new_or_append_updates_context_window_when_provided() {
    let initial = Some(TokenUsageInfo {
        total_token_usage: TokenUsage::default(),
        last_token_usage: TokenUsage::default(),
        model_context_window: Some(258_400),
    });
    let last = Some(TokenUsage {
        input_tokens: 10,
        cached_input_tokens: 0,
        output_tokens: 0,
        reasoning_output_tokens: 0,
        total_tokens: 10,
    });

    let info = TokenUsageInfo::new_or_append(&initial, &last, Some(128_000))
        .expect("new_or_append should return info");

    assert_eq!(info.model_context_window, Some(128_000));
}

#[test]
fn token_usage_info_new_or_append_preserves_context_window_when_not_provided() {
    let initial = Some(TokenUsageInfo {
        total_token_usage: TokenUsage::default(),
        last_token_usage: TokenUsage::default(),
        model_context_window: Some(258_400),
    });
    let last = Some(TokenUsage {
        input_tokens: 10,
        cached_input_tokens: 0,
        output_tokens: 0,
        reasoning_output_tokens: 0,
        total_tokens: 10,
    });

    let info = TokenUsageInfo::new_or_append(&initial, &last, /*model_context_window*/ None)
        .expect("new_or_append should return info");

    assert_eq!(info.model_context_window, Some(258_400));
}

#[test]
fn terminal_turn_snapshot_fields_preserve_legacy_json_shape() -> anyhow::Result<()> {
    let complete: TurnCompleteEvent = serde_json::from_value(serde_json::json!({
        "turn_id": "turn-1",
        "last_agent_message": null,
        "completed_at": 20,
        "duration_ms": 10_000,
        "time_to_first_token_ms": null
    }))?;
    assert_eq!(complete.started_at, None);
    assert_eq!(complete.error, None);
    let complete_json = serde_json::to_value(&complete)?;
    assert!(complete_json.get("started_at").is_none());
    assert!(complete_json.get("error").is_none());

    let aborted: TurnAbortedEvent = serde_json::from_value(serde_json::json!({
        "turn_id": "turn-1",
        "reason": "interrupted",
        "completed_at": 20,
        "duration_ms": 10_000
    }))?;
    assert_eq!(aborted.started_at, None);
    assert!(serde_json::to_value(&aborted)?.get("started_at").is_none());
    Ok(())
}

fn test_selected_snippet_envelope() -> TurnContextRecallSelectedSnippetEnvelope {
    TurnContextRecallSelectedSnippetEnvelope {
        version: TURN_CONTEXT_RECALL_SELECTED_SNIPPET_ENVELOPE_VERSION,
        max_snippets: 4,
        max_snippet_chars: 120,
        selected_snippet_count: 1,
        omitted_snippet_count: 2,
        redacted_snippet_count: 1,
        truncated_snippet_count: 0,
        snippets: vec![TurnContextRecallSelectedSnippet {
            snippet_hash: "fedcba9876543210".into(),
            text: "[redacted-query] bounded memory".into(),
            estimated_tokens: 8,
            redacted: true,
            truncated: false,
        }],
        safety: TurnContextRecallSelectedSnippetSafety {
            ready_for_shadow_handoff: true,
            bounded: true,
            origin_identifiers_exposed: false,
            raw_ranked_payload_exposed: false,
            rank_explanation_exposed: false,
            control_marker_exposed: false,
            query_payload_exposed: false,
            per_origin_list_exposed: false,
        },
    }
}
