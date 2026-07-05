use crate::function_tool::FunctionCallError;
use crate::tools::context::FunctionToolOutput;
use crate::tools::context::ToolInvocation;
use crate::tools::context::ToolPayload;
use crate::tools::context::boxed_tool_output;
use crate::tools::handlers::agent_jobs_spec::create_report_agent_job_result_tool;
use crate::tools::handlers::work_graph_admission::WorkGraphAgentCardManifestObservation;
use crate::tools::handlers::work_graph_admission::WorkGraphRoleManifestShadowDecision;
use crate::tools::handlers::work_graph_admission::agent_job_result_agent_card_manifest;
use crate::tools::handlers::work_graph_admission::build_agent_card_manifest_shadow_decision;
use crate::tools::handlers::work_graph_admission::configured_agent_role_manifest_source;
use crate::tools::registry::CoreToolRuntime;
use crate::tools::registry::ToolExecutor;
use codex_tools::ToolName;
use codex_tools::ToolSpec;

use super::*;

pub struct ReportAgentJobResultHandler;

#[async_trait::async_trait]
impl ToolExecutor<ToolInvocation> for ReportAgentJobResultHandler {
    fn tool_name(&self) -> ToolName {
        ToolName::plain("report_agent_job_result")
    }

    fn spec(&self) -> Option<ToolSpec> {
        Some(create_report_agent_job_result_tool())
    }

    async fn handle(
        &self,
        invocation: ToolInvocation,
    ) -> Result<Box<dyn crate::tools::context::ToolOutput>, FunctionCallError> {
        let ToolInvocation {
            session,
            turn,
            payload,
            ..
        } = invocation;

        let arguments = match payload {
            ToolPayload::Function { arguments } => arguments,
            _ => {
                return Err(FunctionCallError::RespondToModel(
                    "report_agent_job_result handler received unsupported payload".to_string(),
                ));
            }
        };

        handle(session, turn, arguments)
            .await
            .map(boxed_tool_output)
    }
}

impl CoreToolRuntime for ReportAgentJobResultHandler {
    fn matches_kind(&self, payload: &ToolPayload) -> bool {
        matches!(payload, ToolPayload::Function { .. })
    }
}

pub async fn handle(
    session: Arc<Session>,
    turn: Arc<TurnContext>,
    arguments: String,
) -> Result<FunctionToolOutput, FunctionCallError> {
    let args: ReportAgentJobResultArgs = parse_arguments(arguments.as_str())?;
    validate_result_object(&args.result)?;
    let db = required_state_db(&session)?;
    let job = db
        .get_agent_job(args.job_id.as_str())
        .await
        .map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to load agent job {} before recording result: {err}",
                args.job_id
            ))
        })?
        .ok_or_else(|| {
            FunctionCallError::RespondToModel(format!("agent job {} was not found", args.job_id))
        })?;
    validate_result_against_output_schema(job.output_schema_json.as_ref(), &args.result)?;
    let reporting_thread_id = session.conversation_id.to_string();
    let accepted_task_result = build_agent_job_task_result_envelope(
        &args,
        true,
        reporting_thread_id.as_str(),
        turn.trace_id.as_deref(),
        job.output_schema_json.is_some(),
    );
    let accepted_task_result_json = serde_json::to_value(&accepted_task_result).map_err(|err| {
        FunctionCallError::Fatal(format!(
            "failed to serialize accepted TaskResult envelope: {err}"
        ))
    })?;
    let accepted = db
        .report_agent_job_item_result(
            args.job_id.as_str(),
            args.item_id.as_str(),
            reporting_thread_id.as_str(),
            &args.result,
            Some(&accepted_task_result_json),
        )
        .await
        .map_err(|err| {
            let job_id = args.job_id.as_str();
            let item_id = args.item_id.as_str();
            FunctionCallError::RespondToModel(format!(
                "failed to record agent job result for {job_id} / {item_id}: {err}"
            ))
        })?;
    if accepted && args.stop.unwrap_or(false) {
        let message = "cancelled by worker request";
        let _ = db
            .mark_agent_job_cancelled(args.job_id.as_str(), message)
            .await;
    }
    let work_graph_report_only = build_work_graph_report_only_emission(
        &args,
        accepted,
        reporting_thread_id.as_str(),
        turn.trace_id.as_deref(),
    );
    let task_result = if accepted {
        accepted_task_result
    } else {
        build_agent_job_task_result_envelope(
            &args,
            false,
            reporting_thread_id.as_str(),
            turn.trace_id.as_deref(),
            job.output_schema_json.is_some(),
        )
    };
    let content = serde_json::to_string(&ReportAgentJobResultToolResult {
        accepted,
        task_result,
        work_graph_report_only,
        work_graph_lifecycle_shadow_decision: build_report_result_lifecycle_shadow_decision(
            job.output_schema_json.is_some(),
            turn.config.agent_roles.get("agent_job_worker"),
        ),
    })
    .map_err(|err| {
        FunctionCallError::Fatal(format!(
            "failed to serialize report_agent_job_result result: {err}"
        ))
    })?;
    Ok(FunctionToolOutput::from_text(content, Some(true)))
}

fn build_report_result_lifecycle_shadow_decision(
    output_schema_present: bool,
    configured_role: Option<&crate::config::AgentRoleConfig>,
) -> WorkGraphRoleManifestShadowDecision {
    build_agent_card_manifest_shadow_decision(
        agent_job_result_agent_card_manifest(),
        WorkGraphAgentCardManifestObservation {
            role_name: None,
            role_declared: true,
            role_description_present: true,
            configured_manifest_source: configured_agent_role_manifest_source(
                Some("agent_job_worker"),
                configured_role.is_some(),
                configured_role.is_some_and(|role| role.config_file.is_some()),
                configured_role.and_then(|role| role.agent_card_manifest_source.as_deref()),
            ),
            configured_manifest_version: configured_role
                .and_then(|role| role.agent_card_manifest_version.clone()),
            configured_manifest_overlay: configured_role
                .and_then(|role| role.agent_card_manifest.clone()),
            budget_present: true,
            output_contract_present: Some(output_schema_present),
            result_contract_present: None,
            verifier_present: None,
            reducer_present: None,
            attempted_tool: Some("report_agent_job_result"),
            observed_lane: Some("agent_jobs"),
        },
    )
}

fn validate_result_object(result: &Value) -> Result<(), FunctionCallError> {
    if !result.is_object() {
        return Err(FunctionCallError::RespondToModel(
            "result must be a JSON object".to_string(),
        ));
    }
    Ok(())
}

fn validate_result_against_output_schema(
    output_schema: Option<&Value>,
    result: &Value,
) -> Result<(), FunctionCallError> {
    let Some(schema) = output_schema else {
        return Ok(());
    };
    validate_schema_value("result", schema, result)
}

fn validate_schema_value(
    path: &str,
    schema: &Value,
    value: &Value,
) -> Result<(), FunctionCallError> {
    let Some(schema_object) = schema.as_object() else {
        return Err(schema_validation_error(
            path,
            "output_schema entries must be JSON objects",
        ));
    };

    if let Some(expected_type) = schema_object.get("type") {
        validate_schema_type(path, expected_type, value)?;
    }
    if let Some(enum_values) = schema_object.get("enum") {
        validate_schema_enum(path, enum_values, value)?;
    }
    if let Some(const_value) = schema_object.get("const")
        && value != const_value
    {
        return Err(schema_validation_error(path, "does not match const value"));
    }
    if let Some(required) = schema_object.get("required") {
        validate_schema_required(path, required, value)?;
    }
    if let Some(properties) = schema_object.get("properties") {
        validate_schema_properties(path, properties, value)?;
    }
    if let Some(additional_properties) = schema_object.get("additionalProperties") {
        validate_schema_additional_properties(path, additional_properties, schema_object, value)?;
    }
    if let Some(items) = schema_object.get("items") {
        validate_schema_items(path, items, value)?;
    }

    Ok(())
}

fn validate_schema_type(
    path: &str,
    expected_type: &Value,
    value: &Value,
) -> Result<(), FunctionCallError> {
    let accepted_types = match expected_type {
        Value::String(value) => vec![value.as_str()],
        Value::Array(values) => values
            .iter()
            .map(|value| {
                value
                    .as_str()
                    .ok_or_else(|| schema_validation_error(path, "type entries must be strings"))
            })
            .collect::<Result<Vec<_>, _>>()?,
        _ => {
            return Err(schema_validation_error(
                path,
                "type must be a string or an array of strings",
            ));
        }
    };
    if accepted_types
        .iter()
        .any(|expected| schema_type_matches(expected, value))
    {
        return Ok(());
    }
    Err(schema_validation_error(
        path,
        format!(
            "expected type {} but got {}",
            accepted_types.join("|"),
            value_schema_type(value)
        ),
    ))
}

fn validate_schema_enum(
    path: &str,
    enum_values: &Value,
    value: &Value,
) -> Result<(), FunctionCallError> {
    let Some(values) = enum_values.as_array() else {
        return Err(schema_validation_error(path, "enum must be an array"));
    };
    if values.iter().any(|candidate| candidate == value) {
        return Ok(());
    }
    Err(schema_validation_error(
        path,
        "does not match any enum value",
    ))
}

fn validate_schema_required(
    path: &str,
    required: &Value,
    value: &Value,
) -> Result<(), FunctionCallError> {
    let Some(required_fields) = required.as_array() else {
        return Err(schema_validation_error(path, "required must be an array"));
    };
    let Some(object) = value.as_object() else {
        return Err(schema_validation_error(
            path,
            "required can only be applied to objects",
        ));
    };
    for field in required_fields {
        let Some(field) = field.as_str() else {
            return Err(schema_validation_error(
                path,
                "required entries must be strings",
            ));
        };
        if !object.contains_key(field) {
            return Err(schema_validation_error(
                path,
                format!("missing required property {field}"),
            ));
        }
    }
    Ok(())
}

fn validate_schema_properties(
    path: &str,
    properties: &Value,
    value: &Value,
) -> Result<(), FunctionCallError> {
    let Some(properties) = properties.as_object() else {
        return Err(schema_validation_error(
            path,
            "properties must be an object",
        ));
    };
    let Some(object) = value.as_object() else {
        return Err(schema_validation_error(
            path,
            "properties can only be applied to objects",
        ));
    };
    for (property, property_schema) in properties {
        let Some(property_value) = object.get(property) else {
            continue;
        };
        validate_schema_value(
            format!("{path}.{property}").as_str(),
            property_schema,
            property_value,
        )?;
    }
    Ok(())
}

fn validate_schema_additional_properties(
    path: &str,
    additional_properties: &Value,
    schema_object: &serde_json::Map<String, Value>,
    value: &Value,
) -> Result<(), FunctionCallError> {
    let Some(object) = value.as_object() else {
        return Err(schema_validation_error(
            path,
            "additionalProperties can only be applied to objects",
        ));
    };
    let known_properties = schema_object
        .get("properties")
        .and_then(Value::as_object)
        .map_or_else(Default::default, |properties| {
            properties
                .keys()
                .cloned()
                .collect::<std::collections::HashSet<_>>()
        });
    match additional_properties {
        Value::Bool(true) => Ok(()),
        Value::Bool(false) => {
            for property in object.keys() {
                if !known_properties.contains(property) {
                    return Err(schema_validation_error(
                        path,
                        format!("unexpected additional property {property}"),
                    ));
                }
            }
            Ok(())
        }
        Value::Object(_) => {
            for (property, property_value) in object {
                if !known_properties.contains(property) {
                    validate_schema_value(
                        format!("{path}.{property}").as_str(),
                        additional_properties,
                        property_value,
                    )?;
                }
            }
            Ok(())
        }
        _ => Err(schema_validation_error(
            path,
            "additionalProperties must be a boolean or object",
        )),
    }
}

fn validate_schema_items(
    path: &str,
    items: &Value,
    value: &Value,
) -> Result<(), FunctionCallError> {
    let Some(array) = value.as_array() else {
        return Err(schema_validation_error(
            path,
            "items can only be applied to arrays",
        ));
    };
    for (index, item) in array.iter().enumerate() {
        validate_schema_value(format!("{path}[{index}]").as_str(), items, item)?;
    }
    Ok(())
}

fn schema_type_matches(expected: &str, value: &Value) -> bool {
    match expected {
        "object" => value.is_object(),
        "array" => value.is_array(),
        "string" => value.is_string(),
        "number" => value.is_number(),
        "integer" => value.as_i64().is_some() || value.as_u64().is_some(),
        "boolean" => value.is_boolean(),
        "null" => value.is_null(),
        _ => false,
    }
}

fn value_schema_type(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(number) if number.as_i64().is_some() || number.as_u64().is_some() => {
            "integer"
        }
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

fn schema_validation_error(path: &str, reason: impl Into<String>) -> FunctionCallError {
    FunctionCallError::RespondToModel(format!(
        "result does not match output_schema at {path}: {}",
        reason.into()
    ))
}

fn build_agent_job_task_result_envelope(
    args: &ReportAgentJobResultArgs,
    accepted: bool,
    reporting_thread_id: &str,
    trace_id: Option<&str>,
    output_schema_validated: bool,
) -> AgentJobTaskResultEnvelope {
    let status = if accepted { "completed" } else { "blocked" };
    let summary = if accepted {
        "agent job item result accepted as terminal TaskResult envelope"
    } else {
        "agent job item result was not accepted by the assigned task guard"
    };
    let risk = if accepted {
        "TaskResult envelope is durable shadow evidence until WorkGraph result waits are promoted"
    } else {
        "blocked TaskResult envelope must not satisfy result_required waits"
    };
    let next_action = if accepted {
        "allow result_required waits to satisfy from this TaskResult evidence after projection lookup is wired"
    } else {
        "keep the job item unresolved until the assigned worker reports a valid result"
    };

    AgentJobTaskResultEnvelope {
        schema_version: "hepta.task_result.v1",
        task_id: format!("agent-job:{}:{}", args.job_id, args.item_id),
        status: status.to_string(),
        summary: summary.to_string(),
        artifacts: vec![format!("agent-job-result-ref:{}", args.item_id)],
        evidence: vec![
            format!("agent_job_id:{}", args.job_id),
            format!("agent_job_item_id:{}", args.item_id),
            format!("reporting_thread_id:{reporting_thread_id}"),
            "result_object:true".to_string(),
            format!("output_schema_validated:{output_schema_validated}"),
        ],
        risks: vec![risk.to_string()],
        next_actions: vec![next_action.to_string()],
        verifier: "agent_job_task_result_envelope_validator".to_string(),
        reducer: "agent_job_task_result_reducer".to_string(),
        usage: AgentJobTaskResultUsage {
            model_tokens: 0,
            tool_calls: 1,
            command_count: 0,
            budget_state: "not_debited_shadow_task_result",
        },
        trace_id: trace_id
            .filter(|value| !value.trim().is_empty())
            .map(str::to_string)
            .unwrap_or_else(|| format!("trace-agent-job-task-result-{}", args.job_id)),
        span_id: format!("span-agent-job-task-result-{}", args.item_id),
        source_surface_id: "agent_jobs_batch_workers",
        result_contract: "agent_job_task_result_v1",
        output_schema_validated,
        live_blocking_enabled: false,
        live_cutover_enabled: false,
    }
}

fn build_work_graph_report_only_emission(
    args: &ReportAgentJobResultArgs,
    accepted: bool,
    reporting_thread_id: &str,
    trace_id: Option<&str>,
) -> AgentJobWorkGraphReportOnlyEmission {
    let status = if accepted { "succeeded" } else { "blocked" };
    let summary = if accepted {
        "agent job item result accepted for TaskResultEnvelope shadow validation"
    } else {
        "agent job item result was not accepted for the reporting thread"
    };
    let risk = if accepted {
        "tool response emission is not the authoritative WorkGraph event; state shadow event remains non-live"
    } else {
        "result remains unaccepted and must not reduce into terminal task state"
    };
    let next_action = if accepted {
        "compare state runtime shadow projection before promotion"
    } else {
        "leave job item unresolved until a valid worker reports"
    };

    AgentJobWorkGraphReportOnlyEmission {
        task_id: format!("agent-job:{}:{}", args.job_id, args.item_id),
        status: status.to_string(),
        summary: summary.to_string(),
        artifacts: vec![format!("agent-job-result-ref:{}", args.item_id)],
        evidence: vec![
            format!("agent_job_id:{}", args.job_id),
            format!("agent_job_item_id:{}", args.item_id),
            format!("reporting_thread_id:{reporting_thread_id}"),
        ],
        risks: vec![risk.to_string()],
        next_actions: vec![next_action.to_string()],
        verifier: "agent_job_result_report_only_verifier".to_string(),
        reducer: "agent_job_item_result_reducer".to_string(),
        usage: AgentJobWorkGraphReportOnlyUsage {
            model_tokens: 0,
            tool_calls: 1,
            command_count: 0,
            budget_state: "not_debited_report_only",
        },
        trace_id: trace_id
            .filter(|value| !value.trim().is_empty())
            .map(str::to_string)
            .unwrap_or_else(|| format!("trace-agent-job-report-only-{}", args.job_id)),
        span_id: format!("span-agent-job-result-{}", args.item_id),
        source_surface_id: "agent_jobs_batch_workers",
        admission_decision: "allow_report_only_no_live_blocking",
        feature_flag_id: "work_graph_agent_jobs_non_blocking_canary",
        feature_flag_enabled: false,
        canary_stage: "shadow_0ppm_report_only",
        canary_traffic_ppm: 0,
        readback_required: true,
        rollback_replay_required: true,
        blocking_guardrail_preview: true,
        live_blocking_enabled: false,
        live_cutover_enabled: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn assert_schema_error_contains(result: Result<(), FunctionCallError>, expected: &str) {
        let Err(FunctionCallError::RespondToModel(message)) = result else {
            panic!("expected schema validation error");
        };
        assert!(
            message.contains(expected),
            "expected {message:?} to contain {expected:?}"
        );
    }

    #[test]
    fn output_schema_validation_accepts_required_typed_result() {
        let schema = json!({
            "type": "object",
            "required": ["answer", "confidence"],
            "properties": {
                "answer": { "type": "string" },
                "confidence": { "type": "number" },
                "tags": {
                    "type": "array",
                    "items": { "type": "string" }
                }
            },
            "additionalProperties": false
        });

        validate_result_against_output_schema(
            Some(&schema),
            &json!({
                "answer": "ok",
                "confidence": 0.92,
                "tags": ["reviewed", "safe"]
            }),
        )
        .expect("schema should accept valid result");
    }

    #[test]
    fn output_schema_validation_rejects_missing_required_property() {
        let schema = json!({
            "type": "object",
            "required": ["answer"],
            "properties": {
                "answer": { "type": "string" }
            }
        });

        assert_schema_error_contains(
            validate_result_against_output_schema(Some(&schema), &json!({})),
            "missing required property answer",
        );
    }

    #[test]
    fn output_schema_validation_rejects_property_type_mismatch() {
        let schema = json!({
            "type": "object",
            "properties": {
                "score": { "type": "integer" }
            }
        });

        assert_schema_error_contains(
            validate_result_against_output_schema(Some(&schema), &json!({ "score": 1.5 })),
            "result.score",
        );
    }

    #[test]
    fn output_schema_validation_rejects_additional_properties_when_disabled() {
        let schema = json!({
            "type": "object",
            "properties": {
                "answer": { "type": "string" }
            },
            "additionalProperties": false
        });

        assert_schema_error_contains(
            validate_result_against_output_schema(
                Some(&schema),
                &json!({ "answer": "ok", "extra": true }),
            ),
            "unexpected additional property extra",
        );
    }

    #[test]
    fn output_schema_validation_checks_enum_values() {
        let schema = json!({
            "type": "object",
            "properties": {
                "status": { "enum": ["pass", "fail"] }
            }
        });

        assert_schema_error_contains(
            validate_result_against_output_schema(Some(&schema), &json!({ "status": "skip" })),
            "does not match any enum value",
        );
    }

    #[test]
    fn work_graph_report_only_emission_keeps_agent_job_result_non_blocking() {
        let args = ReportAgentJobResultArgs {
            job_id: "job-001".to_string(),
            item_id: "row-1".to_string(),
            result: json!({ "ok": true }),
            stop: None,
        };

        let emission = build_work_graph_report_only_emission(
            &args,
            true,
            "thread-001",
            Some("trace-existing-001"),
        );

        assert_eq!(emission.task_id, "agent-job:job-001:row-1");
        assert_eq!(emission.status, "succeeded");
        assert!(emission.summary.contains("TaskResultEnvelope"));
        assert_eq!(emission.trace_id, "trace-existing-001");
        assert_eq!(emission.span_id, "span-agent-job-result-row-1");
        assert_eq!(emission.source_surface_id, "agent_jobs_batch_workers");
        assert_eq!(
            emission.feature_flag_id,
            "work_graph_agent_jobs_non_blocking_canary"
        );
        assert!(!emission.feature_flag_enabled);
        assert_eq!(emission.canary_stage, "shadow_0ppm_report_only");
        assert_eq!(emission.canary_traffic_ppm, 0);
        assert!(emission.readback_required);
        assert!(emission.rollback_replay_required);
        assert!(emission.blocking_guardrail_preview);
        assert!(!emission.live_blocking_enabled);
        assert!(!emission.live_cutover_enabled);
        assert!(
            emission
                .evidence
                .iter()
                .any(|evidence| evidence == "reporting_thread_id:thread-001")
        );
    }

    #[test]
    fn report_agent_job_result_lifecycle_shadow_allows_agent_job_lane() {
        let decision = build_report_result_lifecycle_shadow_decision(true, None);

        assert_eq!(decision.definition_source, "explicit_agent_card_manifest");
        assert_eq!(
            decision.manifest_id,
            "agent-card:report_agent_job_result:agent_job_worker"
        );
        assert_eq!(decision.attempted_tool, Some("report_agent_job_result"));
        assert_eq!(decision.tool_allowed, Some(true));
        assert_eq!(decision.observed_lane, Some("agent_jobs"));
        assert_eq!(decision.lane_allowed, Some(true));
        assert!(decision.result_contract_present);
        assert!(decision.verifier_present);
        assert!(decision.reducer_present);
        assert!(decision.denial_reasons.is_empty());
        assert!(!decision.live_cutover_enabled);
    }

    #[test]
    fn task_result_envelope_is_terminal_shadow_evidence() {
        let args = ReportAgentJobResultArgs {
            job_id: "job-001".to_string(),
            item_id: "row-1".to_string(),
            result: json!({ "ok": true }),
            stop: None,
        };

        let envelope = build_agent_job_task_result_envelope(
            &args,
            true,
            "thread-001",
            Some("trace-existing-001"),
            true,
        );

        assert_eq!(envelope.schema_version, "hepta.task_result.v1");
        assert_eq!(envelope.task_id, "agent-job:job-001:row-1");
        assert_eq!(envelope.status, "completed");
        assert_eq!(envelope.trace_id, "trace-existing-001");
        assert_eq!(
            envelope.span_id,
            "span-agent-job-task-result-row-1".to_string()
        );
        assert_eq!(envelope.result_contract, "agent_job_task_result_v1");
        assert!(envelope.output_schema_validated);
        assert!(!envelope.live_blocking_enabled);
        assert!(!envelope.live_cutover_enabled);
        assert!(
            envelope
                .evidence
                .iter()
                .any(|evidence| evidence == "reporting_thread_id:thread-001")
        );
    }
}
