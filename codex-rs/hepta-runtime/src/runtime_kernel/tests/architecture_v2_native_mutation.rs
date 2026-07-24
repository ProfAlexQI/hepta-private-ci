include!("architecture_v2_native_mutation_support.rs");

#[cfg(unix)]
#[test]
fn architecture_v2_native_mutation_all_tools_reject_absolute_and_symlink_escape_before_provider() {
    use std::os::unix::fs::symlink;

    let workspace = crate::tool_workspace_root_path();
    let outside = tempfile::tempdir().expect("outside");
    let link_name = format!(".architecture-v2-native-link-{}", uuid::Uuid::new_v4());
    let link = workspace.join("artifacts").join(&link_name);
    fs::create_dir_all(link.parent().expect("artifacts")).expect("artifacts");
    symlink(outside.path(), &link).expect("external symlink");

    for scope in [WritePathScope::WorkspaceOnly, WritePathScope::ArtifactsOnly] {
        for tool in ["write", "edit", "apply_patch", "tts"] {
            let external_name = format!("{tool}-{}.txt", uuid::Uuid::new_v4());
            let external = outside.path().join(&external_name);
            if matches!(tool, "edit" | "apply_patch") {
                fs::write(&external, "alpha\n").expect("external fixture");
            }
            for path in [
                external.display().to_string(),
                format!("artifacts/{link_name}/{external_name}"),
            ] {
                let runtime = RuntimeKernel::new();
                runtime
                    .switch_filesystem_scope(FilesystemScope::AnyPath)
                    .expect("filesystem any");
                runtime.switch_write_path_scope(scope).expect("write scope");
                let arguments = mutation_arguments(tool, &path);
                let error = authorize(&runtime, "session-main", tool, &arguments, "scope-escape")
                    .expect_err("scope escape must fail before provider");
                assert!(error.0.contains("outside"), "{}", error.0);
                assert_eq!(runtime.tools.provider_invocation_count(tool), 0);
                assert_eq!(active_reservations(&runtime), 0);
            }
            if matches!(tool, "edit" | "apply_patch") {
                assert_eq!(fs::read_to_string(&external).expect("external"), "alpha\n");
            } else {
                assert!(!external.exists());
            }
        }
    }

    let tts_gate_runtime = RuntimeKernel::new();
    tts_gate_runtime
        .switch_filesystem_scope(FilesystemScope::AnyPath)
        .expect("filesystem any");
    tts_gate_runtime
        .switch_write_path_scope(WritePathScope::AnyPath)
        .expect("write any");
    tts_gate_runtime
        .set_path_capability_gate("tts", "path", FilesystemScope::AnyPath)
        .expect("path gate");
    tts_gate_runtime
        .set_path_capability_gate("tts", "filename", FilesystemScope::WorkspaceOnly)
        .expect("filename gate");
    let external_tts = outside.path().join("gated-tts.aiff").display().to_string();
    for arguments in [
        json!({"text":"gated","filename":external_tts,"dryRun":true}).to_string(),
        json!({
            "text":"gated",
            "path":external_tts,
            "filename":external_tts,
            "dryRun":true
        })
        .to_string(),
    ] {
        let error = authorize(
            &tts_gate_runtime,
            "session-main",
            "tts",
            &arguments,
            "tts-argument-gate",
        )
        .expect_err("filename WorkspaceOnly gate must dominate AnyPath");
        assert!(error.0.contains("filesystem scope workspace_only"));
    }
    assert_eq!(tts_gate_runtime.tools.provider_invocation_count("tts"), 0);

    let patch_scope_runtime = RuntimeKernel::new();
    patch_scope_runtime
        .switch_filesystem_scope(FilesystemScope::WorkspaceOnly)
        .expect("filesystem workspace");
    patch_scope_runtime
        .switch_write_path_scope(WritePathScope::AnyPath)
        .expect("write any");
    let patch_error = authorize(
        &patch_scope_runtime,
        "session-main",
        "apply_patch",
        &mutation_arguments(
            "apply_patch",
            &outside.path().join("apply_patch.txt").display().to_string(),
        ),
        "patch-global-filesystem-scope",
    )
    .expect_err("apply_patch must retain global filesystem scope");
    assert!(patch_error.0.contains("filesystem scope workspace_only"));
    assert_eq!(
        patch_scope_runtime
            .tools
            .provider_invocation_count("apply_patch"),
        0
    );

    for tool in ["write", "edit", "apply_patch", "tts"] {
        let suffix = uuid::Uuid::new_v4();
        let external = workspace.join(format!(
            ".architecture-v2-native-hardlink-source-{tool}-{suffix}.txt"
        ));
        let alias_relative =
            format!("artifacts/.architecture-v2-native-hardlink-{tool}-{suffix}.txt");
        let alias = workspace.join(&alias_relative);
        fs::write(&external, "alpha\n").expect("hardlink source");
        fs::hard_link(&external, &alias).expect("hardlink alias");
        let runtime = RuntimeKernel::new();
        runtime
            .switch_filesystem_scope(FilesystemScope::WorkspaceOnly)
            .expect("filesystem workspace");
        runtime
            .switch_write_path_scope(WritePathScope::ArtifactsOnly)
            .expect("artifacts write scope");
        let error = authorize(
            &runtime,
            "session-main",
            tool,
            &mutation_arguments(tool, &alias_relative),
            "hardlink-scope-escape",
        )
        .expect_err("hardlink alias must fail before provider");
        assert!(error.0.contains("hard links"), "{}", error.0);
        assert_eq!(runtime.tools.provider_invocation_count(tool), 0);
        assert_eq!(
            fs::read_to_string(&external).expect("unchanged hardlink source"),
            "alpha\n"
        );
        fs::remove_file(alias).expect("remove hardlink alias");
        fs::remove_file(external).expect("remove hardlink source");
    }
    fs::remove_file(link).expect("remove symlink");
}

#[cfg(unix)]
#[tokio::test]
async fn architecture_v2_native_mutation_all_tools_reject_parent_swap_before_provider() {
    use std::os::unix::fs::symlink;

    let workspace = crate::tool_workspace_root_path();
    for tool in ["write", "edit", "apply_patch", "tts"] {
        let runtime = RuntimeKernel::new();
        let outside = tempfile::tempdir().expect("outside");
        let suffix = uuid::Uuid::new_v4();
        let parent = workspace
            .join("artifacts")
            .join(format!(".architecture-v2-native-swap-{tool}-{suffix}"));
        let retained = parent.with_extension("retained");
        fs::create_dir_all(&parent).expect("parent");
        let target = parent.join("target.txt");
        if matches!(tool, "edit" | "apply_patch") {
            fs::write(&target, "alpha\n").expect("fixture");
        }
        let arguments = mutation_arguments(tool, &target.display().to_string());
        let execution = authorize(&runtime, "session-main", tool, &arguments, "parent-swap")
            .expect("authorization");
        fs::rename(&parent, &retained).expect("retain parent");
        symlink(outside.path(), &parent).expect("replacement symlink");

        let mut captured = ExecutionBus::new(&runtime).dispatch(execution).await;
        match captured.terminal() {
            CapturedDispatchTerminal::ToolError(error)
            | CapturedDispatchTerminal::DispatchBlocked(error) => {
                assert!(error.contains("sealed write ancestor"), "{error}");
            }
            terminal => panic!("parent swap must fail closed: {terminal:?}"),
        }
        captured.capture_write_transaction();
        OutcomeRecorder::new(&runtime)
            .finalize_tool_dispatch(&mut captured)
            .expect("failure receipt");
        assert_eq!(runtime.tools.provider_invocation_count(tool), 0);
        assert!(!outside.path().join("target.txt").exists());
        if matches!(tool, "edit" | "apply_patch") {
            assert_eq!(
                fs::read_to_string(retained.join("target.txt")).expect("retained"),
                "alpha\n"
            );
        } else {
            assert!(!retained.join("target.txt").exists());
        }

        fs::remove_file(&parent).expect("remove replacement");
        fs::rename(&retained, &parent).expect("restore parent");
        if target.exists() {
            fs::remove_file(&target).expect("remove fixture");
        }
        fs::remove_dir(parent).expect("remove parent");
    }
}

#[cfg(unix)]
#[tokio::test]
async fn architecture_v2_native_mutation_post_authorization_hardlink_never_observes_new_bytes() {
    let workspace = crate::tool_workspace_root_path();
    let outside = tempfile::tempdir().expect("outside");
    let suffix = uuid::Uuid::new_v4();
    let relative = format!("artifacts/.architecture-v2-native-hardlink-race-{suffix}.txt");
    let target = workspace.join(&relative);
    let outside_alias = outside.path().join("authorized-inode-alias.txt");
    fs::write(&target, "before\n").expect("target fixture");

    let runtime = RuntimeKernel::new();
    let arguments = json!({"path":relative,"content":"after\n"}).to_string();
    let execution = authorize(
        &runtime,
        "session-hardlink-race",
        "write",
        &arguments,
        "post-authorization-hardlink",
    )
    .expect("authorization must retain the original single-link inode");

    fs::hard_link(&target, &outside_alias)
        .expect("hostile alias created after authorization but before commit");
    let _ = execute_success(&runtime, execution).await;

    assert_eq!(
        fs::read_to_string(&target).expect("committed namespace"),
        "after\n"
    );
    assert_eq!(
        fs::read_to_string(&outside_alias).expect("outside alias"),
        "before\n",
        "commit must replace the authorized namespace entry, never mutate its old inode"
    );
    remove_recorded_checkpoints(&runtime, &target);
    fs::remove_file(target).expect("remove target");
}

#[cfg(unix)]
#[tokio::test]
async fn architecture_v2_native_mutation_secure_happy_path_covers_write_edit_patch_and_tts() {
    let runtime = RuntimeKernel::new();
    let workspace = crate::tool_workspace_root_path();
    let suffix = uuid::Uuid::new_v4();
    let relative = format!("artifacts/.architecture-v2-native-happy-{suffix}.txt");
    let target = workspace.join(&relative);

    for (tool, arguments) in [
        ("write", mutation_arguments("write", &relative)),
        ("edit", mutation_arguments("edit", &relative)),
        (
            "apply_patch",
            json!({
                "input":format!(
                    "*** Begin Patch\n*** Update File: {relative}\n@@\n-beta\n+gamma\n*** End Patch"
                )
            })
            .to_string(),
        ),
        (
            "tts",
            json!({
                "text":"preview",
                "filename":format!("artifacts/.architecture-v2-native-tts-{suffix}.aiff"),
                "dryRun":true
            })
            .to_string(),
        ),
    ] {
        let execution = authorize(
            &runtime,
            &format!("session-{tool}"),
            tool,
            &arguments,
            &format!("happy-{tool}"),
        )
        .expect("authorized native mutation");
        let result = execute_success(&runtime, execution).await;
        let structured: serde_json::Value =
            serde_json::from_str(result.structured_json.as_deref().expect("structured"))
                .expect("valid output");
        assert_eq!(structured["result"]["identity_bound"], json!(true));
        if tool != "tts" {
            assert_eq!(
                structured["provider_effect_ack"]["status"],
                json!("committed")
            );
            assert!(
                structured["provider_effect_ack_hash"]
                    .as_str()
                    .is_some_and(|hash| hash.starts_with("sha256:"))
            );
        }
        assert_eq!(runtime.tools.provider_invocation_count(tool), 1);
    }
    let ambiguous_relative = format!("artifacts/.architecture-v2-native-ambiguous-{suffix}.txt");
    let ambiguous_target = workspace.join(&ambiguous_relative);
    let arguments = mutation_arguments("write", &ambiguous_relative);
    let execution = authorize(&runtime, "s", "write", &arguments, "amb")
        .expect("authorized ambiguous mutation");
    let attempt_id = execution.attempt_id().to_string();
    crate::inject_atomic_install_post_commit_failure_for_test();
    let mut captured = ExecutionBus::new(&runtime).dispatch(execution).await;
    let error = captured.outward_error().expect("ambiguous outward error");
    assert!(error.0.starts_with("mutation_durability_ambiguous:"));
    assert!(fs::read_to_string(&ambiguous_target).expect("applied") == "alpha\n");
    captured.capture_write_transaction();
    OutcomeRecorder::new(&runtime)
        .finalize_tool_dispatch(&mut captured)
        .expect("ambiguous effect receipt");
    let record = runtime.outcome_record_by_attempt(&attempt_id).unwrap();
    let record = record.unwrap();
    let evidence = record.canonical_evidence();
    assert!(evidence.contains(r#"["transaction.status","recorded"]"#));
    assert!(evidence.contains(r#"["effect.disposition","recorded"]"#));
    fs::remove_file(ambiguous_target).expect("remove ambiguous target");
    assert_eq!(fs::read_to_string(&target).expect("target"), "gamma\n");
    assert_eq!(active_reservations(&runtime), 0);
    remove_recorded_checkpoints(&runtime, &target);
    fs::remove_file(target).expect("remove target");
}

#[cfg(target_os = "macos")]
#[tokio::test]
async fn architecture_v2_native_mutation_live_tts_commits_exact_staged_effect_and_ack() {
    let runtime = RuntimeKernel::new();
    let relative = format!(
        "artifacts/.architecture-v2-live-tts-{}.aiff",
        uuid::Uuid::new_v4()
    );
    let target = crate::tool_workspace_root_path().join(&relative);
    let arguments = json!({
        "text": "live speech",
        "filename": relative,
        "dryRun": false
    })
    .to_string();
    let execution = authorize(
        &runtime,
        "session-live-tts",
        "tts",
        &arguments,
        "live-tts",
    )
    .expect("live TTS stages exact private audio before durable intent");
    let result = execute_success(&runtime, execution).await;
    let structured: serde_json::Value =
        serde_json::from_str(result.structured_json.as_deref().expect("structured"))
            .expect("valid output");
    assert_eq!(
        structured["result"]["synthesis_staged_before_intent"],
        json!(true)
    );
    assert_eq!(
        structured["provider_effect_ack"]["status"],
        json!("committed")
    );
    assert!(
        structured["provider_effect_ack_hash"]
            .as_str()
            .is_some_and(|hash| hash.starts_with("sha256:"))
    );
    assert_eq!(runtime.tools.provider_invocation_count("tts"), 1);
    assert!(fs::metadata(&target).expect("committed TTS target").len() > 0);
    assert_eq!(active_reservations(&runtime), 0);
    fs::remove_file(target).expect("remove TTS target");
}

#[cfg(unix)]
#[test]
fn architecture_v2_native_mutation_process_registry_conflicts_across_tools_and_kernels() {
    let suffix = uuid::Uuid::new_v4();
    let relative = format!("artifacts/.architecture-v2-native-global-{suffix}.txt");
    let holder_runtime = RuntimeKernel::new();
    let contender_runtime = RuntimeKernel::new();
    let holder_arguments = mutation_arguments("write", &relative);
    let patch_arguments = json!({
        "input":format!(
            "*** Begin Patch\n*** Add File: {relative}\n+contender\n*** End Patch"
        )
    })
    .to_string();

    let holder = authorize(
        &holder_runtime,
        "session-holder",
        "write",
        &holder_arguments,
        "holder",
    )
    .expect("holder");
    let error = authorize(
        &contender_runtime,
        "session-contender",
        "apply_patch",
        &patch_arguments,
        "contender",
    )
    .expect_err("cross-tool identity conflict");
    assert!(
        error.0.contains("tool_execution_reservation"),
        "{}",
        error.0
    );
    assert_eq!(
        contender_runtime
            .tools
            .provider_invocation_count("apply_patch"),
        0
    );
    drop(holder);
    let reacquired = authorize(
        &contender_runtime,
        "session-contender",
        "apply_patch",
        &patch_arguments,
        "reacquired",
    )
    .expect("released identity");
    drop(reacquired);
}

#[cfg(unix)]
#[test]
fn architecture_v2_native_mutation_patch_set_is_stable_and_duplicate_identity_fails_closed() {
    let workspace = crate::tool_workspace_root_path();
    let suffix = uuid::Uuid::new_v4();
    let first = format!("artifacts/.architecture-v2-native-set-a-{suffix}.txt");
    let second = format!("artifacts/.architecture-v2-native-set-b-{suffix}.txt");
    fs::write(workspace.join(&first), "alpha\n").expect("first");
    fs::write(workspace.join(&second), "alpha\n").expect("second");
    let patch = |left: &str, right: &str| {
        json!({
            "input":format!(
                "*** Begin Patch\n*** Update File: {left}\n@@\n-alpha\n+left\n*** Update File: {right}\n@@\n-alpha\n+right\n*** End Patch"
            ),
            "preview_only":true
        })
        .to_string()
    };
    let first_runtime = RuntimeKernel::new();
    let second_runtime = RuntimeKernel::new();
    let holder = authorize(
        &first_runtime,
        "session-holder",
        "apply_patch",
        &patch(&first, &second),
        "ordered-holder",
    )
    .expect("ordered set");
    let error = authorize(
        &second_runtime,
        "session-contender",
        "apply_patch",
        &patch(&second, &first),
        "reverse-contender",
    )
    .expect_err("reverse set must fail fast");
    assert!(
        error.0.contains("tool_execution_reservation"),
        "{}",
        error.0
    );
    assert_eq!(active_reservations(&second_runtime), 0);
    drop(holder);
    let reverse = authorize(
        &second_runtime,
        "session-contender",
        "apply_patch",
        &patch(&second, &first),
        "reverse-reacquired",
    )
    .expect("reverse set reusable");
    drop(reverse);

    let duplicate_runtime = RuntimeKernel::new();
    let duplicate = patch(&first, &first);
    let duplicate_error = authorize(
        &duplicate_runtime,
        "session-duplicate",
        "apply_patch",
        &duplicate,
        "duplicate",
    )
    .expect_err("duplicate identity must fail before reservation");
    assert!(
        duplicate_error.0.contains("repeated or overlapping"),
        "{}",
        duplicate_error.0
    );
    assert_eq!(active_reservations(&duplicate_runtime), 0);
    assert_eq!(
        duplicate_runtime
            .tools
            .provider_invocation_count("apply_patch"),
        0
    );

    let live_runtime = RuntimeKernel::new();
    let live_duplicate = json!({"input":format!(
        "*** Begin Patch\n*** Update File: {first}\n@@\n-alpha\n+first\n*** Update File: {first}\n@@\n-alpha\n+second\n*** End Patch"
    )})
    .to_string();
    let live_error = authorize(
        &live_runtime,
        "session-live",
        "apply_patch",
        &live_duplicate,
        "live-multi-operation",
    )
    .expect_err("multi-operation live patch must fail before any write");
    assert!(live_error.0.contains("non-preview multi-operation"));
    assert_eq!(
        live_runtime.tools.provider_invocation_count("apply_patch"),
        0
    );
    assert_eq!(
        fs::read_to_string(workspace.join(&first)).expect("unchanged first"),
        "alpha\n"
    );
    assert_eq!(active_reservations(&live_runtime), 0);
    fs::remove_file(workspace.join(first)).expect("remove first");
    fs::remove_file(workspace.join(second)).expect("remove second");
}

#[cfg(unix)]
#[test]
fn architecture_v2_native_mutation_delete_and_unbound_tts_fail_before_provider() {
    let workspace = crate::tool_workspace_root_path();
    let relative = format!(
        "artifacts/.architecture-v2-native-delete-{}.txt",
        uuid::Uuid::new_v4()
    );
    let target = workspace.join(&relative);
    fs::write(&target, "keep\n").expect("delete fixture");
    let runtime = RuntimeKernel::new();
    let delete = json!({
        "input":format!(
            "*** Begin Patch\n*** Delete File: {relative}\n*** End Patch"
        )
    })
    .to_string();
    let delete_error = authorize(&runtime, "session-delete", "apply_patch", &delete, "delete")
        .expect_err("delete must fail closed without inode-bound unlink");
    assert!(
        delete_error.0.contains("refuses delete"),
        "{}",
        delete_error.0
    );
    assert_eq!(runtime.tools.provider_invocation_count("apply_patch"), 0);
    assert_eq!(fs::read_to_string(&target).expect("fixture"), "keep\n");

    let tts_arguments = json!({"text":"missing output path","dryRun":true}).to_string();
    let tts_error = authorize(
        &runtime,
        "session-tts",
        "tts",
        &tts_arguments,
        "unbound-tts",
    )
    .expect_err("tts must not recreate a path after authorization");
    assert!(tts_error.0.contains("explicit path or filename"));
    assert_eq!(runtime.tools.provider_invocation_count("tts"), 0);
    assert_eq!(active_reservations(&runtime), 0);
    fs::remove_file(target).expect("remove fixture");
}
