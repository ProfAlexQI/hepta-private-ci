use crate::function_tool::FunctionCallError;
use crate::tools::context::FunctionToolOutput;
use crate::tools::context::ToolInvocation;
use crate::tools::context::ToolPayload;
use crate::tools::context::boxed_tool_output;
use crate::tools::handlers::agent_jobs_spec::create_spawn_agents_on_csv_tool;
use crate::tools::registry::CoreToolRuntime;
use crate::tools::registry::ToolExecutor;
use codex_tools::ToolName;
use codex_tools::ToolSpec;
use codex_utils_absolute_path::AbsolutePathBuf;

use super::*;

pub struct SpawnAgentsOnCsvHandler;

#[async_trait::async_trait]
impl ToolExecutor<ToolInvocation> for SpawnAgentsOnCsvHandler {
    fn tool_name(&self) -> ToolName {
        ToolName::plain("spawn_agents_on_csv")
    }

    fn spec(&self) -> Option<ToolSpec> {
        Some(create_spawn_agents_on_csv_tool())
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
                    "agent jobs handler received unsupported payload".to_string(),
                ));
            }
        };

        handle(session, turn, arguments)
            .await
            .map(boxed_tool_output)
    }
}

impl CoreToolRuntime for SpawnAgentsOnCsvHandler {
    fn matches_kind(&self, payload: &ToolPayload) -> bool {
        matches!(payload, ToolPayload::Function { .. })
    }
}

/// Create a new agent job from a CSV and run it to completion.
///
/// Each CSV row becomes a job item. The instruction string is a template where `{column}`
/// placeholders are filled with values from that row. Results are reported by workers via
/// `report_agent_job_result`, then exported to CSV on completion.
pub async fn handle(
    session: Arc<Session>,
    turn: Arc<TurnContext>,
    arguments: String,
) -> Result<FunctionToolOutput, FunctionCallError> {
    let args: SpawnAgentsOnCsvArgs = parse_arguments(arguments.as_str())?;
    if args.instruction.trim().is_empty() {
        return Err(FunctionCallError::RespondToModel(
            "instruction must be non-empty".to_string(),
        ));
    }

    let cwd = single_local_environment_cwd(&turn)?;
    let db = required_state_db(&session)?;
    let input_path = cwd.join(args.csv_path);
    let input_path_display = input_path.display().to_string();
    let csv_content = tokio::fs::read_to_string(&input_path)
        .await
        .map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to read csv input {input_path_display}: {err}"
            ))
        })?;
    let (headers, rows) = parse_csv(csv_content.as_str()).map_err(|err| {
        FunctionCallError::RespondToModel(format!("failed to parse csv input: {err}"))
    })?;
    if headers.is_empty() {
        return Err(FunctionCallError::RespondToModel(
            "csv input must include a header row".to_string(),
        ));
    }
    ensure_unique_headers(headers.as_slice())?;

    let id_column_index = args.id_column.as_ref().map_or(Ok(None), |column_name| {
        headers
            .iter()
            .position(|header| header == column_name)
            .map(Some)
            .ok_or_else(|| {
                FunctionCallError::RespondToModel(format!(
                    "id_column {column_name} was not found in csv headers"
                ))
            })
    })?;

    let mut items = Vec::with_capacity(rows.len());
    let mut seen_ids = HashSet::new();
    for (idx, row) in rows.into_iter().enumerate() {
        if row.len() != headers.len() {
            let row_index = idx + 2;
            let row_len = row.len();
            let header_len = headers.len();
            return Err(FunctionCallError::RespondToModel(format!(
                "csv row {row_index} has {row_len} fields but header has {header_len}"
            )));
        }

        let source_id = id_column_index
            .and_then(|index| row.get(index).cloned())
            .filter(|value| !value.trim().is_empty());
        let row_index = idx + 1;
        let base_item_id = source_id
            .clone()
            .unwrap_or_else(|| format!("row-{row_index}"));
        let mut item_id = base_item_id.clone();
        let mut suffix = 2usize;
        while !seen_ids.insert(item_id.clone()) {
            item_id = format!("{base_item_id}-{suffix}");
            suffix = suffix.saturating_add(1);
        }

        let row_object = headers
            .iter()
            .zip(row.iter())
            .map(|(header, value)| (header.clone(), Value::String(value.clone())))
            .collect::<serde_json::Map<_, _>>();
        items.push(codex_state::AgentJobItemCreateParams {
            item_id,
            row_index: idx as i64,
            source_id,
            row_json: Value::Object(row_object),
        });
    }

    let job_id = Uuid::new_v4().to_string();
    let requested_concurrency = normalize_concurrency(
        args.max_concurrency.or(args.max_workers),
        turn.config.agent_max_threads,
    );
    let output_schema_present = args.output_schema.is_some();
    let admission_shadow_decision = build_spawn_agents_on_csv_admission_shadow_decision(
        job_id.as_str(),
        items.len(),
        requested_concurrency,
        &session,
        &turn,
        output_schema_present,
    );
    let output_csv_path = args.output_csv_path.map_or_else(
        || default_output_csv_path(&input_path, job_id.as_str()),
        |path| cwd.join(path),
    );
    let job_suffix = &job_id[..8];
    let job_name = format!("agent-job-{job_suffix}");
    let max_runtime_seconds = normalize_max_runtime_seconds(
        args.max_runtime_seconds
            .or(turn.config.agent_job_max_runtime_seconds),
    )?;
    let _job = db
        .create_agent_job(
            &codex_state::AgentJobCreateParams {
                id: job_id.clone(),
                name: job_name,
                instruction: args.instruction,
                auto_export: true,
                max_runtime_seconds,
                output_schema_json: args.output_schema,
                input_headers: headers,
                input_csv_path: input_path.display().to_string(),
                output_csv_path: output_csv_path.display().to_string(),
            },
            items.as_slice(),
        )
        .await
        .map_err(|err| {
            FunctionCallError::RespondToModel(format!("failed to create agent job: {err}"))
        })?;
    let admission_payload = serde_json::to_value(&admission_shadow_decision).map_err(|err| {
        FunctionCallError::Fatal(format!(
            "failed to serialize agent job admission shadow decision: {err}"
        ))
    })?;
    let task_id = admission_shadow_decision
        .task_id
        .as_deref()
        .unwrap_or(job_id.as_str());
    db.append_agent_job_admission_shadow_decision(
        job_id.as_str(),
        task_id,
        admission_shadow_decision.decision,
        admission_payload.clone(),
        turn.trace_id.as_deref(),
    )
    .await
    .map_err(|err| {
        FunctionCallError::RespondToModel(format!(
            "failed to append agent job admission shadow decision for {job_id}: {err}"
        ))
    })?;

    let requested_concurrency = args.max_concurrency.or(args.max_workers);
    let options = match build_runner_options(&session, &turn, requested_concurrency).await {
        Ok(options) => options,
        Err(err) => {
            let error_message = err.to_string();
            let _ = db
                .mark_agent_job_failed(job_id.as_str(), error_message.as_str())
                .await;
            return Err(err);
        }
    };
    db.mark_agent_job_running(job_id.as_str())
        .await
        .map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to transition agent job {job_id} to running: {err}"
            ))
        })?;
    if let Err(err) = run_agent_job_loop(
        session.clone(),
        turn.clone(),
        db.clone(),
        job_id.clone(),
        options,
    )
    .await
    {
        let error_message = format!("job runner failed: {err}");
        let _ = db
            .mark_agent_job_failed(job_id.as_str(), error_message.as_str())
            .await;
        return Err(FunctionCallError::RespondToModel(format!(
            "agent job {job_id} failed: {err}"
        )));
    }

    let job = db
        .get_agent_job(job_id.as_str())
        .await
        .map_err(|err| {
            FunctionCallError::RespondToModel(format!("failed to load agent job {job_id}: {err}"))
        })?
        .ok_or_else(|| {
            FunctionCallError::RespondToModel(format!("agent job {job_id} not found"))
        })?;
    let output_path = PathBuf::from(job.output_csv_path.clone());
    if !tokio::fs::try_exists(&output_path).await.unwrap_or(false) {
        export_job_csv_snapshot(db.clone(), &job)
            .await
            .map_err(|err| {
                FunctionCallError::RespondToModel(format!(
                    "failed to export output csv {job_id}: {err}"
                ))
            })?;
    }
    let progress = db
        .get_agent_job_progress(job_id.as_str())
        .await
        .map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to load agent job progress {job_id}: {err}"
            ))
        })?;
    let mut job_error = job.last_error.clone().filter(|err| !err.trim().is_empty());
    let failed_item_errors = if progress.failed_items > 0 {
        let items = db
            .list_agent_job_items(
                job_id.as_str(),
                Some(codex_state::AgentJobItemStatus::Failed),
                Some(5),
            )
            .await
            .unwrap_or_default();
        let summaries: Vec<_> = items
            .into_iter()
            .filter_map(|item| {
                let last_error = item.last_error.unwrap_or_default();
                if last_error.trim().is_empty() {
                    return None;
                }
                Some(AgentJobFailureSummary {
                    item_id: item.item_id,
                    source_id: item.source_id,
                    last_error,
                })
            })
            .collect();
        if summaries.is_empty() {
            if job_error.is_none() {
                job_error = Some(
                    "agent job has failed items but no error details were recorded".to_string(),
                );
            }
            None
        } else {
            Some(summaries)
        }
    } else {
        None
    };
    let promotion_readiness_shadow_matrix =
        build_default_governed_promotion_readiness_shadow_matrix(std::slice::from_ref(
            &admission_shadow_decision.role_manifest_shadow_decision,
        ));
    let operator_review_promotion_packet =
        build_operator_review_promotion_packet(&promotion_readiness_shadow_matrix);
    let task_id = admission_shadow_decision
        .task_id
        .as_deref()
        .unwrap_or(job_id.as_str());
    let promotion_readiness_shadow_matrix_payload =
        serde_json::to_value(&promotion_readiness_shadow_matrix).map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to serialize agent job promotion readiness matrix: {err}"
            ))
        })?;
    db.append_agent_job_promotion_readiness_matrix_shadow(
        job_id.as_str(),
        task_id,
        promotion_readiness_shadow_matrix.decision,
        promotion_readiness_shadow_matrix_payload.clone(),
        turn.trace_id.as_deref(),
    )
    .await
    .map_err(|err| {
        FunctionCallError::RespondToModel(format!(
            "failed to append agent job promotion readiness matrix for {job_id}: {err}"
        ))
    })?;
    let operator_review_promotion_packet_payload =
        serde_json::to_value(&operator_review_promotion_packet).map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to serialize agent job operator review promotion packet: {err}"
            ))
        })?;
    db.append_agent_job_operator_review_promotion_packet_shadow(
        job_id.as_str(),
        task_id,
        operator_review_promotion_packet.decision,
        operator_review_promotion_packet_payload.clone(),
        turn.trace_id.as_deref(),
    )
    .await
    .map_err(|err| {
        FunctionCallError::RespondToModel(format!(
            "failed to append agent job operator review promotion packet for {job_id}: {err}"
        ))
    })?;
    let promotion_review_readback = db
        .get_agent_job_work_graph_promotion_review_readback(job_id.as_str())
        .await
        .map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to read back agent job promotion review packets for {job_id}: {err}"
            ))
        })?;
    let promotion_review_replay_consistency_decision =
        build_promotion_review_replay_consistency_decision(
            &admission_payload,
            &promotion_readiness_shadow_matrix_payload,
            &operator_review_promotion_packet_payload,
            &promotion_review_readback,
        );
    let promotion_review_replay_consistency_payload =
        serde_json::to_value(&promotion_review_replay_consistency_decision).map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to serialize agent job promotion review replay consistency: {err}"
            ))
        })?;
    db.append_agent_job_promotion_review_replay_consistency_shadow(
        job_id.as_str(),
        task_id,
        promotion_review_replay_consistency_decision.decision,
        promotion_review_replay_consistency_payload,
        turn.trace_id.as_deref(),
    )
    .await
    .map_err(|err| {
        FunctionCallError::RespondToModel(format!(
            "failed to append agent job promotion review replay consistency for {job_id}: {err}"
        ))
    })?;
    let promotion_review_closeout_readback = db
        .get_agent_job_work_graph_promotion_review_readback(job_id.as_str())
        .await
        .map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to read back agent job promotion review replay consistency for {job_id}: {err}"
            ))
        })?;
    let promotion_closeout_receipt = build_promotion_closeout_receipt(
        &operator_review_promotion_packet,
        &promotion_review_replay_consistency_decision,
        &promotion_review_closeout_readback,
    );
    let promotion_closeout_receipt_payload = serde_json::to_value(&promotion_closeout_receipt)
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to serialize agent job promotion closeout receipt: {err}"
            ))
        })?;
    db.append_agent_job_promotion_closeout_receipt_shadow(
        job_id.as_str(),
        task_id,
        promotion_closeout_receipt.decision,
        promotion_closeout_receipt_payload.clone(),
        turn.trace_id.as_deref(),
    )
    .await
    .map_err(|err| {
        FunctionCallError::RespondToModel(format!(
            "failed to append agent job promotion closeout receipt for {job_id}: {err}"
        ))
    })?;
    let promotion_closeout_replay_readback = db
        .get_agent_job_work_graph_promotion_review_readback(job_id.as_str())
        .await
        .map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to read back agent job promotion closeout receipt for {job_id}: {err}"
            ))
        })?;
    let promotion_closeout_replay_consistency_decision =
        build_promotion_closeout_replay_consistency_decision(
            &promotion_closeout_receipt,
            &promotion_closeout_receipt_payload,
            &promotion_closeout_replay_readback,
        );
    let promotion_closeout_replay_consistency_payload =
        serde_json::to_value(&promotion_closeout_replay_consistency_decision).map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to serialize agent job promotion closeout replay consistency: {err}"
            ))
        })?;
    db.append_agent_job_promotion_closeout_replay_consistency_shadow(
        job_id.as_str(),
        task_id,
        promotion_closeout_replay_consistency_decision.decision,
        promotion_closeout_replay_consistency_payload,
        turn.trace_id.as_deref(),
    )
    .await
    .map_err(|err| {
        FunctionCallError::RespondToModel(format!(
            "failed to append agent job promotion closeout replay consistency for {job_id}: {err}"
        ))
    })?;
    let promotion_review_audit_chain_readback = db
        .get_agent_job_work_graph_promotion_review_readback(job_id.as_str())
        .await
        .map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to read back agent job promotion audit chain for {job_id}: {err}"
            ))
        })?;
    let promotion_review_audit_chain_receipt = build_promotion_review_audit_chain_receipt(
        &promotion_closeout_replay_consistency_decision,
        &promotion_review_audit_chain_readback,
    );
    let promotion_review_audit_chain_receipt_payload =
        serde_json::to_value(&promotion_review_audit_chain_receipt).map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to serialize agent job promotion review audit chain receipt: {err}"
            ))
        })?;
    db.append_agent_job_promotion_review_audit_chain_receipt_shadow(
        job_id.as_str(),
        task_id,
        promotion_review_audit_chain_receipt.decision,
        promotion_review_audit_chain_receipt_payload,
        turn.trace_id.as_deref(),
    )
    .await
    .map_err(|err| {
        FunctionCallError::RespondToModel(format!(
            "failed to append agent job promotion review audit chain receipt for {job_id}: {err}"
        ))
    })?;
    let reviewed_flag_precondition_plan_readback = db
        .get_agent_job_work_graph_promotion_review_readback(job_id.as_str())
        .await
        .map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to read back agent job reviewed flag precondition plan inputs for {job_id}: {err}"
            ))
        })?;
    let reviewed_flag_precondition_plan_packet = build_reviewed_flag_precondition_plan_packet(
        &promotion_review_audit_chain_receipt,
        &reviewed_flag_precondition_plan_readback,
    );
    let reviewed_flag_precondition_plan_packet_payload =
        serde_json::to_value(&reviewed_flag_precondition_plan_packet).map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to serialize agent job reviewed flag precondition plan: {err}"
            ))
        })?;
    db.append_agent_job_reviewed_flag_precondition_plan_shadow(
        job_id.as_str(),
        task_id,
        reviewed_flag_precondition_plan_packet.decision,
        reviewed_flag_precondition_plan_packet_payload.clone(),
        turn.trace_id.as_deref(),
    )
    .await
    .map_err(|err| {
        FunctionCallError::RespondToModel(format!(
            "failed to append agent job reviewed flag precondition plan for {job_id}: {err}"
        ))
    })?;
    let reviewed_flag_precondition_plan_replay_readback = db
        .get_agent_job_work_graph_promotion_review_readback(job_id.as_str())
        .await
        .map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to read back agent job reviewed flag precondition plan for {job_id}: {err}"
            ))
        })?;
    let reviewed_flag_precondition_plan_replay_consistency_decision =
        build_reviewed_flag_precondition_plan_replay_consistency_decision(
            &reviewed_flag_precondition_plan_packet,
            &reviewed_flag_precondition_plan_packet_payload,
            &reviewed_flag_precondition_plan_replay_readback,
        );
    let reviewed_flag_precondition_plan_replay_consistency_payload =
        serde_json::to_value(&reviewed_flag_precondition_plan_replay_consistency_decision)
            .map_err(|err| {
                FunctionCallError::Fatal(format!(
                    "failed to serialize agent job reviewed flag precondition plan replay consistency: {err}"
                ))
            })?;
    db.append_agent_job_reviewed_flag_precondition_plan_replay_consistency_shadow(
        job_id.as_str(),
        task_id,
        reviewed_flag_precondition_plan_replay_consistency_decision.decision,
        reviewed_flag_precondition_plan_replay_consistency_payload,
        turn.trace_id.as_deref(),
    )
    .await
    .map_err(|err| {
        FunctionCallError::RespondToModel(format!(
            "failed to append agent job reviewed flag precondition plan replay consistency for {job_id}: {err}"
        ))
    })?;
    let reviewed_flag_readiness_closeout_readback = db
        .get_agent_job_work_graph_promotion_review_readback(job_id.as_str())
        .await
        .map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to read back agent job reviewed flag readiness closeout inputs for {job_id}: {err}"
            ))
        })?;
    let reviewed_flag_readiness_closeout_receipt = build_reviewed_flag_readiness_closeout_receipt(
        &reviewed_flag_precondition_plan_packet,
        &reviewed_flag_precondition_plan_replay_consistency_decision,
        &reviewed_flag_readiness_closeout_readback,
    );
    let reviewed_flag_readiness_closeout_receipt_payload =
        serde_json::to_value(&reviewed_flag_readiness_closeout_receipt).map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to serialize agent job reviewed flag readiness closeout receipt: {err}"
            ))
        })?;
    db.append_agent_job_reviewed_flag_readiness_closeout_receipt_shadow(
        job_id.as_str(),
        task_id,
        reviewed_flag_readiness_closeout_receipt.decision,
        reviewed_flag_readiness_closeout_receipt_payload.clone(),
        turn.trace_id.as_deref(),
    )
    .await
    .map_err(|err| {
        FunctionCallError::RespondToModel(format!(
            "failed to append agent job reviewed flag readiness closeout receipt for {job_id}: {err}"
        ))
    })?;
    let reviewed_flag_readiness_closeout_replay_readback = db
        .get_agent_job_work_graph_promotion_review_readback(job_id.as_str())
        .await
        .map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to read back agent job reviewed flag readiness closeout receipt for {job_id}: {err}"
            ))
        })?;
    let reviewed_flag_readiness_closeout_replay_consistency_decision =
        build_reviewed_flag_readiness_closeout_replay_consistency_decision(
            &reviewed_flag_readiness_closeout_receipt,
            &reviewed_flag_readiness_closeout_receipt_payload,
            &reviewed_flag_readiness_closeout_replay_readback,
        );
    let reviewed_flag_readiness_closeout_replay_consistency_payload =
        serde_json::to_value(&reviewed_flag_readiness_closeout_replay_consistency_decision)
            .map_err(|err| {
                FunctionCallError::Fatal(format!(
                    "failed to serialize agent job reviewed flag readiness closeout replay consistency: {err}"
                ))
            })?;
    db.append_agent_job_reviewed_flag_readiness_closeout_replay_consistency_shadow(
        job_id.as_str(),
        task_id,
        reviewed_flag_readiness_closeout_replay_consistency_decision.decision,
        reviewed_flag_readiness_closeout_replay_consistency_payload,
        turn.trace_id.as_deref(),
    )
    .await
    .map_err(|err| {
        FunctionCallError::RespondToModel(format!(
            "failed to append agent job reviewed flag readiness closeout replay consistency for {job_id}: {err}"
        ))
    })?;
    let reviewed_flag_audit_chain_closeout_readback = db
        .get_agent_job_work_graph_promotion_review_readback(job_id.as_str())
        .await
        .map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to read back agent job reviewed flag audit-chain closeout inputs for {job_id}: {err}"
            ))
        })?;
    let reviewed_flag_audit_chain_closeout_receipt =
        build_reviewed_flag_audit_chain_closeout_receipt(
            &reviewed_flag_readiness_closeout_replay_consistency_decision,
            &reviewed_flag_audit_chain_closeout_readback,
        );
    let reviewed_flag_audit_chain_closeout_receipt_payload =
        serde_json::to_value(&reviewed_flag_audit_chain_closeout_receipt).map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to serialize agent job reviewed flag audit-chain closeout receipt: {err}"
            ))
        })?;
    db.append_agent_job_reviewed_flag_audit_chain_closeout_receipt_shadow(
        job_id.as_str(),
        task_id,
        reviewed_flag_audit_chain_closeout_receipt.decision,
        reviewed_flag_audit_chain_closeout_receipt_payload,
        turn.trace_id.as_deref(),
    )
    .await
    .map_err(|err| {
        FunctionCallError::RespondToModel(format!(
            "failed to append agent job reviewed flag audit-chain closeout receipt for {job_id}: {err}"
        ))
    })?;
    let work_graph_surface_audit_readback = db
        .get_agent_job_work_graph_audit_chain_readback(
            job_id.as_str(),
            work_graph_surface_audit_chain_segment_specs(),
        )
        .await
        .map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to read back agent job WorkGraph surface audit inputs for {job_id}: {err}"
            ))
        })?;
    let work_graph_surface_audit_packet =
        build_work_graph_surface_audit_packet(WorkGraphSurfaceAuditPacketInput {
            job_id: job_id.as_str(),
            promotion_readiness_shadow_matrix: &promotion_readiness_shadow_matrix,
            role_manifest_shadow_decisions: std::slice::from_ref(
                &admission_shadow_decision.role_manifest_shadow_decision,
            ),
            audit_chain_readback: &work_graph_surface_audit_readback,
        });
    let work_graph_surface_audit_packet_payload =
        serde_json::to_value(&work_graph_surface_audit_packet).map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to serialize agent job WorkGraph surface audit packet: {err}"
            ))
        })?;
    db.append_agent_job_work_graph_surface_audit_packet_shadow(
        job_id.as_str(),
        task_id,
        work_graph_surface_audit_packet.decision,
        work_graph_surface_audit_packet_payload,
        turn.trace_id.as_deref(),
    )
    .await
    .map_err(|err| {
        FunctionCallError::RespondToModel(format!(
            "failed to append agent job WorkGraph surface audit packet for {job_id}: {err}"
        ))
    })?;
    let work_graph_surface_audit_packet_summary =
        summarize_work_graph_surface_audit_packet(&work_graph_surface_audit_packet);
    let work_graph_canonical_projection_receipt =
        build_work_graph_canonical_projection_shadow_receipt(
            &work_graph_surface_audit_packet_summary,
        );
    let work_graph_canonical_projection_receipt_payload =
        serde_json::to_value(&work_graph_canonical_projection_receipt).map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to serialize agent job canonical WorkGraph projection receipt: {err}"
            ))
        })?;
    db.append_agent_job_work_graph_canonical_projection_receipt_shadow(
        job_id.as_str(),
        task_id,
        work_graph_canonical_projection_receipt.decision,
        work_graph_canonical_projection_receipt_payload.clone(),
        turn.trace_id.as_deref(),
    )
    .await
    .map_err(|err| {
        FunctionCallError::RespondToModel(format!(
            "failed to append agent job canonical WorkGraph projection receipt for {job_id}: {err}"
        ))
    })?;
    let work_graph_canonical_projection_replay_readback = db
        .get_agent_job_work_graph_audit_chain_readback(
            job_id.as_str(),
            work_graph_canonical_projection_replay_chain_segment_specs(),
        )
        .await
        .map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to read back agent job canonical WorkGraph projection receipt for {job_id}: {err}"
            ))
        })?;
    let canonical_projection_receipt_segment = work_graph_canonical_projection_replay_readback
        .segments
        .iter()
        .find(|segment| segment.segment_id == "canonical_projection_receipt");
    let canonical_projection_replay_segment = work_graph_canonical_projection_replay_readback
        .segments
        .iter()
        .find(|segment| segment.segment_id == "canonical_projection_replay_consistency");
    let work_graph_canonical_projection_replay_consistency_decision =
        build_work_graph_canonical_projection_replay_consistency_decision(
            WorkGraphCanonicalProjectionReplayConsistencyInput {
                source_surface_id: "agent_jobs",
                projection_receipt: &work_graph_canonical_projection_receipt,
                projection_receipt_payload: &work_graph_canonical_projection_receipt_payload,
                latest_projection_receipt_payload: canonical_projection_receipt_segment
                    .and_then(|segment| segment.latest_payload.as_ref()),
                projection_receipt_events: canonical_projection_receipt_segment
                    .map(|segment| segment.event_count)
                    .unwrap_or_default(),
                projection_receipt_readback_ready: canonical_projection_receipt_segment
                    .is_some_and(|segment| segment.readback_ready),
                prior_projection_replay_consistency_events: canonical_projection_replay_segment
                    .map(|segment| segment.event_count)
                    .unwrap_or_default(),
                live_blocking_event_count: work_graph_canonical_projection_replay_readback
                    .live_blocking_event_count,
                live_cutover_event_count: work_graph_canonical_projection_replay_readback
                    .live_cutover_event_count,
            },
        );
    let work_graph_canonical_projection_replay_consistency_payload = serde_json::to_value(
        &work_graph_canonical_projection_replay_consistency_decision,
    )
    .map_err(|err| {
        FunctionCallError::Fatal(format!(
            "failed to serialize agent job canonical WorkGraph projection replay consistency: {err}"
        ))
    })?;
    db.append_agent_job_work_graph_canonical_projection_replay_consistency_shadow(
        job_id.as_str(),
        task_id,
        work_graph_canonical_projection_replay_consistency_decision.decision,
        work_graph_canonical_projection_replay_consistency_payload,
        turn.trace_id.as_deref(),
    )
    .await
    .map_err(|err| {
        FunctionCallError::RespondToModel(format!(
            "failed to append agent job canonical WorkGraph projection replay consistency for {job_id}: {err}"
        ))
    })?;
    let work_graph_canonical_projection_closeout_readback = db
        .get_agent_job_work_graph_audit_chain_readback(
            job_id.as_str(),
            work_graph_canonical_projection_closeout_chain_segment_specs(),
        )
        .await
        .map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to read back agent job canonical WorkGraph projection closeout chain for {job_id}: {err}"
            ))
        })?;
    let canonical_projection_receipt_segment = work_graph_canonical_projection_closeout_readback
        .segments
        .iter()
        .find(|segment| segment.segment_id == "canonical_projection_receipt");
    let canonical_projection_replay_segment = work_graph_canonical_projection_closeout_readback
        .segments
        .iter()
        .find(|segment| segment.segment_id == "canonical_projection_replay_consistency");
    let canonical_projection_closeout_segment = work_graph_canonical_projection_closeout_readback
        .segments
        .iter()
        .find(|segment| segment.segment_id == "canonical_projection_closeout_receipt");
    let work_graph_canonical_projection_closeout_receipt =
        build_work_graph_canonical_projection_closeout_receipt(
            WorkGraphCanonicalProjectionCloseoutReceiptInput {
                source_surface_id: "agent_jobs",
                projection_receipt: &work_graph_canonical_projection_receipt,
                replay_consistency_decision:
                    &work_graph_canonical_projection_replay_consistency_decision,
                projection_receipt_events: canonical_projection_receipt_segment
                    .map(|segment| segment.event_count)
                    .unwrap_or_default(),
                projection_replay_consistency_events: canonical_projection_replay_segment
                    .map(|segment| segment.event_count)
                    .unwrap_or_default(),
                prior_projection_closeout_receipt_events: canonical_projection_closeout_segment
                    .map(|segment| segment.event_count)
                    .unwrap_or_default(),
                projection_receipt_readback_ready: canonical_projection_receipt_segment
                    .is_some_and(|segment| segment.readback_ready),
                projection_replay_consistency_ready: canonical_projection_replay_segment
                    .is_some_and(|segment| segment.ready),
                live_blocking_event_count: work_graph_canonical_projection_closeout_readback
                    .live_blocking_event_count,
                live_cutover_event_count: work_graph_canonical_projection_closeout_readback
                    .live_cutover_event_count,
            },
        );
    let work_graph_canonical_projection_closeout_receipt_payload = serde_json::to_value(
        &work_graph_canonical_projection_closeout_receipt,
    )
    .map_err(|err| {
        FunctionCallError::Fatal(format!(
            "failed to serialize agent job canonical WorkGraph projection closeout receipt: {err}"
        ))
    })?;
    db.append_agent_job_work_graph_canonical_projection_closeout_receipt_shadow(
        job_id.as_str(),
        task_id,
        work_graph_canonical_projection_closeout_receipt.decision,
        work_graph_canonical_projection_closeout_receipt_payload.clone(),
        turn.trace_id.as_deref(),
    )
    .await
    .map_err(|err| {
        FunctionCallError::RespondToModel(format!(
            "failed to append agent job canonical WorkGraph projection closeout receipt for {job_id}: {err}"
        ))
    })?;
    let work_graph_canonical_projection_closeout_readback = db
        .get_agent_job_work_graph_audit_chain_readback(
            job_id.as_str(),
            work_graph_canonical_projection_closeout_chain_segment_specs(),
        )
        .await
        .map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to read back recorded agent job canonical WorkGraph projection closeout receipt for {job_id}: {err}"
            ))
        })?;
    let closeout_segment_recorded = work_graph_canonical_projection_closeout_readback
        .segments
        .iter()
        .any(|segment| {
            segment.segment_id == "canonical_projection_closeout_receipt"
                && segment.readback_ready
                && segment.event_count > 0
        });
    if !closeout_segment_recorded {
        return Err(FunctionCallError::RespondToModel(format!(
            "agent job canonical WorkGraph projection closeout receipt was not readable for {job_id}"
        )));
    }
    let work_graph_canonical_projection_closeout_replay_readback = db
        .get_agent_job_work_graph_audit_chain_readback(
            job_id.as_str(),
            work_graph_canonical_projection_closeout_replay_chain_segment_specs(),
        )
        .await
        .map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to read back agent job canonical WorkGraph projection closeout replay inputs for {job_id}: {err}"
            ))
        })?;
    let canonical_projection_closeout_segment =
        work_graph_canonical_projection_closeout_replay_readback
            .segments
            .iter()
            .find(|segment| segment.segment_id == "canonical_projection_closeout_receipt");
    let canonical_projection_closeout_replay_segment =
        work_graph_canonical_projection_closeout_replay_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id == "canonical_projection_closeout_replay_consistency"
            });
    let work_graph_canonical_projection_closeout_replay_consistency_decision =
        build_work_graph_canonical_projection_closeout_replay_consistency_decision(
            WorkGraphCanonicalProjectionCloseoutReplayConsistencyInput {
                source_surface_id: "agent_jobs",
                closeout_receipt: &work_graph_canonical_projection_closeout_receipt,
                closeout_receipt_payload: &work_graph_canonical_projection_closeout_receipt_payload,
                latest_closeout_receipt_payload: canonical_projection_closeout_segment
                    .and_then(|segment| segment.latest_payload.as_ref()),
                closeout_receipt_events: canonical_projection_closeout_segment
                    .map(|segment| segment.event_count)
                    .unwrap_or_default(),
                closeout_receipt_readback_ready: canonical_projection_closeout_segment
                    .is_some_and(|segment| segment.readback_ready),
                prior_closeout_replay_consistency_events:
                    canonical_projection_closeout_replay_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                live_blocking_event_count: work_graph_canonical_projection_closeout_replay_readback
                    .live_blocking_event_count,
                live_cutover_event_count: work_graph_canonical_projection_closeout_replay_readback
                    .live_cutover_event_count,
            },
        );
    let work_graph_canonical_projection_closeout_replay_consistency_payload =
        serde_json::to_value(
            &work_graph_canonical_projection_closeout_replay_consistency_decision,
        )
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to serialize agent job canonical WorkGraph projection closeout replay consistency: {err}"
            ))
        })?;
    db.append_agent_job_work_graph_canonical_projection_closeout_replay_consistency_shadow(
        job_id.as_str(),
        task_id,
        work_graph_canonical_projection_closeout_replay_consistency_decision.decision,
        work_graph_canonical_projection_closeout_replay_consistency_payload,
        turn.trace_id.as_deref(),
    )
    .await
    .map_err(|err| {
        FunctionCallError::RespondToModel(format!(
            "failed to append agent job canonical WorkGraph projection closeout replay consistency for {job_id}: {err}"
        ))
    })?;
    let work_graph_canonical_projection_closeout_replay_readback = db
        .get_agent_job_work_graph_audit_chain_readback(
            job_id.as_str(),
            work_graph_canonical_projection_closeout_replay_chain_segment_specs(),
        )
        .await
        .map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to read back recorded agent job canonical WorkGraph projection closeout replay consistency for {job_id}: {err}"
            ))
        })?;
    let closeout_replay_segment_recorded = work_graph_canonical_projection_closeout_replay_readback
        .segments
        .iter()
        .any(|segment| {
            segment.segment_id == "canonical_projection_closeout_replay_consistency"
                && segment.readback_ready
                && segment.event_count > 0
        });
    if !closeout_replay_segment_recorded {
        return Err(FunctionCallError::RespondToModel(format!(
            "agent job canonical WorkGraph projection closeout replay consistency was not readable for {job_id}"
        )));
    }
    let work_graph_canonical_projection_audit_chain_closeout_readback = db
        .get_agent_job_work_graph_audit_chain_readback(
            job_id.as_str(),
            work_graph_canonical_projection_audit_chain_closeout_segment_specs(),
        )
        .await
        .map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to read back agent job canonical WorkGraph projection audit-chain closeout inputs for {job_id}: {err}"
            ))
        })?;
    let canonical_projection_receipt_segment =
        work_graph_canonical_projection_audit_chain_closeout_readback
            .segments
            .iter()
            .find(|segment| segment.segment_id == "canonical_projection_receipt");
    let canonical_projection_replay_segment =
        work_graph_canonical_projection_audit_chain_closeout_readback
            .segments
            .iter()
            .find(|segment| segment.segment_id == "canonical_projection_replay_consistency");
    let canonical_projection_closeout_segment =
        work_graph_canonical_projection_audit_chain_closeout_readback
            .segments
            .iter()
            .find(|segment| segment.segment_id == "canonical_projection_closeout_receipt");
    let canonical_projection_closeout_replay_segment =
        work_graph_canonical_projection_audit_chain_closeout_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id == "canonical_projection_closeout_replay_consistency"
            });
    let canonical_projection_audit_chain_closeout_segment =
        work_graph_canonical_projection_audit_chain_closeout_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id == "canonical_projection_audit_chain_closeout_receipt"
            });
    let work_graph_canonical_projection_audit_chain_closeout_receipt =
        build_work_graph_canonical_projection_audit_chain_closeout_receipt(
            WorkGraphCanonicalProjectionAuditChainCloseoutReceiptInput {
                source_surface_id: "agent_jobs",
                projection_receipt: &work_graph_canonical_projection_receipt,
                projection_replay_consistency_decision:
                    &work_graph_canonical_projection_replay_consistency_decision,
                closeout_receipt: &work_graph_canonical_projection_closeout_receipt,
                closeout_replay_consistency_decision:
                    &work_graph_canonical_projection_closeout_replay_consistency_decision,
                projection_receipt_events: canonical_projection_receipt_segment
                    .map(|segment| segment.event_count)
                    .unwrap_or_default(),
                projection_replay_consistency_events: canonical_projection_replay_segment
                    .map(|segment| segment.event_count)
                    .unwrap_or_default(),
                closeout_receipt_events: canonical_projection_closeout_segment
                    .map(|segment| segment.event_count)
                    .unwrap_or_default(),
                closeout_replay_consistency_events: canonical_projection_closeout_replay_segment
                    .map(|segment| segment.event_count)
                    .unwrap_or_default(),
                prior_audit_chain_closeout_receipt_events:
                    canonical_projection_audit_chain_closeout_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                projection_receipt_readback_ready: canonical_projection_receipt_segment
                    .is_some_and(|segment| segment.readback_ready),
                projection_replay_consistency_ready: canonical_projection_replay_segment
                    .is_some_and(|segment| segment.ready),
                closeout_receipt_readback_ready: canonical_projection_closeout_segment
                    .is_some_and(|segment| segment.readback_ready),
                closeout_replay_consistency_ready: canonical_projection_closeout_replay_segment
                    .is_some_and(|segment| segment.ready),
                live_blocking_event_count:
                    work_graph_canonical_projection_audit_chain_closeout_readback
                        .live_blocking_event_count,
                live_cutover_event_count:
                    work_graph_canonical_projection_audit_chain_closeout_readback
                        .live_cutover_event_count,
            },
        );
    let work_graph_canonical_projection_audit_chain_closeout_receipt_payload =
        serde_json::to_value(&work_graph_canonical_projection_audit_chain_closeout_receipt)
            .map_err(|err| {
                FunctionCallError::Fatal(format!(
                    "failed to serialize agent job canonical WorkGraph projection audit-chain closeout receipt: {err}"
                ))
            })?;
    db.append_agent_job_work_graph_canonical_projection_audit_chain_closeout_receipt_shadow(
        job_id.as_str(),
        task_id,
        work_graph_canonical_projection_audit_chain_closeout_receipt.decision,
        work_graph_canonical_projection_audit_chain_closeout_receipt_payload.clone(),
        turn.trace_id.as_deref(),
    )
    .await
    .map_err(|err| {
        FunctionCallError::RespondToModel(format!(
            "failed to append agent job canonical WorkGraph projection audit-chain closeout receipt for {job_id}: {err}"
        ))
    })?;
    let work_graph_canonical_projection_audit_chain_closeout_readback = db
        .get_agent_job_work_graph_audit_chain_readback(
            job_id.as_str(),
            work_graph_canonical_projection_audit_chain_closeout_segment_specs(),
        )
        .await
        .map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to read back recorded agent job canonical WorkGraph projection audit-chain closeout receipt for {job_id}: {err}"
            ))
        })?;
    let audit_chain_closeout_segment_recorded =
        work_graph_canonical_projection_audit_chain_closeout_readback
            .segments
            .iter()
            .any(|segment| {
                segment.segment_id == "canonical_projection_audit_chain_closeout_receipt"
                    && segment.readback_ready
                    && segment.event_count > 0
            });
    if !audit_chain_closeout_segment_recorded {
        return Err(FunctionCallError::RespondToModel(format!(
            "agent job canonical WorkGraph projection audit-chain closeout receipt was not readable for {job_id}"
        )));
    }
    let work_graph_canonical_projection_audit_chain_closeout_replay_readback = db
        .get_agent_job_work_graph_audit_chain_readback(
            job_id.as_str(),
            work_graph_canonical_projection_audit_chain_closeout_replay_segment_specs(),
        )
        .await
        .map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to read back agent job canonical WorkGraph projection audit-chain closeout replay inputs for {job_id}: {err}"
            ))
        })?;
    let canonical_projection_audit_chain_closeout_segment =
        work_graph_canonical_projection_audit_chain_closeout_replay_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id == "canonical_projection_audit_chain_closeout_receipt"
            });
    let canonical_projection_audit_chain_closeout_replay_segment =
        work_graph_canonical_projection_audit_chain_closeout_replay_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id == "canonical_projection_audit_chain_closeout_replay_consistency"
            });
    let work_graph_canonical_projection_audit_chain_closeout_replay_consistency_decision =
        build_work_graph_canonical_projection_audit_chain_closeout_replay_consistency_decision(
            WorkGraphCanonicalProjectionAuditChainCloseoutReplayConsistencyInput {
                source_surface_id: "agent_jobs",
                audit_chain_closeout_receipt:
                    &work_graph_canonical_projection_audit_chain_closeout_receipt,
                audit_chain_closeout_receipt_payload:
                    &work_graph_canonical_projection_audit_chain_closeout_receipt_payload,
                latest_audit_chain_closeout_receipt_payload:
                    canonical_projection_audit_chain_closeout_segment
                        .and_then(|segment| segment.latest_payload.as_ref()),
                audit_chain_closeout_receipt_events:
                    canonical_projection_audit_chain_closeout_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                audit_chain_closeout_receipt_readback_ready:
                    canonical_projection_audit_chain_closeout_segment
                        .is_some_and(|segment| segment.readback_ready),
                prior_audit_chain_closeout_replay_consistency_events:
                    canonical_projection_audit_chain_closeout_replay_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                live_blocking_event_count:
                    work_graph_canonical_projection_audit_chain_closeout_replay_readback
                        .live_blocking_event_count,
                live_cutover_event_count:
                    work_graph_canonical_projection_audit_chain_closeout_replay_readback
                        .live_cutover_event_count,
            },
        );
    let work_graph_canonical_projection_audit_chain_closeout_replay_consistency_payload =
        serde_json::to_value(
            &work_graph_canonical_projection_audit_chain_closeout_replay_consistency_decision,
        )
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to serialize agent job canonical WorkGraph projection audit-chain closeout replay consistency: {err}"
            ))
        })?;
    db.append_agent_job_work_graph_canonical_projection_audit_chain_closeout_replay_consistency_shadow(
        job_id.as_str(),
        task_id,
        work_graph_canonical_projection_audit_chain_closeout_replay_consistency_decision.decision,
        work_graph_canonical_projection_audit_chain_closeout_replay_consistency_payload,
        turn.trace_id.as_deref(),
    )
    .await
    .map_err(|err| {
        FunctionCallError::RespondToModel(format!(
            "failed to append agent job canonical WorkGraph projection audit-chain closeout replay consistency for {job_id}: {err}"
        ))
    })?;
    let work_graph_canonical_projection_audit_chain_closeout_replay_readback = db
        .get_agent_job_work_graph_audit_chain_readback(
            job_id.as_str(),
            work_graph_canonical_projection_audit_chain_closeout_replay_segment_specs(),
        )
        .await
        .map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to read back recorded agent job canonical WorkGraph projection audit-chain closeout replay consistency for {job_id}: {err}"
            ))
        })?;
    let audit_chain_closeout_replay_segment_recorded =
        work_graph_canonical_projection_audit_chain_closeout_replay_readback
            .segments
            .iter()
            .any(|segment| {
                segment.segment_id == "canonical_projection_audit_chain_closeout_replay_consistency"
                    && segment.readback_ready
                    && segment.event_count > 0
            });
    if !audit_chain_closeout_replay_segment_recorded {
        return Err(FunctionCallError::RespondToModel(format!(
            "agent job canonical WorkGraph projection audit-chain closeout replay consistency was not readable for {job_id}"
        )));
    }
    let work_graph_canonical_projection_enablement_review_readback = db
        .get_agent_job_work_graph_audit_chain_readback(
            job_id.as_str(),
            work_graph_canonical_projection_enablement_operator_review_segment_specs(),
        )
        .await
        .map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to read back agent job canonical WorkGraph projection enablement review inputs for {job_id}: {err}"
            ))
        })?;
    let canonical_projection_receipt_segment =
        work_graph_canonical_projection_enablement_review_readback
            .segments
            .iter()
            .find(|segment| segment.segment_id == "canonical_projection_receipt");
    let canonical_projection_replay_segment =
        work_graph_canonical_projection_enablement_review_readback
            .segments
            .iter()
            .find(|segment| segment.segment_id == "canonical_projection_replay_consistency");
    let canonical_projection_closeout_segment =
        work_graph_canonical_projection_enablement_review_readback
            .segments
            .iter()
            .find(|segment| segment.segment_id == "canonical_projection_closeout_receipt");
    let canonical_projection_closeout_replay_segment =
        work_graph_canonical_projection_enablement_review_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id == "canonical_projection_closeout_replay_consistency"
            });
    let canonical_projection_audit_chain_closeout_segment =
        work_graph_canonical_projection_enablement_review_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id == "canonical_projection_audit_chain_closeout_receipt"
            });
    let canonical_projection_audit_chain_closeout_replay_segment =
        work_graph_canonical_projection_enablement_review_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id == "canonical_projection_audit_chain_closeout_replay_consistency"
            });
    let canonical_projection_enablement_operator_review_segment =
        work_graph_canonical_projection_enablement_review_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id == "canonical_projection_enablement_operator_review_packet"
            });
    let work_graph_canonical_projection_enablement_operator_review_packet =
        build_work_graph_canonical_projection_enablement_operator_review_packet(
            WorkGraphCanonicalProjectionEnablementOperatorReviewPacketInput {
                source_surface_id: "agent_jobs",
                projection_receipt: &work_graph_canonical_projection_receipt,
                projection_replay_consistency_decision:
                    &work_graph_canonical_projection_replay_consistency_decision,
                closeout_receipt: &work_graph_canonical_projection_closeout_receipt,
                closeout_replay_consistency_decision:
                    &work_graph_canonical_projection_closeout_replay_consistency_decision,
                audit_chain_closeout_receipt:
                    &work_graph_canonical_projection_audit_chain_closeout_receipt,
                audit_chain_closeout_replay_consistency_decision:
                    &work_graph_canonical_projection_audit_chain_closeout_replay_consistency_decision,
                projection_receipt_events: canonical_projection_receipt_segment
                    .map(|segment| segment.event_count)
                    .unwrap_or_default(),
                projection_replay_consistency_events: canonical_projection_replay_segment
                    .map(|segment| segment.event_count)
                    .unwrap_or_default(),
                closeout_receipt_events: canonical_projection_closeout_segment
                    .map(|segment| segment.event_count)
                    .unwrap_or_default(),
                closeout_replay_consistency_events: canonical_projection_closeout_replay_segment
                    .map(|segment| segment.event_count)
                    .unwrap_or_default(),
                audit_chain_closeout_receipt_events:
                    canonical_projection_audit_chain_closeout_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                audit_chain_closeout_replay_consistency_events:
                    canonical_projection_audit_chain_closeout_replay_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                prior_enablement_operator_review_packet_events:
                    canonical_projection_enablement_operator_review_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                projection_receipt_readback_ready: canonical_projection_receipt_segment
                    .is_some_and(|segment| segment.readback_ready),
                projection_replay_consistency_ready: canonical_projection_replay_segment
                    .is_some_and(|segment| segment.ready),
                closeout_receipt_readback_ready: canonical_projection_closeout_segment
                    .is_some_and(|segment| segment.readback_ready),
                closeout_replay_consistency_ready: canonical_projection_closeout_replay_segment
                    .is_some_and(|segment| segment.ready),
                audit_chain_closeout_receipt_readback_ready:
                    canonical_projection_audit_chain_closeout_segment
                        .is_some_and(|segment| segment.readback_ready),
                audit_chain_closeout_replay_consistency_ready:
                    canonical_projection_audit_chain_closeout_replay_segment
                        .is_some_and(|segment| segment.ready),
                live_blocking_event_count:
                    work_graph_canonical_projection_enablement_review_readback
                        .live_blocking_event_count,
                live_cutover_event_count:
                    work_graph_canonical_projection_enablement_review_readback
                        .live_cutover_event_count,
            },
        );
    let work_graph_canonical_projection_enablement_operator_review_packet_payload =
        serde_json::to_value(
            &work_graph_canonical_projection_enablement_operator_review_packet,
        )
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to serialize agent job canonical WorkGraph projection enablement operator-review packet: {err}"
            ))
        })?;
    db.append_agent_job_work_graph_canonical_projection_enablement_operator_review_packet_shadow(
        job_id.as_str(),
        task_id,
        work_graph_canonical_projection_enablement_operator_review_packet.decision,
        work_graph_canonical_projection_enablement_operator_review_packet_payload.clone(),
        turn.trace_id.as_deref(),
    )
    .await
    .map_err(|err| {
        FunctionCallError::RespondToModel(format!(
            "failed to append agent job canonical WorkGraph projection enablement operator-review packet for {job_id}: {err}"
        ))
    })?;
    let work_graph_canonical_projection_enablement_review_readback = db
        .get_agent_job_work_graph_audit_chain_readback(
            job_id.as_str(),
            work_graph_canonical_projection_enablement_operator_review_segment_specs(),
        )
        .await
        .map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to read back recorded agent job canonical WorkGraph projection enablement operator-review packet for {job_id}: {err}"
            ))
        })?;
    let enablement_operator_review_segment_recorded =
        work_graph_canonical_projection_enablement_review_readback
            .segments
            .iter()
            .any(|segment| {
                segment.segment_id == "canonical_projection_enablement_operator_review_packet"
                    && segment.readback_ready
                    && segment.event_count > 0
            });
    if !enablement_operator_review_segment_recorded {
        return Err(FunctionCallError::RespondToModel(format!(
            "agent job canonical WorkGraph projection enablement operator-review packet was not readable for {job_id}"
        )));
    }
    let work_graph_canonical_projection_enablement_operator_review_replay_readback = db
        .get_agent_job_work_graph_audit_chain_readback(
            job_id.as_str(),
            work_graph_canonical_projection_enablement_operator_review_replay_segment_specs(),
        )
        .await
        .map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to read back agent job canonical WorkGraph projection enablement operator-review replay inputs for {job_id}: {err}"
            ))
        })?;
    let canonical_projection_enablement_operator_review_segment =
        work_graph_canonical_projection_enablement_operator_review_replay_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id == "canonical_projection_enablement_operator_review_packet"
            });
    let canonical_projection_enablement_operator_review_replay_segment =
        work_graph_canonical_projection_enablement_operator_review_replay_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_operator_review_replay_consistency"
            });
    let work_graph_canonical_projection_enablement_operator_review_replay_consistency_decision =
        build_work_graph_canonical_projection_enablement_operator_review_replay_consistency_decision(
            WorkGraphCanonicalProjectionEnablementOperatorReviewReplayConsistencyInput {
                source_surface_id: "agent_jobs",
                enablement_operator_review_packet:
                    &work_graph_canonical_projection_enablement_operator_review_packet,
                enablement_operator_review_packet_payload:
                    &work_graph_canonical_projection_enablement_operator_review_packet_payload,
                latest_enablement_operator_review_packet_payload:
                    canonical_projection_enablement_operator_review_segment
                        .and_then(|segment| segment.latest_payload.as_ref()),
                enablement_operator_review_packet_events:
                    canonical_projection_enablement_operator_review_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                enablement_operator_review_packet_readback_ready:
                    canonical_projection_enablement_operator_review_segment
                        .is_some_and(|segment| segment.readback_ready),
                prior_enablement_operator_review_replay_consistency_events:
                    canonical_projection_enablement_operator_review_replay_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                live_blocking_event_count:
                    work_graph_canonical_projection_enablement_operator_review_replay_readback
                        .live_blocking_event_count,
                live_cutover_event_count:
                    work_graph_canonical_projection_enablement_operator_review_replay_readback
                        .live_cutover_event_count,
            },
        );
    let work_graph_canonical_projection_enablement_operator_review_replay_consistency_payload =
        serde_json::to_value(
            &work_graph_canonical_projection_enablement_operator_review_replay_consistency_decision,
        )
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to serialize agent job canonical WorkGraph projection enablement operator-review replay consistency decision: {err}"
            ))
        })?;
    db.append_agent_job_work_graph_canonical_projection_enablement_operator_review_replay_consistency_shadow(
        job_id.as_str(),
        task_id,
        work_graph_canonical_projection_enablement_operator_review_replay_consistency_decision
            .decision,
        work_graph_canonical_projection_enablement_operator_review_replay_consistency_payload,
        turn.trace_id.as_deref(),
    )
    .await
    .map_err(|err| {
        FunctionCallError::RespondToModel(format!(
            "failed to append agent job canonical WorkGraph projection enablement operator-review replay consistency for {job_id}: {err}"
        ))
    })?;
    let work_graph_canonical_projection_enablement_operator_review_replay_readback = db
        .get_agent_job_work_graph_audit_chain_readback(
            job_id.as_str(),
            work_graph_canonical_projection_enablement_operator_review_replay_segment_specs(),
        )
        .await
        .map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to read back recorded agent job canonical WorkGraph projection enablement operator-review replay consistency for {job_id}: {err}"
            ))
        })?;
    let enablement_operator_review_replay_segment_recorded =
        work_graph_canonical_projection_enablement_operator_review_replay_readback
            .segments
            .iter()
            .any(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_operator_review_replay_consistency"
                    && segment.readback_ready
                    && segment.event_count > 0
            });
    if !enablement_operator_review_replay_segment_recorded {
        return Err(FunctionCallError::RespondToModel(format!(
            "agent job canonical WorkGraph projection enablement operator-review replay consistency was not readable for {job_id}"
        )));
    }
    let work_graph_canonical_projection_enablement_no_live_closeout_readback = db
        .get_agent_job_work_graph_audit_chain_readback(
            &job_id,
            work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_segment_specs(),
        )
        .await
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to read back agent job canonical WorkGraph projection enablement no-live rehearsal closeout inputs for {job_id}: {err}"
            ))
        })?;
    let canonical_projection_enablement_operator_review_segment =
        work_graph_canonical_projection_enablement_no_live_closeout_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id == "canonical_projection_enablement_operator_review_packet"
            });
    let canonical_projection_enablement_operator_review_replay_segment =
        work_graph_canonical_projection_enablement_no_live_closeout_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_operator_review_replay_consistency"
            });
    let canonical_projection_enablement_no_live_closeout_segment =
        work_graph_canonical_projection_enablement_no_live_closeout_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_no_live_rehearsal_closeout_receipt"
            });
    let work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt =
        build_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt(
            WorkGraphCanonicalProjectionEnablementNoLiveRehearsalCloseoutInput {
                source_surface_id: "agent_jobs",
                enablement_operator_review_packet:
                    &work_graph_canonical_projection_enablement_operator_review_packet,
                enablement_operator_review_replay_consistency_decision:
                    &work_graph_canonical_projection_enablement_operator_review_replay_consistency_decision,
                enablement_operator_review_packet_events:
                    canonical_projection_enablement_operator_review_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                enablement_operator_review_replay_consistency_events:
                    canonical_projection_enablement_operator_review_replay_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                prior_enablement_no_live_rehearsal_closeout_events:
                    canonical_projection_enablement_no_live_closeout_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                enablement_operator_review_packet_readback_ready:
                    canonical_projection_enablement_operator_review_segment
                        .is_some_and(|segment| segment.readback_ready),
                enablement_operator_review_replay_consistency_ready:
                    canonical_projection_enablement_operator_review_replay_segment
                        .is_some_and(|segment| segment.readback_ready),
                live_blocking_event_count:
                    work_graph_canonical_projection_enablement_no_live_closeout_readback
                        .live_blocking_event_count,
                live_cutover_event_count:
                    work_graph_canonical_projection_enablement_no_live_closeout_readback
                        .live_cutover_event_count,
            },
        );
    let work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_payload =
        serde_json::to_value(
            &work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt,
        )
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to serialize agent job canonical WorkGraph projection enablement no-live rehearsal closeout receipt: {err}"
            ))
        })?;
    db.append_agent_job_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt_shadow(
        job_id.as_str(),
        task_id,
        work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt.decision,
        work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_payload.clone(),
        turn.trace_id.as_deref(),
    )
    .await
    .map_err(|err| {
        FunctionCallError::Fatal(format!(
            "failed to append agent job canonical WorkGraph projection enablement no-live rehearsal closeout receipt for {job_id}: {err}"
        ))
    })?;
    let work_graph_canonical_projection_enablement_no_live_closeout_readback = db
        .get_agent_job_work_graph_audit_chain_readback(
            &job_id,
            work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_segment_specs(),
        )
        .await
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to read back recorded agent job canonical WorkGraph projection enablement no-live rehearsal closeout receipt for {job_id}: {err}"
            ))
        })?;
    let enablement_no_live_closeout_segment_recorded =
        work_graph_canonical_projection_enablement_no_live_closeout_readback
            .segments
            .iter()
            .any(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_no_live_rehearsal_closeout_receipt"
                    && segment.readback_ready
                    && segment.event_count > 0
            });
    if !enablement_no_live_closeout_segment_recorded {
        return Err(FunctionCallError::RespondToModel(format!(
            "agent job canonical WorkGraph projection enablement no-live rehearsal closeout receipt was not readable for {job_id}"
        )));
    }
    let work_graph_canonical_projection_enablement_no_live_closeout_replay_readback = db
        .get_agent_job_work_graph_audit_chain_readback(
            &job_id,
            work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_segment_specs(),
        )
        .await
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to read back agent job canonical WorkGraph projection enablement no-live rehearsal closeout replay inputs for {job_id}: {err}"
            ))
        })?;
    let canonical_projection_enablement_no_live_closeout_segment =
        work_graph_canonical_projection_enablement_no_live_closeout_replay_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_no_live_rehearsal_closeout_receipt"
            });
    let canonical_projection_enablement_no_live_closeout_replay_segment =
        work_graph_canonical_projection_enablement_no_live_closeout_replay_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency"
            });
    let work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_decision =
        build_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_decision(
            WorkGraphCanonicalProjectionEnablementNoLiveRehearsalCloseoutReplayConsistencyInput {
                source_surface_id: "agent_jobs",
                no_live_rehearsal_closeout_receipt:
                    &work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt,
                no_live_rehearsal_closeout_receipt_payload:
                    &work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_payload,
                latest_no_live_rehearsal_closeout_receipt_payload:
                    canonical_projection_enablement_no_live_closeout_segment
                        .and_then(|segment| segment.latest_payload.as_ref()),
                no_live_rehearsal_closeout_events:
                    canonical_projection_enablement_no_live_closeout_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                no_live_rehearsal_closeout_readback_ready:
                    canonical_projection_enablement_no_live_closeout_segment
                        .is_some_and(|segment| segment.readback_ready),
                prior_no_live_rehearsal_closeout_replay_consistency_events:
                    canonical_projection_enablement_no_live_closeout_replay_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                live_blocking_event_count:
                    work_graph_canonical_projection_enablement_no_live_closeout_replay_readback
                        .live_blocking_event_count,
                live_cutover_event_count:
                    work_graph_canonical_projection_enablement_no_live_closeout_replay_readback
                        .live_cutover_event_count,
            },
        );
    let work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_payload =
        serde_json::to_value(
            &work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_decision,
        )
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to serialize agent job canonical WorkGraph projection enablement no-live rehearsal closeout replay consistency decision: {err}"
            ))
        })?;
    db.append_agent_job_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_shadow(
        job_id.as_str(),
        task_id,
        work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_decision
            .decision,
        work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_payload,
        turn.trace_id.as_deref(),
    )
    .await
    .map_err(|err| {
        FunctionCallError::Fatal(format!(
            "failed to append agent job canonical WorkGraph projection enablement no-live rehearsal closeout replay consistency for {job_id}: {err}"
        ))
    })?;
    let work_graph_canonical_projection_enablement_no_live_closeout_replay_readback = db
        .get_agent_job_work_graph_audit_chain_readback(
            &job_id,
            work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_segment_specs(),
        )
        .await
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to read back recorded agent job canonical WorkGraph projection enablement no-live rehearsal closeout replay consistency for {job_id}: {err}"
            ))
        })?;
    let enablement_no_live_closeout_replay_segment_recorded =
        work_graph_canonical_projection_enablement_no_live_closeout_replay_readback
            .segments
            .iter()
            .any(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency"
                    && segment.readback_ready
                    && segment.event_count > 0
            });
    if !enablement_no_live_closeout_replay_segment_recorded {
        return Err(FunctionCallError::RespondToModel(format!(
            "agent job canonical WorkGraph projection enablement no-live rehearsal closeout replay consistency was not readable for {job_id}"
        )));
    }
    let work_graph_canonical_projection_enablement_audit_chain_closeout_readback = db
        .get_agent_job_work_graph_audit_chain_readback(
            &job_id,
            work_graph_canonical_projection_enablement_audit_chain_closeout_segment_specs(),
        )
        .await
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to read back agent job canonical WorkGraph projection enablement audit-chain closeout inputs for {job_id}: {err}"
            ))
        })?;
    let canonical_projection_enablement_operator_review_segment =
        work_graph_canonical_projection_enablement_audit_chain_closeout_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id == "canonical_projection_enablement_operator_review_packet"
            });
    let canonical_projection_enablement_operator_review_replay_segment =
        work_graph_canonical_projection_enablement_audit_chain_closeout_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_operator_review_replay_consistency"
            });
    let canonical_projection_enablement_no_live_closeout_segment =
        work_graph_canonical_projection_enablement_audit_chain_closeout_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_no_live_rehearsal_closeout_receipt"
            });
    let canonical_projection_enablement_no_live_closeout_replay_segment =
        work_graph_canonical_projection_enablement_audit_chain_closeout_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency"
            });
    let canonical_projection_enablement_audit_chain_closeout_segment =
        work_graph_canonical_projection_enablement_audit_chain_closeout_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id == "canonical_projection_enablement_audit_chain_closeout_receipt"
            });
    let work_graph_canonical_projection_enablement_audit_chain_closeout_receipt =
        build_work_graph_canonical_projection_enablement_audit_chain_closeout_receipt(
            WorkGraphCanonicalProjectionEnablementAuditChainCloseoutInput {
                source_surface_id: "agent_jobs",
                enablement_operator_review_packet:
                    &work_graph_canonical_projection_enablement_operator_review_packet,
                enablement_operator_review_replay_consistency_decision:
                    &work_graph_canonical_projection_enablement_operator_review_replay_consistency_decision,
                no_live_rehearsal_closeout_receipt:
                    &work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt,
                no_live_rehearsal_closeout_replay_consistency_decision:
                    &work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_decision,
                enablement_operator_review_packet_events:
                    canonical_projection_enablement_operator_review_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                enablement_operator_review_replay_consistency_events:
                    canonical_projection_enablement_operator_review_replay_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                no_live_rehearsal_closeout_events:
                    canonical_projection_enablement_no_live_closeout_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                no_live_rehearsal_closeout_replay_consistency_events:
                    canonical_projection_enablement_no_live_closeout_replay_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                prior_enablement_audit_chain_closeout_events:
                    canonical_projection_enablement_audit_chain_closeout_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                enablement_operator_review_packet_readback_ready:
                    canonical_projection_enablement_operator_review_segment
                        .is_some_and(|segment| segment.readback_ready),
                enablement_operator_review_replay_consistency_ready:
                    canonical_projection_enablement_operator_review_replay_segment
                        .is_some_and(|segment| segment.readback_ready),
                no_live_rehearsal_closeout_readback_ready:
                    canonical_projection_enablement_no_live_closeout_segment
                        .is_some_and(|segment| segment.readback_ready),
                no_live_rehearsal_closeout_replay_consistency_ready:
                    canonical_projection_enablement_no_live_closeout_replay_segment
                        .is_some_and(|segment| segment.readback_ready),
                live_blocking_event_count:
                    work_graph_canonical_projection_enablement_audit_chain_closeout_readback
                        .live_blocking_event_count,
                live_cutover_event_count:
                    work_graph_canonical_projection_enablement_audit_chain_closeout_readback
                        .live_cutover_event_count,
            },
        );
    let work_graph_canonical_projection_enablement_audit_chain_closeout_payload =
        serde_json::to_value(
            &work_graph_canonical_projection_enablement_audit_chain_closeout_receipt,
        )
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to serialize agent job canonical WorkGraph projection enablement audit-chain closeout receipt: {err}"
            ))
        })?;
    db.append_agent_job_work_graph_canonical_projection_enablement_audit_chain_closeout_receipt_shadow(
        job_id.as_str(),
        task_id,
        work_graph_canonical_projection_enablement_audit_chain_closeout_receipt.decision,
        work_graph_canonical_projection_enablement_audit_chain_closeout_payload.clone(),
        turn.trace_id.as_deref(),
    )
    .await
    .map_err(|err| {
        FunctionCallError::Fatal(format!(
            "failed to append agent job canonical WorkGraph projection enablement audit-chain closeout receipt for {job_id}: {err}"
        ))
    })?;
    let work_graph_canonical_projection_enablement_audit_chain_closeout_readback = db
        .get_agent_job_work_graph_audit_chain_readback(
            &job_id,
            work_graph_canonical_projection_enablement_audit_chain_closeout_segment_specs(),
        )
        .await
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to read back recorded agent job canonical WorkGraph projection enablement audit-chain closeout receipt for {job_id}: {err}"
            ))
        })?;
    let enablement_audit_chain_closeout_segment_recorded =
        work_graph_canonical_projection_enablement_audit_chain_closeout_readback
            .segments
            .iter()
            .any(|segment| {
                segment.segment_id == "canonical_projection_enablement_audit_chain_closeout_receipt"
                    && segment.readback_ready
                    && segment.event_count > 0
            });
    if !enablement_audit_chain_closeout_segment_recorded {
        return Err(FunctionCallError::RespondToModel(format!(
            "agent job canonical WorkGraph projection enablement audit-chain closeout receipt was not readable for {job_id}"
        )));
    }
    let work_graph_canonical_projection_enablement_audit_chain_closeout_replay_readback = db
        .get_agent_job_work_graph_audit_chain_readback(
            &job_id,
            work_graph_canonical_projection_enablement_audit_chain_closeout_replay_segment_specs(),
        )
        .await
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to read back agent job canonical WorkGraph projection enablement audit-chain closeout replay inputs for {job_id}: {err}"
            ))
        })?;
    let canonical_projection_enablement_audit_chain_closeout_segment =
        work_graph_canonical_projection_enablement_audit_chain_closeout_replay_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id == "canonical_projection_enablement_audit_chain_closeout_receipt"
            });
    let canonical_projection_enablement_audit_chain_closeout_replay_segment =
        work_graph_canonical_projection_enablement_audit_chain_closeout_replay_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_audit_chain_closeout_replay_consistency"
            });
    let work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency_decision =
        build_work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency_decision(
            WorkGraphCanonicalProjectionEnablementAuditChainCloseoutReplayConsistencyInput {
                source_surface_id: "agent_jobs",
                enablement_audit_chain_closeout_receipt:
                    &work_graph_canonical_projection_enablement_audit_chain_closeout_receipt,
                enablement_audit_chain_closeout_receipt_payload:
                    &work_graph_canonical_projection_enablement_audit_chain_closeout_payload,
                latest_enablement_audit_chain_closeout_receipt_payload:
                    canonical_projection_enablement_audit_chain_closeout_segment
                        .and_then(|segment| segment.latest_payload.as_ref()),
                enablement_audit_chain_closeout_events:
                    canonical_projection_enablement_audit_chain_closeout_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                enablement_audit_chain_closeout_readback_ready:
                    canonical_projection_enablement_audit_chain_closeout_segment
                        .is_some_and(|segment| segment.readback_ready),
                prior_enablement_audit_chain_closeout_replay_consistency_events:
                    canonical_projection_enablement_audit_chain_closeout_replay_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                live_blocking_event_count:
                    work_graph_canonical_projection_enablement_audit_chain_closeout_replay_readback
                        .live_blocking_event_count,
                live_cutover_event_count:
                    work_graph_canonical_projection_enablement_audit_chain_closeout_replay_readback
                        .live_cutover_event_count,
            },
        );
    let work_graph_canonical_projection_enablement_audit_chain_closeout_replay_payload =
        serde_json::to_value(
            &work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency_decision,
        )
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to serialize agent job canonical WorkGraph projection enablement audit-chain closeout replay consistency decision: {err}"
            ))
        })?;
    db.append_agent_job_work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency_shadow(
        job_id.as_str(),
        task_id,
        work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency_decision
            .decision,
        work_graph_canonical_projection_enablement_audit_chain_closeout_replay_payload,
        turn.trace_id.as_deref(),
    )
    .await
    .map_err(|err| {
        FunctionCallError::Fatal(format!(
            "failed to append agent job canonical WorkGraph projection enablement audit-chain closeout replay consistency for {job_id}: {err}"
        ))
    })?;
    let work_graph_canonical_projection_enablement_audit_chain_closeout_replay_readback = db
        .get_agent_job_work_graph_audit_chain_readback(
            &job_id,
            work_graph_canonical_projection_enablement_audit_chain_closeout_replay_segment_specs(),
        )
        .await
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to read back recorded agent job canonical WorkGraph projection enablement audit-chain closeout replay consistency for {job_id}: {err}"
            ))
        })?;
    let enablement_audit_chain_closeout_replay_segment_recorded =
        work_graph_canonical_projection_enablement_audit_chain_closeout_replay_readback
            .segments
            .iter()
            .any(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_audit_chain_closeout_replay_consistency"
                    && segment.readback_ready
                    && segment.event_count > 0
            });
    if !enablement_audit_chain_closeout_replay_segment_recorded {
        return Err(FunctionCallError::RespondToModel(format!(
            "agent job canonical WorkGraph projection enablement audit-chain closeout replay consistency was not readable for {job_id}"
        )));
    }
    let work_graph_canonical_projection_enablement_activation_precondition_readback = db
        .get_agent_job_work_graph_audit_chain_readback(
            &job_id,
            work_graph_canonical_projection_enablement_activation_precondition_segment_specs(),
        )
        .await
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to read back agent job canonical WorkGraph projection enablement activation precondition inputs for {job_id}: {err}"
            ))
        })?;
    let canonical_projection_enablement_audit_chain_closeout_segment =
        work_graph_canonical_projection_enablement_activation_precondition_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id == "canonical_projection_enablement_audit_chain_closeout_receipt"
            });
    let canonical_projection_enablement_audit_chain_closeout_replay_segment =
        work_graph_canonical_projection_enablement_activation_precondition_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_audit_chain_closeout_replay_consistency"
            });
    let canonical_projection_enablement_activation_precondition_segment =
        work_graph_canonical_projection_enablement_activation_precondition_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_activation_precondition_operator_packet"
            });
    let work_graph_canonical_projection_enablement_activation_precondition_operator_packet =
        build_work_graph_canonical_projection_enablement_activation_precondition_operator_packet(
            WorkGraphCanonicalProjectionEnablementActivationPreconditionOperatorPacketInput {
                source_surface_id: "agent_jobs",
                enablement_audit_chain_closeout_receipt:
                    &work_graph_canonical_projection_enablement_audit_chain_closeout_receipt,
                enablement_audit_chain_closeout_replay_consistency_decision:
                    &work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency_decision,
                enablement_audit_chain_closeout_events:
                    canonical_projection_enablement_audit_chain_closeout_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                enablement_audit_chain_closeout_replay_consistency_events:
                    canonical_projection_enablement_audit_chain_closeout_replay_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                prior_enablement_activation_precondition_operator_packet_events:
                    canonical_projection_enablement_activation_precondition_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                enablement_audit_chain_closeout_readback_ready:
                    canonical_projection_enablement_audit_chain_closeout_segment
                        .is_some_and(|segment| segment.readback_ready),
                enablement_audit_chain_closeout_replay_consistency_ready:
                    canonical_projection_enablement_audit_chain_closeout_replay_segment
                        .is_some_and(|segment| segment.readback_ready),
                live_blocking_event_count:
                    work_graph_canonical_projection_enablement_activation_precondition_readback
                        .live_blocking_event_count,
                live_cutover_event_count:
                    work_graph_canonical_projection_enablement_activation_precondition_readback
                        .live_cutover_event_count,
            },
        );
    let work_graph_canonical_projection_enablement_activation_precondition_payload =
        serde_json::to_value(
            &work_graph_canonical_projection_enablement_activation_precondition_operator_packet,
        )
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to serialize agent job canonical WorkGraph projection enablement activation precondition operator packet: {err}"
            ))
        })?;
    db.append_agent_job_work_graph_canonical_projection_enablement_activation_precondition_operator_packet_shadow(
        job_id.as_str(),
        task_id,
        work_graph_canonical_projection_enablement_activation_precondition_operator_packet
            .decision,
        work_graph_canonical_projection_enablement_activation_precondition_payload.clone(),
        turn.trace_id.as_deref(),
    )
    .await
    .map_err(|err| {
        FunctionCallError::Fatal(format!(
            "failed to append agent job canonical WorkGraph projection enablement activation precondition operator packet for {job_id}: {err}"
        ))
    })?;
    let work_graph_canonical_projection_enablement_activation_precondition_readback = db
        .get_agent_job_work_graph_audit_chain_readback(
            &job_id,
            work_graph_canonical_projection_enablement_activation_precondition_segment_specs(),
        )
        .await
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to read back recorded agent job canonical WorkGraph projection enablement activation precondition operator packet for {job_id}: {err}"
            ))
        })?;
    let activation_precondition_segment_recorded =
        work_graph_canonical_projection_enablement_activation_precondition_readback
            .segments
            .iter()
            .any(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_activation_precondition_operator_packet"
                    && segment.readback_ready
                    && segment.event_count > 0
            });
    if !activation_precondition_segment_recorded {
        return Err(FunctionCallError::RespondToModel(format!(
            "agent job canonical WorkGraph projection enablement activation precondition operator packet was not readable for {job_id}"
        )));
    }
    let work_graph_canonical_projection_enablement_activation_precondition_replay_readback = db
        .get_agent_job_work_graph_audit_chain_readback(
            &job_id,
            work_graph_canonical_projection_enablement_activation_precondition_replay_segment_specs(),
        )
        .await
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to read back agent job canonical WorkGraph projection enablement activation precondition replay inputs for {job_id}: {err}"
            ))
        })?;
    let canonical_projection_enablement_activation_precondition_segment =
        work_graph_canonical_projection_enablement_activation_precondition_replay_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_activation_precondition_operator_packet"
            });
    let canonical_projection_enablement_activation_precondition_replay_segment =
        work_graph_canonical_projection_enablement_activation_precondition_replay_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_activation_precondition_replay_consistency"
            });
    let work_graph_canonical_projection_enablement_activation_precondition_replay_consistency_decision =
        build_work_graph_canonical_projection_enablement_activation_precondition_replay_consistency_decision(
            WorkGraphCanonicalProjectionEnablementActivationPreconditionReplayConsistencyInput {
                source_surface_id: "agent_jobs",
                activation_precondition_operator_packet:
                    &work_graph_canonical_projection_enablement_activation_precondition_operator_packet,
                activation_precondition_operator_packet_payload:
                    &work_graph_canonical_projection_enablement_activation_precondition_payload,
                latest_activation_precondition_operator_packet_payload:
                    canonical_projection_enablement_activation_precondition_segment
                        .and_then(|segment| segment.latest_payload.as_ref()),
                activation_precondition_operator_packet_events:
                    canonical_projection_enablement_activation_precondition_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                activation_precondition_operator_packet_readback_ready:
                    canonical_projection_enablement_activation_precondition_segment
                        .is_some_and(|segment| segment.readback_ready),
                prior_activation_precondition_replay_consistency_events:
                    canonical_projection_enablement_activation_precondition_replay_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                live_blocking_event_count:
                    work_graph_canonical_projection_enablement_activation_precondition_replay_readback
                        .live_blocking_event_count,
                live_cutover_event_count:
                    work_graph_canonical_projection_enablement_activation_precondition_replay_readback
                        .live_cutover_event_count,
            },
        );
    let work_graph_canonical_projection_enablement_activation_precondition_replay_payload =
        serde_json::to_value(
            &work_graph_canonical_projection_enablement_activation_precondition_replay_consistency_decision,
        )
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to serialize agent job canonical WorkGraph projection enablement activation precondition replay consistency decision: {err}"
            ))
        })?;
    db.append_agent_job_work_graph_canonical_projection_enablement_activation_precondition_replay_consistency_shadow(
        job_id.as_str(),
        task_id,
        work_graph_canonical_projection_enablement_activation_precondition_replay_consistency_decision
            .decision,
        work_graph_canonical_projection_enablement_activation_precondition_replay_payload,
        turn.trace_id.as_deref(),
    )
    .await
    .map_err(|err| {
        FunctionCallError::Fatal(format!(
            "failed to append agent job canonical WorkGraph projection enablement activation precondition replay consistency for {job_id}: {err}"
        ))
    })?;
    let work_graph_canonical_projection_enablement_activation_precondition_replay_readback = db
        .get_agent_job_work_graph_audit_chain_readback(
            &job_id,
            work_graph_canonical_projection_enablement_activation_precondition_replay_segment_specs(),
        )
        .await
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to read back recorded agent job canonical WorkGraph projection enablement activation precondition replay consistency for {job_id}: {err}"
            ))
        })?;
    let activation_precondition_replay_segment_recorded =
        work_graph_canonical_projection_enablement_activation_precondition_replay_readback
            .segments
            .iter()
            .any(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_activation_precondition_replay_consistency"
                    && segment.readback_ready
                    && segment.event_count > 0
            });
    if !activation_precondition_replay_segment_recorded {
        return Err(FunctionCallError::RespondToModel(format!(
            "agent job canonical WorkGraph projection enablement activation precondition replay consistency was not readable for {job_id}"
        )));
    }
    let work_graph_canonical_projection_enablement_activation_no_live_closeout_readback = db
        .get_agent_job_work_graph_audit_chain_readback(
            &job_id,
            work_graph_canonical_projection_enablement_activation_no_live_closeout_segment_specs(),
        )
        .await
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to read back agent job canonical WorkGraph projection enablement activation no-live closeout inputs for {job_id}: {err}"
            ))
        })?;
    let canonical_projection_enablement_activation_precondition_segment =
        work_graph_canonical_projection_enablement_activation_no_live_closeout_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_activation_precondition_operator_packet"
            });
    let canonical_projection_enablement_activation_precondition_replay_segment =
        work_graph_canonical_projection_enablement_activation_no_live_closeout_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_activation_precondition_replay_consistency"
            });
    let canonical_projection_enablement_activation_no_live_closeout_segment =
        work_graph_canonical_projection_enablement_activation_no_live_closeout_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_activation_no_live_closeout_receipt"
            });
    let work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt =
        build_work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt(
            WorkGraphCanonicalProjectionEnablementActivationNoLiveCloseoutInput {
                source_surface_id: "agent_jobs",
                activation_precondition_operator_packet:
                    &work_graph_canonical_projection_enablement_activation_precondition_operator_packet,
                activation_precondition_replay_consistency_decision:
                    &work_graph_canonical_projection_enablement_activation_precondition_replay_consistency_decision,
                activation_precondition_operator_packet_events:
                    canonical_projection_enablement_activation_precondition_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                activation_precondition_replay_consistency_events:
                    canonical_projection_enablement_activation_precondition_replay_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                prior_activation_no_live_closeout_events:
                    canonical_projection_enablement_activation_no_live_closeout_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                activation_precondition_operator_packet_readback_ready:
                    canonical_projection_enablement_activation_precondition_segment
                        .is_some_and(|segment| segment.readback_ready),
                activation_precondition_replay_consistency_ready:
                    canonical_projection_enablement_activation_precondition_replay_segment
                        .is_some_and(|segment| segment.readback_ready),
                live_blocking_event_count:
                    work_graph_canonical_projection_enablement_activation_no_live_closeout_readback
                        .live_blocking_event_count,
                live_cutover_event_count:
                    work_graph_canonical_projection_enablement_activation_no_live_closeout_readback
                        .live_cutover_event_count,
            },
        );
    let work_graph_canonical_projection_enablement_activation_no_live_closeout_payload =
        serde_json::to_value(
            &work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt,
        )
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to serialize agent job canonical WorkGraph projection enablement activation no-live closeout receipt: {err}"
            ))
        })?;
    db.append_agent_job_work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt_shadow(
        job_id.as_str(),
        task_id,
        work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt.decision,
        work_graph_canonical_projection_enablement_activation_no_live_closeout_payload.clone(),
        turn.trace_id.as_deref(),
    )
    .await
    .map_err(|err| {
        FunctionCallError::Fatal(format!(
            "failed to append agent job canonical WorkGraph projection enablement activation no-live closeout receipt for {job_id}: {err}"
        ))
    })?;
    let work_graph_canonical_projection_enablement_activation_no_live_closeout_readback = db
        .get_agent_job_work_graph_audit_chain_readback(
            &job_id,
            work_graph_canonical_projection_enablement_activation_no_live_closeout_segment_specs(),
        )
        .await
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to read back recorded agent job canonical WorkGraph projection enablement activation no-live closeout receipt for {job_id}: {err}"
            ))
        })?;
    let activation_no_live_closeout_segment_recorded =
        work_graph_canonical_projection_enablement_activation_no_live_closeout_readback
            .segments
            .iter()
            .any(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_activation_no_live_closeout_receipt"
                    && segment.readback_ready
                    && segment.event_count > 0
            });
    if !activation_no_live_closeout_segment_recorded {
        return Err(FunctionCallError::RespondToModel(format!(
            "agent job canonical WorkGraph projection enablement activation no-live closeout receipt was not readable for {job_id}"
        )));
    }
    let work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_readback = db
        .get_agent_job_work_graph_audit_chain_readback(
            &job_id,
            work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_segment_specs(),
        )
        .await
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to read back agent job canonical WorkGraph projection enablement activation no-live closeout replay inputs for {job_id}: {err}"
            ))
        })?;
    let canonical_projection_enablement_activation_no_live_closeout_segment =
        work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_activation_no_live_closeout_receipt"
            });
    let canonical_projection_enablement_activation_no_live_closeout_replay_segment =
        work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_activation_no_live_closeout_replay_consistency"
            });
    let work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_decision =
        build_work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_decision(
            WorkGraphCanonicalProjectionEnablementActivationNoLiveCloseoutReplayConsistencyInput {
                source_surface_id: "agent_jobs",
                activation_no_live_closeout_receipt:
                    &work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt,
                activation_no_live_closeout_receipt_payload:
                    &work_graph_canonical_projection_enablement_activation_no_live_closeout_payload,
                latest_activation_no_live_closeout_receipt_payload:
                    canonical_projection_enablement_activation_no_live_closeout_segment
                        .and_then(|segment| segment.latest_payload.as_ref()),
                activation_no_live_closeout_events:
                    canonical_projection_enablement_activation_no_live_closeout_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                activation_no_live_closeout_readback_ready:
                    canonical_projection_enablement_activation_no_live_closeout_segment
                        .is_some_and(|segment| segment.readback_ready),
                prior_activation_no_live_closeout_replay_consistency_events:
                    canonical_projection_enablement_activation_no_live_closeout_replay_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                live_blocking_event_count:
                    work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_readback
                        .live_blocking_event_count,
                live_cutover_event_count:
                    work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_readback
                        .live_cutover_event_count,
            },
        );
    let work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_payload =
        serde_json::to_value(
            &work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_decision,
        )
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to serialize agent job canonical WorkGraph projection enablement activation no-live closeout replay consistency decision: {err}"
            ))
        })?;
    db.append_agent_job_work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_shadow(
        job_id.as_str(),
        task_id,
        work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_decision
            .decision,
        work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_payload,
        turn.trace_id.as_deref(),
    )
    .await
    .map_err(|err| {
        FunctionCallError::Fatal(format!(
            "failed to append agent job canonical WorkGraph projection enablement activation no-live closeout replay consistency for {job_id}: {err}"
        ))
    })?;
    let work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_readback = db
        .get_agent_job_work_graph_audit_chain_readback(
            &job_id,
            work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_segment_specs(),
        )
        .await
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to read back recorded agent job canonical WorkGraph projection enablement activation no-live closeout replay consistency for {job_id}: {err}"
            ))
        })?;
    let activation_no_live_closeout_replay_segment_recorded =
        work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_readback
            .segments
            .iter()
            .any(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_activation_no_live_closeout_replay_consistency"
                    && segment.readback_ready
                    && segment.event_count > 0
            });
    if !activation_no_live_closeout_replay_segment_recorded {
        return Err(FunctionCallError::RespondToModel(format!(
            "agent job canonical WorkGraph projection enablement activation no-live closeout replay consistency was not readable for {job_id}"
        )));
    }
    let work_graph_canonical_projection_enablement_activation_audit_chain_closeout_readback = db
        .get_agent_job_work_graph_audit_chain_readback(
            &job_id,
            work_graph_canonical_projection_enablement_activation_audit_chain_closeout_segment_specs(),
        )
        .await
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to read back agent job canonical WorkGraph projection enablement activation audit-chain closeout inputs for {job_id}: {err}"
            ))
        })?;
    let canonical_projection_enablement_activation_precondition_segment =
        work_graph_canonical_projection_enablement_activation_audit_chain_closeout_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_activation_precondition_operator_packet"
            });
    let canonical_projection_enablement_activation_precondition_replay_segment =
        work_graph_canonical_projection_enablement_activation_audit_chain_closeout_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_activation_precondition_replay_consistency"
            });
    let canonical_projection_enablement_activation_no_live_closeout_segment =
        work_graph_canonical_projection_enablement_activation_audit_chain_closeout_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_activation_no_live_closeout_receipt"
            });
    let canonical_projection_enablement_activation_no_live_closeout_replay_segment =
        work_graph_canonical_projection_enablement_activation_audit_chain_closeout_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_activation_no_live_closeout_replay_consistency"
            });
    let canonical_projection_enablement_activation_audit_chain_closeout_segment =
        work_graph_canonical_projection_enablement_activation_audit_chain_closeout_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_activation_audit_chain_closeout_receipt"
            });
    let work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt =
        build_work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt(
            WorkGraphCanonicalProjectionEnablementActivationAuditChainCloseoutInput {
                source_surface_id: "agent_jobs",
                activation_precondition_operator_packet:
                    &work_graph_canonical_projection_enablement_activation_precondition_operator_packet,
                activation_precondition_replay_consistency_decision:
                    &work_graph_canonical_projection_enablement_activation_precondition_replay_consistency_decision,
                activation_no_live_closeout_receipt:
                    &work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt,
                activation_no_live_closeout_replay_consistency_decision:
                    &work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_decision,
                activation_precondition_operator_packet_events:
                    canonical_projection_enablement_activation_precondition_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                activation_precondition_replay_consistency_events:
                    canonical_projection_enablement_activation_precondition_replay_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                activation_no_live_closeout_events:
                    canonical_projection_enablement_activation_no_live_closeout_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                activation_no_live_closeout_replay_consistency_events:
                    canonical_projection_enablement_activation_no_live_closeout_replay_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                prior_activation_audit_chain_closeout_events:
                    canonical_projection_enablement_activation_audit_chain_closeout_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                activation_precondition_operator_packet_readback_ready:
                    canonical_projection_enablement_activation_precondition_segment
                        .is_some_and(|segment| segment.readback_ready),
                activation_precondition_replay_consistency_ready:
                    canonical_projection_enablement_activation_precondition_replay_segment
                        .is_some_and(|segment| segment.readback_ready),
                activation_no_live_closeout_readback_ready:
                    canonical_projection_enablement_activation_no_live_closeout_segment
                        .is_some_and(|segment| segment.readback_ready),
                activation_no_live_closeout_replay_consistency_ready:
                    canonical_projection_enablement_activation_no_live_closeout_replay_segment
                        .is_some_and(|segment| segment.readback_ready),
                live_blocking_event_count:
                    work_graph_canonical_projection_enablement_activation_audit_chain_closeout_readback
                        .live_blocking_event_count,
                live_cutover_event_count:
                    work_graph_canonical_projection_enablement_activation_audit_chain_closeout_readback
                        .live_cutover_event_count,
            },
        );
    let work_graph_canonical_projection_enablement_activation_audit_chain_closeout_payload =
        serde_json::to_value(
            &work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt,
        )
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to serialize agent job canonical WorkGraph projection enablement activation audit-chain closeout receipt: {err}"
            ))
        })?;
    db.append_agent_job_work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt_shadow(
        job_id.as_str(),
        task_id,
        work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt
            .decision,
        work_graph_canonical_projection_enablement_activation_audit_chain_closeout_payload.clone(),
        turn.trace_id.as_deref(),
    )
    .await
    .map_err(|err| {
        FunctionCallError::Fatal(format!(
            "failed to append agent job canonical WorkGraph projection enablement activation audit-chain closeout for {job_id}: {err}"
        ))
    })?;
    let work_graph_canonical_projection_enablement_activation_audit_chain_closeout_readback = db
        .get_agent_job_work_graph_audit_chain_readback(
            &job_id,
            work_graph_canonical_projection_enablement_activation_audit_chain_closeout_segment_specs(),
        )
        .await
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to read back recorded agent job canonical WorkGraph projection enablement activation audit-chain closeout receipt for {job_id}: {err}"
            ))
        })?;
    let activation_audit_chain_closeout_segment_recorded =
        work_graph_canonical_projection_enablement_activation_audit_chain_closeout_readback
            .segments
            .iter()
            .any(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_activation_audit_chain_closeout_receipt"
                    && segment.readback_ready
                    && segment.event_count > 0
            });
    if !activation_audit_chain_closeout_segment_recorded {
        return Err(FunctionCallError::RespondToModel(format!(
            "agent job canonical WorkGraph projection enablement activation audit-chain closeout receipt was not readable for {job_id}"
        )));
    }
    let work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_readback =
        db.get_agent_job_work_graph_audit_chain_readback(
            &job_id,
            work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_segment_specs(),
        )
        .await
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to read back agent job canonical WorkGraph projection enablement activation audit-chain closeout replay inputs for {job_id}: {err}"
            ))
        })?;
    let canonical_projection_enablement_activation_audit_chain_closeout_segment =
        work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_activation_audit_chain_closeout_receipt"
            });
    let canonical_projection_enablement_activation_audit_chain_closeout_replay_segment =
        work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency"
            });
    let work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_decision =
        build_work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_decision(
            WorkGraphCanonicalProjectionEnablementActivationAuditChainCloseoutReplayConsistencyInput {
                source_surface_id: "agent_jobs",
                activation_audit_chain_closeout_receipt:
                    &work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt,
                activation_audit_chain_closeout_receipt_payload:
                    &work_graph_canonical_projection_enablement_activation_audit_chain_closeout_payload,
                latest_activation_audit_chain_closeout_receipt_payload:
                    canonical_projection_enablement_activation_audit_chain_closeout_segment
                        .and_then(|segment| segment.latest_payload.as_ref()),
                activation_audit_chain_closeout_events:
                    canonical_projection_enablement_activation_audit_chain_closeout_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                activation_audit_chain_closeout_readback_ready:
                    canonical_projection_enablement_activation_audit_chain_closeout_segment
                        .is_some_and(|segment| segment.readback_ready),
                prior_activation_audit_chain_closeout_replay_consistency_events:
                    canonical_projection_enablement_activation_audit_chain_closeout_replay_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                live_blocking_event_count:
                    work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_readback
                        .live_blocking_event_count,
                live_cutover_event_count:
                    work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_readback
                        .live_cutover_event_count,
            },
        );
    let work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_payload =
        serde_json::to_value(
            &work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_decision,
        )
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to serialize agent job canonical WorkGraph projection enablement activation audit-chain closeout replay consistency decision: {err}"
            ))
        })?;
    db.append_agent_job_work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_shadow(
        job_id.as_str(),
        task_id,
        work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_decision
            .decision,
        work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_payload,
        turn.trace_id.as_deref(),
    )
    .await
    .map_err(|err| {
        FunctionCallError::Fatal(format!(
            "failed to append agent job canonical WorkGraph projection enablement activation audit-chain closeout replay consistency for {job_id}: {err}"
        ))
    })?;
    let work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_readback =
        db.get_agent_job_work_graph_audit_chain_readback(
            &job_id,
            work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_segment_specs(),
        )
        .await
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to read back recorded agent job canonical WorkGraph projection enablement activation audit-chain closeout replay consistency for {job_id}: {err}"
            ))
        })?;
    let activation_audit_chain_closeout_replay_segment_recorded =
        work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_readback
            .segments
            .iter()
            .any(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency"
                    && segment.readback_ready
                    && segment.event_count > 0
            });
    if !activation_audit_chain_closeout_replay_segment_recorded {
        return Err(FunctionCallError::RespondToModel(format!(
            "agent job canonical WorkGraph projection enablement activation audit-chain closeout replay consistency was not readable for {job_id}"
        )));
    }
    let work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_readback =
        db.get_agent_job_work_graph_audit_chain_readback(
            &job_id,
            work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_segment_specs(),
        )
        .await
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to read back agent job canonical WorkGraph projection enablement activation operator-approval readiness preflight inputs for {job_id}: {err}"
            ))
        })?;
    let canonical_projection_enablement_activation_audit_chain_closeout_segment =
        work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_activation_audit_chain_closeout_receipt"
            });
    let canonical_projection_enablement_activation_audit_chain_closeout_replay_segment =
        work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency"
            });
    let canonical_projection_enablement_activation_operator_approval_readiness_preflight_segment =
        work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet"
            });
    let work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet =
        build_work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet(
            WorkGraphCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightPacketInput {
                source_surface_id: "agent_jobs",
                activation_audit_chain_closeout_receipt:
                    &work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt,
                activation_audit_chain_closeout_replay_consistency_decision:
                    &work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_decision,
                activation_audit_chain_closeout_events:
                    canonical_projection_enablement_activation_audit_chain_closeout_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                activation_audit_chain_closeout_replay_consistency_events:
                    canonical_projection_enablement_activation_audit_chain_closeout_replay_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                prior_activation_operator_approval_readiness_preflight_packet_events:
                    canonical_projection_enablement_activation_operator_approval_readiness_preflight_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                activation_audit_chain_closeout_readback_ready:
                    canonical_projection_enablement_activation_audit_chain_closeout_segment
                        .is_some_and(|segment| segment.readback_ready),
                activation_audit_chain_closeout_replay_consistency_ready:
                    canonical_projection_enablement_activation_audit_chain_closeout_replay_segment
                        .is_some_and(|segment| segment.ready),
                live_blocking_event_count:
                    work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_readback
                        .live_blocking_event_count,
                live_cutover_event_count:
                    work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_readback
                        .live_cutover_event_count,
            },
        );
    let work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_payload =
        serde_json::to_value(
            &work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet,
        )
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to serialize agent job canonical WorkGraph projection enablement activation operator-approval readiness preflight packet: {err}"
            ))
        })?;
    db.append_agent_job_work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_shadow(
        job_id.as_str(),
        task_id,
        work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet
            .decision,
        work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_payload
            .clone(),
        turn.trace_id.as_deref(),
    )
    .await
    .map_err(|err| {
        FunctionCallError::Fatal(format!(
            "failed to append agent job canonical WorkGraph projection enablement activation operator-approval readiness preflight packet for {job_id}: {err}"
        ))
    })?;
    let work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_readback =
        db.get_agent_job_work_graph_audit_chain_readback(
            &job_id,
            work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_segment_specs(),
        )
        .await
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to read back recorded agent job canonical WorkGraph projection enablement activation operator-approval readiness preflight packet for {job_id}: {err}"
            ))
        })?;
    let activation_operator_approval_readiness_preflight_segment_recorded =
        work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_readback
            .segments
            .iter()
            .any(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet"
                    && segment.readback_ready
                    && segment.event_count > 0
            });
    if !activation_operator_approval_readiness_preflight_segment_recorded {
        return Err(FunctionCallError::RespondToModel(format!(
            "agent job canonical WorkGraph projection enablement activation operator-approval readiness preflight packet was not readable for {job_id}"
        )));
    }
    let work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_readback =
        db.get_agent_job_work_graph_audit_chain_readback(
            &job_id,
            work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_segment_specs(),
        )
        .await
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to read back agent job canonical WorkGraph projection enablement activation operator-approval readiness preflight replay inputs for {job_id}: {err}"
            ))
        })?;
    let canonical_projection_enablement_activation_operator_approval_readiness_preflight_segment =
        work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet"
            });
    let canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_segment =
        work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency"
            });
    let work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision =
        build_work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision(
            WorkGraphCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightReplayConsistencyInput {
                source_surface_id: "agent_jobs",
                activation_operator_approval_readiness_preflight_packet:
                    &work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet,
                activation_operator_approval_readiness_preflight_packet_payload:
                    &work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_payload,
                latest_activation_operator_approval_readiness_preflight_packet_payload:
                    canonical_projection_enablement_activation_operator_approval_readiness_preflight_segment
                        .and_then(|segment| segment.latest_payload.as_ref()),
                activation_operator_approval_readiness_preflight_packet_events:
                    canonical_projection_enablement_activation_operator_approval_readiness_preflight_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                activation_operator_approval_readiness_preflight_packet_readback_ready:
                    canonical_projection_enablement_activation_operator_approval_readiness_preflight_segment
                        .is_some_and(|segment| segment.readback_ready),
                prior_activation_operator_approval_readiness_preflight_replay_consistency_events:
                    canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                live_blocking_event_count:
                    work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_readback
                        .live_blocking_event_count,
                live_cutover_event_count:
                    work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_readback
                        .live_cutover_event_count,
            },
        );
    let work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_payload =
        serde_json::to_value(
            &work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision,
        )
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to serialize agent job canonical WorkGraph projection enablement activation operator-approval readiness preflight replay consistency decision: {err}"
            ))
        })?;
    db.append_agent_job_work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_shadow(
        job_id.as_str(),
        task_id,
        work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision
            .decision,
        work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_payload,
        turn.trace_id.as_deref(),
    )
    .await
    .map_err(|err| {
        FunctionCallError::Fatal(format!(
            "failed to append agent job canonical WorkGraph projection enablement activation operator-approval readiness preflight replay consistency for {job_id}: {err}"
        ))
    })?;
    let work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_readback =
        db.get_agent_job_work_graph_audit_chain_readback(
            &job_id,
            work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_segment_specs(),
        )
        .await
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to read back recorded agent job canonical WorkGraph projection enablement activation operator-approval readiness preflight replay consistency for {job_id}: {err}"
            ))
        })?;
    let activation_operator_approval_readiness_preflight_replay_segment_recorded =
        work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_readback
            .segments
            .iter()
            .any(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency"
                    && segment.readback_ready
                    && segment.event_count > 0
            });
    if !activation_operator_approval_readiness_preflight_replay_segment_recorded {
        return Err(FunctionCallError::RespondToModel(format!(
            "agent job canonical WorkGraph projection enablement activation operator-approval readiness preflight replay consistency was not readable for {job_id}"
        )));
    }
    let work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_readback =
        db.get_agent_job_work_graph_audit_chain_readback(
            &job_id,
            work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_segment_specs(),
        )
        .await
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to read back agent job canonical WorkGraph projection enablement activation approval/review side-effect lock closeout inputs for {job_id}: {err}"
            ))
        })?;
    let canonical_projection_enablement_activation_operator_approval_readiness_preflight_segment =
        work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet"
            });
    let canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_segment =
        work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency"
            });
    let canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_segment =
        work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet"
            });
    let work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet =
        build_work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet(
            WorkGraphCanonicalProjectionEnablementActivationApprovalReviewSideEffectLockCloseoutPacketInput {
                source_surface_id: "agent_jobs",
                activation_operator_approval_readiness_preflight_packet:
                    &work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet,
                activation_operator_approval_readiness_preflight_replay_consistency_decision:
                    &work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision,
                activation_operator_approval_readiness_preflight_packet_events:
                    canonical_projection_enablement_activation_operator_approval_readiness_preflight_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                activation_operator_approval_readiness_preflight_replay_consistency_events:
                    canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                prior_activation_approval_review_side_effect_lock_closeout_packet_events:
                    canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                activation_operator_approval_readiness_preflight_packet_readback_ready:
                    canonical_projection_enablement_activation_operator_approval_readiness_preflight_segment
                        .is_some_and(|segment| segment.readback_ready),
                activation_operator_approval_readiness_preflight_replay_consistency_ready:
                    canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_segment
                        .is_some_and(|segment| segment.ready),
                live_blocking_event_count:
                    work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_readback
                        .live_blocking_event_count,
                live_cutover_event_count:
                    work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_readback
                        .live_cutover_event_count,
            },
        );
    let work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_payload =
        serde_json::to_value(
            &work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet,
        )
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to serialize agent job canonical WorkGraph projection enablement activation approval/review side-effect lock closeout packet: {err}"
            ))
        })?;
    db.append_agent_job_work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet_shadow(
        job_id.as_str(),
        task_id,
        work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet
            .decision,
        work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_payload
            .clone(),
        turn.trace_id.as_deref(),
    )
    .await
    .map_err(|err| {
        FunctionCallError::Fatal(format!(
            "failed to append agent job canonical WorkGraph projection enablement activation approval/review side-effect lock closeout packet for {job_id}: {err}"
        ))
    })?;
    let work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_readback =
        db.get_agent_job_work_graph_audit_chain_readback(
            &job_id,
            work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_segment_specs(),
        )
        .await
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to read back recorded agent job canonical WorkGraph projection enablement activation approval/review side-effect lock closeout packet for {job_id}: {err}"
            ))
        })?;
    let activation_approval_review_side_effect_lock_closeout_segment_recorded =
        work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_readback
            .segments
            .iter()
            .any(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet"
                    && segment.readback_ready
                    && segment.event_count > 0
            });
    if !activation_approval_review_side_effect_lock_closeout_segment_recorded {
        return Err(FunctionCallError::RespondToModel(format!(
            "agent job canonical WorkGraph projection enablement activation approval/review side-effect lock closeout packet was not readable for {job_id}"
        )));
    }
    let work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_readback =
        db.get_agent_job_work_graph_audit_chain_readback(
            &job_id,
            work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_segment_specs(),
        )
        .await
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to read back agent job canonical WorkGraph projection enablement activation approval/review side-effect lock closeout replay inputs for {job_id}: {err}"
            ))
        })?;
    let canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_segment =
        work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet"
            });
    let canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_segment =
        work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_readback
            .segments
            .iter()
            .find(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency"
            });
    let work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_decision =
        build_work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_decision(
            WorkGraphCanonicalProjectionEnablementActivationApprovalReviewSideEffectLockCloseoutReplayConsistencyInput {
                source_surface_id: "agent_jobs",
                activation_approval_review_side_effect_lock_closeout_packet:
                    &work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet,
                activation_approval_review_side_effect_lock_closeout_packet_payload:
                    &work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_payload,
                latest_activation_approval_review_side_effect_lock_closeout_packet_payload:
                    canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_segment
                        .and_then(|segment| segment.latest_payload.as_ref()),
                activation_approval_review_side_effect_lock_closeout_packet_events:
                    canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                activation_approval_review_side_effect_lock_closeout_packet_readback_ready:
                    canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_segment
                        .is_some_and(|segment| segment.readback_ready),
                prior_activation_approval_review_side_effect_lock_closeout_replay_consistency_events:
                    canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_segment
                        .map(|segment| segment.event_count)
                        .unwrap_or_default(),
                live_blocking_event_count:
                    work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_readback
                        .live_blocking_event_count,
                live_cutover_event_count:
                    work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_readback
                        .live_cutover_event_count,
            },
        );
    let work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_payload =
        serde_json::to_value(
            &work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_decision,
        )
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to serialize agent job canonical WorkGraph projection enablement activation approval/review side-effect lock closeout replay consistency decision: {err}"
            ))
        })?;
    db.append_agent_job_work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_shadow(
        job_id.as_str(),
        task_id,
        work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_decision
            .decision,
        work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_payload,
        turn.trace_id.as_deref(),
    )
    .await
    .map_err(|err| {
        FunctionCallError::Fatal(format!(
            "failed to append agent job canonical WorkGraph projection enablement activation approval/review side-effect lock closeout replay consistency for {job_id}: {err}"
        ))
    })?;
    let work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_readback =
        db.get_agent_job_work_graph_audit_chain_readback(
            &job_id,
            work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_segment_specs(),
        )
        .await
        .map_err(|err| {
            FunctionCallError::Fatal(format!(
                "failed to read back recorded agent job canonical WorkGraph projection enablement activation approval/review side-effect lock closeout replay consistency for {job_id}: {err}"
            ))
        })?;
    let activation_approval_review_side_effect_lock_closeout_replay_segment_recorded =
        work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_readback
            .segments
            .iter()
            .any(|segment| {
                segment.segment_id
                    == "canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency"
                    && segment.readback_ready
                    && segment.event_count > 0
            });
    if !activation_approval_review_side_effect_lock_closeout_replay_segment_recorded {
        return Err(FunctionCallError::RespondToModel(format!(
            "agent job canonical WorkGraph projection enablement activation approval/review side-effect lock closeout replay consistency was not readable for {job_id}"
        )));
    }
    let content = serde_json::to_string(&SpawnAgentsOnCsvResult {
        governance_output_index: vec![
            "admission_shadow_decision",
            "promotion_readiness_shadow_matrix",
            "operator_review_promotion_packet",
            "promotion_review_replay_consistency_decision",
            "promotion_closeout_receipt",
            "promotion_closeout_replay_consistency_decision",
            "promotion_review_audit_chain_receipt",
            "reviewed_flag_precondition_plan_packet",
            "reviewed_flag_precondition_plan_replay_consistency_decision",
            "reviewed_flag_readiness_closeout_receipt",
            "reviewed_flag_readiness_closeout_replay_consistency_decision",
            "reviewed_flag_audit_chain_closeout_receipt",
            "work_graph_surface_audit_packet",
            "work_graph_canonical_projection_receipt",
            "work_graph_canonical_projection_replay_consistency_decision",
            "work_graph_canonical_projection_closeout_receipt",
            "work_graph_canonical_projection_closeout_replay_consistency_decision",
            "work_graph_canonical_projection_audit_chain_closeout_receipt",
            "work_graph_canonical_projection_audit_chain_closeout_replay_consistency_decision",
            "work_graph_canonical_projection_enablement_operator_review_packet",
            "work_graph_canonical_projection_enablement_operator_review_replay_consistency_decision",
            "work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt",
            "work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_decision",
            "work_graph_canonical_projection_enablement_audit_chain_closeout_receipt",
            "work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency_decision",
            "work_graph_canonical_projection_enablement_activation_precondition_operator_packet",
            "work_graph_canonical_projection_enablement_activation_precondition_replay_consistency_decision",
            "work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt",
            "work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_decision",
            "work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt",
            "work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_decision",
            "work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet",
            "work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision",
            "work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet",
            "work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_decision",
            "operatorMatrixRows",
            "taskResultContractId",
            "missingTaskResultContractParts",
        ],
        job_id,
        status: job.status.as_str().to_string(),
        output_csv_path: job.output_csv_path,
        total_items: progress.total_items,
        completed_items: progress.completed_items,
        failed_items: progress.failed_items,
        job_error,
        failed_item_errors,
        admission_shadow_decision,
        promotion_readiness_shadow_matrix,
        operator_review_promotion_packet,
        promotion_review_replay_consistency_decision,
        promotion_closeout_receipt,
        promotion_closeout_replay_consistency_decision,
        promotion_review_audit_chain_receipt,
        reviewed_flag_precondition_plan_packet,
        reviewed_flag_precondition_plan_replay_consistency_decision,
        reviewed_flag_readiness_closeout_receipt,
        reviewed_flag_readiness_closeout_replay_consistency_decision,
        reviewed_flag_audit_chain_closeout_receipt,
        work_graph_surface_audit_packet: work_graph_surface_audit_packet_summary,
        work_graph_canonical_projection_receipt,
        work_graph_canonical_projection_replay_consistency_decision,
        work_graph_canonical_projection_closeout_receipt,
        work_graph_canonical_projection_closeout_replay_consistency_decision,
        work_graph_canonical_projection_audit_chain_closeout_receipt,
        work_graph_canonical_projection_audit_chain_closeout_replay_consistency_decision,
        work_graph_canonical_projection_enablement_operator_review_packet,
        work_graph_canonical_projection_enablement_operator_review_replay_consistency_decision,
        work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt,
        work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_decision,
        work_graph_canonical_projection_enablement_audit_chain_closeout_receipt,
        work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency_decision,
        work_graph_canonical_projection_enablement_activation_precondition_operator_packet,
        work_graph_canonical_projection_enablement_activation_precondition_replay_consistency_decision,
        work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt,
        work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_decision,
        work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt,
        work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_decision,
        work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet,
        work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision,
        work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet,
        work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_decision,
    })
    .map_err(|err| {
        FunctionCallError::Fatal(format!(
            "failed to serialize spawn_agents_on_csv result: {err}"
        ))
    })?;
    Ok(FunctionToolOutput::from_text(content, Some(true)))
}

fn single_local_environment_cwd(turn: &TurnContext) -> Result<AbsolutePathBuf, FunctionCallError> {
    let [turn_environment] = turn.environments.turn_environments.as_slice() else {
        return Err(FunctionCallError::RespondToModel(
            "spawn_agents_on_csv requires exactly one local environment".to_string(),
        ));
    };

    if turn_environment.environment.is_remote() {
        return Err(FunctionCallError::RespondToModel(
            "spawn_agents_on_csv is not supported for remote environments".to_string(),
        ));
    }

    turn_environment.native_cwd().map_err(|error| {
        FunctionCallError::RespondToModel(format!(
            "spawn_agents_on_csv requires a host-native local cwd: {error}"
        ))
    })
}
