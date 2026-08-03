use super::*;
use codex_app_server_protocol::PluginSearchParams;
use codex_app_server_protocol::PluginSearchResponse;
use codex_app_server_protocol::PluginSearchResult;
use codex_app_server_protocol::PluginSearchScope;
use codex_core_plugins::remote::DEFAULT_PLUGIN_SEARCH_LIMIT;
use codex_core_plugins::remote::MAX_PLUGIN_SEARCH_CURSOR_BYTES;
use codex_core_plugins::remote::MAX_PLUGIN_SEARCH_CURSOR_CHARS;
use codex_core_plugins::remote::MAX_PLUGIN_SEARCH_LIMIT;
use codex_core_plugins::remote::MAX_PLUGIN_SEARCH_RESPONSE_BYTES;
use codex_core_plugins::remote::MAX_PLUGIN_SEARCH_TERM_BYTES;
use codex_core_plugins::remote::MAX_PLUGIN_SEARCH_TERM_CHARS;
use codex_core_plugins::remote::REMOTE_WORKSPACE_SHARED_WITH_ME_MARKETPLACE_NAME;
use codex_core_plugins::remote::REMOTE_WORKSPACE_SHARED_WITH_ME_PRIVATE_MARKETPLACE_NAME;
use codex_core_plugins::remote::REMOTE_WORKSPACE_SHARED_WITH_ME_UNLISTED_MARKETPLACE_NAME;
use codex_core_plugins::remote::RemotePluginScope;
use codex_core_plugins::remote::RemotePluginSearchRequest;
use codex_core_plugins::remote::search_remote_plugins;
use codex_plugin::PluginId;

const MAX_PLUGIN_SEARCH_CWDS: usize = 16;
const MAX_PLUGIN_SEARCH_CWD_BYTES: usize = 4 * 1024;
const MAX_PLUGIN_SEARCH_CWDS_BYTES: usize = 16 * 1024;
const PLUGIN_SEARCH_READ_OPERATION: &str = "plugin-search-read";
const PLUGIN_SEARCH_EGRESS_TARGET_CLASS: &str = "openai-codex-api:/ps/plugins/search";

struct PluginSearchReadAdmission {
    authorization: hepta_egress::EgressAuthorization,
    // Retaining the broker for the request lifetime proves the effect plan binds the admission.
    // Search intentionally creates no mutation journal, provider mutation ACK, or terminal
    // authority receipt.
    _broker: EffectBroker,
}

#[derive(Serialize)]
struct PluginSearchAuthorityInput<'a> {
    search_term: &'a str,
    scope: Option<PluginSearchScope>,
    cwds: &'a [AbsolutePathBuf],
    cursor: Option<&'a str>,
    limit: u32,
}

impl PluginRequestProcessor {
    pub(crate) async fn plugin_search(
        &self,
        request_id: &ConnectionRequestId,
        params: PluginSearchParams,
    ) -> Result<Option<ClientResponsePayload>, JSONRPCErrorError> {
        self.plugin_search_response(request_id, params)
            .await
            .map(|response| Some(response.into()))
    }

    async fn plugin_search_response(
        &self,
        request_id: &ConnectionRequestId,
        params: PluginSearchParams,
    ) -> Result<PluginSearchResponse, JSONRPCErrorError> {
        let PluginSearchParams {
            search_term,
            scope,
            cwds,
            cursor,
            limit,
        } = params;
        let search_term = search_term.trim();
        let empty_response = || PluginSearchResponse {
            data: Vec::new(),
            next_cursor: None,
        };
        if search_term.is_empty() {
            return Ok(empty_response());
        }
        validate_plugin_search_inputs(search_term, cursor.as_deref(), cwds.as_deref(), limit)?;

        let cwds = cwds.unwrap_or_default();
        let fallback_cwd = cwds.first().map(|cwd| cwd.as_path().to_path_buf());
        let config = self.load_latest_config(fallback_cwd).await?;
        if !config.features.enabled(Feature::Plugins) {
            return Ok(empty_response());
        }
        if !config.features.enabled(Feature::RemotePluginSearch) {
            return Ok(empty_response());
        }

        let scope = if config.features.enabled(Feature::RemotePlugin) {
            scope
        } else {
            match scope {
                None | Some(PluginSearchScope::Workspace) => Some(PluginSearchScope::Workspace),
                Some(PluginSearchScope::Global | PluginSearchScope::Personal) => {
                    return Ok(empty_response());
                }
            }
        };
        let plugin_sharing_enabled = config.features.enabled(Feature::PluginSharing);
        let auth = self.auth_manager.auth().await;
        if !auth.as_ref().is_some_and(CodexAuth::uses_codex_backend)
            || !workspace_plugins_enabled_for_search(
                &config,
                auth.as_ref(),
                &self.workspace_settings_cache,
            )
            .await
        {
            return Ok(empty_response());
        }

        let scope = scope.map(|scope| match scope {
            PluginSearchScope::Global => RemotePluginScope::Global,
            PluginSearchScope::Workspace => RemotePluginScope::Workspace,
            PluginSearchScope::Personal => RemotePluginScope::User,
        });
        let limit = limit.unwrap_or(DEFAULT_PLUGIN_SEARCH_LIMIT);
        let authority_input = PluginSearchAuthorityInput {
            search_term,
            scope: scope.map(|scope| match scope {
                RemotePluginScope::Global => PluginSearchScope::Global,
                RemotePluginScope::Workspace => PluginSearchScope::Workspace,
                RemotePluginScope::User => PluginSearchScope::Personal,
            }),
            cwds: &cwds,
            cursor: cursor.as_deref(),
            limit,
        };
        let admission = plugin_search_read_admission(
            request_id,
            config.codex_home.as_path(),
            &authority_input,
        )?;
        let page = search_remote_plugins(
            &RemotePluginServiceConfig::new(
                config.chatgpt_base_url.clone(),
                config.http_client_factory(),
            ),
            auth.as_ref(),
            admission.authorization.clone(),
            RemotePluginSearchRequest {
                query: search_term,
                scope,
                limit,
                page_token: cursor.as_deref(),
            },
        )
        .await
        .map_err(|error| {
            remote_plugin_catalog_error_to_jsonrpc(error, "search remote plugin catalog")
        })?;
        drop(admission);

        let next_cursor = page.next_page_token;
        let mut data = Vec::with_capacity(page.plugins.len());
        for plugin in page.plugins {
            let plugin_id = PluginId::parse(&plugin.id).map_err(|error| {
                internal_error(format!("invalid remote plugin search result id: {error}"))
            })?;
            if !plugin_sharing_enabled
                && matches!(
                    plugin_id.marketplace_name.as_str(),
                    REMOTE_WORKSPACE_SHARED_WITH_ME_MARKETPLACE_NAME
                        | REMOTE_WORKSPACE_SHARED_WITH_ME_PRIVATE_MARKETPLACE_NAME
                        | REMOTE_WORKSPACE_SHARED_WITH_ME_UNLISTED_MARKETPLACE_NAME
                )
            {
                continue;
            }
            data.push(PluginSearchResult {
                plugin: remote_plugin_summary_to_info(plugin),
                marketplace_name: plugin_id.marketplace_name,
                marketplace_path: None,
            });
        }

        let response = PluginSearchResponse { data, next_cursor };
        let response_bytes = serde_json::to_vec(&response)
            .map_err(|error| internal_error(format!("encode plugin search response: {error}")))?;
        if response_bytes.len() > MAX_PLUGIN_SEARCH_RESPONSE_BYTES {
            return Err(internal_error(
                "remote plugin search response exceeded its serialized response bound",
            ));
        }
        Ok(response)
    }
}

fn validate_plugin_search_inputs(
    search_term: &str,
    cursor: Option<&str>,
    cwds: Option<&[AbsolutePathBuf]>,
    limit: Option<u32>,
) -> Result<(), JSONRPCErrorError> {
    if search_term.len() > MAX_PLUGIN_SEARCH_TERM_BYTES
        || search_term.chars().count() > MAX_PLUGIN_SEARCH_TERM_CHARS
    {
        return Err(invalid_request(
            "plugin/search searchTerm exceeds its text budget",
        ));
    }
    if cursor.is_some_and(|cursor| {
        cursor.is_empty()
            || cursor.len() > MAX_PLUGIN_SEARCH_CURSOR_BYTES
            || cursor.chars().count() > MAX_PLUGIN_SEARCH_CURSOR_CHARS
    }) {
        return Err(invalid_request(
            "plugin/search cursor violates its non-empty text budget",
        ));
    }
    if limit.is_some_and(|limit| !(1..=MAX_PLUGIN_SEARCH_LIMIT).contains(&limit)) {
        return Err(invalid_request(
            "plugin/search limit exceeds its result-count budget",
        ));
    }
    let cwds = cwds.unwrap_or_default();
    if cwds.len() > MAX_PLUGIN_SEARCH_CWDS {
        return Err(invalid_request(
            "plugin/search cwds exceeds its path-count budget",
        ));
    }
    let mut total_bytes = 0usize;
    for cwd in cwds {
        let bytes = cwd.as_path().as_os_str().as_encoded_bytes().len();
        if bytes > MAX_PLUGIN_SEARCH_CWD_BYTES {
            return Err(invalid_request(
                "plugin/search cwd exceeds its per-path text budget",
            ));
        }
        total_bytes = total_bytes.saturating_add(bytes);
        if total_bytes > MAX_PLUGIN_SEARCH_CWDS_BYTES {
            return Err(invalid_request(
                "plugin/search cwds exceeds its aggregate path-text budget",
            ));
        }
    }
    Ok(())
}

async fn workspace_plugins_enabled_for_search(
    config: &Config,
    auth: Option<&CodexAuth>,
    cache: &workspace_settings::WorkspaceSettingsCache,
) -> bool {
    match workspace_settings::codex_plugins_enabled_for_workspace(config, auth, Some(cache)).await {
        Ok(enabled) => enabled,
        Err(error) => {
            warn!(
                error = %error,
                "failed to fetch workspace plugin search permission; denying remote search"
            );
            false
        }
    }
}

fn plugin_search_read_admission(
    request_id: &ConnectionRequestId,
    codex_home: &Path,
    params: &PluginSearchAuthorityInput<'_>,
) -> Result<PluginSearchReadAdmission, JSONRPCErrorError> {
    let request_binding = plugin_digest_hex(
        "app-server-plugin-search-read-request",
        &[&format!("{request_id:?}")],
    );
    let workspace_binding = plugin_digest_hex(
        "app-server-plugin-search-read-workspace",
        &[&codex_home.display().to_string()],
    );
    let session_binding = plugin_digest_hex(
        "app-server-plugin-search-read-session",
        &[&format!("{:?}", request_id.connection_id)],
    );
    let encoded = serde_json::to_vec(params)
        .map_err(|error| internal_error(format!("encode plugin search request: {error}")))?;
    let payload_digest = plugin_content_hash_bytes(
        "app-server-plugin-search-read-payload",
        &[encoded.as_slice()],
    );
    let authority =
        ExactExecutionAuthority::new(request_binding.clone(), workspace_binding, session_binding)
            .map_err(plugin_lifecycle_error)?;
    let admission = ExecutionAdmission::new(
        ExecutionIngress::AppServer,
        PLUGIN_SEARCH_READ_OPERATION,
        authority,
        plugin_digest_hex(
            "app-server-plugin-search-read-intent",
            &[
                PLUGIN_SEARCH_READ_OPERATION,
                PLUGIN_SEARCH_EGRESS_TARGET_CLASS,
            ],
        ),
        payload_digest.clone(),
    )
    .map_err(plugin_lifecycle_error)?;
    let plan = EffectPlan::new(
        admission.admission_hash(),
        "outbound-http-read",
        PLUGIN_SEARCH_EGRESS_TARGET_CLASS,
        payload_digest,
        request_binding,
    )
    .map_err(plugin_lifecycle_error)?;
    let authorization = hepta_egress::EgressAuthorization::EffectBound {
        admission_hash: admission.admission_hash().to_string(),
        effect_plan_hash: plan.effect_plan_hash().to_string(),
    };
    let mut broker = EffectBroker::admit(admission);
    broker
        .record_effect_plan(plan)
        .map_err(plugin_lifecycle_error)?;
    Ok(PluginSearchReadAdmission {
        authorization,
        _broker: broker,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use codex_app_server_protocol::RequestId;
    use codex_app_server_transport::ConnectionId;
    use std::path::Path;
    use tempfile::TempDir;

    fn assert_no_plugin_mutation_storage(codex_home: &Path) {
        assert!(
            !codex_home
                .join(".hepta-authority")
                .join("plugin-mutation")
                .join("journal.json")
                .exists()
        );
        assert!(
            !codex_home
                .join("hepta-plugin-mutation-journal.json")
                .exists()
        );
    }

    #[test]
    fn invalid_inputs_fail_before_network_or_authority() {
        assert!(validate_plugin_search_inputs("q", None, None, Some(0)).is_err());
        assert!(
            validate_plugin_search_inputs(
                &"q".repeat(MAX_PLUGIN_SEARCH_TERM_BYTES + 1),
                None,
                None,
                None,
            )
            .is_err()
        );
        assert!(
            validate_plugin_search_inputs(
                "q",
                Some(&"c".repeat(MAX_PLUGIN_SEARCH_CURSOR_BYTES + 1)),
                None,
                None,
            )
            .is_err()
        );
    }

    #[test]
    fn search_operation_is_not_mutation_authority() {
        assert!(hepta_authority::governed_mutation_spec(PLUGIN_SEARCH_READ_OPERATION).is_none());
    }

    #[test]
    fn authority_and_in_memory_lifecycle_never_store_query_or_cursor_plaintext() {
        let codex_home = TempDir::new().unwrap();
        let request_id = ConnectionRequestId {
            connection_id: ConnectionId(7),
            request_id: RequestId::Integer(11),
        };
        let params = PluginSearchAuthorityInput {
            search_term: "unique-query-secret",
            scope: Some(PluginSearchScope::Global),
            cwds: &[],
            cursor: Some("unique-cursor-secret"),
            limit: 16,
        };
        let admission =
            plugin_search_read_admission(&request_id, codex_home.path(), &params).unwrap();
        let diagnostic = format!("{:?} {:?}", admission.authorization, admission._broker);
        assert!(!diagnostic.contains("unique-query-secret"));
        assert!(!diagnostic.contains("unique-cursor-secret"));
        assert!(diagnostic.contains(PLUGIN_SEARCH_EGRESS_TARGET_CLASS));
        assert!(admission._broker.completed_provider_ack_hash().is_err());
        assert!(admission._broker.completed_receipt_hash().is_err());
        assert_no_plugin_mutation_storage(codex_home.path());
    }
}
