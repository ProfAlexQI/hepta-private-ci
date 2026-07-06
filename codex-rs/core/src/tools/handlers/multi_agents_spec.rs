use codex_protocol::openai_models::ModelPreset;
use codex_tools::JsonSchema;
use codex_tools::ResponsesApiTool;
use codex_tools::ToolSpec;
use serde_json::Map;
use serde_json::Value;
use serde_json::json;
use std::collections::BTreeMap;

const SPAWN_AGENT_INHERITED_MODEL_GUIDANCE: &str = "Spawned agents inherit your current model by default. Omit `model` to use that preferred default; set `model` only when an explicit override is needed.";
const SPAWN_AGENT_MODEL_OVERRIDE_DESCRIPTION: &str = "Optional model override for the new agent. Leave unset to inherit the same model as the parent, which is the preferred default. Only set this when the user explicitly asks for a different model or the task clearly requires one.";
const SPAWN_AGENT_SERVICE_TIER_OVERRIDE_DESCRIPTION: &str = "Optional service tier override for the new agent. Leave unset unless the user explicitly asks for one.";

#[derive(Debug, Clone, Default)]
pub struct SpawnAgentToolOptions {
    pub available_models: Vec<ModelPreset>,
    pub agent_type_description: String,
    pub hide_agent_type_model_reasoning: bool,
    pub include_usage_hint: bool,
    pub usage_hint_text: Option<String>,
    pub max_concurrent_threads_per_session: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WaitAgentTimeoutOptions {
    pub default_timeout_ms: i64,
    pub min_timeout_ms: i64,
    pub max_timeout_ms: i64,
}

impl Default for WaitAgentTimeoutOptions {
    fn default() -> Self {
        Self {
            default_timeout_ms: super::multi_agents_common::DEFAULT_WAIT_TIMEOUT_MS,
            min_timeout_ms: super::multi_agents_common::MIN_WAIT_TIMEOUT_MS,
            max_timeout_ms: super::multi_agents_common::MAX_WAIT_TIMEOUT_MS,
        }
    }
}

pub fn create_spawn_agent_tool_v1(options: SpawnAgentToolOptions) -> ToolSpec {
    let available_models_description = (!options.hide_agent_type_model_reasoning)
        .then(|| spawn_agent_models_description(&options.available_models));
    let return_value_description =
        "Returns the spawned agent id plus the user-facing nickname when available.";
    let mut properties = spawn_agent_common_properties_v1(&options.agent_type_description);
    if options.hide_agent_type_model_reasoning {
        hide_spawn_agent_metadata_options(&mut properties);
    }

    ToolSpec::Function(ResponsesApiTool {
        name: "spawn_agent".to_string(),
        description: spawn_agent_tool_description(
            available_models_description.as_deref(),
            return_value_description,
            options.include_usage_hint,
            options.usage_hint_text,
        ),
        strict: false,
        defer_loading: None,
        parameters: JsonSchema::object(properties, /*required*/ None, Some(false.into())),
        output_schema: Some(spawn_agent_output_schema_v1()),
    })
}

pub fn create_spawn_agent_tool_v2(options: SpawnAgentToolOptions) -> ToolSpec {
    let available_models_description = (!options.hide_agent_type_model_reasoning)
        .then(|| spawn_agent_models_description(&options.available_models));
    let mut properties = spawn_agent_common_properties_v2(&options.agent_type_description);
    if options.hide_agent_type_model_reasoning {
        hide_spawn_agent_metadata_options(&mut properties);
    }
    properties.insert(
        "task_name".to_string(),
        JsonSchema::string(Some(
            "Task name for the new agent. Use lowercase letters, digits, and underscores."
                .to_string(),
        )),
    );

    ToolSpec::Function(ResponsesApiTool {
        name: "spawn_agent".to_string(),
        description: spawn_agent_tool_description_v2(
            available_models_description.as_deref(),
            options.include_usage_hint,
            options.usage_hint_text,
            options.max_concurrent_threads_per_session,
        ),
        strict: false,
        defer_loading: None,
        parameters: JsonSchema::object(
            properties,
            Some(vec!["task_name".to_string(), "message".to_string()]),
            Some(false.into()),
        ),
        output_schema: Some(spawn_agent_output_schema_v2(
            options.hide_agent_type_model_reasoning,
        )),
    })
}

pub fn create_send_input_tool_v1() -> ToolSpec {
    let properties = BTreeMap::from([
        (
            "target".to_string(),
            JsonSchema::string(Some("Agent id to message (from spawn_agent).".to_string())),
        ),
        (
            "message".to_string(),
            JsonSchema::string(Some(
                "Legacy plain-text message to send to the agent. Use either message or items."
                    .to_string(),
            )),
        ),
        ("items".to_string(), create_collab_input_items_schema()),
        (
            "interrupt".to_string(),
            JsonSchema::boolean(Some(
                "When true, stop the agent's current task and handle this immediately. When false (default), queue this message."
                    .to_string(),
            )),
        ),
    ]);

    ToolSpec::Function(ResponsesApiTool {
        name: "send_input".to_string(),
        description: "Send a message to an existing agent. Use interrupt=true to redirect work immediately. You should reuse the agent by send_input if you believe your assigned task is highly dependent on the context of a previous task."
            .to_string(),
        strict: false,
        defer_loading: None,
        parameters: JsonSchema::object(properties, Some(vec!["target".to_string()]), Some(false.into())),
        output_schema: Some(send_input_output_schema()),
    })
}

pub fn create_send_message_tool() -> ToolSpec {
    let properties = BTreeMap::from([
        (
            "target".to_string(),
            JsonSchema::string(Some(
                "Relative or canonical task name to message (from spawn_agent).".to_string(),
            )),
        ),
        (
            "message".to_string(),
            JsonSchema::string(Some(
                "Message text to queue on the target agent.".to_string(),
            )),
        ),
    ]);

    ToolSpec::Function(ResponsesApiTool {
        name: "send_message".to_string(),
        description: "Send a message to an existing agent. The message will be delivered promptly. Does not trigger a new turn."
            .to_string(),
        strict: false,
        defer_loading: None,
        parameters: JsonSchema::object(
            properties,
            Some(vec!["target".to_string(), "message".to_string()]),
            Some(false.into()),
        ),
        output_schema: None,
    })
}

pub fn create_followup_task_tool() -> ToolSpec {
    let properties = BTreeMap::from([
        (
            "target".to_string(),
            JsonSchema::string(Some(
                "Agent id or canonical task name to message (from spawn_agent).".to_string(),
            )),
        ),
        (
            "message".to_string(),
            JsonSchema::string(Some(
                "Message text to send to the target agent.".to_string(),
            )),
        ),
    ]);

    ToolSpec::Function(ResponsesApiTool {
        name: "followup_task".to_string(),
        description: "Send a message to an existing non-root target agent and trigger a turn in that target. If the target is currently mid-turn, the message is queued and will be used to start the target's next turn, after the current turn completes."
            .to_string(),
        strict: false,
        defer_loading: None,
        parameters: JsonSchema::object(properties, Some(vec!["target".to_string(), "message".to_string()]), Some(false.into())),
        output_schema: None,
    })
}

pub fn create_resume_agent_tool() -> ToolSpec {
    let properties = BTreeMap::from([(
        "id".to_string(),
        JsonSchema::string(Some("Agent id to resume.".to_string())),
    )]);

    ToolSpec::Function(ResponsesApiTool {
        name: "resume_agent".to_string(),
        description:
            "Resume a previously closed agent by id so it can receive send_input and wait_agent calls."
                .to_string(),
        strict: false,
        defer_loading: None,
        parameters: JsonSchema::object(properties, Some(vec!["id".to_string()]), Some(false.into())),
        output_schema: Some(resume_agent_output_schema()),
    })
}

pub fn create_wait_agent_tool_v1(options: WaitAgentTimeoutOptions) -> ToolSpec {
    ToolSpec::Function(ResponsesApiTool {
        name: "wait_agent".to_string(),
        description: "Wait for agents to reach a final status. Completed statuses may include the agent's final message. Returns empty status when timed out. Once the agent reaches a final status, a notification message will be received containing the same completed status."
            .to_string(),
        strict: false,
        defer_loading: None,
        parameters: wait_agent_tool_parameters_v1(options),
        output_schema: Some(wait_output_schema_v1()),
    })
}

pub fn create_wait_agent_tool_v2(options: WaitAgentTimeoutOptions) -> ToolSpec {
    ToolSpec::Function(ResponsesApiTool {
        name: "wait_agent".to_string(),
        description: "Wait for a mailbox update from any live agent, including queued messages and final-status notifications. Does not return the content; returns either a summary of which agents have updates (if any), or a timeout summary if no mailbox update arrives before the deadline."
            .to_string(),
        strict: false,
        defer_loading: None,
        parameters: wait_agent_tool_parameters_v2(options),
        output_schema: Some(wait_output_schema_v2()),
    })
}

pub fn create_list_agents_tool() -> ToolSpec {
    let properties = BTreeMap::from([(
        "path_prefix".to_string(),
        JsonSchema::string(Some(
            "Optional task-path prefix (not ending with trailing slash). Accepts the same relative or absolute task-path syntax."
                .to_string(),
        )),
    )]);

    ToolSpec::Function(ResponsesApiTool {
        name: "list_agents".to_string(),
        description:
            "List live agents in the current root thread tree. Optionally filter by task-path prefix."
                .to_string(),
        strict: false,
        defer_loading: None,
        parameters: JsonSchema::object(properties, /*required*/ None, Some(false.into())),
        output_schema: Some(list_agents_output_schema()),
    })
}

pub fn create_close_agent_tool_v1() -> ToolSpec {
    let properties = BTreeMap::from([(
        "target".to_string(),
        JsonSchema::string(Some("Agent id to close (from spawn_agent).".to_string())),
    )]);

    ToolSpec::Function(ResponsesApiTool {
        name: "close_agent".to_string(),
        description: "Close an agent and any open descendants when they are no longer needed, and return the target agent's previous status before shutdown was requested. Don't keep agents open for too long if they are not needed anymore.".to_string(),
        strict: false,
        defer_loading: None,
        parameters: JsonSchema::object(properties, Some(vec!["target".to_string()]), Some(false.into())),
        output_schema: Some(close_agent_output_schema()),
    })
}

pub fn create_close_agent_tool_v2() -> ToolSpec {
    let properties = BTreeMap::from([(
        "target".to_string(),
        JsonSchema::string(Some(
            "Agent id or canonical task name to close (from spawn_agent).".to_string(),
        )),
    )]);

    ToolSpec::Function(ResponsesApiTool {
        name: "close_agent".to_string(),
        description: "Close an agent and any open descendants when they are no longer needed, and return the target agent's previous status before shutdown was requested. Don't keep agents open for too long if they are not needed anymore.".to_string(),
        strict: false,
        defer_loading: None,
        parameters: JsonSchema::object(properties, Some(vec!["target".to_string()]), Some(false.into())),
        output_schema: Some(close_agent_output_schema()),
    })
}

fn agent_status_output_schema() -> Value {
    json!({
        "oneOf": [
            {
                "type": "string",
                "enum": ["pending_init", "running", "interrupted", "shutdown", "not_found"]
            },
            {
                "type": "object",
                "properties": {
                    "completed": {
                        "type": ["string", "null"]
                    }
                },
                "required": ["completed"],
                "additionalProperties": false
            },
            {
                "type": "object",
                "properties": {
                    "errored": {
                        "type": "string"
                    }
                },
                "required": ["errored"],
                "additionalProperties": false
            }
        ]
    })
}

fn spawn_agent_output_schema_v1() -> Value {
    json!({
        "type": "object",
        "properties": {
            "agent_id": {
                "type": "string",
                "description": "Thread identifier for the spawned agent."
            },
            "nickname": {
                "type": ["string", "null"],
                "description": "User-facing nickname for the spawned agent when available."
            }
        },
        "required": ["agent_id", "nickname"],
        "additionalProperties": false
    })
}

fn spawn_agent_output_schema_v2(hide_agent_metadata: bool) -> Value {
    if hide_agent_metadata {
        return json!({
            "type": "object",
            "properties": {
                "task_name": {
                    "type": "string",
                    "description": "Canonical task name for the spawned agent."
                }
            },
            "required": ["task_name"],
            "additionalProperties": false
        });
    }

    json!({
        "type": "object",
        "properties": {
            "task_name": {
                "type": "string",
                "description": "Canonical task name for the spawned agent."
            },
            "nickname": {
                "type": ["string", "null"],
                "description": "User-facing nickname for the spawned agent when available."
            }
        },
        "required": ["task_name", "nickname"],
        "additionalProperties": false
    })
}

fn send_input_output_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "submission_id": {
                "type": "string",
                "description": "Identifier for the queued input submission."
            }
        },
        "required": ["submission_id"],
        "additionalProperties": false
    })
}

fn list_agents_output_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "agents": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "agent_name": {
                            "type": "string",
                            "description": "Canonical task name for the agent when available, otherwise the agent id."
                        },
                        "agent_status": {
                            "description": "Last known status of the agent.",
                            "allOf": [agent_status_output_schema()]
                        },
                        "last_task_message": {
                            "type": ["string", "null"],
                            "description": "Most recent user or inter-agent instruction received by the agent, when available."
                        }
                    },
                    "required": ["agent_name", "agent_status", "last_task_message"],
                    "additionalProperties": false
                },
                "description": "Live agents visible in the current root thread tree."
            }
        },
        "required": ["agents"],
        "additionalProperties": false
    })
}

fn resume_agent_output_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "status": agent_status_output_schema()
        },
        "required": ["status"],
        "additionalProperties": false
    })
}

fn wait_output_schema_v1() -> Value {
    json!({
        "type": "object",
        "properties": {
            "status": {
                "type": "object",
                "description": "Final statuses keyed by agent id.",
                "additionalProperties": agent_status_output_schema()
            },
            "timed_out": {
                "type": "boolean",
                "description": "Whether the wait call returned due to timeout before any agent reached a final status."
            }
        },
        "required": ["status", "timed_out"],
        "additionalProperties": false
    })
}

fn wait_output_schema_v2() -> Value {
    json!({
        "type": "object",
        "properties": wait_output_schema_v2_properties(),
        "required": [
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
        ],
        "additionalProperties": false
    })
}

fn wait_output_schema_v2_properties() -> Map<String, Value> {
    let mut properties = Map::new();
    properties.insert(
        "message".to_string(),
        json!({
            "type": "string",
            "description": "Brief wait summary without the agent's final content."
        }),
    );
    properties.insert(
        "timed_out".to_string(),
        json!({
            "type": "boolean",
            "description": "Whether the wait call returned because no mailbox update arrived before the timeout."
        }),
    );
    properties.insert(
        "barrier_id".to_string(),
        json!({
            "type": "string",
            "description": "Durable wait barrier identifier recorded for this wait call."
        }),
    );
    properties.insert(
        "task_id".to_string(),
        json!({
            "type": ["string", "null"],
            "description": "Optional durable task id this wait barrier is associated with."
        }),
    );
    properties.insert(
        "task_name".to_string(),
        json!({
            "type": ["string", "null"],
            "description": "Optional task name this wait barrier is associated with."
        }),
    );
    properties.insert(
        "task_thread_id".to_string(),
        json!({
            "type": ["string", "null"],
            "description": "Resolved task thread id when result_required task waiting is active."
        }),
    );
    properties.insert(
        "task_status".to_string(),
        json!({
            "anyOf": [
                agent_status_output_schema(),
                {"type": "null"}
            ],
            "description": "Terminal task status when result_required task waiting is active. Completed statuses are redacted to avoid returning child final content."
        }),
    );
    properties.insert(
        "task_result".to_string(),
        json!({
            "type": ["object", "null"],
            "description": "Terminal TaskResult envelope when result_required task waiting is satisfied by durable TaskResult evidence."
        }),
    );
    properties.insert(
        "result_required".to_string(),
        json!({
            "type": "boolean",
            "description": "Whether this barrier expects a terminal TaskResult envelope before later result-aware waits can satisfy it."
        }),
    );
    properties.insert(
        "wait_condition".to_string(),
        json!({
            "type": "string",
            "enum": ["mailbox_change", "task_terminal_status", "task_result_evidence"],
            "description": "Condition that satisfied or timed out for this wait call."
        }),
    );
    insert_wait_shadow_object_property(
        &mut properties,
        "task_result_delivery_shadow",
        "Shadow-only TaskResultEnvelope delivery evidence for result_required waits. This does not mutate live parent state.",
    );
    insert_wait_shadow_object_property(
        &mut properties,
        "parent_reducer_shadow_receipt",
        "Shadow-only parent reducer receipt based on TaskResultEnvelope delivery evidence. This does not reduce into live parent WorkGraph state.",
    );
    insert_wait_shadow_object_property(
        &mut properties,
        "work_graph_wait_task_result_replay_consistency_decision",
        "Shadow-only replay/readback consistency decision comparing direct wait tool output with durable latest delivery and reducer events.",
    );
    insert_wait_shadow_object_property(
        &mut properties,
        "work_graph_wait_surface_audit_replay_consistency_decision",
        "Shadow-only replay/readback consistency decision comparing direct wait surface-audit packet output with durable latest surface-audit packet event.",
    );
    insert_wait_shadow_object_property(
        &mut properties,
        "work_graph_wait_task_result_readback",
        "Durable readback summary for result_required wait TaskResult delivery, parent reducer, replay consistency, and surface-audit shadow events.",
    );
    insert_wait_shadow_object_property(
        &mut properties,
        "work_graph_wait_operator_matrix_row",
        "Operator-facing matrix row for direct wait TaskResult delivery, parent reducer readiness, and the next canonical WorkGraph blocker.",
    );
    insert_wait_shadow_object_property(
        &mut properties,
        "work_graph_wait_surface_audit_packet",
        "Operator-facing surface-audit packet for direct wait TaskResult delivery, parent reducer, and replay/readback segments.",
    );
    insert_wait_shadow_object_property(
        &mut properties,
        "work_graph_global_surface_audit_packet",
        "Global WorkGraph surface-audit summary for direct wait, using the same operatorMatrixRows shape as agent_jobs surface audit output.",
    );
    insert_wait_shadow_object_property(
        &mut properties,
        "work_graph_canonical_projection_receipt",
        "Shadow-only canonical WorkGraph write/read projection receipt built from the global surface-audit operator matrix. Canonical writes and live read-model cutover remain disabled.",
    );
    insert_wait_shadow_object_property(
        &mut properties,
        "work_graph_canonical_projection_replay_consistency_decision",
        "Shadow-only replay/readback consistency decision comparing the canonical WorkGraph projection receipt in the tool output with the durable latest projection receipt payload. Mismatch only fails shadow readiness.",
    );
    insert_wait_shadow_object_property(
        &mut properties,
        "work_graph_canonical_projection_closeout_receipt",
        "Terminal no-cutover closeout receipt for the shadow-only canonical WorkGraph projection path. Canonical writes, reads, feature flags, canary, live blocking, and cutover remain disabled.",
    );
    insert_wait_shadow_object_property(
        &mut properties,
        "work_graph_canonical_projection_closeout_replay_consistency_decision",
        "Shadow-only replay/readback consistency decision for the canonical WorkGraph projection terminal closeout receipt. Mismatch only fails shadow readiness.",
    );
    insert_wait_shadow_object_property(
        &mut properties,
        "work_graph_canonical_projection_audit_chain_closeout_receipt",
        "Final terminal no-cutover receipt closing the canonical WorkGraph projection audit chain after projection, replay, closeout, and closeout replay evidence are read back.",
    );
    insert_wait_shadow_object_property(
        &mut properties,
        "work_graph_canonical_projection_audit_chain_closeout_replay_consistency_decision",
        "Shadow-only replay/readback consistency decision for the final canonical WorkGraph projection audit-chain closeout receipt. Mismatch only fails shadow readiness.",
    );
    insert_wait_shadow_object_property(
        &mut properties,
        "work_graph_canonical_projection_enablement_operator_review_packet",
        "Shadow-only operator-review packet preparing canonical WorkGraph projection enablement evidence from the full projection receipt/replay/closeout chain. It records no approval and keeps reviewed flag, feature flag, canary, blocking, and cutover disabled.",
    );
    insert_wait_shadow_object_property(
        &mut properties,
        "work_graph_canonical_projection_enablement_operator_review_replay_consistency_decision",
        "Shadow-only replay/readback consistency decision for the canonical WorkGraph projection enablement operator-review packet. Mismatch only fails shadow readiness; approval, reviewed flag, canary, blocking, and cutover remain disabled.",
    );
    insert_wait_shadow_object_property(
        &mut properties,
        "work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt",
        "Shadow-only no-live closeout receipt for canonical WorkGraph projection enablement rehearsal. It requires operator-review packet replay consistency while keeping approval, reviewed flag, canary, blocking, and cutover disabled.",
    );
    insert_wait_shadow_object_property(
        &mut properties,
        "work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_decision",
        "Shadow-only replay/readback consistency decision for the canonical WorkGraph projection enablement no-live rehearsal closeout receipt. Mismatch only fails shadow readiness; approval, reviewed flag, canary, blocking, and cutover remain disabled.",
    );
    insert_wait_shadow_object_property(
        &mut properties,
        "work_graph_canonical_projection_enablement_audit_chain_closeout_receipt",
        "Final shadow-only audit-chain closeout receipt for canonical WorkGraph projection enablement rehearsal. It consumes operator-review packet/replay and no-live closeout/replay evidence while keeping approval, reviewed flag, canary, blocking, and cutover disabled.",
    );
    insert_wait_shadow_object_property(
        &mut properties,
        "work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency_decision",
        "Shadow-only replay/readback consistency decision for the canonical WorkGraph projection enablement final audit-chain closeout receipt. Mismatch only fails shadow readiness; approval, reviewed flag, canary, blocking, and cutover remain disabled.",
    );
    insert_wait_shadow_object_property(
        &mut properties,
        "work_graph_canonical_projection_enablement_activation_precondition_operator_packet",
        "Shadow-only activation-precondition operator packet for canonical WorkGraph projection enablement. It consumes the final enablement audit-chain closeout replay evidence while keeping activationAllowed=false, approval recording, reviewed flag, canonical WorkGraph write/read, canary, blocking, and cutover disabled.",
    );
    insert_wait_shadow_object_property(
        &mut properties,
        "work_graph_canonical_projection_enablement_activation_precondition_replay_consistency_decision",
        "Shadow-only replay/readback consistency decision for the canonical WorkGraph projection enablement activation-precondition operator packet. Mismatch only fails shadow readiness; activation, approval recording, reviewed flag, canonical WorkGraph write/read, canary, blocking, and cutover remain disabled.",
    );
    insert_wait_shadow_object_property(
        &mut properties,
        "work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt",
        "Shadow-only no-live closeout receipt for canonical WorkGraph projection enablement activation preconditions. It consumes activation-precondition packet/replay evidence while keeping activationAllowed=false, approval recording, reviewed flag, canonical WorkGraph write/read, canary, blocking, and cutover disabled.",
    );
    insert_wait_shadow_object_property(
        &mut properties,
        "work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_decision",
        "Shadow-only replay/readback consistency decision for the canonical WorkGraph projection enablement activation no-live closeout receipt. Mismatch only fails shadow readiness; activation, approval recording, reviewed flag, canonical WorkGraph write/read, canary, blocking, and cutover remain disabled.",
    );
    insert_wait_shadow_object_property(
        &mut properties,
        "work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt",
        "Shadow-only final activation audit-chain closeout receipt for canonical WorkGraph projection enablement. It consumes activation-precondition packet/replay plus activation no-live closeout/replay evidence while keeping activation, approval recording, reviewed flag, canonical WorkGraph write/read, canary, blocking, and cutover disabled.",
    );
    insert_wait_shadow_object_property(
        &mut properties,
        "work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_decision",
        "Shadow-only replay/readback consistency decision for the canonical WorkGraph projection enablement final activation audit-chain closeout receipt. Mismatch only fails shadow readiness; activation, approval recording, reviewed flag, canonical WorkGraph write/read, canary, blocking, and cutover remain disabled.",
    );
    insert_wait_shadow_object_property(
        &mut properties,
        "work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet",
        "Shadow-only operator-approval/readiness preflight packet for canonical WorkGraph projection enablement activation. It consumes final activation closeout replay evidence, requires future approval record and reviewed flag prerequisites, and keeps activation, approval recording, reviewed flag mutation, canonical WorkGraph write/read, canary, blocking, and cutover disabled.",
    );
    insert_wait_shadow_object_property(
        &mut properties,
        "work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision",
        "Shadow-only replay/readback consistency decision for the canonical WorkGraph projection enablement activation operator-approval/readiness preflight packet. Mismatch only fails shadow readiness; approval recording, reviewed flag mutation, canonical WorkGraph write/read, canary, blocking, and cutover remain disabled.",
    );
    insert_wait_shadow_object_property(
        &mut properties,
        "work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet",
        "Shadow-only approval/review side-effect lock closeout packet for canonical WorkGraph projection enablement activation. It consumes operator-approval/readiness preflight replay evidence and proves approval recording, reviewed flag mutation, canonical WorkGraph write/read, canary, blocking, and cutover remain disabled.",
    );
    insert_wait_shadow_object_property(
        &mut properties,
        "work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_decision",
        "Shadow-only replay/readback consistency decision for the canonical WorkGraph projection enablement activation approval/review side-effect lock closeout packet. Mismatch only fails shadow readiness; approval recording, reviewed flag mutation, canonical WorkGraph write/read, canary, blocking, and cutover remain disabled.",
    );
    properties.insert(
        "durable_mailbox".to_string(),
        durable_mailbox_wait_metadata_schema(),
    );
    insert_wait_shadow_object_property(
        &mut properties,
        "work_graph_lifecycle_shadow_decision",
        "Shadow-only AgentCard lifecycle decision for allowed tool, budget, and lane checks. This is not live-blocking.",
    );
    properties
}

fn insert_wait_shadow_object_property(
    properties: &mut Map<String, Value>,
    name: &'static str,
    description: &'static str,
) {
    properties.insert(
        name.to_string(),
        json!({
            "type": "object",
            "additionalProperties": true,
            "description": description,
        }),
    );
}

fn durable_mailbox_wait_metadata_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "opened_event_recorded": {
                "type": "boolean",
                "description": "Whether the durable wait_barrier_opened shadow event was recorded."
            },
            "terminal_event_recorded": {
                "type": "boolean",
                "description": "Whether a durable satisfied/timed_out shadow event was recorded."
            },
            "task_result_delivery_shadow_event_recorded": {
                "type": "boolean",
                "description": "Whether result_required wait TaskResult delivery shadow evidence was written to the durable mailbox stream."
            },
            "parent_reducer_shadow_receipt_event_recorded": {
                "type": "boolean",
                "description": "Whether result_required wait parent reducer shadow evidence was written to the durable mailbox stream."
            },
            "task_result_replay_consistency_event_recorded": {
                "type": "boolean",
                "description": "Whether result_required wait replay/readback consistency evidence was written to the durable mailbox stream."
            },
            "wait_surface_audit_packet_event_recorded": {
                "type": "boolean",
                "description": "Whether result_required wait surface-audit packet shadow evidence was written to the durable mailbox stream."
            },
            "wait_surface_audit_replay_consistency_event_recorded": {
                "type": "boolean",
                "description": "Whether result_required wait surface-audit replay consistency evidence was written to the durable mailbox stream."
            },
            "wait_canonical_projection_receipt_event_recorded": {
                "type": "boolean",
                "description": "Whether result_required wait canonical WorkGraph projection receipt evidence was written to the durable mailbox stream."
            },
            "wait_canonical_projection_replay_consistency_event_recorded": {
                "type": "boolean",
                "description": "Whether result_required wait canonical WorkGraph projection replay consistency evidence was written to the durable mailbox stream."
            },
            "wait_canonical_projection_closeout_receipt_event_recorded": {
                "type": "boolean",
                "description": "Whether result_required wait canonical WorkGraph projection terminal no-cutover closeout evidence was written to the durable mailbox stream."
            },
            "wait_canonical_projection_closeout_replay_consistency_event_recorded": {
                "type": "boolean",
                "description": "Whether result_required wait canonical WorkGraph projection terminal closeout replay consistency evidence was written to the durable mailbox stream."
            },
            "wait_canonical_projection_audit_chain_closeout_receipt_event_recorded": {
                "type": "boolean",
                "description": "Whether result_required wait canonical WorkGraph projection final audit-chain closeout receipt evidence was written to the durable mailbox stream."
            },
            "wait_canonical_projection_audit_chain_closeout_replay_consistency_event_recorded": {
                "type": "boolean",
                "description": "Whether result_required wait canonical WorkGraph projection final audit-chain closeout replay consistency evidence was written to the durable mailbox stream."
            },
            "wait_canonical_projection_enablement_operator_review_packet_event_recorded": {
                "type": "boolean",
                "description": "Whether result_required wait canonical WorkGraph projection enablement operator-review packet evidence was written to the durable mailbox stream without approval or cutover."
            },
            "wait_canonical_projection_enablement_operator_review_replay_consistency_event_recorded": {
                "type": "boolean",
                "description": "Whether result_required wait canonical WorkGraph projection enablement operator-review replay consistency evidence was written to the durable mailbox stream without approval or cutover."
            },
            "wait_canonical_projection_enablement_no_live_rehearsal_closeout_event_recorded": {
                "type": "boolean",
                "description": "Whether result_required wait canonical WorkGraph projection enablement no-live rehearsal closeout evidence was written to the durable mailbox stream without approval or cutover."
            },
            "wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_event_recorded": {
                "type": "boolean",
                "description": "Whether result_required wait canonical WorkGraph projection enablement no-live rehearsal closeout replay consistency evidence was written to the durable mailbox stream without approval or cutover."
            },
            "wait_canonical_projection_enablement_audit_chain_closeout_event_recorded": {
                "type": "boolean",
                "description": "Whether result_required wait canonical WorkGraph projection enablement final audit-chain closeout evidence was written to the durable mailbox stream without approval or cutover."
            },
            "wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_event_recorded": {
                "type": "boolean",
                "description": "Whether result_required wait canonical WorkGraph projection enablement final audit-chain closeout replay consistency evidence was written to the durable mailbox stream without approval or cutover."
            },
            "wait_canonical_projection_enablement_activation_precondition_operator_packet_event_recorded": {
                "type": "boolean",
                "description": "Whether result_required wait canonical WorkGraph projection enablement activation-precondition operator packet evidence was written to the durable mailbox stream without approval, activation, or cutover."
            },
            "wait_canonical_projection_enablement_activation_precondition_replay_consistency_event_recorded": {
                "type": "boolean",
                "description": "Whether result_required wait canonical WorkGraph projection enablement activation-precondition replay consistency evidence was written to the durable mailbox stream without approval, activation, or cutover."
            },
            "wait_canonical_projection_enablement_activation_no_live_closeout_event_recorded": {
                "type": "boolean",
                "description": "Whether result_required wait canonical WorkGraph projection enablement activation no-live closeout evidence was written to the durable mailbox stream without approval, activation, or cutover."
            },
            "wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_event_recorded": {
                "type": "boolean",
                "description": "Whether result_required wait canonical WorkGraph projection enablement activation no-live closeout replay consistency evidence was written to the durable mailbox stream without approval, activation, or cutover."
            },
            "wait_canonical_projection_enablement_activation_audit_chain_closeout_event_recorded": {
                "type": "boolean",
                "description": "Whether result_required wait canonical WorkGraph projection enablement final activation audit-chain closeout evidence was written to the durable mailbox stream without approval, activation, reviewed flag mutation, or cutover."
            },
            "wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_event_recorded": {
                "type": "boolean",
                "description": "Whether result_required wait canonical WorkGraph projection enablement final activation audit-chain closeout replay consistency evidence was written to the durable mailbox stream without approval, activation, reviewed flag mutation, or cutover."
            },
            "wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_event_recorded": {
                "type": "boolean",
                "description": "Whether result_required wait canonical WorkGraph projection enablement activation operator-approval/readiness preflight evidence was written to the durable mailbox stream while requiring future approval record and reviewed flag prerequisites without approval recording, activation, reviewed flag mutation, or cutover."
            },
            "wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_event_recorded": {
                "type": "boolean",
                "description": "Whether result_required wait canonical WorkGraph projection enablement activation operator-approval/readiness preflight replay consistency evidence was written to the durable mailbox stream without approval recording, activation, reviewed flag mutation, or cutover."
            },
            "wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet_event_recorded": {
                "type": "boolean",
                "description": "Whether result_required wait canonical WorkGraph projection enablement activation approval/review side-effect lock closeout evidence was written to the durable mailbox stream without approval recording, reviewed flag mutation, activation, or cutover."
            },
            "wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_event_recorded": {
                "type": "boolean",
                "description": "Whether result_required wait canonical WorkGraph projection enablement activation approval/review side-effect lock closeout replay consistency evidence was written to the durable mailbox stream without approval recording, reviewed flag mutation, activation, or cutover."
            },
            "live_blocking_enabled": {
                "type": "boolean",
                "description": "Always false while durable wait barriers are shadow-only."
            },
            "live_cutover_enabled": {
                "type": "boolean",
                "description": "Always false while durable wait barriers are shadow-only."
            }
        },
        "required": [
            "opened_event_recorded",
            "terminal_event_recorded",
            "task_result_delivery_shadow_event_recorded",
            "parent_reducer_shadow_receipt_event_recorded",
            "task_result_replay_consistency_event_recorded",
            "wait_surface_audit_packet_event_recorded",
            "wait_surface_audit_replay_consistency_event_recorded",
            "wait_canonical_projection_receipt_event_recorded",
            "wait_canonical_projection_replay_consistency_event_recorded",
            "wait_canonical_projection_closeout_receipt_event_recorded",
            "wait_canonical_projection_closeout_replay_consistency_event_recorded",
            "wait_canonical_projection_audit_chain_closeout_receipt_event_recorded",
            "wait_canonical_projection_audit_chain_closeout_replay_consistency_event_recorded",
            "wait_canonical_projection_enablement_operator_review_packet_event_recorded",
            "wait_canonical_projection_enablement_operator_review_replay_consistency_event_recorded",
            "wait_canonical_projection_enablement_no_live_rehearsal_closeout_event_recorded",
            "wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_event_recorded",
            "wait_canonical_projection_enablement_audit_chain_closeout_event_recorded",
            "wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_event_recorded",
            "wait_canonical_projection_enablement_activation_precondition_operator_packet_event_recorded",
            "wait_canonical_projection_enablement_activation_precondition_replay_consistency_event_recorded",
            "wait_canonical_projection_enablement_activation_no_live_closeout_event_recorded",
            "wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_event_recorded",
            "wait_canonical_projection_enablement_activation_audit_chain_closeout_event_recorded",
            "wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_event_recorded",
            "wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_event_recorded",
            "wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_event_recorded",
            "wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet_event_recorded",
            "wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_event_recorded",
            "live_blocking_enabled",
            "live_cutover_enabled"
        ],
        "additionalProperties": false
    })
}

fn close_agent_output_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "previous_status": {
                "description": "The agent status observed before shutdown was requested.",
                "allOf": [agent_status_output_schema()]
            },
            "work_graph_lifecycle_shadow_decision": {
                "description": "Shadow-only AgentCard lifecycle decision for allowed tool, budget, and lane checks. This is not live-blocking.",
                "type": "object",
                "additionalProperties": true
            }
        },
        "required": ["previous_status", "work_graph_lifecycle_shadow_decision"],
        "additionalProperties": false
    })
}

fn create_collab_input_items_schema() -> JsonSchema {
    let properties = BTreeMap::from([
        (
            "type".to_string(),
            JsonSchema::string(Some(
                "Input item type: text, image, local_image, skill, or mention.".to_string(),
            )),
        ),
        (
            "text".to_string(),
            JsonSchema::string(Some("Text content when type is text.".to_string())),
        ),
        (
            "image_url".to_string(),
            JsonSchema::string(Some("Image URL when type is image.".to_string())),
        ),
        (
            "path".to_string(),
            JsonSchema::string(Some(
                "Path when type is local_image/skill, or structured mention target such as app://<connector-id> or plugin://<plugin-name>@<marketplace-name> when type is mention."
                    .to_string(),
            )),
        ),
        (
            "name".to_string(),
            JsonSchema::string(Some("Display name when type is skill or mention.".to_string())),
        ),
    ]);

    JsonSchema::array(JsonSchema::object(properties, /*required*/ None, Some(false.into())), Some(
            "Structured input items. Use this to pass explicit mentions (for example app:// connector paths)."
                .to_string(),
        ))
}

fn spawn_agent_common_properties_v1(agent_type_description: &str) -> BTreeMap<String, JsonSchema> {
    BTreeMap::from([
        (
            "message".to_string(),
            JsonSchema::string(Some(
                "Initial plain-text task for the new agent. Use either message or items."
                    .to_string(),
            )),
        ),
        ("items".to_string(), create_collab_input_items_schema()),
        (
            "agent_type".to_string(),
            JsonSchema::string(Some(agent_type_description.to_string())),
        ),
        (
            "fork_context".to_string(),
            JsonSchema::boolean(Some(
                "When true, fork the current thread history into the new agent before sending the initial prompt. This must be used when you want the new agent to have exactly the same context as you."
                    .to_string(),
            )),
        ),
        (
            "model".to_string(),
            JsonSchema::string(Some(
                SPAWN_AGENT_MODEL_OVERRIDE_DESCRIPTION.to_string(),
            )),
        ),
        (
            "reasoning_effort".to_string(),
            JsonSchema::string(Some(
                "Optional reasoning effort override for the new agent. Replaces the inherited reasoning effort."
                    .to_string(),
            )),
        ),
        (
            "service_tier".to_string(),
            JsonSchema::string(Some(
                SPAWN_AGENT_SERVICE_TIER_OVERRIDE_DESCRIPTION.to_string(),
            )),
        ),
    ])
}

fn spawn_agent_common_properties_v2(agent_type_description: &str) -> BTreeMap<String, JsonSchema> {
    BTreeMap::from([
        (
            "message".to_string(),
            JsonSchema::string(Some("Initial plain-text task for the new agent.".to_string())),
        ),
        (
            "agent_type".to_string(),
            JsonSchema::string(Some(agent_type_description.to_string())),
        ),
        (
            "fork_turns".to_string(),
            JsonSchema::string(Some(
                "Optional number of turns to fork. Defaults to `all`. Use `none`, `all`, or a positive integer string such as `3` to fork only the most recent turns."
                    .to_string(),
            )),
        ),
        (
            "model".to_string(),
            JsonSchema::string(Some(
                SPAWN_AGENT_MODEL_OVERRIDE_DESCRIPTION.to_string(),
            )),
        ),
        (
            "reasoning_effort".to_string(),
            JsonSchema::string(Some(
                "Optional reasoning effort override for the new agent. Replaces the inherited reasoning effort."
                    .to_string(),
            )),
        ),
        (
            "service_tier".to_string(),
            JsonSchema::string(Some(
                SPAWN_AGENT_SERVICE_TIER_OVERRIDE_DESCRIPTION.to_string(),
            )),
        ),
    ])
}

fn hide_spawn_agent_metadata_options(properties: &mut BTreeMap<String, JsonSchema>) {
    properties.remove("agent_type");
    properties.remove("model");
    properties.remove("reasoning_effort");
    properties.remove("service_tier");
}

fn spawn_agent_tool_description(
    available_models_description: Option<&str>,
    return_value_description: &str,
    include_usage_hint: bool,
    usage_hint_text: Option<String>,
) -> String {
    let agent_role_guidance = available_models_description.unwrap_or_default();

    let tool_description = format!(
        r#"
        {agent_role_guidance}
        Spawn a sub-agent for a well-scoped task. {return_value_description} {SPAWN_AGENT_INHERITED_MODEL_GUIDANCE}"#
    );

    if !include_usage_hint {
        return tool_description;
    }
    if let Some(usage_hint_text) = usage_hint_text {
        return format!(
            r#"
        {tool_description}
{usage_hint_text}"#
        );
    }
    let agent_role_usage_hint = available_models_description
        .map(|_| {
            "Agent-role guidance below only helps choose which agent to use after spawning is already authorized; it never authorizes spawning by itself."
        })
        .unwrap_or_default();
    format!(
        r#"
        {tool_description}
This spawn_agent tool provides you access to sub-agents that inherit your current model by default. Do not set the `model` field unless the user explicitly asks for a different model or there is a clear task-specific reason. You should follow the rules and guidelines below to use this tool.

Only use `spawn_agent` if and only if the user explicitly asks for sub-agents, delegation, or parallel agent work.
Requests for depth, thoroughness, research, investigation, or detailed codebase analysis do not count as permission to spawn.
{agent_role_usage_hint}

### When to delegate vs. do the subtask yourself
- First, quickly analyze the overall user task and form a succinct high-level plan. Identify which tasks are immediate blockers on the critical path, and which tasks are sidecar tasks that are needed but can run in parallel without blocking the next local step. As part of that plan, explicitly decide what immediate task you should do locally right now. Do this planning step before delegating to agents so you do not hand off the immediate blocking task to a submodel and then waste time waiting on it.
- Use a subagent when a subtask is easy enough for it to handle and can run in parallel with your local work. Prefer delegating concrete, bounded sidecar tasks that materially advance the main task without blocking your immediate next local step.
- Do not delegate urgent blocking work when your immediate next step depends on that result. If the very next action is blocked on that task, the main rollout should usually do it locally to keep the critical path moving.
- Keep work local when the subtask is too difficult to delegate well and when it is tightly coupled, urgent, or likely to block your immediate next step.

### Designing delegated subtasks
- Subtasks must be concrete, well-defined, and self-contained.
- Delegated subtasks must materially advance the main task.
- Do not duplicate work between the main rollout and delegated subtasks.
- Avoid issuing multiple delegate calls on the same unresolved thread unless the new delegated task is genuinely different and necessary.
- Narrow the delegated ask to the concrete output you need next.
- For coding tasks, prefer delegating concrete code-change worker subtasks over read-only explorer analysis when the subagent can make a bounded patch in a clear write scope.
- When delegating coding work, instruct the submodel to edit files directly in its forked workspace and list the file paths it changed in the final answer.
- For code-edit subtasks, decompose work so each delegated task has a disjoint write set.

### After you delegate
- Call wait_agent very sparingly. Only call wait_agent when you need the result immediately for the next critical-path step and you are blocked until it returns.
- Do not redo delegated subagent tasks yourself; focus on integrating results or tackling non-overlapping work.
- While the subagent is running in the background, do meaningful non-overlapping work immediately.
- Do not repeatedly wait by reflex.
- When a delegated coding task returns, quickly review the uploaded changes, then integrate or refine them.

### Parallel delegation patterns
- Run multiple independent information-seeking subtasks in parallel when you have distinct questions that can be answered independently.
- Split implementation into disjoint codebase slices and spawn multiple agents for them in parallel when the write scopes do not overlap.
- Delegate verification only when it can run in parallel with ongoing implementation and is likely to catch a concrete risk before final integration.
- The key is to find opportunities to spawn multiple independent subtasks in parallel within the same round, while ensuring each subtask is well-defined, self-contained, and materially advances the main task."#
    )
}

fn spawn_agent_tool_description_v2(
    available_models_description: Option<&str>,
    include_usage_hint: bool,
    usage_hint_text: Option<String>,
    max_concurrent_threads_per_session: Option<usize>,
) -> String {
    let agent_role_guidance = available_models_description.unwrap_or_default();
    let concurrency_guidance = max_concurrent_threads_per_session
        .map(|limit| {
            format!(
                "This session is configured with `max_concurrent_threads_per_session = {limit}` for concurrently open agent threads."
            )
        })
        .unwrap_or_default();

    let tool_description = format!(
        r#"
        {agent_role_guidance}
        Spawns an agent to work on the specified task. If your current task is `/root/task1` and you spawn_agent with task_name "task_3" the agent will have canonical task name `/root/task1/task_3`.
You are then able to refer to this agent as `task_3` or `/root/task1/task_3` interchangeably. However an agent `/root/task2/task_3` would only be able to communicate with this agent via its canonical name `/root/task1/task_3`.
The spawned agent will have the same tools as you and the ability to spawn its own subagents.
{SPAWN_AGENT_INHERITED_MODEL_GUIDANCE}
It will be able to send you and other running agents messages, and its final answer will be provided to you when it finishes.
The new agent's canonical task name will be provided to it along with the message.
{concurrency_guidance}"#
    );

    if !include_usage_hint {
        return tool_description;
    }
    if let Some(usage_hint_text) = usage_hint_text {
        return format!(
            r#"
        {tool_description}
{usage_hint_text}"#
        );
    }
    tool_description
}

fn spawn_agent_models_description(models: &[ModelPreset]) -> String {
    let visible_models: Vec<&ModelPreset> =
        models.iter().filter(|model| model.show_in_picker).collect();
    if visible_models.is_empty() {
        return "No picker-visible model overrides are currently loaded.".to_string();
    }

    let model_descriptions = visible_models
        .into_iter()
        .map(|model| {
            let efforts = model
                .supported_reasoning_efforts
                .iter()
                .map(|preset| format!("{} ({})", preset.effort, preset.description))
                .collect::<Vec<_>>()
                .join(", ");
            let service_tiers = if model.service_tiers.is_empty() {
                "none".to_string()
            } else {
                model
                    .service_tiers
                    .iter()
                    .map(|tier| format!("{} ({}: {})", tier.id, tier.name, tier.description))
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            format!(
                "- {} (`{}`): {} Default reasoning effort: {}. Supported reasoning efforts: {}. Supported service tiers: {}.",
                model.display_name,
                model.model,
                model.description,
                model.default_reasoning_effort,
                efforts,
                service_tiers
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "Available model overrides (optional; inherited parent model is preferred):\n{model_descriptions}"
    )
}

fn wait_agent_tool_parameters_v1(options: WaitAgentTimeoutOptions) -> JsonSchema {
    let properties = BTreeMap::from([
        (
            "targets".to_string(),
            JsonSchema::array(
                JsonSchema::string(/*description*/ None),
                Some(
                    "Agent ids to wait on. Pass multiple ids to wait for whichever finishes first."
                        .to_string(),
                ),
            ),
        ),
        (
            "timeout_ms".to_string(),
            JsonSchema::number(Some(format!(
                "Optional timeout in milliseconds. Defaults to {}, min {}, max {}. Prefer longer waits (minutes) to avoid busy polling.",
                options.default_timeout_ms, options.min_timeout_ms, options.max_timeout_ms,
            ))),
        ),
    ]);

    JsonSchema::object(
        properties,
        Some(vec!["targets".to_string()]),
        Some(false.into()),
    )
}

fn wait_agent_tool_parameters_v2(options: WaitAgentTimeoutOptions) -> JsonSchema {
    let properties = BTreeMap::from([
        (
            "timeout_ms".to_string(),
            JsonSchema::number(Some(format!(
                "Optional timeout in milliseconds. Defaults to {}, min {}, max {}.",
                options.default_timeout_ms, options.min_timeout_ms, options.max_timeout_ms,
            ))),
        ),
        (
            "task_name".to_string(),
            JsonSchema::string(Some(
                "Optional task name to associate with the durable wait barrier.".to_string(),
            )),
        ),
        (
            "task_id".to_string(),
            JsonSchema::string(Some(
                "Optional durable task id to associate with the wait barrier.".to_string(),
            )),
        ),
        (
            "barrier_id".to_string(),
            JsonSchema::string(Some(
                "Optional durable barrier id. Defaults to a generated wait-agent barrier id."
                    .to_string(),
            )),
        ),
        (
            "result_required".to_string(),
            JsonSchema::boolean(Some(
                "Whether this wait requires a terminal TaskResult envelope in future result-aware waits."
                    .to_string(),
            )),
        ),
    ]);

    JsonSchema::object(properties, /*required*/ None, Some(false.into()))
}

#[cfg(test)]
#[path = "multi_agents_spec_tests.rs"]
mod tests;
