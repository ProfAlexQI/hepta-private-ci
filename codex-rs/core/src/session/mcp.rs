use super::*;
use codex_mcp::ElicitationReviewRequest;
use codex_mcp::ElicitationReviewer;
use codex_mcp::ElicitationReviewerHandle;
use codex_protocol::config_types::ApprovalsReviewer;
use codex_protocol::mcp_approval_meta::APPROVAL_KIND_KEY as MCP_ELICITATION_APPROVAL_KIND_KEY;
use codex_protocol::mcp_approval_meta::APPROVAL_KIND_MCP_TOOL_CALL as MCP_ELICITATION_APPROVAL_KIND_MCP_TOOL_CALL;
use codex_protocol::mcp_approval_meta::APPROVALS_REVIEWER_KEY as MCP_ELICITATION_APPROVALS_REVIEWER_KEY;
use codex_protocol::mcp_approval_meta::CONNECTOR_DESCRIPTION_KEY as MCP_ELICITATION_CONNECTOR_DESCRIPTION_KEY;
use codex_protocol::mcp_approval_meta::CONNECTOR_ID_KEY as MCP_ELICITATION_CONNECTOR_ID_KEY;
use codex_protocol::mcp_approval_meta::CONNECTOR_NAME_KEY as MCP_ELICITATION_CONNECTOR_NAME_KEY;
use codex_protocol::mcp_approval_meta::REQUEST_TYPE_APPROVAL_REQUEST as MCP_ELICITATION_REQUEST_TYPE_APPROVAL_REQUEST;
use codex_protocol::mcp_approval_meta::REQUEST_TYPE_KEY as MCP_ELICITATION_REQUEST_TYPE_KEY;
use codex_protocol::mcp_approval_meta::TOOL_DESCRIPTION_KEY as MCP_ELICITATION_TOOL_DESCRIPTION_KEY;
use codex_protocol::mcp_approval_meta::TOOL_NAME_KEY as MCP_ELICITATION_TOOL_NAME_KEY;
use codex_protocol::mcp_approval_meta::TOOL_PARAMS_KEY as MCP_ELICITATION_TOOL_PARAMS_KEY;
use codex_protocol::mcp_approval_meta::TOOL_TITLE_KEY as MCP_ELICITATION_TOOL_TITLE_KEY;
use rmcp::model::CreateElicitationRequestParams;
use rmcp::model::ElicitationAction;
use rmcp::model::Meta;
use serde_json::Map;

const MCP_ELICITATION_DECLINE_MESSAGE_KEY: &str = "message";

#[derive(Debug, PartialEq)]
enum GuardianElicitationReview {
    NotRequested,
    Decline(&'static str),
    ApprovalRequest(Box<crate::guardian::GuardianApprovalRequest>),
}

struct GuardianMcpElicitationReviewer {
    session: std::sync::Weak<Session>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum McpRefreshPublication {
    Published,
    Superseded,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum UnpublishedMcpReplacementError {
    MissingParts,
}

pub(super) struct UnpublishedMcpReplacement {
    manager: Option<McpConnectionManager>,
    cancel_token: Option<CancellationToken>,
}

impl UnpublishedMcpReplacement {
    pub(super) fn new(manager: McpConnectionManager, cancel_token: CancellationToken) -> Self {
        Self {
            manager: Some(manager),
            cancel_token: Some(cancel_token),
        }
    }

    fn manager(&self) -> Result<&McpConnectionManager, UnpublishedMcpReplacementError> {
        self.manager
            .as_ref()
            .ok_or(UnpublishedMcpReplacementError::MissingParts)
    }

    pub(super) fn take(
        &mut self,
    ) -> Result<(McpConnectionManager, CancellationToken), UnpublishedMcpReplacementError> {
        match (self.manager.take(), self.cancel_token.take()) {
            (Some(manager), Some(cancel_token)) => Ok((manager, cancel_token)),
            (manager, cancel_token) => {
                self.manager = manager;
                self.cancel_token = cancel_token;
                Err(UnpublishedMcpReplacementError::MissingParts)
            }
        }
    }
}

impl Drop for UnpublishedMcpReplacement {
    fn drop(&mut self) {
        if let Some(cancel_token) = self.cancel_token.take() {
            cancel_token.cancel();
        }
    }
}

impl GuardianMcpElicitationReviewer {
    fn new(session: &Arc<Session>) -> Self {
        Self {
            session: Arc::downgrade(session),
        }
    }
}

impl ElicitationReviewer for GuardianMcpElicitationReviewer {
    fn review(
        &self,
        request: ElicitationReviewRequest,
    ) -> BoxFuture<'static, anyhow::Result<Option<ElicitationResponse>>> {
        let session = self.session.clone();
        Box::pin(async move {
            let Some(session) = session.upgrade() else {
                return Ok(None);
            };
            review_guardian_mcp_elicitation(session, request).await
        })
    }
}

impl Session {
    pub(crate) fn mcp_elicitation_reviewer(self: &Arc<Self>) -> ElicitationReviewerHandle {
        Arc::new(GuardianMcpElicitationReviewer::new(self))
    }

    #[expect(
        clippy::await_holding_invalid_type,
        reason = "active turn checks and turn state updates must remain atomic"
    )]
    pub async fn request_mcp_server_elicitation(
        &self,
        turn_context: &TurnContext,
        request_id: RequestId,
        params: McpServerElicitationRequestParams,
    ) -> Option<ElicitationResponse> {
        if self
            .services
            .mcp_connection_manager
            .read()
            .await
            .elicitations_auto_deny()
        {
            return Some(ElicitationResponse {
                action: codex_rmcp_client::ElicitationAction::Accept,
                content: Some(serde_json::json!({})),
                meta: None,
            });
        }

        let server_name = params.server_name.clone();
        let request = match params.request {
            McpServerElicitationRequest::Form {
                meta,
                message,
                requested_schema,
            } => {
                let requested_schema = match serde_json::to_value(requested_schema) {
                    Ok(requested_schema) => requested_schema,
                    Err(err) => {
                        warn!(
                            "failed to serialize MCP elicitation schema for server_name: {server_name}, request_id: {request_id}: {err:#}"
                        );
                        return None;
                    }
                };
                codex_protocol::approvals::ElicitationRequest::Form {
                    meta,
                    message,
                    requested_schema,
                }
            }
            McpServerElicitationRequest::Url {
                meta,
                message,
                url,
                elicitation_id,
            } => codex_protocol::approvals::ElicitationRequest::Url {
                meta,
                message,
                url,
                elicitation_id,
            },
        };

        let (tx_response, rx_response) = oneshot::channel();
        let prev_entry = {
            let mut active = self.active_turn.lock().await;
            match active.as_mut() {
                Some(at) => {
                    let mut ts = at.turn_state.lock().await;
                    ts.insert_pending_elicitation(
                        server_name.clone(),
                        request_id.clone(),
                        tx_response,
                    )
                }
                None => None,
            }
        };
        if prev_entry.is_some() {
            warn!(
                "Overwriting existing pending elicitation for server_name: {server_name}, request_id: {request_id}"
            );
        }
        let id = match request_id {
            rmcp::model::NumberOrString::String(value) => {
                codex_protocol::mcp::RequestId::String(value.to_string())
            }
            rmcp::model::NumberOrString::Number(value) => {
                codex_protocol::mcp::RequestId::Integer(value)
            }
        };
        let event = EventMsg::ElicitationRequest(ElicitationRequestEvent {
            turn_id: params.turn_id,
            server_name,
            id,
            request,
        });
        turn_context
            .turn_metadata_state
            .mark_user_input_requested_during_turn();
        self.send_event(turn_context, event).await;
        rx_response.await.ok()
    }

    #[expect(
        clippy::await_holding_invalid_type,
        reason = "active turn checks and manager fallback must stay serialized"
    )]
    pub async fn resolve_elicitation(
        &self,
        server_name: String,
        id: RequestId,
        response: ElicitationResponse,
    ) -> anyhow::Result<()> {
        let entry = {
            let mut active = self.active_turn.lock().await;
            match active.as_mut() {
                Some(at) => {
                    let mut ts = at.turn_state.lock().await;
                    ts.remove_pending_elicitation(&server_name, &id)
                }
                None => None,
            }
        };
        if let Some(tx_response) = entry {
            tx_response
                .send(response)
                .map_err(|e| anyhow::anyhow!("failed to send elicitation response: {e:?}"))?;
            return Ok(());
        }

        self.services
            .mcp_connection_manager
            .read()
            .await
            .resolve_elicitation(server_name, id, response)
            .await
    }

    #[expect(
        clippy::await_holding_invalid_type,
        reason = "MCP resource calls are serialized through the session-owned manager guard"
    )]
    pub async fn list_resources(
        &self,
        server: &str,
        params: Option<PaginatedRequestParams>,
    ) -> anyhow::Result<ListResourcesResult> {
        self.services
            .mcp_connection_manager
            .read()
            .await
            .list_resources(server, params)
            .await
    }

    #[expect(
        clippy::await_holding_invalid_type,
        reason = "MCP resource calls are serialized through the session-owned manager guard"
    )]
    pub async fn list_resource_templates(
        &self,
        server: &str,
        params: Option<PaginatedRequestParams>,
    ) -> anyhow::Result<ListResourceTemplatesResult> {
        self.services
            .mcp_connection_manager
            .read()
            .await
            .list_resource_templates(server, params)
            .await
    }

    #[expect(
        clippy::await_holding_invalid_type,
        reason = "MCP resource calls are serialized through the session-owned manager guard"
    )]
    pub async fn read_resource(
        &self,
        server: &str,
        params: ReadResourceRequestParams,
    ) -> anyhow::Result<ReadResourceResult> {
        self.services
            .mcp_connection_manager
            .read()
            .await
            .read_resource(server, params)
            .await
    }

    #[expect(
        clippy::await_holding_invalid_type,
        reason = "MCP tool calls are serialized through the session-owned manager guard"
    )]
    pub async fn call_tool(
        &self,
        server: &str,
        tool: &str,
        arguments: Option<serde_json::Value>,
        meta: Option<serde_json::Value>,
    ) -> anyhow::Result<CallToolResult> {
        self.services
            .mcp_connection_manager
            .read()
            .await
            .call_tool(server, tool, arguments, meta)
            .await
    }

    async fn refresh_mcp_servers_inner(
        &self,
        turn_context: &TurnContext,
        configured_mcp_servers: Option<HashMap<String, McpServerConfig>>,
        store_mode: OAuthCredentialsStoreMode,
        elicitation_authority: Option<codex_protocol::protocol::McpElicitationAuthority>,
        elicitation_reviewer: Option<ElicitationReviewerHandle>,
        expected_explicit_generation: Option<u64>,
    ) -> McpRefreshPublication {
        let Some(auth_snapshot) =
            crate::state::FrozenMcpAuthSnapshot::capture(self.services.auth_manager.as_ref()).await
        else {
            warn!("failed to capture a consistent MCP auth snapshot");
            return McpRefreshPublication::Failed;
        };
        let auth_changes = self.services.auth_manager.auth_change_receiver();
        let config = self.get_config().await;
        let elicitation_authority = elicitation_authority.unwrap_or_else(|| {
            codex_protocol::protocol::McpElicitationAuthority {
                approval_policy: config.permissions.approval_policy.value(),
                permission_profile: config.permissions.effective_permission_profile(),
                approvals_reviewer: config.approvals_reviewer,
            }
        });
        let approval_policy =
            codex_config::Constrained::allow_any(elicitation_authority.approval_policy);
        let mcp_config = config
            .to_mcp_config(self.services.plugins_manager.as_ref())
            .await;
        let tool_plugin_provenance = codex_mcp::tool_plugin_provenance(&mcp_config);
        let configured_mcp_servers = configured_mcp_servers
            .unwrap_or_else(|| codex_mcp::configured_mcp_servers(&mcp_config));
        let mcp_servers = effective_mcp_servers_from_configured(
            configured_mcp_servers,
            &mcp_config,
            auth_snapshot.auth(),
        );
        let host_owned_codex_apps_enabled =
            host_owned_codex_apps_enabled(&mcp_config, auth_snapshot.auth());
        let mcp_runtime_environment = match turn_context.environments.primary() {
            Some(turn_environment) => McpRuntimeEnvironment::new(
                Arc::clone(&turn_environment.environment),
                turn_environment.cwd.to_path_buf(),
            ),
            None => McpRuntimeEnvironment::new(
                self.services
                    .environment_manager
                    .default_environment()
                    .unwrap_or_else(|| self.services.environment_manager.local_environment()),
                #[allow(deprecated)]
                turn_context.cwd.to_path_buf(),
            ),
        };
        let auth_statuses = compute_auth_statuses(
            mcp_servers.iter(),
            store_mode,
            auth_snapshot.auth(),
            mcp_runtime_environment.clone(),
        )
        .await;
        let (refreshed_manager, cancel_token) = McpConnectionManager::new(
            &mcp_servers,
            store_mode,
            auth_statuses,
            &approval_policy,
            turn_context.sub_id.clone(),
            self.get_tx_event(),
            elicitation_authority.permission_profile.clone(),
            mcp_runtime_environment,
            config.codex_home.to_path_buf(),
            codex_apps_tools_cache_key(auth_snapshot.auth()),
            host_owned_codex_apps_enabled,
            mcp_config.client_elicitation_capability,
            tool_plugin_provenance,
            auth_snapshot.auth(),
            elicitation_reviewer,
        )
        .await;
        let mut replacement = UnpublishedMcpReplacement::new(refreshed_manager, cancel_token);
        {
            let current_manager = self.services.mcp_connection_manager.read().await;
            let Ok(refreshed_manager) = replacement.manager() else {
                warn!("MCP refresh replacement lost its unpublished manager");
                return McpRefreshPublication::Failed;
            };
            refreshed_manager.set_elicitations_auto_deny(current_manager.elicitations_auto_deny());
        }

        #[cfg(test)]
        if expected_explicit_generation.is_some() {
            let gate = self.mcp_server_refresh_test_gate.lock().await.take();
            if let Some((reached, release)) = gate {
                reached.wait().await;
                release.wait().await;
            }
        }

        let mut refresh_state = match expected_explicit_generation {
            Some(generation) => {
                let state = self.mcp_server_refresh_state.lock().await;
                if !state.is_pending(generation) {
                    let Ok((mut stale_manager, cancel_token)) = replacement.take() else {
                        warn!("superseded MCP refresh replacement lost unpublished parts");
                        return McpRefreshPublication::Failed;
                    };
                    cancel_token.cancel();
                    drop(state);
                    stale_manager.shutdown().await;
                    return McpRefreshPublication::Superseded;
                }
                Some(state)
            }
            None => None,
        };

        let Some(latest_auth_snapshot) =
            crate::state::FrozenMcpAuthSnapshot::capture(self.services.auth_manager.as_ref()).await
        else {
            let Ok((mut stale_manager, cancel_token)) = replacement.take() else {
                warn!("MCP replacement lost unpublished parts after auth snapshot failure");
                return McpRefreshPublication::Failed;
            };
            cancel_token.cancel();
            drop(refresh_state);
            stale_manager.shutdown().await;
            return McpRefreshPublication::Failed;
        };
        if auth_changes.has_changed().unwrap_or(true)
            || !auth_snapshot.matches(&latest_auth_snapshot)
        {
            if let (Some(generation), Some(state)) =
                (expected_explicit_generation, refresh_state.as_mut())
            {
                state.supersede_for_auth_change(generation);
            }
            let Ok((mut stale_manager, cancel_token)) = replacement.take() else {
                warn!("auth-superseded MCP replacement lost unpublished parts");
                return McpRefreshPublication::Failed;
            };
            cancel_token.cancel();
            drop(refresh_state);
            stale_manager.shutdown().await;
            return McpRefreshPublication::Superseded;
        }

        let mut manager = self.services.mcp_connection_manager.write().await;
        let mut startup_token = self.services.mcp_startup_cancellation_token.lock().await;
        let Some(auth_publication_guard) = self.services.auth_manager.auth_publication_guard()
        else {
            drop(startup_token);
            drop(manager);
            drop(refresh_state);
            let Ok((mut stale_manager, cancel_token)) = replacement.take() else {
                warn!("MCP replacement lost unpublished parts after auth gate failure");
                return McpRefreshPublication::Failed;
            };
            cancel_token.cancel();
            stale_manager.shutdown().await;
            return McpRefreshPublication::Failed;
        };
        if auth_changes.has_changed().unwrap_or(true)
            || self.services.auth_manager.auth_revision() != auth_snapshot.revision()
        {
            if let (Some(generation), Some(state)) =
                (expected_explicit_generation, refresh_state.as_mut())
            {
                state.supersede_for_auth_change(generation);
            }
            drop(auth_publication_guard);
            drop(startup_token);
            drop(manager);
            drop(refresh_state);
            let Ok((mut stale_manager, cancel_token)) = replacement.take() else {
                warn!("auth-superseded MCP replacement lost unpublished parts");
                return McpRefreshPublication::Failed;
            };
            cancel_token.cancel();
            stale_manager.shutdown().await;
            return McpRefreshPublication::Superseded;
        }
        let Ok((refreshed_manager, cancel_token)) = replacement.take() else {
            warn!("MCP refresh replacement lost unpublished parts before publication");
            return McpRefreshPublication::Failed;
        };
        let mut old_manager = manager.publish(
            refreshed_manager,
            auth_snapshot.binding(),
            elicitation_authority,
        );
        let old_cancel_token = std::mem::replace(&mut *startup_token, cancel_token);
        if old_cancel_token.is_cancelled() {
            startup_token.cancel();
        }
        old_cancel_token.cancel();
        if let (Some(generation), Some(state)) =
            (expected_explicit_generation, refresh_state.as_mut())
        {
            debug_assert!(state.consume_published(generation));
        }
        drop(auth_publication_guard);
        drop(startup_token);
        drop(manager);
        drop(refresh_state);
        old_manager.shutdown().await;
        McpRefreshPublication::Published
    }

    pub(crate) async fn refresh_mcp_servers_if_requested(
        &self,
        turn_context: &TurnContext,
        elicitation_reviewer: Option<ElicitationReviewerHandle>,
    ) {
        let _refresh_owner = self.mcp_server_refresh_lock.lock().await;
        loop {
            let intent = { self.mcp_server_refresh_state.lock().await.pending() };
            let (configured_mcp_servers, store_mode, elicitation_authority, expected_generation) =
                match intent {
                    Some(intent) => {
                        let McpServerRefreshConfig {
                            mcp_servers,
                            mcp_oauth_credentials_store_mode,
                            elicitation_authority,
                        } = intent.config;
                        let mcp_servers = match serde_json::from_value::<
                            HashMap<String, McpServerConfig>,
                        >(mcp_servers)
                        {
                            Ok(servers) => servers,
                            Err(err) => {
                                warn!("failed to parse MCP server refresh config: {err}");
                                return;
                            }
                        };
                        let store_mode = match serde_json::from_value::<OAuthCredentialsStoreMode>(
                            mcp_oauth_credentials_store_mode,
                        ) {
                            Ok(mode) => mode,
                            Err(err) => {
                                warn!("failed to parse MCP OAuth refresh config: {err}");
                                return;
                            }
                        };
                        (
                            Some(mcp_servers),
                            store_mode,
                            elicitation_authority,
                            Some(intent.generation),
                        )
                    }
                    None => {
                        let Some(auth_snapshot) = crate::state::FrozenMcpAuthSnapshot::capture(
                            self.services.auth_manager.as_ref(),
                        )
                        .await
                        else {
                            warn!("failed to capture current auth before MCP refresh");
                            return;
                        };
                        if self
                            .services
                            .mcp_connection_manager
                            .read()
                            .await
                            .auth_matches(&auth_snapshot.binding())
                        {
                            return;
                        }
                        let config = self.get_config().await;
                        (None, config.mcp_oauth_credentials_store_mode, None, None)
                    }
                };

            match self
                .refresh_mcp_servers_inner(
                    turn_context,
                    configured_mcp_servers,
                    store_mode,
                    elicitation_authority,
                    elicitation_reviewer.clone(),
                    expected_generation,
                )
                .await
            {
                McpRefreshPublication::Published => return,
                McpRefreshPublication::Superseded => continue,
                McpRefreshPublication::Failed => return,
            }
        }
    }

    pub(crate) async fn refresh_mcp_servers_now(
        &self,
        turn_context: &TurnContext,
        mcp_servers: HashMap<String, McpServerConfig>,
        store_mode: OAuthCredentialsStoreMode,
        elicitation_reviewer: Option<ElicitationReviewerHandle>,
    ) {
        let _refresh_owner = self.mcp_server_refresh_lock.lock().await;
        loop {
            match self
                .refresh_mcp_servers_inner(
                    turn_context,
                    Some(mcp_servers.clone()),
                    store_mode,
                    None,
                    elicitation_reviewer.clone(),
                    None,
                )
                .await
            {
                McpRefreshPublication::Superseded => continue,
                McpRefreshPublication::Published | McpRefreshPublication::Failed => return,
            }
        }
    }

    #[cfg(test)]
    pub(crate) async fn mcp_startup_cancellation_token(&self) -> CancellationToken {
        self.services
            .mcp_startup_cancellation_token
            .lock()
            .await
            .clone()
    }

    pub(crate) async fn cancel_mcp_startup(&self) {
        self.services
            .mcp_startup_cancellation_token
            .lock()
            .await
            .cancel();
    }
}

async fn review_guardian_mcp_elicitation(
    session: Arc<Session>,
    request: ElicitationReviewRequest,
) -> anyhow::Result<Option<ElicitationResponse>> {
    let Some((turn_context, _cancellation_token)) =
        session.active_turn_context_and_cancellation_token().await
    else {
        return Ok(None);
    };

    let elicitation_authority = session
        .services
        .mcp_connection_manager
        .read()
        .await
        .elicitation_authority()
        .clone();
    let approval_policy = elicitation_authority.approval_policy;
    match approval_policy {
        AskForApproval::Never => {
            if codex_mcp::mcp_permission_prompt_is_auto_approved(
                approval_policy,
                &elicitation_authority.permission_profile,
                codex_mcp::McpPermissionPromptAutoApproveContext::default(),
            ) && matches!(
                &request.elicitation,
                CreateElicitationRequestParams::FormElicitationParams {
                    requested_schema,
                    ..
                } if requested_schema.properties.is_empty()
            ) {
                return Ok(Some(ElicitationResponse {
                    action: ElicitationAction::Accept,
                    content: Some(serde_json::json!({})),
                    meta: None,
                }));
            }
            return Ok(Some(mcp_elicitation_decline_without_message()));
        }
        AskForApproval::Granular(config) if !config.allows_mcp_elicitations() => {
            return Ok(Some(mcp_elicitation_decline_without_message()));
        }
        AskForApproval::OnRequest | AskForApproval::UnlessTrusted | AskForApproval::Granular(_) => {
        }
        AskForApproval::OnFailure => return Ok(None),
    }
    if !crate::guardian::routes_approval_policy_to_guardian(
        approval_policy,
        elicitation_authority.approvals_reviewer,
    ) {
        return Ok(None);
    }

    let guardian_request = match guardian_elicitation_review_request(&request) {
        GuardianElicitationReview::NotRequested => return Ok(None),
        GuardianElicitationReview::Decline(reason) => {
            warn!(
                server_name = %request.server_name,
                request_id = %mcp_elicitation_request_id(&request.request_id),
                reason,
                "declining Guardian MCP elicitation before review"
            );
            return Ok(Some(mcp_elicitation_decline_without_message()));
        }
        GuardianElicitationReview::ApprovalRequest(guardian_request) => *guardian_request,
    };

    let review_id = crate::guardian::new_guardian_review_id();
    let decision = crate::guardian::review_approval_request(
        &session,
        &turn_context,
        review_id.clone(),
        guardian_request,
        /*retry_reason*/ None,
    )
    .await;
    Ok(Some(
        mcp_elicitation_response_from_guardian_decision(session.as_ref(), &review_id, decision)
            .await,
    ))
}

fn guardian_elicitation_review_request(
    request: &ElicitationReviewRequest,
) -> GuardianElicitationReview {
    let (meta, requested_schema) = match &request.elicitation {
        CreateElicitationRequestParams::FormElicitationParams {
            meta,
            requested_schema,
            ..
        } => (meta, Some(requested_schema)),
        CreateElicitationRequestParams::UrlElicitationParams { meta, .. } => {
            return if meta_requests_approval_request(meta) {
                GuardianElicitationReview::Decline(
                    "guardian MCP elicitation review only supports form elicitations",
                )
            } else {
                GuardianElicitationReview::NotRequested
            };
        }
    };

    let Some(meta) = meta.as_ref().map(|meta| &meta.0) else {
        return GuardianElicitationReview::NotRequested;
    };
    if metadata_str(meta, MCP_ELICITATION_REQUEST_TYPE_KEY)
        != Some(MCP_ELICITATION_REQUEST_TYPE_APPROVAL_REQUEST)
    {
        return GuardianElicitationReview::NotRequested;
    }
    if metadata_str(meta, MCP_ELICITATION_APPROVAL_KIND_KEY)
        != Some(MCP_ELICITATION_APPROVAL_KIND_MCP_TOOL_CALL)
    {
        return GuardianElicitationReview::Decline(
            "guardian MCP elicitation metadata must declare mcp_tool_call approval kind",
        );
    }
    if requested_schema.is_some_and(|schema| !schema.properties.is_empty()) {
        return GuardianElicitationReview::Decline(
            "guardian MCP elicitation review only supports empty form schemas",
        );
    }

    let Some(tool_name) = metadata_owned_string(meta, MCP_ELICITATION_TOOL_NAME_KEY) else {
        return GuardianElicitationReview::Decline(
            "guardian MCP elicitation metadata must include a non-empty tool_name",
        );
    };
    let arguments = match meta.get(MCP_ELICITATION_TOOL_PARAMS_KEY) {
        Some(value @ Value::Object(_)) => Some(value.clone()),
        Some(_) => {
            return GuardianElicitationReview::Decline(
                "guardian MCP elicitation tool_params must be an object",
            );
        }
        None => Some(Value::Object(Map::new())),
    };

    GuardianElicitationReview::ApprovalRequest(Box::new(
        crate::guardian::GuardianApprovalRequest::McpToolCall {
            id: format!(
                "mcp_elicitation:{}:{}",
                request.server_name,
                mcp_elicitation_request_id(&request.request_id)
            ),
            server: request.server_name.clone(),
            tool_name,
            arguments,
            connector_id: metadata_owned_string(meta, MCP_ELICITATION_CONNECTOR_ID_KEY),
            connector_name: metadata_owned_string(meta, MCP_ELICITATION_CONNECTOR_NAME_KEY),
            connector_description: metadata_owned_string(
                meta,
                MCP_ELICITATION_CONNECTOR_DESCRIPTION_KEY,
            ),
            tool_title: metadata_owned_string(meta, MCP_ELICITATION_TOOL_TITLE_KEY),
            tool_description: metadata_owned_string(meta, MCP_ELICITATION_TOOL_DESCRIPTION_KEY),
            annotations: None,
        },
    ))
}

fn meta_requests_approval_request(meta: &Option<Meta>) -> bool {
    meta.as_ref()
        .and_then(|meta| metadata_str(&meta.0, MCP_ELICITATION_REQUEST_TYPE_KEY))
        == Some(MCP_ELICITATION_REQUEST_TYPE_APPROVAL_REQUEST)
}

fn metadata_str<'a>(meta: &'a Map<String, Value>, key: &str) -> Option<&'a str> {
    meta.get(key).and_then(Value::as_str)
}

fn metadata_owned_string(meta: &Map<String, Value>, key: &str) -> Option<String> {
    metadata_str(meta, key)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

fn mcp_elicitation_request_id(id: &RequestId) -> String {
    match id {
        rmcp::model::NumberOrString::String(value) => value.to_string(),
        rmcp::model::NumberOrString::Number(value) => value.to_string(),
    }
}

async fn mcp_elicitation_response_from_guardian_decision(
    session: &Session,
    review_id: &str,
    decision: ReviewDecision,
) -> ElicitationResponse {
    let denial_message = match decision {
        ReviewDecision::Denied => {
            Some(crate::guardian::guardian_rejection_message(session, review_id).await)
        }
        _ => None,
    };
    mcp_elicitation_response_from_guardian_decision_parts(decision, denial_message)
}

fn mcp_elicitation_response_from_guardian_decision_parts(
    decision: ReviewDecision,
    denial_message: Option<String>,
) -> ElicitationResponse {
    match decision {
        ReviewDecision::Approved
        | ReviewDecision::ApprovedForSession
        | ReviewDecision::ApprovedExecpolicyAmendment { .. }
        | ReviewDecision::NetworkPolicyAmendment { .. } => ElicitationResponse {
            action: ElicitationAction::Accept,
            content: Some(serde_json::json!({})),
            meta: Some(mcp_elicitation_auto_meta()),
        },
        ReviewDecision::Denied => mcp_elicitation_decline_with_message(
            denial_message.unwrap_or_else(|| "Guardian denied this request.".to_string()),
        ),
        ReviewDecision::TimedOut => {
            mcp_elicitation_decline_with_message(crate::guardian::guardian_timeout_message())
        }
        ReviewDecision::Abort => ElicitationResponse {
            action: ElicitationAction::Cancel,
            content: None,
            meta: Some(mcp_elicitation_auto_meta()),
        },
    }
}

fn mcp_elicitation_decline_with_message(message: String) -> ElicitationResponse {
    ElicitationResponse {
        action: ElicitationAction::Decline,
        content: None,
        meta: Some(serde_json::json!({
            MCP_ELICITATION_DECLINE_MESSAGE_KEY: message,
            MCP_ELICITATION_APPROVALS_REVIEWER_KEY: ApprovalsReviewer::AutoReview,
        })),
    }
}

fn mcp_elicitation_decline_without_message() -> ElicitationResponse {
    ElicitationResponse {
        action: ElicitationAction::Decline,
        content: None,
        meta: Some(mcp_elicitation_auto_meta()),
    }
}

fn mcp_elicitation_auto_meta() -> serde_json::Value {
    serde_json::json!({
        MCP_ELICITATION_APPROVALS_REVIEWER_KEY: ApprovalsReviewer::AutoReview,
    })
}

#[cfg(test)]
#[path = "mcp_tests.rs"]
mod tests;
