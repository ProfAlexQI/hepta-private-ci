use super::*;
use codex_protocol::openai_models::ModelPreset;
use codex_protocol::openai_models::ModelServiceTier;
use codex_protocol::openai_models::ReasoningEffort;
use codex_protocol::openai_models::ReasoningEffortPreset;
use codex_tools::JsonSchemaPrimitiveType;
use codex_tools::JsonSchemaType;
use pretty_assertions::assert_eq;
use serde_json::json;

fn model_preset(id: &str, show_in_picker: bool) -> ModelPreset {
    ModelPreset {
        id: id.to_string(),
        model: format!("{id}-model"),
        display_name: format!("{id} display"),
        description: format!("{id} description"),
        default_reasoning_effort: ReasoningEffort::Medium,
        supported_reasoning_efforts: vec![ReasoningEffortPreset {
            effort: ReasoningEffort::Medium,
            description: "Balanced".to_string(),
        }],
        supports_personality: false,
        additional_speed_tiers: Vec::new(),
        service_tiers: vec![ModelServiceTier {
            id: "priority".to_string(),
            name: "Fast".to_string(),
            description: "1.5x speed, increased usage".to_string(),
        }],
        is_default: false,
        upgrade: None,
        show_in_picker,
        availability_nux: None,
        supported_in_api: true,
        input_modalities: Vec::new(),
    }
}

#[test]
fn spawn_agent_tool_v2_requires_task_name_and_lists_visible_models() {
    let tool = create_spawn_agent_tool_v2(SpawnAgentToolOptions {
        available_models: vec![
            model_preset("visible", /*show_in_picker*/ true),
            model_preset("hidden", /*show_in_picker*/ false),
        ],
        agent_type_description: "role help".to_string(),
        hide_agent_type_model_reasoning: false,
        include_usage_hint: true,
        usage_hint_text: None,
        max_concurrent_threads_per_session: Some(4),
    });

    let ToolSpec::Function(ResponsesApiTool {
        description,
        parameters,
        output_schema,
        ..
    }) = tool
    else {
        panic!("spawn_agent should be a function tool");
    };
    assert_eq!(
        parameters.schema_type,
        Some(JsonSchemaType::Single(JsonSchemaPrimitiveType::Object))
    );
    let properties = parameters
        .properties
        .as_ref()
        .expect("spawn_agent should use object params");
    assert!(description.contains("Spawns an agent to work on the specified task."));
    assert!(description.contains("The spawned agent will have the same tools as you"));
    assert!(description.contains("`max_concurrent_threads_per_session = 4`"));
    assert!(description.contains(SPAWN_AGENT_INHERITED_MODEL_GUIDANCE));
    assert!(
        description
            .contains("Available model overrides (optional; inherited parent model is preferred):")
    );
    assert!(description.contains("visible display (`visible-model`)"));
    assert!(
        description
            .contains("Supported service tiers: priority (Fast: 1.5x speed, increased usage).")
    );
    assert!(!description.contains("hidden display (`hidden-model`)"));
    assert!(properties.contains_key("task_name"));
    assert!(properties.contains_key("message"));
    assert!(properties.contains_key("fork_turns"));
    assert!(!properties.contains_key("items"));
    assert!(!properties.contains_key("fork_context"));
    assert_eq!(
        properties.get("agent_type"),
        Some(&JsonSchema::string(Some("role help".to_string())))
    );
    assert_eq!(
        properties
            .get("model")
            .and_then(|schema| schema.description.as_deref()),
        Some(SPAWN_AGENT_MODEL_OVERRIDE_DESCRIPTION)
    );
    assert_eq!(
        properties
            .get("service_tier")
            .and_then(|schema| schema.description.as_deref()),
        Some(SPAWN_AGENT_SERVICE_TIER_OVERRIDE_DESCRIPTION)
    );
    assert_eq!(
        parameters.required.as_ref(),
        Some(&vec!["task_name".to_string(), "message".to_string()])
    );
    assert_eq!(
        output_schema.expect("spawn_agent output schema")["required"],
        json!(["task_name", "nickname"])
    );
}

#[test]
fn spawn_agent_tool_v1_keeps_legacy_fork_context_field() {
    let tool = create_spawn_agent_tool_v1(SpawnAgentToolOptions {
        available_models: Vec::new(),
        agent_type_description: "role help".to_string(),
        hide_agent_type_model_reasoning: false,
        include_usage_hint: true,
        usage_hint_text: None,
        max_concurrent_threads_per_session: None,
    });

    let ToolSpec::Function(ResponsesApiTool { parameters, .. }) = tool else {
        panic!("spawn_agent should be a function tool");
    };
    assert_eq!(
        parameters.schema_type,
        Some(JsonSchemaType::Single(JsonSchemaPrimitiveType::Object))
    );
    let properties = parameters
        .properties
        .as_ref()
        .expect("spawn_agent should use object params");

    assert!(properties.contains_key("fork_context"));
    assert!(!properties.contains_key("fork_turns"));
    assert_eq!(
        properties
            .get("model")
            .and_then(|schema| schema.description.as_deref()),
        Some(SPAWN_AGENT_MODEL_OVERRIDE_DESCRIPTION)
    );
    assert_eq!(
        properties
            .get("service_tier")
            .and_then(|schema| schema.description.as_deref()),
        Some(SPAWN_AGENT_SERVICE_TIER_OVERRIDE_DESCRIPTION)
    );
}

#[test]
fn spawn_agent_tool_hides_service_tier_with_spawn_metadata() {
    let tool = create_spawn_agent_tool_v2(SpawnAgentToolOptions {
        available_models: vec![model_preset("visible", /*show_in_picker*/ true)],
        agent_type_description: "role help".to_string(),
        hide_agent_type_model_reasoning: true,
        include_usage_hint: true,
        usage_hint_text: None,
        max_concurrent_threads_per_session: Some(4),
    });

    let ToolSpec::Function(ResponsesApiTool { parameters, .. }) = tool else {
        panic!("spawn_agent should be a function tool");
    };
    let properties = parameters
        .properties
        .as_ref()
        .expect("spawn_agent should use object params");

    assert!(!properties.contains_key("agent_type"));
    assert!(!properties.contains_key("model"));
    assert!(!properties.contains_key("reasoning_effort"));
    assert!(!properties.contains_key("service_tier"));
}

#[test]
fn send_message_tool_requires_message_and_has_no_output_schema() {
    let ToolSpec::Function(ResponsesApiTool {
        parameters,
        output_schema,
        ..
    }) = create_send_message_tool()
    else {
        panic!("send_message should be a function tool");
    };
    assert_eq!(
        parameters.schema_type,
        Some(JsonSchemaType::Single(JsonSchemaPrimitiveType::Object))
    );
    let properties = parameters
        .properties
        .as_ref()
        .expect("send_message should use object params");
    assert!(properties.contains_key("target"));
    assert!(properties.contains_key("message"));
    assert!(!properties.contains_key("interrupt"));
    assert!(!properties.contains_key("items"));
    assert_eq!(
        properties
            .get("target")
            .and_then(|schema| schema.description.as_deref()),
        Some("Relative or canonical task name to message (from spawn_agent).")
    );
    assert_eq!(
        parameters.required.as_ref(),
        Some(&vec!["target".to_string(), "message".to_string()])
    );
    assert_eq!(output_schema, None);
}

#[test]
fn followup_task_tool_requires_message_and_has_no_output_schema() {
    let ToolSpec::Function(ResponsesApiTool {
        parameters,
        output_schema,
        ..
    }) = create_followup_task_tool()
    else {
        panic!("followup_task should be a function tool");
    };
    assert_eq!(
        parameters.schema_type,
        Some(JsonSchemaType::Single(JsonSchemaPrimitiveType::Object))
    );
    let properties = parameters
        .properties
        .as_ref()
        .expect("followup_task should use object params");
    assert!(properties.contains_key("target"));
    assert!(properties.contains_key("message"));
    assert!(!properties.contains_key("items"));
    assert_eq!(
        parameters.required.as_ref(),
        Some(&vec!["target".to_string(), "message".to_string()])
    );
    assert_eq!(output_schema, None);
}

#[test]
fn wait_agent_tool_v2_uses_timeout_only_summary_output() {
    let ToolSpec::Function(ResponsesApiTool {
        description,
        parameters,
        output_schema,
        ..
    }) = create_wait_agent_tool_v2(WaitAgentTimeoutOptions {
        default_timeout_ms: 30_000,
        min_timeout_ms: 10_000,
        max_timeout_ms: 3_600_000,
    })
    else {
        panic!("wait_agent should be a function tool");
    };
    assert_eq!(
        parameters.schema_type,
        Some(JsonSchemaType::Single(JsonSchemaPrimitiveType::Object))
    );
    let properties = parameters
        .properties
        .as_ref()
        .expect("wait_agent should use object params");
    assert!(!properties.contains_key("targets"));
    assert!(properties.contains_key("timeout_ms"));
    assert!(properties.contains_key("task_name"));
    assert!(properties.contains_key("task_id"));
    assert!(properties.contains_key("barrier_id"));
    assert!(properties.contains_key("result_required"));
    assert!(description.contains(
        "Does not return the content; returns either a summary of which agents have updates (if any)"
    ));
    assert_eq!(
        properties
            .get("timeout_ms")
            .and_then(|schema| schema.description.as_deref()),
        Some("Optional timeout in milliseconds. Defaults to 30000, min 10000, max 3600000.")
    );
    assert_eq!(parameters.required.as_ref(), None);
    assert_eq!(
        output_schema.expect("wait output schema")["properties"]["message"]["description"],
        json!("Brief wait summary without the agent's final content.")
    );
    let output_schema = create_wait_agent_tool_v2(WaitAgentTimeoutOptions {
        default_timeout_ms: 30_000,
        min_timeout_ms: 10_000,
        max_timeout_ms: 3_600_000,
    });
    let ToolSpec::Function(ResponsesApiTool { output_schema, .. }) = output_schema else {
        panic!("wait_agent should be a function tool");
    };
    let output_schema = output_schema.expect("wait output schema");
    assert_eq!(
        output_schema["required"],
        json!([
            "message",
            "timed_out",
            "barrier_id",
            "task_id",
            "task_name",
            "task_thread_id",
            "task_status",
            "task_result",
            "result_required",
            "wait_condition",
            "durable_mailbox",
            "work_graph_lifecycle_shadow_decision"
        ])
    );
    assert_eq!(
        output_schema["properties"]["wait_condition"]["enum"],
        json!([
            "mailbox_change",
            "task_terminal_status",
            "task_result_evidence"
        ])
    );
    assert_eq!(
        output_schema["properties"]["durable_mailbox"]["properties"]["live_cutover_enabled"]["description"],
        json!("Always false while durable wait barriers are shadow-only.")
    );
    assert_eq!(
        output_schema["properties"]["durable_mailbox"]["properties"]["wait_canonical_projection_enablement_operator_review_packet_event_recorded"]
            ["description"],
        json!(
            "Whether result_required wait canonical WorkGraph projection enablement operator-review packet evidence was written to the durable mailbox stream without approval or cutover."
        )
    );
    assert_eq!(
        output_schema["properties"]["durable_mailbox"]["properties"]["wait_canonical_projection_enablement_operator_review_replay_consistency_event_recorded"]
            ["description"],
        json!(
            "Whether result_required wait canonical WorkGraph projection enablement operator-review replay consistency evidence was written to the durable mailbox stream without approval or cutover."
        )
    );
    assert_eq!(
        output_schema["properties"]["durable_mailbox"]["properties"]["wait_canonical_projection_enablement_no_live_rehearsal_closeout_event_recorded"]
            ["description"],
        json!(
            "Whether result_required wait canonical WorkGraph projection enablement no-live rehearsal closeout evidence was written to the durable mailbox stream without approval or cutover."
        )
    );
    assert_eq!(
        output_schema["properties"]["durable_mailbox"]["properties"]["wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_event_recorded"]
            ["description"],
        json!(
            "Whether result_required wait canonical WorkGraph projection enablement no-live rehearsal closeout replay consistency evidence was written to the durable mailbox stream without approval or cutover."
        )
    );
    assert_eq!(
        output_schema["properties"]["durable_mailbox"]["properties"]["wait_canonical_projection_enablement_audit_chain_closeout_event_recorded"]
            ["description"],
        json!(
            "Whether result_required wait canonical WorkGraph projection enablement final audit-chain closeout evidence was written to the durable mailbox stream without approval or cutover."
        )
    );
    assert_eq!(
        output_schema["properties"]["durable_mailbox"]["properties"]["wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_event_recorded"]
            ["description"],
        json!(
            "Whether result_required wait canonical WorkGraph projection enablement final audit-chain closeout replay consistency evidence was written to the durable mailbox stream without approval or cutover."
        )
    );
    assert_eq!(
        output_schema["properties"]["durable_mailbox"]["properties"]["wait_canonical_projection_enablement_activation_precondition_operator_packet_event_recorded"]
            ["description"],
        json!(
            "Whether result_required wait canonical WorkGraph projection enablement activation-precondition operator packet evidence was written to the durable mailbox stream without approval, activation, or cutover."
        )
    );
    assert_eq!(
        output_schema["properties"]["durable_mailbox"]["properties"]["wait_canonical_projection_enablement_activation_precondition_replay_consistency_event_recorded"]
            ["description"],
        json!(
            "Whether result_required wait canonical WorkGraph projection enablement activation-precondition replay consistency evidence was written to the durable mailbox stream without approval, activation, or cutover."
        )
    );
    assert_eq!(
        output_schema["properties"]["durable_mailbox"]["properties"]["wait_canonical_projection_enablement_activation_no_live_closeout_event_recorded"]
            ["description"],
        json!(
            "Whether result_required wait canonical WorkGraph projection enablement activation no-live closeout evidence was written to the durable mailbox stream without approval, activation, or cutover."
        )
    );
    assert_eq!(
        output_schema["properties"]["durable_mailbox"]["properties"]["wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_event_recorded"]
            ["description"],
        json!(
            "Whether result_required wait canonical WorkGraph projection enablement activation no-live closeout replay consistency evidence was written to the durable mailbox stream without approval, activation, or cutover."
        )
    );
    assert_eq!(
        output_schema["properties"]["durable_mailbox"]["properties"]["wait_canonical_projection_enablement_activation_audit_chain_closeout_event_recorded"]
            ["description"],
        json!(
            "Whether result_required wait canonical WorkGraph projection enablement final activation audit-chain closeout evidence was written to the durable mailbox stream without approval, activation, reviewed flag mutation, or cutover."
        )
    );
    assert_eq!(
        output_schema["properties"]["durable_mailbox"]["properties"]["wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_event_recorded"]
            ["description"],
        json!(
            "Whether result_required wait canonical WorkGraph projection enablement final activation audit-chain closeout replay consistency evidence was written to the durable mailbox stream without approval, activation, reviewed flag mutation, or cutover."
        )
    );
    assert_eq!(
        output_schema["properties"]["durable_mailbox"]["properties"]["wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_event_recorded"]
            ["description"],
        json!(
            "Whether result_required wait canonical WorkGraph projection enablement activation operator-approval/readiness preflight evidence was written to the durable mailbox stream while requiring future approval record and reviewed flag prerequisites without approval recording, activation, reviewed flag mutation, or cutover."
        )
    );
    assert_eq!(
        output_schema["properties"]["durable_mailbox"]["properties"]["wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_event_recorded"]
            ["description"],
        json!(
            "Whether result_required wait canonical WorkGraph projection enablement activation operator-approval/readiness preflight replay consistency evidence was written to the durable mailbox stream without approval recording, activation, reviewed flag mutation, or cutover."
        )
    );
    assert_eq!(
        output_schema["properties"]["durable_mailbox"]["properties"]["wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet_event_recorded"]
            ["description"],
        json!(
            "Whether result_required wait canonical WorkGraph projection enablement activation approval/review side-effect lock closeout evidence was written to the durable mailbox stream without approval recording, reviewed flag mutation, activation, or cutover."
        )
    );
    assert_eq!(
        output_schema["properties"]["durable_mailbox"]["properties"]["wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_event_recorded"]
            ["description"],
        json!(
            "Whether result_required wait canonical WorkGraph projection enablement activation approval/review side-effect lock closeout replay consistency evidence was written to the durable mailbox stream without approval recording, reviewed flag mutation, activation, or cutover."
        )
    );
    assert_eq!(
        output_schema["properties"]["work_graph_canonical_projection_enablement_activation_precondition_operator_packet"]
            ["description"],
        json!(
            "Shadow-only activation-precondition operator packet for canonical WorkGraph projection enablement. It consumes the final enablement audit-chain closeout replay evidence while keeping activationAllowed=false, approval recording, reviewed flag, canonical WorkGraph write/read, canary, blocking, and cutover disabled."
        )
    );
    assert_eq!(
        output_schema["properties"]["work_graph_canonical_projection_enablement_activation_precondition_replay_consistency_decision"]
            ["description"],
        json!(
            "Shadow-only replay/readback consistency decision for the canonical WorkGraph projection enablement activation-precondition operator packet. Mismatch only fails shadow readiness; activation, approval recording, reviewed flag, canonical WorkGraph write/read, canary, blocking, and cutover remain disabled."
        )
    );
    assert_eq!(
        output_schema["properties"]["work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt"]
            ["description"],
        json!(
            "Shadow-only no-live closeout receipt for canonical WorkGraph projection enablement activation preconditions. It consumes activation-precondition packet/replay evidence while keeping activationAllowed=false, approval recording, reviewed flag, canonical WorkGraph write/read, canary, blocking, and cutover disabled."
        )
    );
    assert_eq!(
        output_schema["properties"]["work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_decision"]
            ["description"],
        json!(
            "Shadow-only replay/readback consistency decision for the canonical WorkGraph projection enablement activation no-live closeout receipt. Mismatch only fails shadow readiness; activation, approval recording, reviewed flag, canonical WorkGraph write/read, canary, blocking, and cutover remain disabled."
        )
    );
    assert_eq!(
        output_schema["properties"]["work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt"]
            ["description"],
        json!(
            "Shadow-only final activation audit-chain closeout receipt for canonical WorkGraph projection enablement. It consumes activation-precondition packet/replay plus activation no-live closeout/replay evidence while keeping activation, approval recording, reviewed flag, canonical WorkGraph write/read, canary, blocking, and cutover disabled."
        )
    );
    assert_eq!(
        output_schema["properties"]["work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_decision"]
            ["description"],
        json!(
            "Shadow-only replay/readback consistency decision for the canonical WorkGraph projection enablement final activation audit-chain closeout receipt. Mismatch only fails shadow readiness; activation, approval recording, reviewed flag, canonical WorkGraph write/read, canary, blocking, and cutover remain disabled."
        )
    );
    assert_eq!(
        output_schema["properties"]["work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet"]
            ["description"],
        json!(
            "Shadow-only operator-approval/readiness preflight packet for canonical WorkGraph projection enablement activation. It consumes final activation closeout replay evidence, requires future approval record and reviewed flag prerequisites, and keeps activation, approval recording, reviewed flag mutation, canonical WorkGraph write/read, canary, blocking, and cutover disabled."
        )
    );
    assert_eq!(
        output_schema["properties"]["work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision"]
            ["description"],
        json!(
            "Shadow-only replay/readback consistency decision for the canonical WorkGraph projection enablement activation operator-approval/readiness preflight packet. Mismatch only fails shadow readiness; approval recording, reviewed flag mutation, canonical WorkGraph write/read, canary, blocking, and cutover remain disabled."
        )
    );
    assert_eq!(
        output_schema["properties"]["work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet"]
            ["description"],
        json!(
            "Shadow-only approval/review side-effect lock closeout packet for canonical WorkGraph projection enablement activation. It consumes operator-approval/readiness preflight replay evidence and proves approval recording, reviewed flag mutation, canonical WorkGraph write/read, canary, blocking, and cutover remain disabled."
        )
    );
    assert_eq!(
        output_schema["properties"]["work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_decision"]
            ["description"],
        json!(
            "Shadow-only replay/readback consistency decision for the canonical WorkGraph projection enablement activation approval/review side-effect lock closeout packet. Mismatch only fails shadow readiness; approval recording, reviewed flag mutation, canonical WorkGraph write/read, canary, blocking, and cutover remain disabled."
        )
    );
    assert_eq!(
        output_schema["properties"]["work_graph_lifecycle_shadow_decision"]["description"],
        json!(
            "Shadow-only AgentCard lifecycle decision for allowed tool, budget, and lane checks. This is not live-blocking."
        )
    );
}

#[test]
fn list_agents_tool_includes_path_prefix_and_agent_fields() {
    let ToolSpec::Function(ResponsesApiTool {
        parameters,
        output_schema,
        ..
    }) = create_list_agents_tool()
    else {
        panic!("list_agents should be a function tool");
    };
    assert_eq!(
        parameters.schema_type,
        Some(JsonSchemaType::Single(JsonSchemaPrimitiveType::Object))
    );
    let properties = parameters
        .properties
        .as_ref()
        .expect("list_agents should use object params");
    assert!(properties.contains_key("path_prefix"));
    assert_eq!(
        properties
            .get("path_prefix")
            .and_then(|schema| schema.description.as_deref()),
        Some(
            "Optional task-path prefix (not ending with trailing slash). Accepts the same relative or absolute task-path syntax."
        )
    );
    assert_eq!(
        output_schema.expect("list_agents output schema")["properties"]["agents"]["items"]["required"],
        json!(["agent_name", "agent_status", "last_task_message"])
    );
}

#[test]
fn list_agents_tool_status_schema_includes_interrupted() {
    let ToolSpec::Function(ResponsesApiTool { output_schema, .. }) = create_list_agents_tool()
    else {
        panic!("list_agents should be a function tool");
    };

    assert_eq!(
        output_schema.expect("list_agents output schema")["properties"]["agents"]["items"]["properties"]
            ["agent_status"]["allOf"][0]["oneOf"][0]["enum"],
        json!([
            "pending_init",
            "running",
            "interrupted",
            "shutdown",
            "not_found"
        ])
    );
}
