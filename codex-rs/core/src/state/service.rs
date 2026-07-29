use std::collections::HashMap;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;

use crate::SkillsManager;
use crate::agent::AgentControl;
use crate::attestation::AttestationProvider;
use crate::client::ModelClient;
use crate::config::StartedNetworkProxy;
use crate::exec_policy::ExecPolicyManager;
use crate::guardian::GuardianRejection;
use crate::guardian::GuardianRejectionCircuitBreaker;
use crate::mcp::McpManager;
use crate::tools::code_mode::CodeModeService;
use crate::tools::network_approval::NetworkApprovalService;
use crate::tools::sandboxing::ApprovalStore;
use crate::unified_exec::UnifiedExecProcessManager;
use arc_swap::ArcSwap;
use codex_analytics::AnalyticsEventsClient;
use codex_core_plugins::PluginsManager;
use codex_exec_server::EnvironmentManager;
use codex_extension_api::ExtensionData;
use codex_extension_api::ExtensionRegistry;
use codex_hooks::Hooks;
use codex_http_client::RouteAwareClientPool;
use codex_login::AuthManager;
use codex_login::CodexAuth;
use codex_login::auth::AuthManagerSnapshot;
use codex_mcp::McpConnectionManager;
use codex_models_manager::manager::SharedModelsManager;
use codex_otel::SessionTelemetry;
use codex_protocol::protocol::McpElicitationAuthority;
use codex_rollout::state_db::StateDbHandle;
use codex_rollout_trace::ThreadTraceContext;
use codex_thread_store::LiveThread;
use codex_thread_store::ThreadStore;
use std::path::PathBuf;
use tokio::runtime::Handle;
use tokio::sync::Mutex;
use tokio::sync::RwLock;
use tokio::sync::watch;
use tokio_util::sync::CancellationToken;

pub(crate) struct FrozenMcpAuthSnapshot {
    auth: Option<CodexAuth>,
    binding: McpAuthBinding,
}

#[derive(Clone)]
pub(crate) struct McpAuthBinding {
    revision: u64,
    auth_mode: Option<codex_app_server_protocol::AuthMode>,
    account_id: Option<String>,
    account_email: Option<String>,
    chatgpt_user_id: Option<String>,
    is_fedramp_account: bool,
    token_fingerprint: Option<[u8; 32]>,
}

impl FrozenMcpAuthSnapshot {
    pub(crate) async fn capture(auth_manager: &AuthManager) -> Option<Self> {
        auth_manager
            .auth_snapshot()
            .await
            .map(Self::from_auth_manager_snapshot)
    }

    fn from_auth_manager_snapshot(snapshot: AuthManagerSnapshot) -> Self {
        let (auth, revision) = snapshot.into_parts();
        let auth_ref = auth.as_ref();
        Self {
            binding: McpAuthBinding {
                revision,
                auth_mode: auth_ref.map(CodexAuth::api_auth_mode),
                account_id: auth_ref.and_then(CodexAuth::get_account_id),
                account_email: auth_ref.and_then(CodexAuth::get_account_email),
                chatgpt_user_id: auth_ref.and_then(CodexAuth::get_chatgpt_user_id),
                is_fedramp_account: auth_ref.is_some_and(CodexAuth::is_fedramp_account),
                token_fingerprint: auth_ref.and_then(CodexAuth::credential_fingerprint),
            },
            auth,
        }
    }

    pub(crate) fn auth(&self) -> Option<&CodexAuth> {
        self.auth.as_ref()
    }

    pub(crate) fn binding(&self) -> McpAuthBinding {
        self.binding.clone()
    }

    pub(crate) fn revision(&self) -> u64 {
        self.binding.revision
    }

    pub(crate) fn matches(&self, other: &Self) -> bool {
        self.binding.matches(&other.binding)
    }
}

impl McpAuthBinding {
    pub(crate) fn matches(&self, other: &Self) -> bool {
        let token_matches = constant_time_optional_fingerprint_eq(
            &self.token_fingerprint,
            &other.token_fingerprint,
        );
        self.revision == other.revision
            && self.auth_mode == other.auth_mode
            && self.account_id == other.account_id
            && self.account_email == other.account_email
            && self.chatgpt_user_id == other.chatgpt_user_id
            && self.is_fedramp_account == other.is_fedramp_account
            && token_matches
    }
}

fn constant_time_optional_fingerprint_eq(
    left: &Option<[u8; 32]>,
    right: &Option<[u8; 32]>,
) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => {
            let mut difference = 0_u8;
            for index in 0..left.len() {
                difference |= left[index] ^ right[index];
            }
            difference == 0
        }
        (None, None) => true,
        (Some(_), None) | (None, Some(_)) => false,
    }
}

pub(crate) struct PublishedMcpConnectionManager {
    generation: u64,
    auth_binding: McpAuthBinding,
    elicitation_authority: McpElicitationAuthority,
    manager: McpConnectionManager,
}

impl PublishedMcpConnectionManager {
    pub(crate) fn new(
        manager: McpConnectionManager,
        auth_binding: McpAuthBinding,
        elicitation_authority: McpElicitationAuthority,
    ) -> Self {
        Self {
            generation: 0,
            auth_binding,
            elicitation_authority,
            manager,
        }
    }

    pub(crate) fn generation(&self) -> u64 {
        self.generation
    }

    pub(crate) fn auth_matches(&self, auth_binding: &McpAuthBinding) -> bool {
        self.auth_binding.matches(auth_binding)
    }

    pub(crate) fn elicitation_authority(&self) -> &McpElicitationAuthority {
        &self.elicitation_authority
    }

    pub(crate) fn update_elicitation_authority(
        &mut self,
        elicitation_authority: McpElicitationAuthority,
    ) {
        let approval_policy =
            codex_config::Constrained::allow_any(elicitation_authority.approval_policy);
        self.manager.set_approval_policy(&approval_policy);
        self.manager
            .set_permission_profile(elicitation_authority.permission_profile.clone());
        self.elicitation_authority = elicitation_authority;
    }

    pub(crate) fn publish(
        &mut self,
        manager: McpConnectionManager,
        auth_binding: McpAuthBinding,
        elicitation_authority: McpElicitationAuthority,
    ) -> McpConnectionManager {
        self.generation = self.generation.saturating_add(1);
        self.auth_binding = auth_binding;
        self.elicitation_authority = elicitation_authority;
        std::mem::replace(&mut self.manager, manager)
    }
}

impl Deref for PublishedMcpConnectionManager {
    type Target = McpConnectionManager;

    fn deref(&self) -> &Self::Target {
        &self.manager
    }
}

impl DerefMut for PublishedMcpConnectionManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.manager
    }
}

pub(crate) struct SessionServices {
    pub(crate) mcp_connection_manager: Arc<RwLock<PublishedMcpConnectionManager>>,
    pub(crate) mcp_startup_cancellation_token: Mutex<CancellationToken>,
    pub(crate) unified_exec_manager: UnifiedExecProcessManager,
    #[cfg_attr(not(unix), allow(dead_code))]
    pub(crate) shell_zsh_path: Option<PathBuf>,
    #[cfg_attr(not(unix), allow(dead_code))]
    pub(crate) main_execve_wrapper_exe: Option<PathBuf>,
    pub(crate) analytics_events_client: AnalyticsEventsClient,
    pub(crate) hooks: ArcSwap<Hooks>,
    pub(crate) rollout_thread_trace: ThreadTraceContext,
    pub(crate) user_shell: Arc<crate::shell::Shell>,
    pub(crate) shell_snapshot_tx: watch::Sender<Option<Arc<crate::shell_snapshot::ShellSnapshot>>>,
    pub(crate) show_raw_agent_reasoning: bool,
    pub(crate) exec_policy: Arc<ExecPolicyManager>,
    pub(crate) auth_manager: Arc<AuthManager>,
    /// Upload-only clients shared across turns without logging signed blob URLs.
    pub(crate) openai_file_upload_client_pool: RouteAwareClientPool,
    pub(crate) models_manager: SharedModelsManager,
    pub(crate) session_telemetry: SessionTelemetry,
    pub(crate) tool_approvals: Mutex<ApprovalStore>,
    pub(crate) guardian_rejections: Mutex<HashMap<String, GuardianRejection>>,
    pub(crate) guardian_rejection_circuit_breaker: Mutex<GuardianRejectionCircuitBreaker>,
    pub(crate) runtime_handle: Handle,
    pub(crate) skills_manager: Arc<SkillsManager>,
    pub(crate) plugins_manager: Arc<PluginsManager>,
    pub(crate) mcp_manager: Arc<McpManager>,
    pub(crate) extensions: Arc<ExtensionRegistry<crate::config::Config>>,
    pub(crate) session_extension_data: ExtensionData,
    pub(crate) thread_extension_data: ExtensionData,
    pub(crate) agent_control: AgentControl,
    pub(crate) network_proxy: Option<StartedNetworkProxy>,
    pub(crate) network_approval: Arc<NetworkApprovalService>,
    pub(crate) state_db: Option<StateDbHandle>,
    pub(crate) live_thread: Option<LiveThread>,
    pub(crate) thread_store: Arc<dyn ThreadStore>,
    pub(crate) attestation_provider: Option<Arc<dyn AttestationProvider>>,
    /// Session-scoped model client shared across turns.
    pub(crate) model_client: ModelClient,
    pub(crate) code_mode_service: CodeModeService,
    /// Shared process-level environment registry. Sessions carry an `Arc` handle so they can pass
    /// the same manager through child-thread spawn paths without reconstructing it.
    pub(crate) environment_manager: Arc<EnvironmentManager>,
}
