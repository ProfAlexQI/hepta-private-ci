use super::*;

#[test]
fn architecture_v2_provider_idempotency_requires_the_exact_staged_identity() {
    let attempt = uuid::Uuid::new_v4().to_string();
    let key = format!("hepta-execution:{attempt}:sha256:{}", "a".repeat(64));
    let exact = ToolContext {
        session_id: Some(SessionId("session-provider-identity".into())),
        correlation_id: Some(CorrelationId("correlation-provider-identity".into())),
        execution_attempt_id: Some(attempt.clone()),
        idempotency_key: Some(key.clone()),
    };
    assert!(crate::ProviderExecutionIdentity::from_exact_context(&exact, &attempt, &key).is_ok());

    for context in [
        ToolContext {
            execution_attempt_id: None,
            idempotency_key: None,
            ..exact.clone()
        },
        ToolContext {
            execution_attempt_id: Some(uuid::Uuid::new_v4().to_string()),
            ..exact.clone()
        },
        ToolContext {
            idempotency_key: Some(format!(
                "hepta-execution:{attempt}:sha256:{}",
                "b".repeat(64)
            )),
            ..exact
        },
    ] {
        assert!(
            crate::ProviderExecutionIdentity::from_exact_context(&context, &attempt, &key).is_err()
        );
    }
}

#[tokio::test]
async fn architecture_v2_provider_idempotency_quarantines_every_live_native_mutation() {
    let registry = ToolRegistry::new();
    let cases = [
        (
            "message",
            json!({"action":"send","channel":"telegram","target":"test","message":"local","dryRun":false,"confirmSend":true}),
        ),
        ("sessions_send", json!({"message":"local","execute":true})),
        ("sessions_spawn", json!({"task":"local","execute":true})),
        (
            "subagents",
            json!({"action":"steer","target":"test","message":"local"}),
        ),
        ("subagents", json!({"action":"kill","target":"test"})),
        ("subagents", json!({"action":"stop","target":"test"})),
        ("canvas", json!({"action":"sample-run"})),
        (
            "feishu_app_scopes",
            json!({"dryRun":false,"liveProbe":true}),
        ),
        ("feishu_chat", json!({"dryRun":false,"liveProbe":true})),
        ("feishu_doc", json!({"dryRun":false,"liveProbe":true})),
        ("feishu_drive", json!({"dryRun":false,"liveProbe":true})),
        ("feishu_wiki", json!({"dryRun":false,"liveProbe":true})),
        (
            "feishu_bitable_get_meta",
            json!({"dryRun":false,"liveProbe":true}),
        ),
        (
            "feishu_bitable_list_fields",
            json!({"dryRun":false,"liveProbe":true}),
        ),
        (
            "feishu_bitable_list_records",
            json!({"dryRun":false,"liveProbe":true}),
        ),
        (
            "feishu_bitable_get_record",
            json!({"dryRun":false,"liveProbe":true}),
        ),
        (
            "feishu_bitable_create_record",
            json!({"dryRun":false,"liveProbe":true}),
        ),
        (
            "feishu_bitable_update_record",
            json!({"dryRun":false,"liveProbe":true}),
        ),
        (
            "feishu_bitable_create_app",
            json!({"dryRun":false,"liveProbe":true}),
        ),
        (
            "feishu_bitable_create_field",
            json!({"dryRun":false,"liveProbe":true}),
        ),
    ];

    for (tool, input) in cases {
        let error = registry
            .invoke(
                tool,
                provider_test_context("session-live-quarantine", tool),
                ToolCallRequest {
                    name: tool.into(),
                    input_json: input.to_string(),
                },
            )
            .await
            .expect_err("live provider mutation must remain quarantined");
        assert!(
            error.0.contains("remains quarantined"),
            "{tool} unexpectedly failed as: {error}"
        );
    }
}
