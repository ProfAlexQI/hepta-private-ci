use super::*;

#[test]
fn architecture_v2_process_control_is_high_risk_mutating_and_read_only_profiles_block_it() {
    let runtime = RuntimeKernel::new();
    assert!(runtime.tools.execution_metadata("process").is_err());
    assert!(runtime.tools.risk_tier("process").is_err());

    let registry = ToolRegistry::new_with_quarantined_exec_process_for_test();
    let metadata = registry
        .execution_metadata("process")
        .expect("process metadata");
    assert!(!metadata.read_only);
    assert!(metadata.destructive);
    assert!(!metadata.idempotent);
    assert_eq!(
        registry.risk_tier("process").expect("process risk"),
        hepta_core::RiskTier::High
    );

    runtime
        .switch_execution_profile(ExecutionProfile::ReadOnlyTools)
        .expect("read-only profile");
    for action in ["write", "kill", "terminate", "clear", "remove"] {
        let error = runtime
            .ensure_execution_profile_allows_tool(&SessionId("session-main".into()), "process")
            .expect_err("mutating process control must be blocked");
        assert!(
            error.0.contains("unknown tool: process"),
            "{action}: {error}"
        );
    }
}

#[tokio::test]
async fn architecture_v2_process_control_rejects_forged_and_path_shaped_ids_before_effects() {
    let registry = ToolRegistry::new_with_quarantined_exec_process_for_test();
    let current_pid = std::process::id();
    let forged_current_process = format!("hepta-proc-1-{current_pid}");
    let forged_error = registry
        .invoke(
            "process",
            provider_test_context("session-process-security", "forged-process-kill"),
            ToolCallRequest {
                name: "process".into(),
                input_json: json!({
                    "action": "kill",
                    "sessionId": forged_current_process,
                })
                .to_string(),
            },
        )
        .await
        .expect_err("unissued current-PID token must never signal");
    assert!(
        forged_error
            .0
            .contains("no runtime-issued native background process")
    );

    let directory = tempfile::tempdir().expect("tempdir");
    let victim = directory.path().join("victim.log");
    fs::write(&victim, "keep\n").expect("victim");
    for (action, id) in [
        ("log", victim.display().to_string()),
        ("remove", "../../victim".to_string()),
        ("clear", "../hepta-proc-1-1".to_string()),
    ] {
        let error = registry
            .invoke(
                "process",
                provider_test_context("session-process-security", &format!("process-{action}")),
                ToolCallRequest {
                    name: "process".into(),
                    input_json: json!({"action": action, "sessionId": id}).to_string(),
                },
            )
            .await
            .expect_err("path-shaped process id must fail closed");
        assert!(
            error
                .0
                .contains("process sessionId must be a runtime-issued opaque token"),
            "{error}"
        );
    }
    assert_eq!(fs::read_to_string(victim).expect("victim"), "keep\n");
}
