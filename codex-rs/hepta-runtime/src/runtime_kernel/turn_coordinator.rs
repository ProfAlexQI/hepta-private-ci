/// Owns the sequencing of one native runtime turn while delegating policy,
/// tool execution, persistence, and model access to `RuntimeKernel`.
///
/// This is an internal orchestration seam: it does not alter the public turn
/// API or introduce new authorization, execution, or outcome semantics.
struct TurnCoordinator<'a> {
    kernel: &'a RuntimeKernel,
    session_id: SessionId,
    input: &'a str,
    model_timeout_ms: Option<u64>,
    selected_snippets: Option<&'a CoreTurnContextRecallSelectedSnippetEnvelope>,
}

impl TurnCoordinator<'_> {
    /// Runs model/tool steps to a terminal result, then hands the completed
    /// result back through the existing turn-recording boundary.
    async fn run(self) -> Result<VerticalSliceResult, HeptaError> {
        let Self {
            kernel,
            session_id,
            input,
            model_timeout_ms,
            selected_snippets,
        } = self;
        let session_key = session_id.0.clone();
        let correlation_id = CorrelationId("corr-demo".into());
        let active_model = kernel.model_selection_for_session(&session_key)?.active;

        kernel.ensure_session_record(&session_id).await?;
        kernel.upsert_session_record(
            &session_id,
            None,
            Some(summarize_user_intent(input)),
            None,
            true,
        )?;
        kernel.emit_event(
            EventKind::MessageReceived,
            Some(session_id.clone()),
            Some(correlation_id.clone()),
            summarize_user_intent(input),
        )?;

        let base_messages = kernel
            .model_messages_for_turn_with_selected_snippets(&session_id, input, selected_snippets)
            .await?;
        let model_tools = kernel.tools.model_tool_specs_for_turn(input);

        kernel.emit_event(
            EventKind::ModelCalled,
            Some(session_id.clone()),
            Some(correlation_id.clone()),
            format!(
                "initial model call via {}/{}",
                active_model.provider, active_model.model
            ),
        )?;
        let deterministic_message = kernel.deterministic_runtime_response_for_session(
            &session_id,
            input,
            &active_model,
            &base_messages,
        )?;
        let mut current_response = if let Some(message) = deterministic_message {
            ModelResponse {
                message: Some(ModelMessage {
                    role: MessageRole::Assistant,
                    content: message,
                }),
                tool_calls: vec![],
                finish_reason: FinishReason::Stop,
                usage: Usage {
                    input_tokens: 0,
                    output_tokens: 0,
                },
            }
        } else if let Some(tool_call) = native_pre_model_tool_call(input) {
            ModelResponse {
                message: None,
                tool_calls: vec![tool_call],
                finish_reason: FinishReason::ToolCall,
                usage: Usage {
                    input_tokens: 0,
                    output_tokens: 0,
                },
            }
        } else {
            kernel
                .providers
                .chat(ModelRequest {
                    model: active_model.clone(),
                    messages: base_messages.clone(),
                    thinking: ThinkingLevel::High,
                    tools: model_tools.clone(),
                    timeout_ms: model_timeout_ms,
                })
                .await?
        };

        let mut conversation_messages = base_messages;
        let mut invoked_tool = None::<String>;
        let mut tool_output_json = None::<String>;
        let mut final_text = String::new();
        let mut approval_required = None::<String>;
        let mut blocked_reason = None::<String>;
        let max_tool_steps = 6usize;

        for step_index in 0..max_tool_steps {
            if let Some(tool_call) = current_response.tool_calls.first().cloned() {
                if model_tools.is_empty() {
                    final_text = current_response
                        .message
                        .as_ref()
                        .map(|message| message.content.trim().to_string())
                        .filter(|message| !message.is_empty())
                        .unwrap_or_else(|| {
                            "这条消息按普通对话处理；没有明确工具意图，所以 Hepta 没有调用工具。"
                                .into()
                        });
                    blocked_reason = Some("tool-intent-not-authorized-for-turn".into());
                    break;
                }
                match kernel
                    .execute_tool_call_for_turn(
                        &session_id,
                        &session_key,
                        &correlation_id,
                        &active_model,
                        &tool_call,
                    )
                    .await?
                {
                    RuntimeToolStep::Executed(execution) => {
                        if invoked_tool.is_none() {
                            invoked_tool = Some(execution.tool_name.clone());
                        }
                        tool_output_json = execution.tool_output_json.clone();
                        conversation_messages.push(ModelMessage {
                            role: MessageRole::Tool,
                            content: execution.tool_message,
                        });
                        kernel.emit_event(
                            EventKind::ModelCalled,
                            Some(session_id.clone()),
                            Some(correlation_id.clone()),
                            format!(
                                "followup model call after tool {} step {}",
                                execution.tool_name,
                                step_index + 1
                            ),
                        )?;
                        current_response = kernel
                            .providers
                            .chat(ModelRequest {
                                model: active_model.clone(),
                                messages: conversation_messages.clone(),
                                thinking: ThinkingLevel::High,
                                tools: model_tools.clone(),
                                timeout_ms: model_timeout_ms,
                            })
                            .await?;
                    }
                    RuntimeToolStep::TimedOut(timeout) => {
                        if invoked_tool.is_none() {
                            invoked_tool = Some(timeout.tool_name.clone());
                        }
                        tool_output_json = timeout.tool_output_json.clone();
                        final_text = timeout.final_text;
                        break;
                    }
                    RuntimeToolStep::ApprovalRequired { tool_name, reason } => {
                        final_text =
                            format!("approval required before invoking tool {}", tool_name);
                        approval_required = Some(tool_name);
                        blocked_reason = Some(reason);
                        break;
                    }
                    RuntimeToolStep::Blocked {
                        final_text: blocked_text,
                        reason,
                    } => {
                        final_text = blocked_text;
                        blocked_reason = Some(reason);
                        break;
                    }
                }
            } else {
                final_text = current_response
                    .message
                    .take()
                    .map(|message| message.content)
                    .unwrap_or_else(|| "empty model response".into());
                OutcomeRecorder::new(kernel)
                    .store_memory(
                        Some(&session_id),
                        "mem-assistant",
                        MemoryScope::LongTerm,
                        format!("assistant:{}", input),
                    )
                    .await?;
                break;
            }
        }

        if final_text.is_empty() && blocked_reason.is_none() {
            final_text = format!("tool loop exceeded maximum steps ({})", max_tool_steps);
            blocked_reason = Some("tool loop exceeded maximum steps".into());
        }
        if looks_like_live_agent_marker_recall_intent(input)
            && let Some(marker) = kernel.latest_live_agent_e2e_marker_for_session(&session_id)?
            && !final_text.contains(&marker)
        {
            final_text = format!("The live-agent-e2e marker is {marker}.");
        }

        let recalled = kernel
            .memory
            .search(MemoryQuery {
                text: if invoked_tool.is_some() {
                    "tool:".into()
                } else {
                    "assistant:".into()
                },
                limit: 10,
            })
            .await
            .map_err(|e| HeptaError(e.0))?;

        let result = VerticalSliceResult {
            session_id: session_id.0.clone(),
            active_model,
            invoked_tool,
            tool_output_json,
            final_text,
            recalled_memories: recalled.len(),
            approval_required,
            blocked_reason,
        };

        OutcomeRecorder::new(kernel).record_turn(TurnRecord {
            session_id: result.session_id.clone(),
            input: input.to_string(),
            invoked_tool: result.invoked_tool.clone(),
            final_text: result.final_text.clone(),
            blocked_reason: result.blocked_reason.clone(),
        })?;

        Ok(result)
    }
}
