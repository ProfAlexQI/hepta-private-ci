impl RuntimeKernel {
    pub fn new() -> Self {
        let providers = ProviderRegistry::new();
        let active = providers.default_model();

        Self {
            providers,
            tools: ToolRegistry::new(),
            memory: InMemoryStore::default(),
            policy: ConfigurablePolicyEngine::default(),
            approval_state: Arc::new(Mutex::new(ApprovalState::default())),
            history_state: Arc::new(Mutex::new(Vec::new())),
            event_state: Arc::new(Mutex::new(EventState::new_with_boot_event())),
            model_state: Arc::new(Mutex::new(ModelState {
                default_active: active,
                sessions: Vec::new(),
            })),
            execution_profile_state: Arc::new(Mutex::new(ExecutionProfileState {
                default_profile: ExecutionProfile::FullAccess,
                sessions: Vec::new(),
            })),
            filesystem_scope_state: Arc::new(Mutex::new(FilesystemScopeState {
                default_scope: FilesystemScope::WorkspaceOnly,
                sessions: Vec::new(),
            })),
            capability_gate_state: Arc::new(Mutex::new(CapabilityGateState {
                sessions: Vec::new(),
            })),
            write_path_scope_state: Arc::new(Mutex::new(WritePathScopeState {
                default_scope: WritePathScope::ArtifactsOnly,
                sessions: Vec::new(),
            })),
            write_transaction_state: Arc::new(Mutex::new(Vec::new())),
            write_transaction_group_state: Arc::new(Mutex::new(
                WriteTransactionGroupState::default(),
            )),
            write_lock_state: Arc::new(Mutex::new(WriteLockState::default())),
            rollback_failure_injection_state: Arc::new(Mutex::new(Vec::new())),
            session_state: Arc::new(Mutex::new(SessionState {
                active_session_id: "session-main".into(),
            })),
            worker_task_state: Arc::new(Mutex::new(WorkerTaskState::default())),
            multi_agent_runtime_state: Arc::new(Mutex::new(MultiAgentRuntimeState::default())),
            topic_session_state: Arc::new(Mutex::new(TopicSessionState::default())),
            topic_graph_state: Arc::new(Mutex::new(TopicGraphState::default())),
            neuron_state: Arc::new(Mutex::new(NeuronState::default())),
            intuition_feedback_state: Arc::new(Mutex::new(IntuitionFeedbackState::default())),
            model_router_feedback_state: Arc::new(Mutex::new(ModelRouterFeedbackState::default())),
        }
    }

    pub fn model_selection(&self) -> Result<ModelSelection, HeptaError> {
        let active_session_id = self.active_session_id()?;
        self.model_selection_for_session(&active_session_id)
    }

    pub fn model_selection_for_session(
        &self,
        session_id: &str,
    ) -> Result<ModelSelection, HeptaError> {
        let session_id = session_id.trim();
        if session_id.is_empty() {
            return Err(HeptaError("session id must not be empty".into()));
        }
        let guard = self
            .model_state
            .lock()
            .map_err(|_| HeptaError("model state mutex poisoned".into()))?;
        Ok(ModelSelection {
            active: Self::resolve_model_for_session_from_state(&guard, session_id),
            available: self.providers.available_models(),
        })
    }

    pub fn execution_profile(&self) -> Result<ExecutionProfile, HeptaError> {
        let active_session_id = self.active_session_id()?;
        self.execution_profile_for_session(&active_session_id)
    }

    pub fn execution_profile_for_session(
        &self,
        session_id: &str,
    ) -> Result<ExecutionProfile, HeptaError> {
        let session_id = session_id.trim();
        if session_id.is_empty() {
            return Err(HeptaError("session id must not be empty".into()));
        }
        let guard = self
            .execution_profile_state
            .lock()
            .map_err(|_| HeptaError("execution profile state mutex poisoned".into()))?;
        Ok(Self::resolve_execution_profile_for_session_from_state(
            &guard, session_id,
        ))
    }

    pub fn switch_execution_profile(
        &self,
        target: ExecutionProfile,
    ) -> Result<SwitchExecutionProfileResult, HeptaError> {
        let active_session_id = self.active_session_id()?;
        self.switch_execution_profile_in_session(&active_session_id, target)
    }

    pub fn switch_execution_profile_in_session(
        &self,
        session_id: &str,
        target: ExecutionProfile,
    ) -> Result<SwitchExecutionProfileResult, HeptaError> {
        let session_id = session_id.trim();
        if session_id.is_empty() {
            return Err(HeptaError("session id must not be empty".into()));
        }
        self.ensure_session_record_sync(session_id)?;
        let mut guard = self
            .execution_profile_state
            .lock()
            .map_err(|_| HeptaError("execution profile state mutex poisoned".into()))?;
        let previous = Self::resolve_execution_profile_for_session_from_state(&guard, session_id);
        if let Some(existing) = guard
            .sessions
            .iter_mut()
            .find(|item| item.session_id == session_id)
        {
            existing.profile = target;
        } else {
            guard.sessions.push(SessionExecutionProfileBinding {
                session_id: session_id.to_string(),
                profile: target,
            });
        }
        drop(guard);
        let result = SwitchExecutionProfileResult {
            previous,
            current: target,
        };
        self.emit_event(
            EventKind::ExecutionProfileChanged,
            Some(SessionId(session_id.to_string())),
            None,
            format!(
                "switched execution profile {} -> {}",
                format_execution_profile(result.previous),
                format_execution_profile(result.current)
            ),
        )?;
        Ok(result)
    }

    pub fn filesystem_scope(&self) -> Result<FilesystemScope, HeptaError> {
        let active_session_id = self.active_session_id()?;
        self.filesystem_scope_for_session(&active_session_id)
    }

    pub fn filesystem_scope_for_session(
        &self,
        session_id: &str,
    ) -> Result<FilesystemScope, HeptaError> {
        let session_id = session_id.trim();
        if session_id.is_empty() {
            return Err(HeptaError("session id must not be empty".into()));
        }
        let guard = self
            .filesystem_scope_state
            .lock()
            .map_err(|_| HeptaError("filesystem scope state mutex poisoned".into()))?;
        Ok(Self::resolve_filesystem_scope_for_session_from_state(
            &guard, session_id,
        ))
    }

    pub fn switch_filesystem_scope(
        &self,
        target: FilesystemScope,
    ) -> Result<SwitchFilesystemScopeResult, HeptaError> {
        let active_session_id = self.active_session_id()?;
        self.switch_filesystem_scope_in_session(&active_session_id, target)
    }

    pub fn switch_filesystem_scope_in_session(
        &self,
        session_id: &str,
        target: FilesystemScope,
    ) -> Result<SwitchFilesystemScopeResult, HeptaError> {
        let session_id = session_id.trim();
        if session_id.is_empty() {
            return Err(HeptaError("session id must not be empty".into()));
        }
        self.ensure_session_record_sync(session_id)?;
        let mut guard = self
            .filesystem_scope_state
            .lock()
            .map_err(|_| HeptaError("filesystem scope state mutex poisoned".into()))?;
        let previous = Self::resolve_filesystem_scope_for_session_from_state(&guard, session_id);
        if let Some(existing) = guard
            .sessions
            .iter_mut()
            .find(|item| item.session_id == session_id)
        {
            existing.scope = target;
        } else {
            guard.sessions.push(SessionFilesystemScopeBinding {
                session_id: session_id.to_string(),
                scope: target,
            });
        }
        drop(guard);
        let result = SwitchFilesystemScopeResult {
            previous,
            current: target,
        };
        self.emit_event(
            EventKind::FilesystemScopeChanged,
            Some(SessionId(session_id.to_string())),
            None,
            format!(
                "switched filesystem scope {} -> {}",
                format_filesystem_scope(result.previous),
                format_filesystem_scope(result.current)
            ),
        )?;
        Ok(result)
    }

    pub fn write_path_scope(&self) -> Result<WritePathScope, HeptaError> {
        let active_session_id = self.active_session_id()?;
        self.write_path_scope_for_session(&active_session_id)
    }

    pub fn write_path_scope_for_session(
        &self,
        session_id: &str,
    ) -> Result<WritePathScope, HeptaError> {
        let session_id = session_id.trim();
        if session_id.is_empty() {
            return Err(HeptaError("session id must not be empty".into()));
        }
        let guard = self
            .write_path_scope_state
            .lock()
            .map_err(|_| HeptaError("write path scope state mutex poisoned".into()))?;
        Ok(Self::resolve_write_path_scope_for_session_from_state(
            &guard, session_id,
        ))
    }

    pub fn switch_write_path_scope(
        &self,
        target: WritePathScope,
    ) -> Result<SwitchWritePathScopeResult, HeptaError> {
        let active_session_id = self.active_session_id()?;
        self.switch_write_path_scope_in_session(&active_session_id, target)
    }

    pub fn switch_write_path_scope_in_session(
        &self,
        session_id: &str,
        target: WritePathScope,
    ) -> Result<SwitchWritePathScopeResult, HeptaError> {
        let session_id = session_id.trim();
        if session_id.is_empty() {
            return Err(HeptaError("session id must not be empty".into()));
        }
        self.ensure_session_record_sync(session_id)?;
        let mut guard = self
            .write_path_scope_state
            .lock()
            .map_err(|_| HeptaError("write path scope state mutex poisoned".into()))?;
        let previous = Self::resolve_write_path_scope_for_session_from_state(&guard, session_id);
        if let Some(existing) = guard
            .sessions
            .iter_mut()
            .find(|item| item.session_id == session_id)
        {
            existing.scope = target;
        } else {
            guard.sessions.push(SessionWritePathScopeBinding {
                session_id: session_id.to_string(),
                scope: target,
            });
        }
        drop(guard);
        let result = SwitchWritePathScopeResult {
            previous,
            current: target,
        };
        self.emit_event(
            EventKind::WritePathScopeChanged,
            Some(SessionId(session_id.to_string())),
            None,
            format!(
                "switched write path scope {} -> {}",
                format_write_path_scope(result.previous),
                format_write_path_scope(result.current)
            ),
        )?;
        Ok(result)
    }

    pub fn capability_gate_report(&self) -> Result<CapabilityGateReport, HeptaError> {
        let active_session_id = self.active_session_id()?;
        self.capability_gate_report_for_session(&active_session_id)
    }

    pub fn capability_gate_report_for_session(
        &self,
        session_id: &str,
    ) -> Result<CapabilityGateReport, HeptaError> {
        Ok(CapabilityGateReport {
            session_id: session_id.to_string(),
            default_filesystem_scope: self.filesystem_scope_for_session(session_id)?,
            path_gates: self.path_capability_gates_for_session(session_id)?,
        })
    }

    pub fn path_capability_gates_for_session(
        &self,
        session_id: &str,
    ) -> Result<Vec<PathCapabilityGate>, HeptaError> {
        let session_id = session_id.trim();
        if session_id.is_empty() {
            return Err(HeptaError("session id must not be empty".into()));
        }
        let guard = self
            .capability_gate_state
            .lock()
            .map_err(|_| HeptaError("capability gate state mutex poisoned".into()))?;
        Ok(Self::resolve_path_capability_gates_for_session_from_state(
            &guard, session_id,
        ))
    }

    pub fn set_path_capability_gate(
        &self,
        tool_name: &str,
        argument_name: &str,
        scope: FilesystemScope,
    ) -> Result<PathCapabilityGate, HeptaError> {
        let active_session_id = self.active_session_id()?;
        self.set_path_capability_gate_in_session(
            &active_session_id,
            tool_name,
            argument_name,
            scope,
        )
    }

    pub fn set_path_capability_gate_in_session(
        &self,
        session_id: &str,
        tool_name: &str,
        argument_name: &str,
        scope: FilesystemScope,
    ) -> Result<PathCapabilityGate, HeptaError> {
        let session_id = session_id.trim();
        let tool_name = tool_name.trim();
        let argument_name = argument_name.trim();
        if session_id.is_empty() {
            return Err(HeptaError("session id must not be empty".into()));
        }
        if tool_name.is_empty() {
            return Err(HeptaError("tool name must not be empty".into()));
        }
        if argument_name.is_empty() {
            return Err(HeptaError("argument name must not be empty".into()));
        }
        self.ensure_session_record_sync(session_id)?;
        self.tools.schema(tool_name)?;
        ensure_tool_schema_has_field(
            &self.tools.schema(tool_name)?.input_schema_json,
            tool_name,
            argument_name,
        )?;

        let mut guard = self
            .capability_gate_state
            .lock()
            .map_err(|_| HeptaError("capability gate state mutex poisoned".into()))?;
        let binding = Self::ensure_capability_binding_mut(&mut guard, session_id);
        if let Some(existing) = binding
            .path_gates
            .iter_mut()
            .find(|gate| gate.tool_name == tool_name && gate.argument_name == argument_name)
        {
            existing.scope = scope;
            let updated = existing.clone();
            drop(guard);
            self.emit_event(
                EventKind::CapabilityGateChanged,
                Some(SessionId(session_id.to_string())),
                None,
                format!(
                    "set path capability gate {} {}.{} -> {}",
                    updated.id,
                    updated.tool_name,
                    updated.argument_name,
                    format_filesystem_scope(updated.scope)
                ),
            )?;
            return Ok(updated);
        }

        let gate = PathCapabilityGate {
            id: format!(
                "cap-{}-{}-{}",
                session_id,
                tool_name,
                binding.path_gates.len() + 1
            ),
            tool_name: tool_name.to_string(),
            argument_name: argument_name.to_string(),
            scope,
        };
        binding.path_gates.push(gate.clone());
        drop(guard);
        self.emit_event(
            EventKind::CapabilityGateChanged,
            Some(SessionId(session_id.to_string())),
            None,
            format!(
                "set path capability gate {} {}.{} -> {}",
                gate.id,
                gate.tool_name,
                gate.argument_name,
                format_filesystem_scope(gate.scope)
            ),
        )?;
        Ok(gate)
    }

    pub fn remove_path_capability_gate(&self, rule_id: &str) -> Result<bool, HeptaError> {
        let active_session_id = self.active_session_id()?;
        self.remove_path_capability_gate_in_session(&active_session_id, rule_id)
    }

    pub fn remove_path_capability_gate_in_session(
        &self,
        session_id: &str,
        rule_id: &str,
    ) -> Result<bool, HeptaError> {
        let session_id = session_id.trim();
        let rule_id = rule_id.trim();
        if session_id.is_empty() {
            return Err(HeptaError("session id must not be empty".into()));
        }
        if rule_id.is_empty() {
            return Err(HeptaError("rule id must not be empty".into()));
        }

        let mut guard = self
            .capability_gate_state
            .lock()
            .map_err(|_| HeptaError("capability gate state mutex poisoned".into()))?;
        let Some(binding) = guard
            .sessions
            .iter_mut()
            .find(|item| item.session_id == session_id)
        else {
            return Ok(false);
        };
        let before = binding.path_gates.len();
        binding.path_gates.retain(|gate| gate.id != rule_id);
        let removed = before != binding.path_gates.len();
        if binding.path_gates.is_empty() {
            guard.sessions.retain(|item| item.session_id != session_id);
        }
        drop(guard);
        if removed {
            self.emit_event(
                EventKind::CapabilityGateChanged,
                Some(SessionId(session_id.to_string())),
                None,
                format!("removed path capability gate {}", rule_id),
            )?;
        }
        Ok(removed)
    }

    pub fn switch_model(&self, target: &str) -> Result<SwitchModelResult, HeptaError> {
        let active_session_id = self.active_session_id()?;
        self.switch_model_in_session(&active_session_id, target)
    }

    pub fn switch_model_in_session(
        &self,
        session_id: &str,
        target: &str,
    ) -> Result<SwitchModelResult, HeptaError> {
        let session_id = session_id.trim();
        if session_id.is_empty() {
            return Err(HeptaError("session id must not be empty".into()));
        }
        self.ensure_session_record_sync(session_id)?;
        let mut guard = self
            .model_state
            .lock()
            .map_err(|_| HeptaError("model state mutex poisoned".into()))?;

        let current = Self::resolve_model_for_session_from_state(&guard, session_id);
        let maybe = self.providers.find_model(target);

        let next = maybe.ok_or_else(|| HeptaError(format!("unknown model: {}", target)))?;
        if let Some(existing) = guard
            .sessions
            .iter_mut()
            .find(|model| model.session_id == session_id)
        {
            existing.selected_model = next.clone();
        } else {
            guard.sessions.push(SessionModelState {
                session_id: session_id.to_string(),
                selected_model: next.clone(),
            });
        }

        let result = SwitchModelResult {
            previous: current,
            current: next,
        };
        self.emit_event(
            EventKind::ModelSwitched,
            Some(SessionId(session_id.to_string())),
            None,
            format!(
                "switched model {}/{} -> {}/{}",
                result.previous.provider,
                result.previous.model,
                result.current.provider,
                result.current.model
            ),
        )?;
        Ok(result)
    }

    pub fn tool_names(&self) -> Vec<String> {
        self.tools.names()
    }

    pub fn tool_descriptors(&self) -> Vec<ToolDescriptor> {
        self.tools.descriptors()
    }

    pub fn provider_names(&self) -> Vec<String> {
        self.providers.names()
    }

    pub fn provider_catalog(&self) -> ProviderCatalog {
        ProviderCatalog {
            providers: self.providers.descriptors(),
        }
    }

    pub async fn policy_report(&self) -> Result<PolicyReport, HeptaError> {
        let active_session_id = self.active_session_id()?;
        let active_model = self.model_selection()?.active;
        let approvals = self.approval_snapshot_for_session(&active_session_id)?;
        let default_rules = self.policy.default_rules();
        let custom_rules = self
            .policy
            .custom_rules()
            .map_err(|err| HeptaError(err.0))?;

        let mut effective_tool_decisions = Vec::new();
        for tool in self.tool_descriptors() {
            let decision = self
                .policy
                .evaluate_tool(PolicyEvaluationContext {
                    session_id: Some(SessionId(active_session_id.clone())),
                    model: Some(active_model.clone()),
                    tool_name: tool.name.clone(),
                    risk_tier: tool.risk_tier,
                })
                .await
                .map_err(|err| HeptaError(err.0))?;
            effective_tool_decisions.push(PolicyToolDecisionReport {
                tool_name: tool.name,
                risk_tier: tool.risk_tier,
                requirement: decision.requirement,
                reason: decision.reason,
                matched_rule_id: decision.matched_rule_id,
            });
        }

        Ok(PolicyReport {
            active_session_id,
            active_model,
            default_rules,
            custom_rules,
            effective_tool_decisions,
            granted_approvals: approvals.granted_tools.len(),
            pending_approvals: approvals.pending.len(),
        })
    }

    pub fn add_policy_rule(
        &self,
        session_id: Option<&str>,
        provider_name: Option<&str>,
        tool_name: Option<&str>,
        risk_tier: Option<RiskTier>,
        requirement: ApprovalRequirement,
        reason: Option<&str>,
    ) -> Result<PolicyRule, HeptaError> {
        let normalized_session_id = session_id
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(|value| value.to_string());
        let normalized_provider_name = provider_name
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(|value| value.to_string());
        let normalized_tool_name = tool_name
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(|value| value.to_string());

        if let Some(provider_name) = normalized_provider_name.as_deref() {
            if !self
                .provider_names()
                .iter()
                .any(|name| name == provider_name)
            {
                return Err(HeptaError(format!("unknown provider: {}", provider_name)));
            }
        }
        if let Some(tool_name) = normalized_tool_name.as_deref() {
            if !self.tools.contains(tool_name) {
                return Err(HeptaError(format!("unknown tool: {}", tool_name)));
            }
        }
        if let Some(session_id) = normalized_session_id.as_deref() {
            self.ensure_session_record_sync(session_id)?;
        }

        let next_index = self
            .policy
            .custom_rules()
            .map_err(|err| HeptaError(err.0))?
            .len()
            + 1;

        let rule = PolicyRule {
            id: format!("policy-{}-{}", current_unix_ms()?, next_index),
            session_id: normalized_session_id,
            provider_name: normalized_provider_name,
            tool_name: normalized_tool_name,
            risk_tier,
            requirement,
            reason: reason
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(|value| value.to_string())
                .unwrap_or_else(|| "custom policy rule".into()),
        };

        let stored = self
            .policy
            .add_rule(rule)
            .map_err(|err| HeptaError(err.0))?;
        self.emit_event(
            EventKind::PolicyUpdated,
            None,
            None,
            format!("added policy rule {}", stored.id),
        )?;
        Ok(stored)
    }

    pub fn remove_policy_rule(&self, rule_id: &str) -> Result<String, HeptaError> {
        let rule_id = rule_id.trim();
        if rule_id.is_empty() {
            return Err(HeptaError("policy rule id must not be empty".into()));
        }
        let removed = self
            .policy
            .remove_rule(rule_id)
            .map_err(|err| HeptaError(err.0))?;
        if !removed {
            return Err(HeptaError(format!("unknown policy rule: {}", rule_id)));
        }
        self.emit_event(
            EventKind::PolicyUpdated,
            None,
            None,
            format!("removed policy rule {}", rule_id),
        )?;
        Ok(format!("removed policy rule {}", rule_id))
    }

    pub fn reset_policy_rules(&self) -> Result<String, HeptaError> {
        let removed = self.policy.clear_rules().map_err(|err| HeptaError(err.0))?;
        self.emit_event(
            EventKind::PolicyUpdated,
            None,
            None,
            format!("cleared {} custom policy rule(s)", removed),
        )?;
        Ok(format!("cleared {} custom policy rule(s)", removed))
    }

    pub fn validate_tool_input(&self, tool_name: &str, input_json: &str) -> Result<(), HeptaError> {
        self.tools.validate_input(tool_name, input_json)
    }

    pub fn validate_tool_output(
        &self,
        tool_name: &str,
        output_json: &str,
    ) -> Result<(), HeptaError> {
        self.tools.validate_output(tool_name, output_json)
    }

    pub fn approval_snapshot(&self) -> Result<ApprovalSnapshot, HeptaError> {
        let active_session_id = self.active_session_id()?;
        self.approval_snapshot_for_session(&active_session_id)
    }

    pub fn approval_snapshot_for_session(
        &self,
        session_id: &str,
    ) -> Result<ApprovalSnapshot, HeptaError> {
        let session_id = session_id.trim();
        if session_id.is_empty() {
            return Err(HeptaError("session id must not be empty".into()));
        }
        let guard = self
            .approval_state
            .lock()
            .map_err(|_| HeptaError("approval state mutex poisoned".into()))?;
        Ok(guard.snapshot_for(session_id))
    }

    pub fn approve_tool(&self, tool_name: &str) -> Result<String, HeptaError> {
        let active_session_id = self.active_session_id()?;
        self.approve_tool_in_session(&active_session_id, tool_name)
    }

    pub fn approve_tool_in_session(
        &self,
        session_id: &str,
        tool_name: &str,
    ) -> Result<String, HeptaError> {
        if !self.tools.contains(tool_name) {
            return Err(HeptaError(format!("unknown tool: {}", tool_name)));
        }
        let session_id = session_id.trim();
        if session_id.is_empty() {
            return Err(HeptaError("session id must not be empty".into()));
        }
        self.ensure_session_record_sync(session_id)?;
        let mut guard = self
            .approval_state
            .lock()
            .map_err(|_| HeptaError("approval state mutex poisoned".into()))?;
        guard.grant(session_id, tool_name);
        drop(guard);
        self.emit_event(
            EventKind::ApprovalGranted,
            Some(SessionId(session_id.to_string())),
            None,
            format!("approved tool {}", tool_name),
        )?;
        Ok(format!(
            "approved tool for session {}: {}",
            session_id, tool_name
        ))
    }

    pub fn active_session_id(&self) -> Result<String, HeptaError> {
        let guard = self
            .session_state
            .lock()
            .map_err(|_| HeptaError("session state mutex poisoned".into()))?;
        Ok(guard.active_session_id.clone())
    }

    pub fn switch_session(&self, session_id: &str) -> Result<String, HeptaError> {
        let session_id = session_id.trim();
        if session_id.is_empty() {
            return Err(HeptaError("session id must not be empty".into()));
        }

        {
            let mut guard = self
                .session_state
                .lock()
                .map_err(|_| HeptaError("session state mutex poisoned".into()))?;
            guard.active_session_id = session_id.to_string();
        }

        self.ensure_session_record_sync(session_id)?;
        self.emit_event(
            EventKind::SessionSwitched,
            Some(SessionId(session_id.to_string())),
            None,
            format!("switched active session to {}", session_id),
        )?;
        Ok(format!("switched active session to {}", session_id))
    }

    pub fn active_session_snapshot(&self) -> Result<SessionSnapshot, HeptaError> {
        let active_session_id = self.active_session_id()?;
        match self.session_snapshot_for_id(&active_session_id) {
            Ok(session) => Ok(session),
            Err(err) if err.0 == format!("session not found: {}", active_session_id) => Err(
                HeptaError(format!("active session not found: {}", active_session_id)),
            ),
            Err(err) => Err(err),
        }
    }

    pub fn rename_active_session(&self, title: &str) -> Result<String, HeptaError> {
        let title = title.trim();
        if title.is_empty() {
            return Err(HeptaError("session title must not be empty".into()));
        }
        let session_id = SessionId(self.active_session_id()?);
        self.upsert_session_record(&session_id, Some(title.to_string()), None, None, true)?;
        self.emit_event(
            EventKind::SessionRenamed,
            Some(session_id.clone()),
            None,
            format!("renamed session to \"{}\"", title),
        )?;
        Ok(format!(
            "renamed active session {} to \"{}\"",
            session_id.0, title
        ))
    }

    pub fn archive_session(&self, session_id: Option<&str>) -> Result<String, HeptaError> {
        let session_id = session_id
            .map(str::trim)
            .filter(|item| !item.is_empty())
            .map(|item| item.to_string())
            .unwrap_or(self.active_session_id()?);
        let record = self.session_snapshot_for_id(&session_id)?;

        if record.archived_at_unix_ms.is_some() {
            return Ok(format!("session {} is already archived", session_id));
        }

        if self.active_session_id()? == session_id {
            let fallback = self.choose_fallback_session_id(Some(&session_id))?;
            let mut guard = self
                .session_state
                .lock()
                .map_err(|_| HeptaError("session state mutex poisoned".into()))?;
            guard.active_session_id = fallback;
        }

        self.upsert_session_record(
            &SessionId(record.session_id.clone()),
            None,
            None,
            Some(Some(current_unix_ms()?)),
            false,
        )?;
        self.emit_event(
            EventKind::SessionArchived,
            Some(SessionId(record.session_id.clone())),
            None,
            format!("archived session {}", session_id),
        )?;
        Ok(format!("archived session {}", session_id))
    }

    pub fn unarchive_session(&self, session_id: &str) -> Result<String, HeptaError> {
        let session_id = session_id.trim();
        if session_id.is_empty() {
            return Err(HeptaError("session id must not be empty".into()));
        }
        self.ensure_session_record_sync(session_id)?;
        self.upsert_session_record(
            &SessionId(session_id.to_string()),
            None,
            None,
            Some(None),
            false,
        )?;
        self.emit_event(
            EventKind::SessionUnarchived,
            Some(SessionId(session_id.to_string())),
            None,
            format!("unarchived session {}", session_id),
        )?;
        Ok(format!("unarchived session {}", session_id))
    }

    pub fn delete_session(&self, session_id: &str) -> Result<String, HeptaError> {
        let session_id = session_id.trim();
        if session_id.is_empty() {
            return Err(HeptaError("session id must not be empty".into()));
        }

        let active_session_id = self.active_session_id()?;
        if active_session_id == session_id {
            let fallback = self.choose_fallback_session_id(Some(session_id))?;
            let mut guard = self
                .session_state
                .lock()
                .map_err(|_| HeptaError("session state mutex poisoned".into()))?;
            guard.active_session_id = fallback;
        }

        let removed = self
            .memory
            .remove_session_sync(&SessionId(session_id.to_string()))
            .map_err(|err| HeptaError(err.0))?;
        if removed.is_none() {
            return Err(HeptaError(format!("unknown session: {}", session_id)));
        }

        {
            let mut approval_state = self
                .approval_state
                .lock()
                .map_err(|_| HeptaError("approval state mutex poisoned".into()))?;
            approval_state.remove_session(session_id);
        }
        {
            let mut history_state = self
                .history_state
                .lock()
                .map_err(|_| HeptaError("history state mutex poisoned".into()))?;
            history_state.retain(|turn| turn.session_id != session_id);
        }
        {
            let mut model_state = self
                .model_state
                .lock()
                .map_err(|_| HeptaError("model state mutex poisoned".into()))?;
            model_state
                .sessions
                .retain(|item| item.session_id != session_id);
        }
        {
            let mut execution_profile_state = self
                .execution_profile_state
                .lock()
                .map_err(|_| HeptaError("execution profile state mutex poisoned".into()))?;
            execution_profile_state
                .sessions
                .retain(|item| item.session_id != session_id);
        }
        {
            let mut filesystem_scope_state = self
                .filesystem_scope_state
                .lock()
                .map_err(|_| HeptaError("filesystem scope state mutex poisoned".into()))?;
            filesystem_scope_state
                .sessions
                .retain(|item| item.session_id != session_id);
        }
        {
            let mut capability_gate_state = self
                .capability_gate_state
                .lock()
                .map_err(|_| HeptaError("capability gate state mutex poisoned".into()))?;
            capability_gate_state
                .sessions
                .retain(|item| item.session_id != session_id);
        }
        {
            let mut write_path_scope_state = self
                .write_path_scope_state
                .lock()
                .map_err(|_| HeptaError("write path scope state mutex poisoned".into()))?;
            write_path_scope_state
                .sessions
                .retain(|item| item.session_id != session_id);
        }
        self.replace_topic_export_state_for_session(session_id, Vec::new(), Vec::new())?;
        self.replace_neuron_state_for_session(session_id, Vec::new())?;
        self.replace_intuition_feedback_for_session(session_id, Vec::new())?;
        self.replace_model_router_feedback_for_session(session_id, Vec::new())?;

        self.emit_event(
            EventKind::SessionDeleted,
            Some(SessionId(session_id.to_string())),
            None,
            format!("deleted session {}", session_id),
        )?;

        Ok(format!("deleted session {}", session_id))
    }

    pub fn prune_sessions(&self, max_sessions: usize) -> Result<String, HeptaError> {
        if max_sessions == 0 {
            return Err(HeptaError("max session count must be at least 1".into()));
        }

        let active_session_id = self.active_session_id()?;
        let sessions = self.sessions()?;
        let total_sessions = sessions.len();
        if total_sessions <= max_sessions {
            return Ok(format!(
                "no pruning needed, sessions={} max={}",
                total_sessions, max_sessions
            ));
        }

        let mut candidates = sessions
            .into_iter()
            .filter(|session| !session.is_active)
            .collect::<Vec<_>>();
        candidates.sort_by_key(|session| {
            (
                if session.archived_at_unix_ms.is_some() {
                    0_u8
                } else {
                    1_u8
                },
                session.last_active_unix_ms,
            )
        });

        let delete_count = total_sessions.saturating_sub(max_sessions);
        let targets = candidates
            .into_iter()
            .take(delete_count)
            .map(|session| session.session_id)
            .collect::<Vec<_>>();

        if targets.is_empty() {
            return Ok(format!(
                "no prune candidates available, active session protected: {}",
                active_session_id
            ));
        }

        for session_id in &targets {
            self.delete_session(session_id)?;
        }

        self.emit_event(
            EventKind::SessionsPruned,
            None,
            None,
            format!(
                "pruned {} session(s): {}",
                targets.len(),
                targets.join(", ")
            ),
        )?;

        Ok(format!(
            "pruned {} session(s): {}",
            targets.len(),
            targets.join(", ")
        ))
    }

    pub fn export_session(
        &self,
        session_id: &str,
        path: &str,
    ) -> Result<SessionExportReport, HeptaError> {
        let session_id = session_id.trim();
        if session_id.is_empty() {
            return Err(HeptaError("session id must not be empty".into()));
        }
        let path = path.trim();
        if path.is_empty() {
            return Err(HeptaError("export path must not be empty".into()));
        }

        let export = self.session_export(session_id)?;
        let approvals_granted = export.approval.granted_tools.len();
        let approvals_pending = export.approval.pending.len();
        let history_entries = export.history.len();
        let topic_session_count = export.topic_sessions.len();
        let topic_graph_edge_count = export.topic_graph_edges.len();
        let neuron_count = export.neurons.len();
        let intuition_feedback_count = export.intuition_feedback.len();
        let model_router_feedback_count = export.model_router_feedback.len();
        let exported_at_unix_ms = export.exported_at_unix_ms;
        let title = export.session.title.clone();
        let model = export.model.clone();
        let archived = export.session.archived_at_unix_ms.is_some();
        let export_json = serde_json::to_string_pretty(&export)
            .map_err(|err| HeptaError(format!("failed to serialize session export: {}", err)))?;
        let export_path = PathBuf::from(path);
        if let Some(parent) = export_path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent).map_err(|err| {
                    HeptaError(format!(
                        "failed to create export directory {}: {}",
                        parent.display(),
                        err
                    ))
                })?;
            }
        }
        fs::write(&export_path, export_json).map_err(|err| {
            HeptaError(format!(
                "failed to write session export {}: {}",
                export_path.display(),
                err
            ))
        })?;
        let report = SessionExportReport {
            session_id: session_id.to_string(),
            export_path: export_path.display().to_string(),
            exported_at_unix_ms,
            title,
            model,
            archived,
            approvals_granted,
            approvals_pending,
            history_entries,
            topic_session_count,
            topic_graph_edge_count,
            neuron_count,
            intuition_feedback_count,
            model_router_feedback_count,
        };
        self.emit_event(
            EventKind::SessionExported,
            Some(SessionId(session_id.to_string())),
            None,
            format!("exported session to {}", report.export_path),
        )?;
        Ok(report)
    }

    pub fn import_session(&self, path: &str) -> Result<SessionImportReport, HeptaError> {
        let path = path.trim();
        if path.is_empty() {
            return Err(HeptaError("import path must not be empty".into()));
        }

        let import_path = PathBuf::from(path);
        let import_json = fs::read_to_string(&import_path).map_err(|err| {
            HeptaError(format!(
                "failed to read session import {}: {}",
                import_path.display(),
                err
            ))
        })?;
        let export: SessionExport = serde_json::from_str(&import_json).map_err(|err| {
            HeptaError(format!(
                "failed to parse session import {}: {}",
                import_path.display(),
                err
            ))
        })?;
        if export.version != 1 {
            return Err(HeptaError(format!(
                "unsupported session export version: {}",
                export.version
            )));
        }

        let session_id = export.session.session_id.0.clone();
        let imported_title = export.session.title.clone();
        let imported_model = export.model.clone();
        let imported_archived = export.session.archived_at_unix_ms.is_some();
        let approvals_granted = export.approval.granted_tools.len();
        let approvals_pending = export.approval.pending.len();
        let history_entries = export.history.len();
        let topic_session_count = export.topic_sessions.len();
        let topic_graph_edge_count = export.topic_graph_edges.len();
        let neuron_count = export.neurons.len();
        let intuition_feedback_count = export.intuition_feedback.len();
        let model_router_feedback_count = export.model_router_feedback.len();
        self.apply_session_export(export)?;
        let report = SessionImportReport {
            session_id,
            import_path: import_path.display().to_string(),
            imported_title,
            imported_model,
            imported_archived,
            approvals_granted,
            approvals_pending,
            history_entries,
            topic_session_count,
            topic_graph_edge_count,
            neuron_count,
            intuition_feedback_count,
            model_router_feedback_count,
        };
        self.emit_event(
            EventKind::SessionImported,
            Some(SessionId(report.session_id.clone())),
            None,
            format!("imported session from {}", report.import_path),
        )?;
        Ok(report)
    }

    pub fn backup_index(&self, target_path: Option<&str>) -> Result<BackupIndexReport, HeptaError> {
        let workspace_root = self.workspace_root()?;
        let backup_root = workspace_root.join("artifacts/backups/write_file");
        let filter_target_path = target_path
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(|value| {
                resolve_path_within_root(&workspace_root, Path::new(value))
                    .display()
                    .to_string()
            });

        let mut files = Vec::new();
        collect_files_recursive(&backup_root, &mut files)?;
        let mut backups = files
            .into_iter()
            .filter_map(|path| parse_backup_entry(&workspace_root, &backup_root, &path).transpose())
            .collect::<Result<Vec<_>, HeptaError>>()?;

        if let Some(filter_target_path) = filter_target_path.as_deref() {
            backups.retain(|entry| entry.target_path == filter_target_path);
        }
        backups.sort_by(|left, right| right.created_at_unix_ms.cmp(&left.created_at_unix_ms));

        Ok(BackupIndexReport {
            backup_root: backup_root.display().to_string(),
            filter_target_path,
            backups,
        })
    }

    pub fn restore_backup(&self, backup_ref: &str) -> Result<RestoreBackupReport, HeptaError> {
        let backup_ref = backup_ref.trim();
        if backup_ref.is_empty() {
            return Err(HeptaError("backup reference must not be empty".into()));
        }
        let workspace_root = self.workspace_root()?;
        let backup_root = workspace_root.join("artifacts/backups/write_file");
        let backup_path = resolve_backup_reference(&backup_root, backup_ref)?;
        let backup =
            parse_backup_entry(&workspace_root, &backup_root, &backup_path)?.ok_or_else(|| {
                HeptaError(format!(
                    "backup not found under {}: {}",
                    backup_root.display(),
                    backup_ref
                ))
            })?;
        let active_session_id = self.active_session_id()?;
        self.ensure_write_path_scope_allows_path_string(
            &SessionId(active_session_id.clone()),
            "restore_backup",
            &backup.target_path,
        )?;
        self.ensure_write_target_unlocked(
            &active_session_id,
            &backup.target_path,
            "restore_backup",
        )?;

        let target_path = PathBuf::from(&backup.target_path);
        let target_existed_before_restore = target_path.exists();
        let previous_target_backup_path = if target_existed_before_restore {
            let existing = fs::read(&target_path).map_err(|err| {
                HeptaError(format!(
                    "failed to read current target {} before restore: {}",
                    target_path.display(),
                    err
                ))
            })?;
            let planned_backup = preview_backup_path(&workspace_root, &target_path)
                .map_err(|err| HeptaError(err.0))?;
            if let Some(parent) = planned_backup.parent() {
                fs::create_dir_all(parent).map_err(|err| {
                    HeptaError(format!(
                        "failed to create restore-backup parent {}: {}",
                        parent.display(),
                        err
                    ))
                })?;
            }
            fs::write(&planned_backup, existing).map_err(|err| {
                HeptaError(format!(
                    "failed to write restore backup {}: {}",
                    planned_backup.display(),
                    err
                ))
            })?;
            Some(planned_backup.display().to_string())
        } else {
            None
        };

        let backup_bytes = fs::read(&backup_path).map_err(|err| {
            HeptaError(format!(
                "failed to read backup {}: {}",
                backup_path.display(),
                err
            ))
        })?;
        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent).map_err(|err| {
                HeptaError(format!(
                    "failed to create restore target parent {}: {}",
                    parent.display(),
                    err
                ))
            })?;
        }
        fs::write(&target_path, &backup_bytes).map_err(|err| {
            HeptaError(format!(
                "failed to restore {} from {}: {}",
                target_path.display(),
                backup_path.display(),
                err
            ))
        })?;

        let active_session = SessionId(active_session_id.clone());
        let transaction_id = self.record_restore_backup_transaction(
            &active_session,
            &backup.target_path,
            target_existed_before_restore,
            backup_bytes.len() as u64,
            previous_target_backup_path.clone(),
            backup.backup_path.clone(),
        )?;

        let report = RestoreBackupReport {
            transaction_id,
            backup_id: backup.id.clone(),
            backup_path: backup.backup_path.clone(),
            restored_target_path: backup.target_path.clone(),
            restored_bytes: backup_bytes.len() as u64,
            target_existed_before_restore,
            previous_target_backup_path,
        };
        self.emit_event(
            EventKind::BackupRestored,
            Some(active_session),
            None,
            format!(
                "restored backup {} to {}",
                report.backup_id, report.restored_target_path
            ),
        )?;
        Ok(report)
    }

    pub fn preview_prune_backups(
        &self,
        target_path: Option<&str>,
        keep_latest_per_target: usize,
        max_age_ms: Option<u64>,
    ) -> Result<BackupPruneReport, HeptaError> {
        self.plan_backup_prune(target_path, keep_latest_per_target, max_age_ms, false)
    }

    pub fn prune_backups(
        &self,
        target_path: Option<&str>,
        keep_latest_per_target: usize,
        max_age_ms: Option<u64>,
    ) -> Result<BackupPruneReport, HeptaError> {
        let report =
            self.plan_backup_prune(target_path, keep_latest_per_target, max_age_ms, true)?;
        if report.deleted_count > 0 {
            self.emit_event(
                EventKind::BackupsPruned,
                Some(SessionId(self.active_session_id()?)),
                None,
                format!(
                    "pruned {} backups under {}",
                    report.deleted_count, report.backup_root
                ),
            )?;
        }
        Ok(report)
    }

    pub fn write_transactions(
        &self,
        target_path: Option<&str>,
    ) -> Result<WriteTransactionIndexReport, HeptaError> {
        let workspace_root = self.workspace_root()?;
        let filter_target_path = target_path
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(|value| {
                resolve_path_within_root(&workspace_root, Path::new(value))
                    .display()
                    .to_string()
            });

        let mut transactions = self
            .write_transaction_state
            .lock()
            .map_err(|_| HeptaError("write transaction state mutex poisoned".into()))?
            .clone();
        if let Some(filter_target_path) = filter_target_path.as_deref() {
            transactions.retain(|entry| entry.target_path == filter_target_path);
        }
        transactions.sort_by(|left, right| right.created_at_unix_ms.cmp(&left.created_at_unix_ms));

        Ok(WriteTransactionIndexReport {
            filter_target_path,
            transactions,
        })
    }

    pub fn begin_write_transaction_group(
        &self,
        group_id: Option<&str>,
    ) -> Result<BeginWriteTransactionGroupReport, HeptaError> {
        let session_id = self.active_session_id()?;
        let group_id = self.next_write_transaction_group_id(group_id)?;
        let opened_at_unix_ms = current_unix_ms()?;
        {
            let mut guard = self
                .write_transaction_group_state
                .lock()
                .map_err(|_| HeptaError("write transaction group state mutex poisoned".into()))?;
            if guard
                .active_bindings
                .iter()
                .any(|binding| binding.session_id == session_id)
            {
                return Err(HeptaError(format!(
                    "session {} already has an active write transaction group",
                    session_id
                )));
            }
            guard.groups.push(WriteTransactionGroup {
                group_id: group_id.clone(),
                session_id: session_id.clone(),
                opened_at_unix_ms,
                closed_at_unix_ms: None,
                transaction_ids: Vec::new(),
            });
            guard
                .active_bindings
                .push(SessionWriteTransactionGroupBinding {
                    session_id: session_id.clone(),
                    active_group_id: group_id.clone(),
                });
        }
        self.emit_event(
            EventKind::WriteTransactionGroupOpened,
            Some(SessionId(session_id.clone())),
            None,
            format!("opened write transaction group {}", group_id),
        )?;
        Ok(BeginWriteTransactionGroupReport {
            session_id,
            group_id,
            opened_at_unix_ms,
        })
    }

    pub fn end_write_transaction_group(
        &self,
    ) -> Result<EndWriteTransactionGroupReport, HeptaError> {
        let session_id = self.active_session_id()?;
        let closed_at_unix_ms = current_unix_ms()?;
        let (group_id, transaction_count) = {
            let mut guard = self
                .write_transaction_group_state
                .lock()
                .map_err(|_| HeptaError("write transaction group state mutex poisoned".into()))?;
            let binding_index = guard
                .active_bindings
                .iter()
                .position(|binding| binding.session_id == session_id)
                .ok_or_else(|| {
                    HeptaError(format!(
                        "session {} has no active write transaction group",
                        session_id
                    ))
                })?;
            let group_id = guard.active_bindings.remove(binding_index).active_group_id;
            let group = guard
                .groups
                .iter_mut()
                .find(|group| group.group_id == group_id)
                .ok_or_else(|| {
                    HeptaError(format!("unknown write transaction group: {}", group_id))
                })?;
            group.closed_at_unix_ms = Some(closed_at_unix_ms);
            (group_id, group.transaction_ids.len())
        };
        self.emit_event(
            EventKind::WriteTransactionGroupClosed,
            Some(SessionId(session_id.clone())),
            None,
            format!("closed write transaction group {}", group_id),
        )?;
        Ok(EndWriteTransactionGroupReport {
            session_id,
            group_id,
            closed_at_unix_ms,
            transaction_count,
        })
    }

    pub fn write_transaction_groups(&self) -> Result<WriteTransactionGroupIndexReport, HeptaError> {
        let session_id = self.active_session_id()?;
        let guard = self
            .write_transaction_group_state
            .lock()
            .map_err(|_| HeptaError("write transaction group state mutex poisoned".into()))?;
        let active_group_id = guard
            .active_bindings
            .iter()
            .find(|binding| binding.session_id == session_id)
            .map(|binding| binding.active_group_id.clone());
        let mut groups = guard
            .groups
            .iter()
            .filter(|group| group.session_id == session_id)
            .cloned()
            .collect::<Vec<_>>();
        groups.sort_by(|left, right| right.opened_at_unix_ms.cmp(&left.opened_at_unix_ms));
        Ok(WriteTransactionGroupIndexReport {
            session_id,
            active_group_id,
            groups,
        })
    }

    pub fn write_locks(&self) -> Result<WriteLockReport, HeptaError> {
        self.prune_stale_write_locks_internal(false)?;
        let guard = self
            .write_lock_state
            .lock()
            .map_err(|_| HeptaError("write lock state mutex poisoned".into()))?;
        let target_locks = guard
            .target_locks
            .iter()
            .cloned()
            .map(|lock| {
                let attempt = if let Some(attempt_id) = lock.rollback_attempt_id.as_deref() {
                    self.live_rollback_group_attempt_by_id(attempt_id)?
                } else {
                    None
                };
                Ok(rollback_locks::build_write_target_lock_report(
                    lock,
                    attempt.as_ref(),
                ))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let group_locks = guard
            .group_locks
            .iter()
            .cloned()
            .map(|lock| {
                let attempt = if let Some(attempt_id) = lock.rollback_attempt_id.as_deref() {
                    self.live_rollback_group_attempt_by_id(attempt_id)?
                } else {
                    None
                };
                Ok(rollback_locks::build_write_group_lock_report(
                    lock,
                    attempt.as_ref(),
                ))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rollback_locks::build_write_lock_report(
            WRITE_LOCK_REPORT_SCHEMA_VERSION,
            target_locks,
            group_locks,
        ))
    }

    pub fn prune_stale_write_locks(&self) -> Result<WriteLockPruneReport, HeptaError> {
        self.prune_stale_write_locks_internal(true)
    }

    pub fn rollback_write_plan(&self, group_id: &str) -> Result<RollbackPlanReport, HeptaError> {
        let session_id = self.active_session_id()?;
        let group_id = group_id.trim();
        if group_id.is_empty() {
            return Err(HeptaError("group id must not be empty".into()));
        }
        let (group, active) = self.find_write_transaction_group(&session_id, group_id)?;
        let transactions = self
            .write_transaction_state
            .lock()
            .map_err(|_| HeptaError("write transaction state mutex poisoned".into()))?
            .clone();
        let mut steps = Vec::new();
        for transaction_id in group.transaction_ids.iter().rev() {
            let maybe_entry = transactions
                .iter()
                .find(|entry| entry.transaction_id == *transaction_id)
                .cloned();
            let step = if let Some(entry) = maybe_entry {
                let already_rolled_back = entry.rolled_back_at_unix_ms.is_some();
                let blocking_reason = if already_rolled_back {
                    None
                } else {
                    match entry.rollback_strategy.as_str() {
                        "restore_checkpoint" => match entry.rollback_checkpoint_path.as_deref() {
                            Some(path) if PathBuf::from(path).exists() => None,
                            Some(path) => Some(format!("rollback checkpoint missing: {}", path)),
                            None => Some("rollback checkpoint missing".into()),
                        },
                        "delete_target" => None,
                        other => Some(format!("unsupported rollback strategy {}", other)),
                    }
                };
                RollbackPlanStep {
                    transaction_id: entry.transaction_id,
                    target_path: entry.target_path,
                    rollback_strategy: entry.rollback_strategy,
                    rollback_checkpoint_path: entry.rollback_checkpoint_path,
                    ready: blocking_reason.is_none(),
                    already_rolled_back,
                    blocking_reason,
                }
            } else {
                RollbackPlanStep {
                    transaction_id: transaction_id.clone(),
                    target_path: String::new(),
                    rollback_strategy: String::new(),
                    rollback_checkpoint_path: None,
                    ready: false,
                    already_rolled_back: false,
                    blocking_reason: Some("transaction not found".into()),
                }
            };
            steps.push(step);
        }
        let closed = group.closed_at_unix_ms.is_some();
        let executable = closed
            && !active
            && steps
                .iter()
                .all(|step| step.already_rolled_back || step.ready);
        Ok(RollbackPlanReport {
            session_id,
            group_id: group.group_id,
            active,
            closed,
            executable,
            steps,
        })
    }

    pub fn rollback_write_group(&self, group_id: &str) -> Result<RollbackGroupReport, HeptaError> {
        self.rollback_write_group_internal(group_id, None)
    }

    pub fn rollback_group_status(
        &self,
        group_id: &str,
    ) -> Result<RollbackGroupStatusReport, HeptaError> {
        let plan = self.rollback_write_plan(group_id)?;
        let latest_attempt =
            self.latest_rollback_group_attempt(&plan.session_id, &plan.group_id)?;
        let attempt_lifecycle =
            self.rollback_group_attempt_lifecycle(&plan.session_id, &plan.group_id)?;
        let lock_diagnostics = self.rollback_group_lock_diagnostics(
            &plan.session_id,
            &plan.group_id,
            latest_attempt
                .as_ref()
                .map(|attempt| attempt.attempt_id.as_str()),
        )?;
        let group_lock_attempt_id = lock_diagnostics.group_lock_attempt_id.clone();
        let active_attempt_id = attempt_lifecycle.active_attempt_id.clone();
        let (suggested_next_action, resume_command) = match latest_attempt.as_ref() {
            Some(attempt)
                if attempt.status == RollbackGroupAttemptStatus::PartialFailed
                    && lock_diagnostics.latest_attempt_owns_lock_set =>
            {
                (
                    format!("resume partial rollback for group {}", plan.group_id),
                    Some(format!("/resume-rollback-group {}", plan.group_id)),
                )
            }
            Some(attempt)
                if attempt.status == RollbackGroupAttemptStatus::PartialFailed
                    && lock_diagnostics.orphaned_lock_count > 0 =>
            {
                (
                    format!(
                        "prune orphaned locks before resuming group {}",
                        plan.group_id
                    ),
                    Some("/prune-stale-locks".into()),
                )
            }
            Some(attempt) if attempt.status == RollbackGroupAttemptStatus::Completed => (
                if lock_diagnostics.group_locked {
                    format!("prune leftover locks for completed group {}", plan.group_id)
                } else {
                    "group rollback already completed".into()
                },
                if lock_diagnostics.group_locked {
                    Some("/prune-stale-locks".into())
                } else {
                    None
                },
            ),
            _ if lock_diagnostics.group_locked && lock_diagnostics.orphaned_lock_count > 0 => (
                format!("prune orphaned locks for group {}", plan.group_id),
                Some("/prune-stale-locks".into()),
            ),
            _ if plan.executable => (
                format!("run rollback for group {}", plan.group_id),
                Some(format!("/rollback-group {}", plan.group_id)),
            ),
            _ => (
                format!("fix rollback plan blockers for group {}", plan.group_id),
                None,
            ),
        };
        Ok(RollbackGroupStatusReport {
            schema_version: ROLLBACK_GROUP_STATUS_SCHEMA_VERSION,
            group_locked: lock_diagnostics.group_locked,
            group_lock_attempt_id: group_lock_attempt_id.clone(),
            target_lock_count: lock_diagnostics.target_lock_count,
            orphaned_lock_count: lock_diagnostics.orphaned_lock_count,
            latest_attempt_owns_lock_set: lock_diagnostics.latest_attempt_owns_lock_set,
            attempt_count: attempt_lifecycle.attempt_count,
            superseded_attempt_count: attempt_lifecycle.superseded_attempt_count,
            active_attempt_id: active_attempt_id.clone(),
            lock_diagnostics: RollbackGroupLockDiagnosticsReport {
                group_lock_attempt_id,
                target_lock_count: lock_diagnostics.target_lock_count,
                orphaned_lock_count: lock_diagnostics.orphaned_lock_count,
                latest_attempt_owns_lock_set: lock_diagnostics.latest_attempt_owns_lock_set,
            },
            attempt_lifecycle: RollbackGroupAttemptLifecycleReport {
                attempt_count: attempt_lifecycle.attempt_count,
                superseded_attempt_count: attempt_lifecycle.superseded_attempt_count,
                active_attempt_id,
            },
            session_id: plan.session_id,
            group_id: plan.group_id,
            executable_now: plan.executable,
            latest_attempt,
            suggested_next_action,
            resume_command,
        })
    }

    pub fn resume_rollback_write_group(
        &self,
        group_id: &str,
    ) -> Result<RollbackGroupReport, HeptaError> {
        let session_id = self.active_session_id()?;
        let group_id = group_id.trim();
        if group_id.is_empty() {
            return Err(HeptaError("group id must not be empty".into()));
        }
        let latest_attempt = self
            .latest_rollback_group_attempt(&session_id, group_id)?
            .ok_or_else(|| {
                HeptaError(format!("no rollback attempt exists for group {}", group_id))
            })?;
        if latest_attempt.status != RollbackGroupAttemptStatus::PartialFailed {
            return Err(HeptaError(format!(
                "latest rollback attempt for group {} is not partial_failed",
                group_id
            )));
        }
        self.rollback_write_group_internal(group_id, Some(latest_attempt.attempt_id))
    }

    fn rollback_write_group_internal(
        &self,
        group_id: &str,
        resumed_from_attempt_id: Option<String>,
    ) -> Result<RollbackGroupReport, HeptaError> {
        let plan = self.rollback_write_plan(group_id)?;
        if !plan.executable {
            return Err(HeptaError(format!(
                "rollback plan for group {} is not executable",
                plan.group_id
            )));
        }
        let attempt_id = self.next_rollback_group_attempt_id()?;
        let started_at_unix_ms = current_unix_ms()?;
        let locked_target_paths = plan
            .steps
            .iter()
            .map(|step| step.target_path.clone())
            .filter(|target_path| !target_path.is_empty())
            .collect::<Vec<_>>();
        self.acquire_group_rollback_locks(
            &plan.session_id,
            &plan.group_id,
            &attempt_id,
            &locked_target_paths,
        )?;
        let mut executed_transaction_ids = Vec::new();
        let mut skipped_already_rolled_back_ids = Vec::new();
        let mut pending_transaction_ids = plan
            .steps
            .iter()
            .filter(|step| !step.already_rolled_back)
            .map(|step| step.transaction_id.clone())
            .collect::<Vec<_>>();
        let mut target_paths_restored = Vec::new();
        let mut failed_transaction_id = None;
        let mut failure_reason = None;
        if let Some(resumed_from_attempt_id) = resumed_from_attempt_id.as_ref() {
            self.emit_event_with_payload(
                EventKind::WriteGroupRollbackResumed,
                Some(SessionId(plan.session_id.clone())),
                None,
                format!(
                    "resumed rollback for group {} from attempt {}",
                    plan.group_id, resumed_from_attempt_id
                ),
                Some(json!({
                    "schema_version": ROLLBACK_EVENT_PAYLOAD_SCHEMA_VERSION,
                    "group_id": plan.group_id.clone(),
                    "resumed_from_attempt_id": resumed_from_attempt_id.clone(),
                    "resumed_attempt_id": attempt_id.clone(),
                })),
            )?;
        }
        for step in &plan.steps {
            if step.already_rolled_back {
                skipped_already_rolled_back_ids.push(step.transaction_id.clone());
                continue;
            }
            match self.rollback_write_transaction(&step.transaction_id) {
                Ok(report) => {
                    executed_transaction_ids.push(report.transaction_id);
                    pending_transaction_ids.retain(|id| id != &step.transaction_id);
                    target_paths_restored.push(report.target_path);
                }
                Err(err) => {
                    failed_transaction_id = Some(step.transaction_id.clone());
                    failure_reason = Some(err.0);
                    break;
                }
            }
        }
        let finished_at_unix_ms = current_unix_ms()?;
        let status = if failed_transaction_id.is_some() {
            RollbackGroupAttemptStatus::PartialFailed
        } else {
            RollbackGroupAttemptStatus::Completed
        };
        let attempt = RollbackGroupAttempt {
            attempt_id: attempt_id.clone(),
            session_id: plan.session_id.clone(),
            group_id: plan.group_id.clone(),
            started_at_unix_ms,
            finished_at_unix_ms: Some(finished_at_unix_ms),
            status: status.clone(),
            resumed_from_attempt_id: resumed_from_attempt_id.clone(),
            superseded_by_attempt_id: None,
            executed_transaction_ids: executed_transaction_ids.clone(),
            skipped_already_rolled_back_ids: skipped_already_rolled_back_ids.clone(),
            pending_transaction_ids: pending_transaction_ids.clone(),
            failed_transaction_id: failed_transaction_id.clone(),
            failure_reason: failure_reason.clone(),
            target_paths_restored: target_paths_restored.clone(),
        };
        {
            let mut guard = self
                .write_transaction_group_state
                .lock()
                .map_err(|_| HeptaError("write transaction group state mutex poisoned".into()))?;
            if let Some(resumed_from_attempt_id) = resumed_from_attempt_id.as_ref() {
                if let Some(previous_attempt) =
                    guard.rollback_attempts.iter_mut().find(|previous_attempt| {
                        previous_attempt.session_id == plan.session_id
                            && previous_attempt.group_id == plan.group_id
                            && previous_attempt.attempt_id == *resumed_from_attempt_id
                    })
                {
                    previous_attempt.superseded_by_attempt_id = Some(attempt_id.clone());
                }
            }
            guard.rollback_attempts.push(attempt);
        }
        let suggested_next_action = if failed_transaction_id.is_some() {
            format!("inspect rollback status and resume group {}", plan.group_id)
        } else {
            format!("rollback for group {} completed", plan.group_id)
        };
        let resume_command = failed_transaction_id
            .as_ref()
            .map(|_| format!("/resume-rollback-group {}", plan.group_id));
        if let Some(failed_transaction_id) = failed_transaction_id.clone() {
            self.emit_event_with_payload(
                EventKind::WriteGroupRollbackFailed,
                Some(SessionId(plan.session_id.clone())),
                None,
                format!(
                    "rollback group {} failed at {}: {}",
                    plan.group_id,
                    failed_transaction_id,
                    failure_reason
                        .clone()
                        .unwrap_or_else(|| "unknown failure".into())
                ),
                Some(json!({
                    "schema_version": ROLLBACK_EVENT_PAYLOAD_SCHEMA_VERSION,
                    "group_id": plan.group_id.clone(),
                    "attempt_id": attempt_id.clone(),
                    "resumed_from_attempt_id": resumed_from_attempt_id.clone(),
                    "status": "partial_failed",
                    "failed_transaction_id": failed_transaction_id.clone(),
                    "failure_reason": failure_reason.clone(),
                    "executed_transaction_ids": executed_transaction_ids.clone(),
                    "pending_transaction_ids": pending_transaction_ids.clone(),
                    "target_paths_restored": target_paths_restored.clone(),
                })),
            )?;
            return Ok(RollbackGroupReport {
                session_id: plan.session_id,
                group_id: plan.group_id,
                attempt_id,
                status,
                resumed_from_attempt_id,
                executed_transaction_ids,
                skipped_already_rolled_back_ids,
                pending_transaction_ids,
                failed_transaction_id: Some(failed_transaction_id),
                failure_reason,
                target_paths_restored,
                suggested_next_action,
                resume_command,
            });
        }
        self.release_group_rollback_locks(&plan.session_id, &plan.group_id)?;
        self.emit_event_with_payload(
            EventKind::WriteGroupRolledBack,
            Some(SessionId(plan.session_id.clone())),
            None,
            format!("rolled back write transaction group {}", plan.group_id),
            Some(json!({
                "schema_version": ROLLBACK_EVENT_PAYLOAD_SCHEMA_VERSION,
                "group_id": plan.group_id.clone(),
                "attempt_id": attempt_id.clone(),
                "resumed_from_attempt_id": resumed_from_attempt_id.clone(),
                "status": "completed",
                "executed_transaction_ids": executed_transaction_ids.clone(),
                "skipped_already_rolled_back_ids": skipped_already_rolled_back_ids.clone(),
                "pending_transaction_ids": pending_transaction_ids.clone(),
                "target_paths_restored": target_paths_restored.clone(),
            })),
        )?;
        Ok(RollbackGroupReport {
            session_id: plan.session_id,
            group_id: plan.group_id,
            attempt_id,
            status,
            resumed_from_attempt_id,
            executed_transaction_ids,
            skipped_already_rolled_back_ids,
            pending_transaction_ids,
            failed_transaction_id,
            failure_reason,
            target_paths_restored,
            suggested_next_action,
            resume_command,
        })
    }

    pub fn rollback_write_transaction(
        &self,
        transaction_id: &str,
    ) -> Result<RollbackWriteReport, HeptaError> {
        let transaction_id = transaction_id.trim();
        if transaction_id.is_empty() {
            return Err(HeptaError("transaction id must not be empty".into()));
        }

        let entry = {
            let guard = self
                .write_transaction_state
                .lock()
                .map_err(|_| HeptaError("write transaction state mutex poisoned".into()))?;
            guard
                .iter()
                .find(|entry| entry.transaction_id == transaction_id)
                .cloned()
                .ok_or_else(|| {
                    HeptaError(format!("unknown write transaction: {}", transaction_id))
                })?
        };

        if entry.rolled_back_at_unix_ms.is_some() {
            return Err(HeptaError(format!(
                "write transaction {} already rolled back",
                transaction_id
            )));
        }

        {
            let mut guard = self
                .rollback_failure_injection_state
                .lock()
                .map_err(|_| HeptaError("rollback failure injection mutex poisoned".into()))?;
            if let Some(index) = guard
                .iter()
                .position(|candidate| candidate == transaction_id)
            {
                guard.remove(index);
                return Err(HeptaError(format!(
                    "injected rollback failure for transaction {}",
                    transaction_id
                )));
            }
        }

        let active_session_id = self.active_session_id()?;
        self.ensure_write_path_scope_allows_path_string(
            &SessionId(active_session_id.clone()),
            "rollback_write_transaction",
            &entry.target_path,
        )?;

        let workspace_root = self.workspace_root()?;
        let target_path = PathBuf::from(&entry.target_path);
        let previous_target_backup_path = if target_path.exists() {
            let existing = fs::read(&target_path).map_err(|err| {
                HeptaError(format!(
                    "failed to read current target {} before rollback: {}",
                    target_path.display(),
                    err
                ))
            })?;
            let planned_backup = preview_backup_path(&workspace_root, &target_path)
                .map_err(|err| HeptaError(err.0))?;
            if let Some(parent) = planned_backup.parent() {
                fs::create_dir_all(parent).map_err(|err| {
                    HeptaError(format!(
                        "failed to create rollback backup parent {}: {}",
                        parent.display(),
                        err
                    ))
                })?;
            }
            fs::write(&planned_backup, existing).map_err(|err| {
                HeptaError(format!(
                    "failed to write rollback safety backup {}: {}",
                    planned_backup.display(),
                    err
                ))
            })?;
            Some(planned_backup.display().to_string())
        } else {
            None
        };

        match entry.rollback_strategy.as_str() {
            "restore_checkpoint" => {
                let checkpoint_path =
                    entry.rollback_checkpoint_path.as_deref().ok_or_else(|| {
                        HeptaError(format!(
                            "write transaction {} is missing rollback checkpoint",
                            transaction_id
                        ))
                    })?;
                let checkpoint_bytes = fs::read(checkpoint_path).map_err(|err| {
                    HeptaError(format!(
                        "failed to read rollback checkpoint {}: {}",
                        checkpoint_path, err
                    ))
                })?;
                if let Some(parent) = target_path.parent() {
                    fs::create_dir_all(parent).map_err(|err| {
                        HeptaError(format!(
                            "failed to create rollback target parent {}: {}",
                            parent.display(),
                            err
                        ))
                    })?;
                }
                fs::write(&target_path, checkpoint_bytes).map_err(|err| {
                    HeptaError(format!(
                        "failed to restore {} during rollback: {}",
                        target_path.display(),
                        err
                    ))
                })?;
            }
            "delete_target" => {
                if target_path.exists() {
                    fs::remove_file(&target_path).map_err(|err| {
                        HeptaError(format!(
                            "failed to delete {} during rollback: {}",
                            target_path.display(),
                            err
                        ))
                    })?;
                }
            }
            other => {
                return Err(HeptaError(format!(
                    "unsupported rollback strategy {} for transaction {}",
                    other, transaction_id
                )));
            }
        }

        let rolled_back_at_unix_ms = current_unix_ms()?;
        {
            let mut guard = self
                .write_transaction_state
                .lock()
                .map_err(|_| HeptaError("write transaction state mutex poisoned".into()))?;
            let stored = guard
                .iter_mut()
                .find(|candidate| candidate.transaction_id == transaction_id)
                .ok_or_else(|| {
                    HeptaError(format!("unknown write transaction: {}", transaction_id))
                })?;
            stored.rolled_back_at_unix_ms = Some(rolled_back_at_unix_ms);
        }

        let report = RollbackWriteReport {
            transaction_id: entry.transaction_id.clone(),
            target_path: entry.target_path.clone(),
            rollback_strategy: entry.rollback_strategy.clone(),
            rollback_checkpoint_path: entry.rollback_checkpoint_path.clone(),
            previous_target_backup_path,
            target_exists_after_rollback: target_path.exists(),
        };
        self.emit_event(
            EventKind::WriteRolledBack,
            Some(SessionId(active_session_id)),
            None,
            format!(
                "rolled back write transaction {} for {}",
                report.transaction_id, report.target_path
            ),
        )?;
        Ok(report)
    }

    pub fn fork_session(
        &self,
        source_session_id: &str,
        target_session_id: &str,
    ) -> Result<SessionForkReport, HeptaError> {
        let source_session_id = source_session_id.trim();
        let target_session_id = target_session_id.trim();
        if source_session_id.is_empty() || target_session_id.is_empty() {
            return Err(HeptaError(
                "source and target session ids must not be empty".into(),
            ));
        }
        if source_session_id == target_session_id {
            return Err(HeptaError(
                "source and target session ids must differ".into(),
            ));
        }
        match self.existing_session_snapshot_for_id(target_session_id) {
            Ok(_) => {
                return Err(HeptaError(format!(
                    "target session already exists: {}",
                    target_session_id
                )));
            }
            Err(err) if err.0 == format!("session not found: {}", target_session_id) => {}
            Err(err) => return Err(err),
        }

        let mut export = self.session_export(source_session_id)?;
        let now = current_unix_ms()?;
        export.session.session_id = SessionId(target_session_id.to_string());
        export.session.title = format!("{} (fork)", export.session.title);
        export.session.created_at_unix_ms = now;
        export.session.last_active_unix_ms = now;
        export.session.archived_at_unix_ms = None;
        export.history = export
            .history
            .into_iter()
            .map(|mut turn| {
                turn.session_id = target_session_id.to_string();
                turn
            })
            .collect();
        Self::rebind_session_export_topic_state(&mut export, source_session_id, target_session_id);
        let topic_session_count = export.topic_sessions.len();
        let topic_graph_edge_count = export.topic_graph_edges.len();

        self.apply_session_export(export)?;
        let forked = self
            .existing_session_snapshot_for_id(target_session_id)
            .map_err(|err| {
                if err.0 == format!("session not found: {}", target_session_id) {
                    HeptaError(format!(
                        "forked session not found after creation: {}",
                        target_session_id
                    ))
                } else {
                    err
                }
            })?;
        let approvals = self.approval_snapshot_for_session(target_session_id)?;
        let history_entries = self.history(Some(target_session_id), usize::MAX)?.len();
        let report = SessionForkReport {
            source_session_id: source_session_id.to_string(),
            target_session_id: target_session_id.to_string(),
            target_title: forked.title,
            target_model: forked.model,
            target_archived: forked.archived_at_unix_ms.is_some(),
            approvals_granted: approvals.granted_tools.len(),
            approvals_pending: approvals.pending.len(),
            history_entries,
            topic_session_count,
            topic_graph_edge_count,
            active_session_after_fork: self.active_session_id()?,
        };
        self.emit_event(
            EventKind::SessionForked,
            Some(SessionId(report.target_session_id.clone())),
            None,
            format!("forked from {}", report.source_session_id),
        )?;
        Ok(report)
    }

    pub fn merge_session(
        &self,
        source_session_id: &str,
        target_session_id: &str,
        options: MergeOptions,
    ) -> Result<MergeExecutionReport, HeptaError> {
        let source_session_id = source_session_id.trim();
        let target_session_id = target_session_id.trim();
        if source_session_id.is_empty() || target_session_id.is_empty() {
            return Err(HeptaError(
                "source and target session ids must not be empty".into(),
            ));
        }
        if source_session_id == target_session_id {
            return Err(HeptaError(
                "source and target session ids must differ".into(),
            ));
        }

        let source_export = self.session_export(source_session_id)?;
        let history_plan = self.plan_history_merge(target_session_id, &source_export.history)?;
        let target_approvals = self.approval_snapshot_for_session(target_session_id)?;
        let merged_approvals =
            merge_approval_snapshots(target_approvals.clone(), source_export.approval.clone());
        let target_granted_set = target_approvals
            .granted_tools
            .iter()
            .cloned()
            .collect::<HashSet<_>>();
        let target_pending_set = target_approvals
            .pending
            .iter()
            .map(pending_approval_signature)
            .collect::<HashSet<_>>();
        let (target_topic_sessions_before, target_topic_graph_edges_before) =
            self.topic_export_state_for_session(target_session_id)?;
        let source_topic_session_count = source_export.topic_sessions.len();
        let source_topic_graph_edge_count = source_export.topic_graph_edges.len();
        let target_topic_session_count_before = target_topic_sessions_before.len();
        let target_topic_graph_edge_count_before = target_topic_graph_edges_before.len();
        let topic_state_merge_outcome = simulate_topic_state_merge(
            source_session_id,
            target_session_id,
            target_topic_sessions_before,
            target_topic_graph_edges_before,
            source_export.topic_sessions.clone(),
            source_export.topic_graph_edges.clone(),
        );
        let target_topic_session_count_after = topic_state_merge_outcome.topic_sessions.len();
        let target_topic_graph_edge_count_after = topic_state_merge_outcome.topic_graph_edges.len();
        let approvals_added_to_target =
            ordered_unique_difference(merged_approvals.granted_tools.clone(), &target_granted_set);
        let pending_added_to_target = ordered_unique_difference(
            merged_approvals
                .pending
                .iter()
                .map(pending_approval_signature)
                .collect(),
            &target_pending_set,
        );
        let target_record = self
            .existing_session_snapshot_for_id(target_session_id)
            .map_err(|err| {
                if err.0 == format!("session not found: {}", target_session_id) {
                    HeptaError(format!("unknown target session: {}", target_session_id))
                } else {
                    err
                }
            })?;
        let merged_last_user_intent_summary = source_export
            .session
            .last_user_intent_summary
            .clone()
            .or(target_record.last_user_intent_summary.clone());
        let target_title_after = if options.adopt_title {
            source_export.session.title.clone()
        } else {
            target_record.title.clone()
        };
        let target_model_after = if options.adopt_model {
            source_export.model.clone()
        } else {
            self.model_selection_for_session(target_session_id)?.active
        };

        self.upsert_session_record(
            &SessionId(target_record.session_id.clone()),
            if options.adopt_title {
                Some(source_export.session.title.clone())
            } else {
                None
            },
            source_export
                .session
                .last_user_intent_summary
                .or(target_record.last_user_intent_summary.clone()),
            Some(target_record.archived_at_unix_ms),
            true,
        )?;

        if options.adopt_model {
            self.set_session_model(target_session_id, source_export.model.clone())?;
        }

        {
            let mut approval_state = self
                .approval_state
                .lock()
                .map_err(|_| HeptaError("approval state mutex poisoned".into()))?;
            approval_state.remove_session(target_session_id);
            if !merged_approvals.granted_tools.is_empty() || !merged_approvals.pending.is_empty() {
                approval_state.sessions.push(SessionApprovalState {
                    session_id: target_session_id.to_string(),
                    granted_tools: merged_approvals.granted_tools,
                    pending: merged_approvals.pending,
                });
            }
        }
        {
            let mut history_state = self
                .history_state
                .lock()
                .map_err(|_| HeptaError("history state mutex poisoned".into()))?;
            history_state.extend(history_plan.append_turns);
        }
        self.replace_topic_export_state_for_session(
            target_session_id,
            topic_state_merge_outcome.topic_sessions.clone(),
            topic_state_merge_outcome.topic_graph_edges.clone(),
        )?;

        if options.delete_source {
            if self.active_session_id()? == source_session_id {
                let mut guard = self
                    .session_state
                    .lock()
                    .map_err(|_| HeptaError("session state mutex poisoned".into()))?;
                guard.active_session_id = target_session_id.to_string();
            }
            self.delete_session(source_session_id)?;
        }

        let report = MergeExecutionReport {
            source_session_id: source_session_id.to_string(),
            target_session_id: target_session_id.to_string(),
            options,
            target_title_after,
            target_model_after,
            target_archived_after: target_record.archived_at_unix_ms.is_some(),
            source_deleted_after_merge: options.delete_source,
            merged_last_user_intent_summary,
            approvals_added_to_target,
            pending_added_to_target,
            appended_history_entries: history_plan.new_history_entries_to_append.len(),
            skipped_duplicate_history_entries: history_plan.duplicate_history_entries_skipped.len(),
            source_topic_session_count,
            target_topic_session_count_before,
            target_topic_session_count_after,
            source_topic_graph_edge_count,
            target_topic_graph_edge_count_before,
            target_topic_graph_edge_count_after,
            new_history_entries_appended: history_plan.new_history_entries_to_append,
            duplicate_history_entries_skipped: history_plan.duplicate_history_entries_skipped,
        };
        self.emit_event(
            EventKind::SessionMerged,
            Some(SessionId(report.target_session_id.clone())),
            None,
            format!(
                "merged {} into {} (delete_source={})",
                report.source_session_id,
                report.target_session_id,
                report.source_deleted_after_merge
            ),
        )?;
        Ok(report)
    }

    pub fn diff_sessions(
        &self,
        left_session_id: &str,
        right_session_id: &str,
    ) -> Result<SessionDiffReport, HeptaError> {
        let left_session_id = left_session_id.trim();
        let right_session_id = right_session_id.trim();
        if left_session_id.is_empty() || right_session_id.is_empty() {
            return Err(HeptaError(
                "left and right session ids must not be empty".into(),
            ));
        }
        if left_session_id == right_session_id {
            return Err(HeptaError("left and right session ids must differ".into()));
        }

        let sessions = self.sessions()?;
        let left = sessions
            .iter()
            .find(|session| session.session_id == left_session_id)
            .cloned()
            .ok_or_else(|| HeptaError(format!("unknown session: {}", left_session_id)))?;
        let right = sessions
            .iter()
            .find(|session| session.session_id == right_session_id)
            .cloned()
            .ok_or_else(|| HeptaError(format!("unknown session: {}", right_session_id)))?;

        let left_approvals = self.approval_snapshot_for_session(left_session_id)?;
        let right_approvals = self.approval_snapshot_for_session(right_session_id)?;

        let (left_history, right_history) = {
            let guard = self
                .history_state
                .lock()
                .map_err(|_| HeptaError("history state mutex poisoned".into()))?;
            let left_history = guard
                .iter()
                .filter(|turn| turn.session_id == left_session_id)
                .cloned()
                .collect::<Vec<_>>();
            let right_history = guard
                .iter()
                .filter(|turn| turn.session_id == right_session_id)
                .cloned()
                .collect::<Vec<_>>();
            (left_history, right_history)
        };

        let left_granted = left_approvals.granted_tools;
        let right_granted = right_approvals.granted_tools;
        let right_granted_set = right_granted.iter().cloned().collect::<HashSet<_>>();
        let left_granted_set = left_granted.iter().cloned().collect::<HashSet<_>>();

        let left_pending = left_approvals
            .pending
            .into_iter()
            .map(|item| pending_approval_signature(&item))
            .collect::<Vec<_>>();
        let right_pending = right_approvals
            .pending
            .into_iter()
            .map(|item| pending_approval_signature(&item))
            .collect::<Vec<_>>();
        let right_pending_set = right_pending.iter().cloned().collect::<HashSet<_>>();
        let left_pending_set = left_pending.iter().cloned().collect::<HashSet<_>>();

        let left_history_signatures = left_history
            .iter()
            .map(turn_record_signature)
            .collect::<Vec<_>>();
        let right_history_signatures = right_history
            .iter()
            .map(turn_record_signature)
            .collect::<Vec<_>>();
        let right_history_set = right_history_signatures
            .iter()
            .cloned()
            .collect::<HashSet<_>>();
        let left_history_set = left_history_signatures
            .iter()
            .cloned()
            .collect::<HashSet<_>>();
        let shared_history_count = left_history_set.intersection(&right_history_set).count();

        Ok(SessionDiffReport {
            left_session_id: left.session_id,
            right_session_id: right.session_id,
            left_title: left.title,
            right_title: right.title,
            left_model: left.model,
            right_model: right.model,
            left_archived: left.archived_at_unix_ms.is_some(),
            right_archived: right.archived_at_unix_ms.is_some(),
            left_last_user_intent_summary: left.last_user_intent_summary,
            right_last_user_intent_summary: right.last_user_intent_summary,
            left_history_count: left_history.len(),
            right_history_count: right_history.len(),
            shared_history_count,
            approvals_only_left: ordered_unique_difference(left_granted, &right_granted_set),
            approvals_only_right: ordered_unique_difference(right_granted, &left_granted_set),
            pending_only_left: ordered_unique_difference(left_pending, &right_pending_set),
            pending_only_right: ordered_unique_difference(right_pending, &left_pending_set),
            history_only_left: ordered_unique_difference(
                left_history_signatures,
                &right_history_set,
            ),
            history_only_right: ordered_unique_difference(
                right_history_signatures,
                &left_history_set,
            ),
        })
    }

    pub fn preview_merge_session(
        &self,
        source_session_id: &str,
        target_session_id: &str,
        options: MergeOptions,
    ) -> Result<MergePreviewReport, HeptaError> {
        let source_session_id = source_session_id.trim();
        let target_session_id = target_session_id.trim();
        if source_session_id.is_empty() || target_session_id.is_empty() {
            return Err(HeptaError(
                "source and target session ids must not be empty".into(),
            ));
        }
        if source_session_id == target_session_id {
            return Err(HeptaError(
                "source and target session ids must differ".into(),
            ));
        }

        let source_export = self.session_export(source_session_id)?;
        let target = self
            .existing_session_snapshot_for_id(target_session_id)
            .map_err(|err| {
                if err.0 == format!("session not found: {}", target_session_id) {
                    HeptaError(format!("unknown target session: {}", target_session_id))
                } else {
                    err
                }
            })?;

        let target_approvals = self.approval_snapshot_for_session(target_session_id)?;
        let merged_approvals =
            merge_approval_snapshots(target_approvals.clone(), source_export.approval.clone());
        let history_plan = self.plan_history_merge(target_session_id, &source_export.history)?;
        let target_history_count = self.history(Some(target_session_id), usize::MAX)?.len();
        let (target_topic_sessions_before, target_topic_graph_edges_before) =
            self.topic_export_state_for_session(target_session_id)?;
        let source_topic_session_count = source_export.topic_sessions.len();
        let source_topic_graph_edge_count = source_export.topic_graph_edges.len();
        let target_topic_session_count_before = target_topic_sessions_before.len();
        let target_topic_graph_edge_count_before = target_topic_graph_edges_before.len();
        let topic_state_merge_outcome = simulate_topic_state_merge(
            source_session_id,
            target_session_id,
            target_topic_sessions_before,
            target_topic_graph_edges_before,
            source_export.topic_sessions.clone(),
            source_export.topic_graph_edges.clone(),
        );

        let target_granted_set = target_approvals
            .granted_tools
            .iter()
            .cloned()
            .collect::<HashSet<_>>();
        let target_pending_set = target_approvals
            .pending
            .iter()
            .map(pending_approval_signature)
            .collect::<HashSet<_>>();

        let approvals_added_to_target =
            ordered_unique_difference(merged_approvals.granted_tools, &target_granted_set);
        let pending_added_to_target = ordered_unique_difference(
            merged_approvals
                .pending
                .into_iter()
                .map(|item| pending_approval_signature(&item))
                .collect(),
            &target_pending_set,
        );

        let merged_last_user_intent_summary = source_export
            .session
            .last_user_intent_summary
            .clone()
            .or(target.last_user_intent_summary.clone());

        Ok(MergePreviewReport {
            source_session_id: source_session_id.to_string(),
            target_session_id: target_session_id.to_string(),
            options,
            source_title: source_export.session.title.clone(),
            source_model: source_export.model.clone(),
            target_title_before: target.title.clone(),
            target_title_after: if options.adopt_title {
                source_export.session.title.clone()
            } else {
                target.title.clone()
            },
            target_model_before: target.model.clone(),
            target_model_after: if options.adopt_model {
                source_export.model.clone()
            } else {
                target.model.clone()
            },
            target_archived_before: target.archived_at_unix_ms.is_some(),
            target_archived_after: target.archived_at_unix_ms.is_some(),
            source_deleted_after_merge: options.delete_source,
            target_last_user_intent_summary_before: target.last_user_intent_summary.clone(),
            source_last_user_intent_summary: source_export.session.last_user_intent_summary,
            merged_last_user_intent_summary,
            source_history_count: source_export.history.len(),
            target_history_count,
            history_entries_to_append: history_plan.new_history_entries_to_append.len(),
            history_entries_skipped_as_duplicates: history_plan
                .duplicate_history_entries_skipped
                .len(),
            source_topic_session_count,
            target_topic_session_count_before,
            target_topic_session_count_after: topic_state_merge_outcome.topic_sessions.len(),
            source_topic_graph_edge_count,
            target_topic_graph_edge_count_before,
            target_topic_graph_edge_count_after: topic_state_merge_outcome.topic_graph_edges.len(),
            approvals_added_to_target,
            pending_added_to_target,
            duplicate_history_entries_skipped: history_plan.duplicate_history_entries_skipped,
            new_history_entries_to_append: history_plan.new_history_entries_to_append,
        })
    }

    pub fn save_snapshot(&self, path: &str) -> Result<String, HeptaError> {
        self.write_snapshot(path, true)
    }

    pub fn persist_snapshot(&self, path: &str) -> Result<String, HeptaError> {
        self.write_snapshot(path, false)
    }

    fn write_snapshot(&self, path: &str, emit_audit_event: bool) -> Result<String, HeptaError> {
        let snapshot = self.runtime_snapshot()?;
        let snapshot_json = serde_json::to_string_pretty(&snapshot)
            .map_err(|err| HeptaError(format!("failed to serialize runtime snapshot: {}", err)))?;
        let snapshot_path = PathBuf::from(path);
        if let Some(parent) = snapshot_path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent).map_err(|err| {
                    HeptaError(format!(
                        "failed to create snapshot directory {}: {}",
                        parent.display(),
                        err
                    ))
                })?;
            }
        }
        fs::write(&snapshot_path, snapshot_json).map_err(|err| {
            HeptaError(format!(
                "failed to write runtime snapshot {}: {}",
                snapshot_path.display(),
                err
            ))
        })?;
        if emit_audit_event {
            self.emit_event(
                EventKind::SnapshotSaved,
                None,
                None,
                format!("saved runtime snapshot to {}", snapshot_path.display()),
            )?;
        }
        Ok(format!(
            "saved runtime snapshot to {}",
            snapshot_path.display()
        ))
    }

    pub fn load_snapshot(&self, path: &str) -> Result<String, HeptaError> {
        let snapshot_path = PathBuf::from(path);
        let snapshot_json = fs::read_to_string(&snapshot_path).map_err(|err| {
            HeptaError(format!(
                "failed to read runtime snapshot {}: {}",
                snapshot_path.display(),
                err
            ))
        })?;
        let snapshot: RuntimeSnapshot = serde_json::from_str(&snapshot_json).map_err(|err| {
            HeptaError(format!(
                "failed to parse runtime snapshot {}: {}",
                snapshot_path.display(),
                err
            ))
        })?;
        if snapshot.version != 1 {
            return Err(HeptaError(format!(
                "unsupported runtime snapshot version: {}",
                snapshot.version
            )));
        }
        self.apply_runtime_snapshot(snapshot)?;
        self.emit_event(
            EventKind::SnapshotLoaded,
            None,
            None,
            format!("loaded runtime snapshot from {}", snapshot_path.display()),
        )?;
        Ok(format!(
            "loaded runtime snapshot from {}",
            snapshot_path.display()
        ))
    }

    pub async fn policy_summary(&self) -> Result<Vec<String>, HeptaError> {
        let report = self.policy_report().await?;
        let mut lines = vec!["Policy summary:".to_string()];
        lines.push(format!("- active session: {}", report.active_session_id));
        lines.push(format!(
            "- active model: {}/{}",
            report.active_model.provider, report.active_model.model
        ));
        lines.push(format!("- default rules: {}", report.default_rules.len()));
        lines.push(format!("- custom rules: {}", report.custom_rules.len()));
        lines.push(format!("- granted approvals: {}", report.granted_approvals));
        lines.push(format!("- pending approvals: {}", report.pending_approvals));
        lines.push("- effective decisions:".into());
        for item in report.effective_tool_decisions {
            lines.push(format!(
                "  - {}: {} via {} ({})",
                item.tool_name,
                format_approval_requirement(item.requirement),
                item.matched_rule_id.unwrap_or_else(|| "<none>".into()),
                item.reason
            ));
        }
        Ok(lines)
    }

    pub fn sessions(&self) -> Result<Vec<SessionSnapshot>, HeptaError> {
        let active_session_id = self.active_session_id()?;
        self.ensure_session_record_sync(&active_session_id)?;
        let mut sessions = self
            .memory
            .list_sessions()
            .map_err(|err| HeptaError(err.0))?;
        sessions.reverse();
        let mut deduped = Vec::new();
        for session in sessions {
            let session_id = session.session_id.0.clone();
            if deduped
                .iter()
                .any(|item: &SessionSnapshot| item.session_id == session_id)
            {
                continue;
            }
            let (topic_session_count, topic_graph_edge_count) =
                self.topic_state_counts_for_session(&session_id)?;
            deduped.push(SessionSnapshot {
                is_active: session_id == active_session_id,
                session_id: session_id.clone(),
                agent_id: session.agent_id.0,
                title: session.title,
                model: self.model_selection_for_session(&session_id)?.active,
                created_at_unix_ms: session.created_at_unix_ms,
                last_active_unix_ms: session.last_active_unix_ms,
                last_user_intent_summary: session.last_user_intent_summary,
                archived_at_unix_ms: session.archived_at_unix_ms,
                topic_session_count,
                topic_graph_edge_count,
            });
        }
        deduped.reverse();
        Ok(deduped)
    }

    fn session_snapshot_for_id(&self, session_id: &str) -> Result<SessionSnapshot, HeptaError> {
        self.ensure_session_record_sync(session_id)?;
        self.existing_session_snapshot_for_id(session_id)
    }

    fn existing_session_snapshot_for_id(
        &self,
        session_id: &str,
    ) -> Result<SessionSnapshot, HeptaError> {
        self.sessions()?
            .into_iter()
            .find(|session| session.session_id == session_id)
            .ok_or_else(|| HeptaError(format!("session not found: {}", session_id)))
    }

    fn topic_state_counts_for_session(
        &self,
        session_id: &str,
    ) -> Result<(usize, usize), HeptaError> {
        let (topic_sessions, topic_graph_edges) =
            self.topic_export_state_for_session(session_id)?;
        Ok((topic_sessions.len(), topic_graph_edges.len()))
    }
}

fn format_risk_tier(risk_tier: RiskTier) -> &'static str {
    match risk_tier {
        RiskTier::Low => "low",
        RiskTier::Medium => "medium",
        RiskTier::High => "high",
    }
}

fn format_approval_requirement(requirement: ApprovalRequirement) -> &'static str {
    match requirement {
        ApprovalRequirement::None => "none",
        ApprovalRequirement::Ask => "ask",
        ApprovalRequirement::Deny => "deny",
    }
}

fn format_execution_profile(profile: ExecutionProfile) -> &'static str {
    match profile {
        ExecutionProfile::FullAccess => "full_access",
        ExecutionProfile::ReadOnlyTools => "read_only_tools",
        ExecutionProfile::NoTools => "no_tools",
    }
}

fn format_filesystem_scope(scope: FilesystemScope) -> &'static str {
    match scope {
        FilesystemScope::WorkspaceOnly => "workspace_only",
        FilesystemScope::AnyPath => "any_path",
    }
}

fn format_write_path_scope(scope: WritePathScope) -> &'static str {
    match scope {
        WritePathScope::ArtifactsOnly => "artifacts_only",
        WritePathScope::WorkspaceOnly => "workspace_only",
        WritePathScope::AnyPath => "any_path",
    }
}
