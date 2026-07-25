impl RuntimeKernel {
    pub async fn native_turn_messages_with_context_recall_handoff(
        &self,
        session_id: &str,
        input: &str,
        experimental_api_enabled: bool,
    ) -> Result<RuntimeNativeTurnContextRecallHandoff, HeptaError> {
        let session_id = session_id.trim();
        if session_id.is_empty() {
            return Err(HeptaError("session id must not be empty".into()));
        }
        let session_id = SessionId(session_id.to_string());
        let turn_handoff = self.context_recall_turn_handoff(
            &session_id.0,
            Some(input),
            /*recent_window_limit*/ 4,
            /*transcript_limit*/ 4,
            /*memory_limit*/ 4,
            /*allow_cross_session*/ true,
            experimental_api_enabled,
        )?;
        let selected_snippet_count =
            native_selected_snippet_prompt_count(turn_handoff.selected_snippets.as_ref());
        let messages = self
            .model_messages_for_turn_with_selected_snippets(
                &session_id,
                input,
                turn_handoff.selected_snippets.as_ref(),
            )
            .await?;

        Ok(RuntimeNativeTurnContextRecallHandoff {
            provider_rollup: turn_handoff.provider_rollup,
            selected_snippets_present: selected_snippet_count > 0,
            selected_snippet_count,
            messages,
        })
    }

    pub async fn run_demo_turn_in_session_with_context_recall_handoff(
        &self,
        session_id: &str,
        input: &str,
        experimental_api_enabled: bool,
    ) -> Result<RuntimeNativeTurnContextRecallRun, HeptaError> {
        let session_id = session_id.trim();
        if session_id.is_empty() {
            return Err(HeptaError("session id must not be empty".into()));
        }
        let session_id = SessionId(session_id.to_string());
        let turn_handoff = self.context_recall_turn_handoff(
            &session_id.0,
            Some(input),
            /*recent_window_limit*/ 4,
            /*transcript_limit*/ 4,
            /*memory_limit*/ 4,
            /*allow_cross_session*/ true,
            experimental_api_enabled,
        )?;
        let selected_snippet_count =
            native_selected_snippet_prompt_count(turn_handoff.selected_snippets.as_ref());
        let result = self
            .run_demo_turn_for_session_impl_with_selected_snippets(
                session_id,
                input,
                /*model_timeout_ms*/ None,
                turn_handoff.selected_snippets.as_ref(),
            )
            .await?;

        Ok(RuntimeNativeTurnContextRecallRun {
            provider_rollup: turn_handoff.provider_rollup,
            selected_snippets_present: selected_snippet_count > 0,
            selected_snippet_count,
            result,
        })
    }

    async fn model_messages_for_turn_with_selected_snippets(
        &self,
        session_id: &SessionId,
        input: &str,
        selected_snippets: Option<&CoreTurnContextRecallSelectedSnippetEnvelope>,
    ) -> Result<Vec<ModelMessage>, HeptaError> {
        let mut context_sections = vec![
            "You are Hepta's native runtime agent. Use the supplied session context and available tools when useful. If the user asks what was remembered earlier, answer from the session context instead of giving a generic definition. Answer directly; do not reveal hidden reasoning, planning notes, or internal analysis.".to_string(),
        ];

        let transcript_entries = self
            .memory
            .list_transcript_entries()
            .map_err(|err| HeptaError(err.0))?;
        let mut recent = transcript_entries
            .into_iter()
            .filter(|entry| {
                entry.session_id == *session_id
                    && matches!(
                        entry.role,
                        Some(MessageRole::User) | Some(MessageRole::Assistant)
                    )
            })
            .collect::<Vec<_>>();
        recent.sort_by_key(|entry| entry.sequence);
        if !recent.is_empty() {
            let transcript = recent
                .into_iter()
                .rev()
                .take(12)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .map(|entry| {
                    let role = match entry.role {
                        Some(MessageRole::User) => "User",
                        Some(MessageRole::Assistant) => "Assistant",
                        _ => "Context",
                    };
                    format!("{}: {}", role, truncate_for_context(&entry.content, 900))
                })
                .collect::<Vec<_>>()
                .join("\n");
            context_sections.push(format!("Recent session transcript:\n{}", transcript));
        }

        let keyword = memory_context_keyword(input);
        if !keyword.is_empty() {
            let hits = self
                .memory
                .search(MemoryQuery {
                    text: keyword,
                    limit: 6,
                })
                .await
                .map_err(|err| HeptaError(err.0))?;
            if !hits.is_empty() {
                let memories = hits
                    .into_iter()
                    .map(|record| format!("- {}", truncate_for_context(&record.content, 700)))
                    .collect::<Vec<_>>()
                    .join("\n");
                context_sections.push(format!("Relevant memory records:\n{}", memories));
            }
        }
        if let Some(selected_context_recall) =
            selected_snippets.and_then(format_selected_context_recall_for_native_turn)
        {
            context_sections.push(selected_context_recall);
        }

        Ok(vec![
            ModelMessage {
                role: MessageRole::System,
                content: context_sections.join("\n\n"),
            },
            ModelMessage {
                role: MessageRole::User,
                content: input.into(),
            },
        ])
    }

    fn tool_scope_block(
        &self,
        session_id: &SessionId,
        tool_name: &str,
        arguments_json: &str,
    ) -> Option<RuntimeToolStep> {
        if let Err(err) = self.ensure_execution_profile_allows_tool(session_id, tool_name) {
            return Some(RuntimeToolStep::Blocked {
                final_text: format!("execution profile blocked tool {tool_name}"),
                reason: err.0,
            });
        }
        if let Err(err) =
            self.ensure_filesystem_scope_allows_tool_input(session_id, tool_name, arguments_json)
        {
            return Some(RuntimeToolStep::Blocked {
                final_text: format!("filesystem scope blocked tool {tool_name}"),
                reason: err.0,
            });
        }
        if let Err(err) =
            self.ensure_write_path_scope_allows_tool_input(session_id, tool_name, arguments_json)
        {
            return Some(RuntimeToolStep::Blocked {
                final_text: format!("write path scope blocked tool {tool_name}"),
                reason: err.0,
            });
        }
        if let Err(err) = self.ensure_destructive_write_semantics(tool_name, arguments_json) {
            return Some(RuntimeToolStep::Blocked {
                final_text: format!("write semantics blocked tool {tool_name}"),
                reason: err.0,
            });
        }
        None
    }

    fn remember_exact_pending_for_turn(
        &self,
        session_id: &SessionId,
        session_key: &str,
        correlation_id: &CorrelationId,
        material: ExactApprovalMaterial,
    ) -> Result<RuntimeToolStep, HeptaError> {
        let tool_name = material.tool_name.clone();
        let reason = material.reason.clone();
        let binding = short_hash(material.binding_hash().as_str()).to_string();
        let mut approvals = self
            .approval_state
            .lock()
            .map_err(|_| HeptaError("approval state mutex poisoned".into()))?;
        approvals.remember_pending_exact(session_key, material);
        drop(approvals);
        self.emit_event(
            EventKind::ApprovalRequested,
            Some(session_id.clone()),
            Some(correlation_id.clone()),
            format!("tool {tool_name} candidate {binding} requires exact approval: {reason}"),
        )?;
        Ok(RuntimeToolStep::ApprovalRequired { tool_name, reason })
    }

    async fn execute_tool_call_for_turn(
        &self,
        session_id: &SessionId,
        session_key: &str,
        correlation_id: &CorrelationId,
        active_model: &ModelRef,
        tool_call: &ToolCall,
    ) -> Result<RuntimeToolStep, HeptaError> {
        if let Err(err) = self.validate_tool_input(&tool_call.name, &tool_call.arguments_json) {
            return Ok(RuntimeToolStep::Blocked {
                final_text: format!("tool input validation failed for {}", tool_call.name),
                reason: err.0,
            });
        }

        let risk = self.tools.risk_tier(&tool_call.name)?;
        let decision = self
            .policy
            .evaluate_tool(PolicyEvaluationContext {
                session_id: Some(session_id.clone()),
                model: Some(active_model.clone()),
                tool_name: tool_call.name.clone(),
                risk_tier: risk,
            })
            .await
            .map_err(|err| HeptaError(err.0))?;

        if decision.requirement == ApprovalRequirement::Deny {
            return Ok(RuntimeToolStep::Blocked {
                final_text: format!("policy denied tool {}", tool_call.name),
                reason: decision.reason,
            });
        }

        let prepared = SafetyGateClient::prepare_candidate(
            self,
            session_key,
            active_model,
            &tool_call.name,
            &tool_call.arguments_json,
            &decision,
        )?;
        let (approved, grant_required) = match decision.requirement {
            ApprovalRequirement::Ask => {
                let candidate_approval = self
                    .approval_state
                    .lock()
                    .map_err(|_| HeptaError("approval state mutex poisoned".into()))?
                    .candidate_approval(session_key, &prepared);
                match candidate_approval {
                    CandidateApproval::Exact(approved) => (*approved, true),
                    CandidateApproval::Missing => {
                        return self.remember_exact_pending_for_turn(
                            session_id,
                            session_key,
                            correlation_id,
                            prepared,
                        );
                    }
                }
            }
            ApprovalRequirement::None => (prepared, false),
            ApprovalRequirement::Deny => {
                return Ok(RuntimeToolStep::Blocked {
                    final_text: format!("policy denied tool {}", tool_call.name),
                    reason: decision.reason,
                });
            }
        };

        if let Some(blocked) =
            self.tool_scope_block(session_id, &tool_call.name, &approved.canonical_arguments)
        {
            return Ok(blocked);
        }

        let execution_epoch = match self.capture_execution_epoch(session_key) {
            Ok(epoch) => epoch,
            Err(err) => {
                return Ok(RuntimeToolStep::Blocked {
                    final_text: format!(
                        "commit-time safety coordination blocked tool {}",
                        tool_call.name
                    ),
                    reason: err.0,
                });
            }
        };
        let current_decision = self
            .policy
            .evaluate_tool(PolicyEvaluationContext {
                session_id: Some(session_id.clone()),
                model: Some(active_model.clone()),
                tool_name: tool_call.name.clone(),
                risk_tier: risk,
            })
            .await
            .map_err(|err| HeptaError(err.0))?;
        if current_decision.requirement == ApprovalRequirement::Deny {
            return Ok(RuntimeToolStep::Blocked {
                final_text: format!("policy denied tool {}", tool_call.name),
                reason: current_decision.reason,
            });
        }

        if let Some(blocked) =
            self.tool_scope_block(session_id, &tool_call.name, &tool_call.arguments_json)
        {
            return Ok(blocked);
        }
        let presented = SafetyGateClient::prepare_candidate(
            self,
            session_key,
            active_model,
            &tool_call.name,
            &tool_call.arguments_json,
            &current_decision,
        )?;
        if current_decision.requirement == ApprovalRequirement::Ask && !grant_required {
            return self.remember_exact_pending_for_turn(
                session_id,
                session_key,
                correlation_id,
                presented,
            );
        }

        let execution_lease = match self.begin_execution_lease(execution_epoch) {
            Ok(lease) => lease,
            Err(err) => {
                return Ok(RuntimeToolStep::Blocked {
                    final_text: format!(
                        "commit-time safety coordination blocked tool {}",
                        tool_call.name
                    ),
                    reason: err.0,
                });
            }
        };
        let execution_lease = match execution_lease.bind_tool_resources(
            self,
            &session_id.0,
            &tool_call.name,
            &presented.canonical_arguments,
        ) {
            Ok(lease) => lease,
            Err(err) => {
                return Ok(RuntimeToolStep::Blocked {
                    final_text: format!("resource binding blocked tool {}", tool_call.name),
                    reason: err.0,
                });
            }
        };
        let authorized_execution = if grant_required {
            let mut approvals = self
                .approval_state
                .lock()
                .map_err(|_| HeptaError("approval state mutex poisoned".into()))?;
            SafetyGateClient::authorize_execution_and_consume(
                self,
                &mut approvals,
                session_key,
                session_id,
                correlation_id,
                &approved,
                &presented,
                execution_lease,
            )
        } else {
            SafetyGateClient::authorize_execution_without_grant(
                self,
                session_id,
                correlation_id,
                &approved,
                &presented,
                execution_lease,
            )
        };
        let authorized_execution = match authorized_execution {
            Ok(authorized_execution) => authorized_execution,
            Err(err) => {
                return Ok(RuntimeToolStep::Blocked {
                    final_text: format!("commit-time safety gate blocked tool {}", tool_call.name),
                    reason: err.0,
                });
            }
        };

        let mut captured = ExecutionBus::new(self).dispatch(authorized_execution).await;
        captured.capture_write_transaction();

        let (attempt_id, durable_intent_recorded, effect_plan_recorded, provider_effect_ack_hash) =
            captured
                .execution()
                .map(|execution| {
                    (
                        execution.attempt_id().to_string(),
                        execution.execution_intent().is_some(),
                        execution
                            .execution_intent()
                            .and_then(hepta_memory::ExecutionIntent::effect_plan_hash)
                            .is_some(),
                        execution
                            .execution_effect_ack()
                            .map(hepta_memory::ExecutionEffectAck::ack_hash)
                            .map(ToString::to_string),
                    )
                })
                .ok_or_else(|| {
                    HeptaError("tool dispatch lost execution receipt authority".into())
                })?;
        OutcomeRecorder::new(self).finalize_tool_dispatch(&mut captured)?;
        let terminal = self
            .outcome_sink
            .read_by_attempt(&attempt_id)
            .map_err(|error| HeptaError(format!("terminal receipt readback failed: {error}")))?
            .ok_or_else(|| {
                HeptaError(format!(
                    "terminal receipt readback missing for attempt {attempt_id}"
                ))
            })?;
        let terminal_status = match terminal.receipt().status() {
            OutcomeStatus::Succeeded => "succeeded".to_string(),
            OutcomeStatus::Failed { error_code } => {
                format!("failed:{error_code}")
            }
            OutcomeStatus::Cancelled { reason_code } => {
                format!("cancelled:{reason_code}")
            }
            _ => "unknown".to_string(),
        };
        let execution_receipt = RuntimeExecutionReceipt {
            attempt_id,
            durable_intent_recorded,
            effect_plan_recorded,
            provider_effect_ack_hash,
            terminal_receipt_id: terminal.receipt().id().to_string(),
            terminal_receipt_hash: terminal.receipt().receipt_hash().to_string(),
            terminal_outcome_hash: terminal.receipt().outcome_hash().to_string(),
            terminal_evidence_hash: terminal.canonical_evidence_hash().to_string(),
            terminal_status,
        };
        let outward_error = captured.outward_error().cloned();
        let tool_result = captured.tool_result().cloned();
        if let Some(error) = outward_error {
            return Err(error);
        }
        let tool_result = tool_result.ok_or_else(|| {
            HeptaError(format!(
                "tool {} completed without a result",
                tool_call.name
            ))
        })?;
        let tool_output_json = tool_result.structured_json.clone();

        OutcomeRecorder::new(self)
            .store_memory(
                Some(session_id),
                "mem-tool",
                MemoryScope::LongTerm,
                format_tool_memory_content(&tool_result),
            )
            .await?;

        if tool_result_is_timeout(&tool_call.name, &tool_result) {
            return Ok(RuntimeToolStep::TimedOut(RuntimeToolTimeout {
                tool_name: tool_call.name.clone(),
                tool_output_json,
                execution_receipt,
                final_text: tool_result.content.clone(),
            }));
        }
        Ok(RuntimeToolStep::Executed(RuntimeToolExecution {
            tool_name: tool_call.name.clone(),
            tool_output_json,
            execution_receipt,
            tool_message: format_tool_message(&tool_result),
        }))
    }

    pub async fn run_demo_turn(&self, input: &str) -> Result<VerticalSliceResult, HeptaError> {
        let session_id = SessionId(self.active_session_id()?);
        self.run_demo_turn_for_session(session_id, input).await
    }

    pub async fn run_demo_turn_in_session(
        &self,
        session_id: &str,
        input: &str,
    ) -> Result<VerticalSliceResult, HeptaError> {
        self.run_demo_turn_in_session_with_model_timeout(session_id, input, None)
            .await
    }

    pub async fn run_demo_turn_in_session_with_model_timeout(
        &self,
        session_id: &str,
        input: &str,
        model_timeout_ms: Option<u64>,
    ) -> Result<VerticalSliceResult, HeptaError> {
        let session_id = session_id.trim();
        if session_id.is_empty() {
            return Err(HeptaError("session id must not be empty".into()));
        }
        self.run_demo_turn_for_session_impl(
            SessionId(session_id.to_string()),
            input,
            model_timeout_ms,
        )
        .await
    }

    async fn run_demo_turn_for_session(
        &self,
        session_id: SessionId,
        input: &str,
    ) -> Result<VerticalSliceResult, HeptaError> {
        self.run_demo_turn_for_session_impl(session_id, input, None)
            .await
    }

    async fn run_demo_turn_for_session_impl(
        &self,
        session_id: SessionId,
        input: &str,
        model_timeout_ms: Option<u64>,
    ) -> Result<VerticalSliceResult, HeptaError> {
        self.run_demo_turn_for_session_impl_with_selected_snippets(
            session_id,
            input,
            model_timeout_ms,
            None,
        )
        .await
    }

    async fn run_demo_turn_for_session_impl_with_selected_snippets(
        &self,
        session_id: SessionId,
        input: &str,
        model_timeout_ms: Option<u64>,
        selected_snippets: Option<&CoreTurnContextRecallSelectedSnippetEnvelope>,
    ) -> Result<VerticalSliceResult, HeptaError> {
        TurnCoordinator {
            kernel: self,
            session_id,
            input,
            model_timeout_ms,
            selected_snippets,
        }
        .run()
        .await
    }

    fn deterministic_runtime_response_for_session(
        &self,
        session_id: &SessionId,
        input: &str,
        active_model: &ModelRef,
        messages: &[ModelMessage],
    ) -> Result<Option<String>, HeptaError> {
        if looks_like_live_agent_marker_recall_intent(input) {
            if let Some(marker) = self.latest_live_agent_e2e_marker_for_session(session_id)? {
                return Ok(Some(format!("The live-agent-e2e marker is {marker}.")));
            }
        }
        Ok(deterministic_runtime_response(
            input,
            active_model,
            messages,
        ))
    }

    fn latest_live_agent_e2e_marker_for_session(
        &self,
        session_id: &SessionId,
    ) -> Result<Option<String>, HeptaError> {
        let recent = self.recent_session_window(&session_id.0, 16)?;
        let remembered_by_user = recent
            .iter()
            .rev()
            .filter(|entry| matches!(entry.role, Some(MessageRole::User)))
            .filter(|entry| looks_like_live_agent_marker_remember_intent(&entry.content))
            .find_map(|entry| extract_live_agent_e2e_marker(&entry.content));
        if remembered_by_user.is_some() {
            return Ok(remembered_by_user);
        }
        Ok(recent
            .iter()
            .rev()
            .filter(|entry| matches!(entry.role, Some(MessageRole::User)))
            .find_map(|entry| extract_live_agent_e2e_marker(&entry.content)))
    }

    fn ensure_execution_profile_allows_tool(
        &self,
        session_id: &SessionId,
        tool_name: &str,
    ) -> Result<(), HeptaError> {
        let profile = self.execution_profile_for_session(&session_id.0)?;
        let metadata = self.tools.execution_metadata(tool_name)?;
        match profile {
            ExecutionProfile::FullAccess => Ok(()),
            ExecutionProfile::ReadOnlyTools => {
                if metadata.read_only && !metadata.destructive {
                    Ok(())
                } else {
                    Err(HeptaError(format!(
                        "execution profile {} blocks non-read-only tool {}",
                        format_execution_profile(profile),
                        tool_name
                    )))
                }
            }
            ExecutionProfile::NoTools => Err(HeptaError(format!(
                "execution profile {} blocks tool {}",
                format_execution_profile(profile),
                tool_name
            ))),
        }
    }

    fn ensure_filesystem_scope_allows_tool_input(
        &self,
        session_id: &SessionId,
        tool_name: &str,
        input_json: &str,
    ) -> Result<(), HeptaError> {
        let write_argument_name = write_path_argument_name_for_tool(tool_name);
        let Some(argument_name) = path_argument_name_for_tool(tool_name).or(write_argument_name)
        else {
            return Ok(());
        };

        let scope = self
            .path_capability_gates_for_session(&session_id.0)?
            .into_iter()
            .find(|gate| gate.tool_name == tool_name && gate.argument_name == argument_name)
            .map(|gate| gate.scope)
            .unwrap_or(self.filesystem_scope_for_session(&session_id.0)?);
        match scope {
            FilesystemScope::AnyPath => Ok(()),
            FilesystemScope::WorkspaceOnly => {
                let requested_path = parse_required_string_field(input_json, argument_name)
                    .map_err(|err| HeptaError(err.0))?;
                let workspace_root = self.workspace_root()?;
                let resolved = if write_argument_name.is_some() {
                    resolve_write_path_within_root(&workspace_root, Path::new(&requested_path))?
                        .canonical_path
                } else {
                    resolve_path_within_root(&workspace_root, Path::new(&requested_path))
                };
                if resolved.starts_with(&workspace_root) {
                    Ok(())
                } else {
                    Err(HeptaError(format!(
                        "filesystem scope {} blocks {} {} {} outside workspace {}",
                        format_filesystem_scope(scope),
                        tool_name,
                        argument_name,
                        requested_path,
                        workspace_root.display()
                    )))
                }
            }
        }
    }

    fn ensure_write_path_scope_allows_tool_input(
        &self,
        session_id: &SessionId,
        tool_name: &str,
        input_json: &str,
    ) -> Result<(), HeptaError> {
        let Some(argument_name) = write_path_argument_name_for_tool(tool_name) else {
            return Ok(());
        };

        let requested_path = parse_required_string_field(input_json, argument_name)
            .map_err(|err| HeptaError(err.0))?;
        self.ensure_write_path_scope_allows_path_string(session_id, tool_name, &requested_path)
    }

    fn ensure_write_path_scope_allows_path_string(
        &self,
        session_id: &SessionId,
        tool_name: &str,
        requested_path: &str,
    ) -> Result<(), HeptaError> {
        let workspace_root = self.workspace_root()?;
        let artifacts_root = workspace_root.join("artifacts");
        let resolved = resolve_write_path_within_root(&workspace_root, Path::new(&requested_path))?
            .canonical_path;
        let scope = self.write_path_scope_for_session(&session_id.0)?;

        match scope {
            WritePathScope::AnyPath => Ok(()),
            WritePathScope::WorkspaceOnly => {
                if resolved.starts_with(&workspace_root) {
                    Ok(())
                } else {
                    Err(HeptaError(format!(
                        "write path scope {} blocks {} path {} outside workspace {}",
                        format_write_path_scope(scope),
                        tool_name,
                        requested_path,
                        workspace_root.display()
                    )))
                }
            }
            WritePathScope::ArtifactsOnly => {
                if resolved.starts_with(&artifacts_root) {
                    Ok(())
                } else {
                    Err(HeptaError(format!(
                        "write path scope {} blocks {} path {} outside artifacts root {}",
                        format_write_path_scope(scope),
                        tool_name,
                        requested_path,
                        artifacts_root.display()
                    )))
                }
            }
        }
    }

    /// Checks both independent path authorities against the exact canonical
    /// target that will be sealed into the execution reservation.
    fn ensure_resolved_write_path_scopes_allow_for_arguments(
        &self,
        session_id: &SessionId,
        tool_name: &str,
        argument_names: &[&str],
        requested_path: &str,
        resolved: &Path,
    ) -> Result<(), HeptaError> {
        let workspace_root = self.workspace_root()?;
        let default_filesystem_scope = self.filesystem_scope_for_session(&session_id.0)?;
        let gates = self.path_capability_gates_for_session(&session_id.0)?;
        let filesystem_scope = argument_names
            .iter()
            .map(|argument_name| {
                gates
                    .iter()
                    .find(|gate| {
                        gate.tool_name == tool_name && gate.argument_name == *argument_name
                    })
                    .map(|gate| gate.scope)
                    .unwrap_or(default_filesystem_scope)
            })
            .find(|scope| *scope == FilesystemScope::WorkspaceOnly)
            .unwrap_or(default_filesystem_scope);
        if filesystem_scope == FilesystemScope::WorkspaceOnly
            && !resolved.starts_with(&workspace_root)
        {
            return Err(HeptaError(format!(
                "filesystem scope {} blocks {} path {} outside workspace {}",
                format_filesystem_scope(filesystem_scope),
                tool_name,
                requested_path,
                workspace_root.display()
            )));
        }

        let write_scope = self.write_path_scope_for_session(&session_id.0)?;
        let allowed_root = match write_scope {
            WritePathScope::AnyPath => return Ok(()),
            WritePathScope::WorkspaceOnly => workspace_root,
            WritePathScope::ArtifactsOnly => workspace_root.join("artifacts"),
        };
        if resolved.starts_with(&allowed_root) {
            Ok(())
        } else {
            Err(HeptaError(format!(
                "write path scope {} blocks {} path {} outside {} root {}",
                format_write_path_scope(write_scope),
                tool_name,
                requested_path,
                if write_scope == WritePathScope::ArtifactsOnly {
                    "artifacts"
                } else {
                    "workspace"
                },
                allowed_root.display()
            )))
        }
    }

    fn ensure_destructive_write_semantics(
        &self,
        tool_name: &str,
        input_json: &str,
    ) -> Result<(), HeptaError> {
        if tool_name != "write_file" {
            return Ok(());
        }

        let requested_path =
            parse_required_string_field(input_json, "path").map_err(|err| HeptaError(err.0))?;
        let mode = parse_optional_string_field(input_json, "mode")
            .map_err(|err| HeptaError(err.0))?
            .unwrap_or_else(|| "create".to_string());
        let preview_only = parse_optional_bool_field(input_json, "preview_only")
            .map_err(|err| HeptaError(err.0))?
            .unwrap_or(false);
        let confirm_destructive = parse_optional_bool_field(input_json, "confirm_destructive")
            .map_err(|err| HeptaError(err.0))?
            .unwrap_or(false);
        let workspace_root = self.workspace_root()?;
        let resolved = resolve_write_path_within_root(&workspace_root, Path::new(&requested_path))?;
        let exists = fs::symlink_metadata(&resolved.canonical_path).is_ok();

        if preview_only {
            return match mode.as_str() {
                "create" | "overwrite" | "append" => Ok(()),
                other => Err(HeptaError(format!(
                    "write_file received unsupported mode {}",
                    other
                ))),
            };
        }

        match mode.as_str() {
            "create" => {
                if exists {
                    Err(HeptaError(format!(
                        "write_file refuses to overwrite existing path {} without mode=overwrite and confirm_destructive=true, or mode=append",
                        requested_path
                    )))
                } else {
                    Ok(())
                }
            }
            "overwrite" => {
                if exists && !confirm_destructive {
                    Err(HeptaError(format!(
                        "write_file overwrite for existing path {} requires confirm_destructive=true",
                        requested_path
                    )))
                } else {
                    Ok(())
                }
            }
            "append" => Ok(()),
            other => Err(HeptaError(format!(
                "write_file received unsupported mode {}",
                other
            ))),
        }
    }

    fn workspace_root(&self) -> Result<PathBuf, HeptaError> {
        let root = discover_workspace_root();
        fs::canonicalize(&root).map_err(|err| {
            HeptaError(format!(
                "failed to resolve workspace root {}: {}",
                root.display(),
                err
            ))
        })
    }

    async fn ensure_session_record(&self, session_id: &SessionId) -> Result<(), HeptaError> {
        self.upsert_session_record(session_id, None, None, None, true)
    }

    fn ensure_session_record_sync(&self, session_id: &str) -> Result<(), HeptaError> {
        self.upsert_session_record(&SessionId(session_id.to_string()), None, None, None, true)
    }

    fn upsert_session_record(
        &self,
        session_id: &SessionId,
        title_override: Option<String>,
        last_user_intent_summary: Option<String>,
        archived_at_unix_ms: Option<Option<u64>>,
        touch_last_active: bool,
    ) -> Result<(), HeptaError> {
        self.upsert_session_record_internal(
            session_id,
            title_override,
            last_user_intent_summary,
            archived_at_unix_ms,
            touch_last_active,
            None,
        )
    }

    fn upsert_session_record_with_agent(
        &self,
        session_id: &SessionId,
        title_override: Option<String>,
        last_user_intent_summary: Option<String>,
        archived_at_unix_ms: Option<Option<u64>>,
        touch_last_active: bool,
        agent_id_override: Option<AgentId>,
    ) -> Result<(), HeptaError> {
        self.upsert_session_record_internal(
            session_id,
            title_override,
            last_user_intent_summary,
            archived_at_unix_ms,
            touch_last_active,
            agent_id_override,
        )
    }

    fn upsert_session_record_internal(
        &self,
        session_id: &SessionId,
        title_override: Option<String>,
        last_user_intent_summary: Option<String>,
        archived_at_unix_ms: Option<Option<u64>>,
        touch_last_active: bool,
        agent_id_override: Option<AgentId>,
    ) -> Result<(), HeptaError> {
        let now = current_unix_ms()?;
        let existing = self
            .memory
            .list_sessions()
            .map_err(|err| HeptaError(err.0))?
            .into_iter()
            .find(|record| record.session_id == *session_id);

        let record = match existing {
            Some(record) => SessionRecord {
                session_id: record.session_id,
                agent_id: agent_id_override.unwrap_or(record.agent_id),
                title: title_override.unwrap_or(record.title),
                created_at_unix_ms: record.created_at_unix_ms,
                last_active_unix_ms: if touch_last_active {
                    now
                } else {
                    record.last_active_unix_ms
                },
                last_user_intent_summary: last_user_intent_summary
                    .or(record.last_user_intent_summary),
                archived_at_unix_ms: archived_at_unix_ms.unwrap_or(record.archived_at_unix_ms),
            },
            None => SessionRecord {
                session_id: session_id.clone(),
                agent_id: agent_id_override.unwrap_or_else(|| AgentId("main".into())),
                title: title_override.unwrap_or_else(|| format!("Hepta session {}", session_id.0)),
                created_at_unix_ms: now,
                last_active_unix_ms: now,
                last_user_intent_summary,
                archived_at_unix_ms: archived_at_unix_ms.unwrap_or(None),
            },
        };

        self.memory
            .upsert_session_sync(record)
            .map_err(|err| HeptaError(err.0))
    }

    fn set_session_model(&self, session_id: &str, model: ModelRef) -> Result<(), HeptaError> {
        let _mutation = self.begin_session_context_mutation(session_id)?;
        self.set_session_model_unleased(session_id, model)
    }

    fn set_session_model_unleased(
        &self,
        session_id: &str,
        model: ModelRef,
    ) -> Result<(), HeptaError> {
        let mut guard = self
            .model_state
            .lock()
            .map_err(|_| HeptaError("model state mutex poisoned".into()))?;
        if let Some(existing) = guard
            .sessions
            .iter_mut()
            .find(|item| item.session_id == session_id)
        {
            existing.selected_model = model;
        } else {
            guard.sessions.push(SessionModelState {
                session_id: session_id.to_string(),
                selected_model: model,
            });
        }
        Ok(())
    }

    fn session_export(&self, session_id: &str) -> Result<SessionExport, HeptaError> {
        let session_id = session_id.trim();
        let session = self
            .memory
            .list_sessions()
            .map_err(|err| HeptaError(err.0))?
            .into_iter()
            .find(|record| record.session_id.0 == session_id)
            .ok_or_else(|| HeptaError(format!("unknown session: {}", session_id)))?;
        let (topic_sessions, topic_graph_edges) =
            self.topic_export_state_for_session(session_id)?;

        Ok(SessionExport {
            version: 1,
            exported_at_unix_ms: current_unix_ms()?,
            model: self.model_selection_for_session(session_id)?.active,
            execution_profile: self.execution_profile_for_session(session_id)?,
            filesystem_scope: self.filesystem_scope_for_session(session_id)?,
            path_capability_gates: self.path_capability_gates_for_session(session_id)?,
            write_path_scope: self.write_path_scope_for_session(session_id)?,
            approval: self.approval_snapshot_for_session(session_id)?,
            history: self.history(Some(session_id), usize::MAX)?,
            write_transactions: self.write_transactions_for_session(session_id)?,
            write_transaction_groups: self.write_transaction_groups_for_session(session_id)?,
            active_write_transaction_group_id: self
                .active_write_transaction_group_id_for_session(session_id)?,
            rollback_group_attempts: self.rollback_group_attempts_for_session(session_id)?,
            write_target_locks: self.write_locks_for_session(session_id)?.0,
            write_group_locks: self.write_locks_for_session(session_id)?.1,
            topic_sessions,
            topic_graph_edges,
            neurons: self.neuron_export_state_for_session(session_id)?,
            intuition_feedback: self.intuition_feedback_for_session(session_id)?,
            model_router_feedback: self.model_router_feedback_for_session(session_id)?,
            session,
        })
    }

    fn topic_export_state_for_session(
        &self,
        session_id: &str,
    ) -> Result<(Vec<TopicSession>, Vec<RuntimeTopicGraphEdgeRecord>), HeptaError> {
        let topic_session_state = self
            .topic_session_state
            .lock()
            .map_err(|_| HeptaError("topic session state mutex poisoned".into()))?;
        let mut topic_sessions = topic_session_state
            .sessions
            .iter()
            .filter(|topic_session| {
                topic_session
                    .linked_surface_session_ids
                    .iter()
                    .any(|linked| linked.0 == session_id)
            })
            .cloned()
            .collect::<Vec<_>>();
        for topic_session in &mut topic_sessions {
            topic_session.graph_edges.clear();
        }

        let exported_topic_session_ids = topic_sessions
            .iter()
            .map(|topic_session| topic_session.topic_session_id.clone())
            .collect::<HashSet<_>>();
        let topic_graph_state = self
            .topic_graph_state
            .lock()
            .map_err(|_| HeptaError("topic graph state mutex poisoned".into()))?;
        let topic_graph_edges = topic_graph_state
            .edges
            .iter()
            .filter(|record| {
                exported_topic_session_ids.contains(&record.source_topic_session_id)
                    && exported_topic_session_ids.contains(&record.edge.target_topic_session_id)
            })
            .cloned()
            .collect::<Vec<_>>();

        Ok((topic_sessions, topic_graph_edges))
    }

    fn replace_topic_export_state_for_session(
        &self,
        session_id: &str,
        topic_sessions: Vec<TopicSession>,
        topic_graph_edges: Vec<RuntimeTopicGraphEdgeRecord>,
    ) -> Result<(), HeptaError> {
        let derived_topic_graph_edges = if topic_graph_edges.is_empty() {
            topic_sessions
                .iter()
                .flat_map(|topic_session| {
                    topic_session.graph_edges.iter().cloned().map(|edge| {
                        RuntimeTopicGraphEdgeRecord {
                            source_topic_session_id: topic_session.topic_session_id.clone(),
                            edge,
                        }
                    })
                })
                .collect::<Vec<_>>()
        } else {
            topic_graph_edges
        };

        let imported_topic_session_ids = topic_sessions
            .iter()
            .map(|topic_session| topic_session.topic_session_id.clone())
            .collect::<HashSet<_>>();
        let mut normalized_topic_sessions = topic_sessions;
        for topic_session in &mut normalized_topic_sessions {
            topic_session.graph_edges.clear();
        }
        let normalized_topic_graph_edges = derived_topic_graph_edges
            .into_iter()
            .filter(|record| {
                imported_topic_session_ids.contains(&record.source_topic_session_id)
                    && imported_topic_session_ids.contains(&record.edge.target_topic_session_id)
            })
            .collect::<Vec<_>>();

        let mut topic_session_state = self
            .topic_session_state
            .lock()
            .map_err(|_| HeptaError("topic session state mutex poisoned".into()))?;
        let existing_topic_session_ids = topic_session_state
            .sessions
            .iter()
            .filter(|topic_session| {
                topic_session
                    .linked_surface_session_ids
                    .iter()
                    .any(|linked| linked.0 == session_id)
            })
            .map(|topic_session| topic_session.topic_session_id.clone())
            .collect::<HashSet<_>>();
        topic_session_state.sessions.retain(|topic_session| {
            !topic_session
                .linked_surface_session_ids
                .iter()
                .any(|linked| linked.0 == session_id)
        });
        topic_session_state
            .sessions
            .extend(normalized_topic_sessions.into_iter());
        drop(topic_session_state);

        let mut topic_graph_state = self
            .topic_graph_state
            .lock()
            .map_err(|_| HeptaError("topic graph state mutex poisoned".into()))?;
        topic_graph_state.edges.retain(|record| {
            !existing_topic_session_ids.contains(&record.source_topic_session_id)
                && !existing_topic_session_ids.contains(&record.edge.target_topic_session_id)
        });
        topic_graph_state.edges.extend(normalized_topic_graph_edges);

        Ok(())
    }

    pub(crate) fn neuron_export_state_for_session(
        &self,
        session_id: &str,
    ) -> Result<Vec<RuntimeNeuronRecord>, HeptaError> {
        let neuron_state = self
            .neuron_state
            .lock()
            .map_err(|_| HeptaError("neuron state mutex poisoned".into()))?;
        Ok(neuron_state
            .neurons
            .iter()
            .filter(|record| record.session_id == session_id)
            .cloned()
            .collect())
    }

    pub(crate) fn stored_neurons_for_session(
        &self,
        session_id: &str,
    ) -> Result<Vec<HeptaNeuron>, HeptaError> {
        Ok(self
            .neuron_export_state_for_session(session_id)?
            .into_iter()
            .map(|record| record.neuron)
            .collect())
    }

    pub(crate) fn upsert_neurons_for_session(
        &self,
        session_id: &str,
        neurons: Vec<HeptaNeuron>,
    ) -> Result<(), HeptaError> {
        if neurons.is_empty() {
            return Ok(());
        }
        let mut neuron_state = self
            .neuron_state
            .lock()
            .map_err(|_| HeptaError("neuron state mutex poisoned".into()))?;
        for neuron in neurons {
            if let Some(existing) = neuron_state.neurons.iter_mut().find(|record| {
                record.session_id == session_id && record.neuron.neuron_id == neuron.neuron_id
            }) {
                existing.neuron = neuron;
            } else {
                neuron_state.neurons.push(RuntimeNeuronRecord {
                    session_id: session_id.to_string(),
                    neuron,
                });
            }
        }
        Ok(())
    }

    pub(crate) fn replace_neuron_state_for_session(
        &self,
        session_id: &str,
        neurons: Vec<RuntimeNeuronRecord>,
    ) -> Result<(), HeptaError> {
        let mut neuron_state = self
            .neuron_state
            .lock()
            .map_err(|_| HeptaError("neuron state mutex poisoned".into()))?;
        neuron_state
            .neurons
            .retain(|record| record.session_id != session_id);
        neuron_state.neurons.extend(neurons);
        Ok(())
    }

    pub(crate) fn intuition_feedback_for_session(
        &self,
        session_id: &str,
    ) -> Result<Vec<IntuitionFeedbackRecord>, HeptaError> {
        let feedback_state = self
            .intuition_feedback_state
            .lock()
            .map_err(|_| HeptaError("intuition feedback state mutex poisoned".into()))?;
        Ok(feedback_state
            .records
            .iter()
            .filter(|record| record.surface_session_id.0 == session_id)
            .cloned()
            .collect())
    }

    pub(crate) fn push_intuition_feedback_record(
        &self,
        record: IntuitionFeedbackRecord,
    ) -> Result<(), HeptaError> {
        let mut feedback_state = self
            .intuition_feedback_state
            .lock()
            .map_err(|_| HeptaError("intuition feedback state mutex poisoned".into()))?;
        feedback_state.records.push(record);
        Ok(())
    }

    pub(crate) fn replace_intuition_feedback_for_session(
        &self,
        session_id: &str,
        records: Vec<IntuitionFeedbackRecord>,
    ) -> Result<(), HeptaError> {
        let mut feedback_state = self
            .intuition_feedback_state
            .lock()
            .map_err(|_| HeptaError("intuition feedback state mutex poisoned".into()))?;
        feedback_state
            .records
            .retain(|record| record.surface_session_id.0 != session_id);
        feedback_state.records.extend(records);
        Ok(())
    }

    pub(crate) fn model_router_feedback_for_session(
        &self,
        session_id: &str,
    ) -> Result<Vec<TopicAwareModelFeedbackRecord>, HeptaError> {
        let feedback_state = self
            .model_router_feedback_state
            .lock()
            .map_err(|_| HeptaError("model router feedback state mutex poisoned".into()))?;
        Ok(feedback_state
            .records
            .iter()
            .filter(|record| record.session_id == session_id)
            .cloned()
            .collect())
    }

    pub(crate) fn push_model_router_feedback_record(
        &self,
        record: TopicAwareModelFeedbackRecord,
    ) -> Result<(), HeptaError> {
        let mut feedback_state = self
            .model_router_feedback_state
            .lock()
            .map_err(|_| HeptaError("model router feedback state mutex poisoned".into()))?;
        feedback_state.records.push(record);
        Ok(())
    }

    pub(crate) fn replace_model_router_feedback_for_session(
        &self,
        session_id: &str,
        records: Vec<TopicAwareModelFeedbackRecord>,
    ) -> Result<(), HeptaError> {
        let mut feedback_state = self
            .model_router_feedback_state
            .lock()
            .map_err(|_| HeptaError("model router feedback state mutex poisoned".into()))?;
        feedback_state
            .records
            .retain(|record| record.session_id != session_id);
        feedback_state.records.extend(records);
        Ok(())
    }

    fn apply_session_export(&self, export: SessionExport) -> Result<(), HeptaError> {
        let session_id = export.session.session_id.0.clone();
        let _mutation = self.begin_session_context_mutation(&session_id)?;
        self.apply_session_export_unleased(export)
    }

    fn apply_session_export_unleased(&self, export: SessionExport) -> Result<(), HeptaError> {
        let session_id = export.session.session_id.0.clone();

        if !self.providers.contains_model_ref(&export.model) {
            return Err(HeptaError(format!(
                "cannot import session {} with unknown model {}/{}",
                session_id, export.model.provider, export.model.model
            )));
        }

        self.memory
            .upsert_session_sync(export.session)
            .map_err(|err| HeptaError(err.0))?;

        {
            let mut approval_state = self
                .approval_state
                .lock()
                .map_err(|_| HeptaError("approval state mutex poisoned".into()))?;
            approval_state.set_legacy_snapshot(&session_id, export.approval);
        }
        SafetyGateClient::reset_context_for_session(self, &session_id)?;
        {
            let mut history_state = self
                .history_state
                .lock()
                .map_err(|_| HeptaError("history state mutex poisoned".into()))?;
            history_state.retain(|turn| turn.session_id != session_id);
            history_state.extend(export.history.into_iter().rev());
        }
        {
            let mut write_transaction_state = self
                .write_transaction_state
                .lock()
                .map_err(|_| HeptaError("write transaction state mutex poisoned".into()))?;
            write_transaction_state.retain(|entry| entry.session_id != session_id);
            write_transaction_state.extend(export.write_transactions.into_iter().rev());
        }
        {
            let mut write_transaction_group_state = self
                .write_transaction_group_state
                .lock()
                .map_err(|_| HeptaError("write transaction group state mutex poisoned".into()))?;
            write_transaction_group_state
                .groups
                .retain(|group| group.session_id != session_id);
            write_transaction_group_state
                .rollback_attempts
                .retain(|attempt| attempt.session_id != session_id);
            write_transaction_group_state
                .active_bindings
                .retain(|binding| binding.session_id != session_id);
            write_transaction_group_state
                .groups
                .extend(export.write_transaction_groups.into_iter().rev());
            write_transaction_group_state
                .rollback_attempts
                .extend(export.rollback_group_attempts.into_iter().rev());
            if let Some(active_group_id) = export.active_write_transaction_group_id {
                write_transaction_group_state.active_bindings.push(
                    SessionWriteTransactionGroupBinding {
                        session_id: session_id.clone(),
                        active_group_id,
                    },
                );
            }
        }
        {
            let mut write_lock_state = self
                .write_lock_state
                .lock()
                .map_err(|_| HeptaError("write lock state mutex poisoned".into()))?;
            write_lock_state
                .target_locks
                .retain(|lock| lock.session_id != session_id);
            write_lock_state
                .group_locks
                .retain(|lock| lock.session_id != session_id);
            write_lock_state
                .target_locks
                .extend(export.write_target_locks.into_iter().rev());
            write_lock_state
                .group_locks
                .extend(export.write_group_locks.into_iter().rev());
        }
        {
            let mut model_state = self
                .model_state
                .lock()
                .map_err(|_| HeptaError("model state mutex poisoned".into()))?;
            model_state
                .sessions
                .retain(|item| item.session_id != session_id);
            model_state.sessions.push(SessionModelState {
                session_id: session_id.clone(),
                selected_model: export.model,
            });
        }
        {
            let mut execution_profile_state = self
                .execution_profile_state
                .lock()
                .map_err(|_| HeptaError("execution profile state mutex poisoned".into()))?;
            execution_profile_state
                .sessions
                .retain(|item| item.session_id != session_id);
            execution_profile_state
                .sessions
                .push(SessionExecutionProfileBinding {
                    session_id: session_id.clone(),
                    profile: export.execution_profile,
                });
        }
        {
            let mut filesystem_scope_state = self
                .filesystem_scope_state
                .lock()
                .map_err(|_| HeptaError("filesystem scope state mutex poisoned".into()))?;
            filesystem_scope_state
                .sessions
                .retain(|item| item.session_id != session_id);
            filesystem_scope_state
                .sessions
                .push(SessionFilesystemScopeBinding {
                    session_id: session_id.clone(),
                    scope: export.filesystem_scope,
                });
        }
        {
            let mut capability_gate_state = self
                .capability_gate_state
                .lock()
                .map_err(|_| HeptaError("capability gate state mutex poisoned".into()))?;
            capability_gate_state
                .sessions
                .retain(|item| item.session_id != session_id);
            if !export.path_capability_gates.is_empty() {
                capability_gate_state
                    .sessions
                    .push(SessionCapabilityGateBinding {
                        session_id: session_id.clone(),
                        path_gates: export.path_capability_gates,
                    });
            }
        }
        {
            let mut write_path_scope_state = self
                .write_path_scope_state
                .lock()
                .map_err(|_| HeptaError("write path scope state mutex poisoned".into()))?;
            write_path_scope_state
                .sessions
                .retain(|item| item.session_id != session_id);
            write_path_scope_state
                .sessions
                .push(SessionWritePathScopeBinding {
                    session_id: session_id.clone(),
                    scope: export.write_path_scope,
                });
        }
        self.replace_topic_export_state_for_session(
            &session_id,
            export.topic_sessions,
            export.topic_graph_edges,
        )?;
        self.replace_neuron_state_for_session(&session_id, export.neurons)?;
        self.replace_intuition_feedback_for_session(&session_id, export.intuition_feedback)?;
        self.replace_model_router_feedback_for_session(&session_id, export.model_router_feedback)?;
        if self.active_session_id()? == session_id {
            let archived = self
                .memory
                .list_sessions()
                .map_err(|err| HeptaError(err.0))?
                .into_iter()
                .find(|record| record.session_id.0 == session_id)
                .and_then(|record| record.archived_at_unix_ms)
                .is_some();
            if archived {
                let fallback = self.choose_fallback_session_id(Some(&session_id))?;
                let mut guard = self
                    .session_state
                    .lock()
                    .map_err(|_| HeptaError("session state mutex poisoned".into()))?;
                guard.active_session_id = fallback;
            }
        }
        Ok(())
    }

    fn choose_fallback_session_id(
        &self,
        excluded_session_id: Option<&str>,
    ) -> Result<String, HeptaError> {
        let excluded_session_id = excluded_session_id.unwrap_or_default();
        let mut sessions = self
            .memory
            .list_sessions()
            .map_err(|err| HeptaError(err.0))?;
        sessions.sort_by_key(|session| std::cmp::Reverse(session.last_active_unix_ms));
        if let Some(candidate) = sessions.into_iter().find(|session| {
            session.session_id.0 != excluded_session_id && session.archived_at_unix_ms.is_none()
        }) {
            return Ok(candidate.session_id.0);
        }

        let fallback = if excluded_session_id == "session-main" {
            "session-fallback".to_string()
        } else {
            "session-main".to_string()
        };
        self.ensure_session_record_sync(&fallback)?;
        Ok(fallback)
    }

    fn runtime_snapshot(&self) -> Result<RuntimeSnapshot, HeptaError> {
        let model_state = self
            .model_state
            .lock()
            .map_err(|_| HeptaError("model state mutex poisoned".into()))?;
        let approval_state = self
            .approval_state
            .lock()
            .map_err(|_| HeptaError("approval state mutex poisoned".into()))?;
        let history_state = self
            .history_state
            .lock()
            .map_err(|_| HeptaError("history state mutex poisoned".into()))?;
        let event_state = self
            .event_state
            .lock()
            .map_err(|_| HeptaError("event state mutex poisoned".into()))?;
        let write_transaction_state = self
            .write_transaction_state
            .lock()
            .map_err(|_| HeptaError("write transaction state mutex poisoned".into()))?;
        let write_transaction_group_state = self
            .write_transaction_group_state
            .lock()
            .map_err(|_| HeptaError("write transaction group state mutex poisoned".into()))?;
        let write_lock_state = self
            .write_lock_state
            .lock()
            .map_err(|_| HeptaError("write lock state mutex poisoned".into()))?;
        let execution_profile_state = self
            .execution_profile_state
            .lock()
            .map_err(|_| HeptaError("execution profile state mutex poisoned".into()))?;
        let filesystem_scope_state = self
            .filesystem_scope_state
            .lock()
            .map_err(|_| HeptaError("filesystem scope state mutex poisoned".into()))?;
        let capability_gate_state = self
            .capability_gate_state
            .lock()
            .map_err(|_| HeptaError("capability gate state mutex poisoned".into()))?;
        let write_path_scope_state = self
            .write_path_scope_state
            .lock()
            .map_err(|_| HeptaError("write path scope state mutex poisoned".into()))?;
        let topic_session_state = self
            .topic_session_state
            .lock()
            .map_err(|_| HeptaError("topic session state mutex poisoned".into()))?;
        let topic_graph_state = self
            .topic_graph_state
            .lock()
            .map_err(|_| HeptaError("topic graph state mutex poisoned".into()))?;
        let neuron_state = self
            .neuron_state
            .lock()
            .map_err(|_| HeptaError("neuron state mutex poisoned".into()))?;
        let intuition_feedback_state = self
            .intuition_feedback_state
            .lock()
            .map_err(|_| HeptaError("intuition feedback state mutex poisoned".into()))?;
        let model_router_feedback_state = self
            .model_router_feedback_state
            .lock()
            .map_err(|_| HeptaError("model router feedback state mutex poisoned".into()))?;
        let worker_task_state = self
            .worker_task_state
            .lock()
            .map_err(|_| HeptaError("worker task state mutex poisoned".into()))?;
        let multi_agent_runtime_state = self
            .multi_agent_runtime_state
            .lock()
            .map_err(|_| HeptaError("multi-agent runtime state mutex poisoned".into()))?;
        let policy_rules = self
            .policy
            .custom_rules()
            .map_err(|err| HeptaError(err.0))?;

        Ok(RuntimeSnapshot {
            version: 1,
            active_model: model_state.default_active.clone(),
            available_models: self.providers.available_models(),
            session_models: model_state.sessions.clone(),
            active_session_id: self.active_session_id()?,
            policy_rules,
            approvals: approval_state.all_sessions(),
            history: history_state.clone(),
            session_execution_profiles: execution_profile_state.sessions.clone(),
            session_filesystem_scopes: filesystem_scope_state.sessions.clone(),
            session_capability_gates: capability_gate_state.sessions.clone(),
            session_write_path_scopes: write_path_scope_state.sessions.clone(),
            events: event_state.snapshot(),
            write_transactions: write_transaction_state.clone(),
            write_transaction_groups: write_transaction_group_state.groups.clone(),
            active_write_transaction_groups: write_transaction_group_state.active_bindings.clone(),
            rollback_group_attempts: write_transaction_group_state.rollback_attempts.clone(),
            write_target_locks: write_lock_state.target_locks.clone(),
            write_group_locks: write_lock_state.group_locks.clone(),
            topic_sessions: topic_session_state.sessions.clone(),
            topic_graph_edges: topic_graph_state.edges.clone(),
            neurons: neuron_state.neurons.clone(),
            intuition_feedback: intuition_feedback_state.records.clone(),
            model_router_feedback: model_router_feedback_state.records.clone(),
            worker_tasks: worker_task_state.records.clone(),
            multi_agent_runtime: multi_agent_runtime_state.clone(),
            memory: self.memory.snapshot().map_err(|err| HeptaError(err.0))?,
        })
    }

    fn apply_runtime_snapshot(&self, snapshot: RuntimeSnapshot) -> Result<(), HeptaError> {
        let _mutation = self.begin_global_context_mutation()?;
        self.apply_runtime_snapshot_unleased(snapshot)
    }

    fn apply_runtime_snapshot_unleased(&self, snapshot: RuntimeSnapshot) -> Result<(), HeptaError> {
        if !self.providers.contains_model_ref(&snapshot.active_model) {
            return Err(HeptaError(format!(
                "cannot load snapshot with unknown active model {}/{}",
                snapshot.active_model.provider, snapshot.active_model.model
            )));
        }
        if let Some(unknown) = snapshot
            .session_models
            .iter()
            .find(|binding| !self.providers.contains_model_ref(&binding.selected_model))
        {
            return Err(HeptaError(format!(
                "cannot load snapshot with unknown session model {} -> {}/{}",
                unknown.session_id, unknown.selected_model.provider, unknown.selected_model.model
            )));
        }
        {
            let mut model_state = self
                .model_state
                .lock()
                .map_err(|_| HeptaError("model state mutex poisoned".into()))?;
            let runtime_default = self.providers.default_model();
            model_state.default_active = if is_builtin_demo_model(&snapshot.active_model)
                && !is_builtin_demo_model(&runtime_default)
            {
                runtime_default
            } else {
                snapshot.active_model
            };
            model_state.sessions = snapshot.session_models;
        }
        {
            let mut session_state = self
                .session_state
                .lock()
                .map_err(|_| HeptaError("session state mutex poisoned".into()))?;
            session_state.active_session_id = snapshot.active_session_id;
        }
        self.policy
            .replace_rules(snapshot.policy_rules)
            .map_err(|err| HeptaError(err.0))?;
        {
            let mut approval_state = self
                .approval_state
                .lock()
                .map_err(|_| HeptaError("approval state mutex poisoned".into()))?;
            approval_state.replace_legacy_sessions(snapshot.approvals);
        }
        SafetyGateClient::reset_all_context(self)?;
        {
            let mut execution_profile_state = self
                .execution_profile_state
                .lock()
                .map_err(|_| HeptaError("execution profile state mutex poisoned".into()))?;
            execution_profile_state.sessions = snapshot.session_execution_profiles;
        }
        {
            let mut filesystem_scope_state = self
                .filesystem_scope_state
                .lock()
                .map_err(|_| HeptaError("filesystem scope state mutex poisoned".into()))?;
            filesystem_scope_state.sessions = snapshot.session_filesystem_scopes;
        }
        {
            let mut capability_gate_state = self
                .capability_gate_state
                .lock()
                .map_err(|_| HeptaError("capability gate state mutex poisoned".into()))?;
            capability_gate_state.sessions = snapshot.session_capability_gates;
        }
        {
            let mut write_path_scope_state = self
                .write_path_scope_state
                .lock()
                .map_err(|_| HeptaError("write path scope state mutex poisoned".into()))?;
            write_path_scope_state.sessions = snapshot.session_write_path_scopes;
        }
        {
            let mut write_transaction_state = self
                .write_transaction_state
                .lock()
                .map_err(|_| HeptaError("write transaction state mutex poisoned".into()))?;
            *write_transaction_state = snapshot.write_transactions;
        }
        {
            let mut write_transaction_group_state = self
                .write_transaction_group_state
                .lock()
                .map_err(|_| HeptaError("write transaction group state mutex poisoned".into()))?;
            write_transaction_group_state.groups = snapshot.write_transaction_groups;
            write_transaction_group_state.active_bindings =
                snapshot.active_write_transaction_groups;
            write_transaction_group_state.rollback_attempts = snapshot.rollback_group_attempts;
        }
        {
            let mut write_lock_state = self
                .write_lock_state
                .lock()
                .map_err(|_| HeptaError("write lock state mutex poisoned".into()))?;
            write_lock_state.target_locks = snapshot.write_target_locks;
            write_lock_state.group_locks = snapshot.write_group_locks;
        }
        {
            let mut topic_session_state = self
                .topic_session_state
                .lock()
                .map_err(|_| HeptaError("topic session state mutex poisoned".into()))?;
            topic_session_state.sessions = snapshot.topic_sessions;
        }
        {
            let mut topic_graph_state = self
                .topic_graph_state
                .lock()
                .map_err(|_| HeptaError("topic graph state mutex poisoned".into()))?;
            topic_graph_state.edges = snapshot.topic_graph_edges;
        }
        {
            let mut neuron_state = self
                .neuron_state
                .lock()
                .map_err(|_| HeptaError("neuron state mutex poisoned".into()))?;
            neuron_state.neurons = snapshot.neurons;
        }
        {
            let mut intuition_feedback_state = self
                .intuition_feedback_state
                .lock()
                .map_err(|_| HeptaError("intuition feedback state mutex poisoned".into()))?;
            intuition_feedback_state.records = snapshot.intuition_feedback;
        }
        {
            let mut model_router_feedback_state = self
                .model_router_feedback_state
                .lock()
                .map_err(|_| HeptaError("model router feedback state mutex poisoned".into()))?;
            model_router_feedback_state.records = snapshot.model_router_feedback;
        }
        {
            let mut worker_task_state = self
                .worker_task_state
                .lock()
                .map_err(|_| HeptaError("worker task state mutex poisoned".into()))?;
            worker_task_state.records = snapshot.worker_tasks;
        }
        {
            let mut multi_agent_runtime_state = self
                .multi_agent_runtime_state
                .lock()
                .map_err(|_| HeptaError("multi-agent runtime state mutex poisoned".into()))?;
            *multi_agent_runtime_state = snapshot.multi_agent_runtime;
        }
        {
            let mut history_state = self
                .history_state
                .lock()
                .map_err(|_| HeptaError("history state mutex poisoned".into()))?;
            *history_state = snapshot.history;
        }
        {
            let mut event_state = self
                .event_state
                .lock()
                .map_err(|_| HeptaError("event state mutex poisoned".into()))?;
            event_state.replace(snapshot.events);
        }
        self.memory
            .restore(snapshot.memory)
            .map_err(|err| HeptaError(err.0))?;
        Ok(())
    }

    fn rebind_session_export_topic_state(
        export: &mut SessionExport,
        source_session_id: &str,
        target_session_id: &str,
    ) {
        if source_session_id == target_session_id {
            return;
        }

        let mut topic_session_id_map = HashMap::new();
        let mut topic_id_map = HashMap::new();

        for topic_session in &mut export.topic_sessions {
            let previous_topic_session_id = topic_session.topic_session_id.clone();
            let previous_topic_id = topic_session.topic_id.0.clone();
            let next_topic_session_id = rebind_bootstrap_topic_session_id(
                &previous_topic_session_id,
                source_session_id,
                target_session_id,
            );
            let next_topic_id =
                rebind_bootstrap_topic_id(&previous_topic_id, source_session_id, target_session_id);

            topic_session_id_map.insert(previous_topic_session_id, next_topic_session_id.clone());
            topic_id_map.insert(previous_topic_id, next_topic_id.clone());
            topic_session.topic_session_id = next_topic_session_id;
            topic_session.topic_id = hepta_core::TopicId(next_topic_id);
            topic_session.linked_surface_session_ids =
                vec![SessionId(target_session_id.to_string())];
            for span in &mut topic_session.linked_transcript_spans {
                if span.session_id.0 == source_session_id {
                    span.session_id = SessionId(target_session_id.to_string());
                }
            }
            topic_session.graph_edges.clear();
        }

        for record in &mut export.topic_graph_edges {
            if let Some(remapped) = topic_session_id_map.get(&record.source_topic_session_id) {
                record.source_topic_session_id = remapped.clone();
            }
            if let Some(remapped) = topic_session_id_map.get(&record.edge.target_topic_session_id) {
                record.edge.target_topic_session_id = remapped.clone();
            }
        }

        let valid_topic_session_ids = topic_session_id_map
            .values()
            .cloned()
            .collect::<HashSet<_>>();
        export.topic_graph_edges.retain(|record| {
            valid_topic_session_ids.contains(&record.source_topic_session_id)
                && valid_topic_session_ids.contains(&record.edge.target_topic_session_id)
        });

        for record in &mut export.neurons {
            record.session_id = target_session_id.to_string();
            record.neuron.linked_session_ids = vec![SessionId(target_session_id.to_string())];
            for topic_session_id in &mut record.neuron.linked_topic_session_ids {
                if let Some(remapped) = topic_session_id_map.get(topic_session_id) {
                    *topic_session_id = remapped.clone();
                }
            }
            if let Some(remapped) = topic_id_map.get(&record.neuron.topic_id.0) {
                record.neuron.topic_id = hepta_core::TopicId(remapped.clone());
                record.neuron.neuron_id = hepta_core::NeuronId(format!("neuron-{}", remapped));
            }
            for span in &mut record.neuron.important_transcript_spans {
                if span.session_id.0 == source_session_id {
                    span.session_id = SessionId(target_session_id.to_string());
                }
            }
        }

        for record in &mut export.intuition_feedback {
            record.surface_session_id = SessionId(target_session_id.to_string());
            for topic_id in &mut record.source_topic_ids {
                if let Some(remapped) = topic_id_map.get(&topic_id.0) {
                    *topic_id = hepta_core::TopicId(remapped.clone());
                }
            }
        }

        for record in &mut export.model_router_feedback {
            record.session_id = target_session_id.to_string();
            for topic_id in &mut record.topic_ids {
                if let Some(remapped) = topic_id_map.get(&topic_id.0) {
                    *topic_id = hepta_core::TopicId(remapped.clone());
                }
            }
        }
    }

    fn model_key(model: &ModelRef) -> String {
        format!("{}/{}", model.provider, model.model)
    }

    fn resolve_execution_profile_for_session_from_state(
        state: &ExecutionProfileState,
        session_id: &str,
    ) -> ExecutionProfile {
        state
            .sessions
            .iter()
            .find(|item| item.session_id == session_id)
            .map(|item| item.profile)
            .unwrap_or(state.default_profile)
    }

    fn resolve_filesystem_scope_for_session_from_state(
        state: &FilesystemScopeState,
        session_id: &str,
    ) -> FilesystemScope {
        state
            .sessions
            .iter()
            .find(|item| item.session_id == session_id)
            .map(|item| item.scope)
            .unwrap_or(state.default_scope)
    }

    fn resolve_path_capability_gates_for_session_from_state(
        state: &CapabilityGateState,
        session_id: &str,
    ) -> Vec<PathCapabilityGate> {
        state
            .sessions
            .iter()
            .find(|item| item.session_id == session_id)
            .map(|item| item.path_gates.clone())
            .unwrap_or_default()
    }

    fn resolve_write_path_scope_for_session_from_state(
        state: &WritePathScopeState,
        session_id: &str,
    ) -> WritePathScope {
        state
            .sessions
            .iter()
            .find(|item| item.session_id == session_id)
            .map(|item| item.scope)
            .unwrap_or(state.default_scope)
    }

    fn ensure_capability_binding_mut<'a>(
        state: &'a mut CapabilityGateState,
        session_id: &str,
    ) -> &'a mut SessionCapabilityGateBinding {
        if let Some(index) = state
            .sessions
            .iter()
            .position(|item| item.session_id == session_id)
        {
            return &mut state.sessions[index];
        }
        state.sessions.push(SessionCapabilityGateBinding {
            session_id: session_id.to_string(),
            path_gates: Vec::new(),
        });
        let inserted_index = state.sessions.len() - 1;
        &mut state.sessions[inserted_index]
    }

    fn resolve_model_for_session_from_state(
        model_state: &ModelState,
        session_id: &str,
    ) -> ModelRef {
        model_state
            .sessions
            .iter()
            .find(|model| model.session_id == session_id)
            .map(|model| model.selected_model.clone())
            .unwrap_or_else(|| model_state.default_active.clone())
    }
}
