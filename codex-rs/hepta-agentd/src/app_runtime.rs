use std::collections::BTreeMap;
use std::num::NonZeroUsize;
use std::sync::Arc;

use codex_app_server::AppServerRuntimeOptions;
use codex_app_server::AppServerTransport;
use codex_app_server::AppServerWebsocketAuthSettings;
use codex_app_server::RemoteControlStartupMode;
use codex_app_server::ThreadStoreConfig;
use codex_arg0::Arg0DispatchPaths;
use codex_config::LoaderOverrides;
use codex_features::Feature;
use codex_hepta_contracts::AuthorityAction;
use codex_hepta_contracts::AuthorityGrant;
use codex_hepta_contracts::Authorized;
use codex_hepta_contracts::CognitiveWriteCapability;
use codex_hepta_contracts::SessionServeCapability;
use codex_hepta_memory::CognitiveRuntime;
use codex_protocol::protocol::SessionSource;
use codex_utils_absolute_path::AbsolutePathBuf;
use codex_utils_cli::CliConfigOverrides;

use crate::AgentdIdentity;
use crate::AgentdState;
use crate::composition::authority_for_identity;
use crate::qualification_writer::qualification_turn_writer_host;

pub(crate) struct AgentAppServerService {
    identity: AgentdIdentity,
    arg0_paths: Arg0DispatchPaths,
    cognitive_runtime: CognitiveRuntime,
    cognitive_write: Option<Authorized<CognitiveWriteCapability>>,
    authority: AuthorityGrant,
    state: Arc<AgentdState>,
    _session_serve: Authorized<SessionServeCapability>,
}

impl AgentAppServerService {
    pub(crate) fn new(
        identity: AgentdIdentity,
        arg0_paths: Arg0DispatchPaths,
        cognitive_runtime: CognitiveRuntime,
        cognitive_write: Option<Authorized<CognitiveWriteCapability>>,
        authority: AuthorityGrant,
        state: Arc<AgentdState>,
    ) -> std::io::Result<Self> {
        authority
            .validate_binding(&identity.agent_id, identity.spawn_generation)
            .map_err(|error| std::io::Error::other(error.to_string()))?;
        let session_serve = authority
            .authorize::<SessionServeCapability>()
            .map_err(|error| std::io::Error::other(error.to_string()))?;
        validate_cognitive_write_capability(&identity, &authority, cognitive_write.as_ref())?;
        Ok(Self {
            identity,
            arg0_paths,
            cognitive_runtime,
            cognitive_write,
            authority,
            state,
            _session_serve: session_serve,
        })
    }

    pub(crate) async fn run(self) -> std::io::Result<()> {
        run_app_server(
            self.identity,
            self.arg0_paths,
            self.cognitive_runtime,
            self.cognitive_write,
            self.authority,
            self.state,
        )
        .await
    }
}

async fn run_app_server(
    identity: AgentdIdentity,
    arg0_paths: Arg0DispatchPaths,
    cognitive_runtime: CognitiveRuntime,
    cognitive_write: Option<Authorized<CognitiveWriteCapability>>,
    authority: AuthorityGrant,
    state: Arc<AgentdState>,
) -> std::io::Result<()> {
    let socket_path = AbsolutePathBuf::from_absolute_path(&identity.app_server_socket)?;
    let config_overrides = app_server_config_overrides(cognitive_write.is_some());
    let runtime_options = app_server_runtime_options_for_agent_with_authority(
        &identity,
        state,
        cognitive_runtime,
        cognitive_write,
        &authority,
    )?;
    codex_app_server::run_main_with_transport_options(
        arg0_paths,
        config_overrides,
        LoaderOverrides::default(),
        /*strict_config*/ true,
        /*default_analytics_enabled*/ false,
        AppServerTransport::UnixSocket { socket_path },
        SessionSource::Custom("hepta-agentd".to_string()),
        AppServerWebsocketAuthSettings::default(),
        runtime_options,
    )
    .await
}

fn app_server_config_overrides(cognitive_write_enabled: bool) -> CliConfigOverrides {
    CliConfigOverrides {
        raw_overrides: vec![
            "features.hepta_governance=true".to_string(),
            "features.hepta_turn_recovery=true".to_string(),
            "features.hepta_memory=true".to_string(),
            "features.hepta_memory_read_only=true".to_string(),
            // This value is derived from possession of the typed capability,
            // not reconstructed from request or managed configuration.
            format!("features.hepta_cognitive_write={cognitive_write_enabled}"),
        ],
    }
}

pub(crate) fn app_server_runtime_options(
    identity: &AgentdIdentity,
    cognitive_runtime: CognitiveRuntime,
) -> std::io::Result<AppServerRuntimeOptions> {
    let authority = authority_for_identity(identity)
        .map_err(|error| std::io::Error::other(error.to_string()))?;
    let cognitive_write = cognitive_write_for_authority(&authority)?;
    app_server_runtime_options_with_writer(
        identity,
        cognitive_runtime,
        &authority,
        cognitive_write.is_some(),
        None,
    )
}

pub(crate) fn app_server_runtime_options_for_agent(
    identity: &AgentdIdentity,
    state: Arc<AgentdState>,
    cognitive_runtime: CognitiveRuntime,
) -> std::io::Result<AppServerRuntimeOptions> {
    let authority = authority_for_identity(identity)
        .map_err(|error| std::io::Error::other(error.to_string()))?;
    let cognitive_write = cognitive_write_for_authority(&authority)?;
    app_server_runtime_options_for_agent_with_authority(
        identity,
        state,
        cognitive_runtime,
        cognitive_write,
        &authority,
    )
}

fn app_server_runtime_options_for_agent_with_authority(
    identity: &AgentdIdentity,
    state: Arc<AgentdState>,
    cognitive_runtime: CognitiveRuntime,
    cognitive_write: Option<Authorized<CognitiveWriteCapability>>,
    authority: &AuthorityGrant,
) -> std::io::Result<AppServerRuntimeOptions> {
    validate_cognitive_write_capability(identity, authority, cognitive_write.as_ref())?;
    let writer = qualification_turn_writer_host(
        identity,
        state,
        &cognitive_runtime,
        cognitive_write.as_ref(),
    );
    app_server_runtime_options_with_writer(
        identity,
        cognitive_runtime,
        authority,
        cognitive_write.is_some(),
        writer,
    )
}

fn app_server_runtime_options_with_writer(
    identity: &AgentdIdentity,
    cognitive_runtime: CognitiveRuntime,
    authority: &AuthorityGrant,
    cognitive_write_enabled: bool,
    qualification_turn_writer: Option<codex_hepta_memory_extension::QualificationTurnWriterHost>,
) -> std::io::Result<AppServerRuntimeOptions> {
    authority
        .validate_binding(&identity.agent_id, identity.spawn_generation)
        .map_err(|error| std::io::Error::other(error.to_string()))?;
    if cognitive_write_enabled != authority.allows(AuthorityAction::WriteCognitiveState) {
        return Err(std::io::Error::other(
            "typed cognitive-write capability does not match the selected authority profile",
        ));
    }
    let turn_queue_capacity = usize::try_from(identity.resources.turn_queue_capacity)
        .map_err(|_| std::io::Error::other("turn queue capacity does not fit this platform"))?;
    let turn_queue_capacity = NonZeroUsize::new(turn_queue_capacity).ok_or_else(|| {
        std::io::Error::other("agent manifest contains a zero turn queue capacity")
    })?;
    Ok(AppServerRuntimeOptions {
        remote_control_startup_mode: RemoteControlStartupMode::DisabledEphemeral,
        install_shutdown_signal_handler: false,
        turn_queue_capacity: Some(turn_queue_capacity),
        required_sqlite_home: Some(AbsolutePathBuf::from_absolute_path(&identity.home_root)?),
        required_thread_store_mode: Some(ThreadStoreConfig::Local),
        hepta_cognitive_runtime: cognitive_runtime,
        // The owning agent supplies the qualification-only policy to the
        // explicit host owner. The legacy turn callback remains disabled.
        hepta_local_turn_lifecycle_enabled: false,
        hepta_local_development_policy: Some(
            codex_hepta_memory::LocalDevelopmentLifecyclePolicy::qualification_only(),
        ),
        hepta_qualification_turn_writer_enabled: cognitive_write_enabled,
        hepta_qualification_turn_writer: qualification_turn_writer,
        // This embedding-owned capability boundary is applied after managed
        // config and request overrides, so those layers cannot widen it.
        required_feature_states: BTreeMap::from([(
            Feature::HeptaCognitiveWrite,
            cognitive_write_enabled,
        )]),
        ..Default::default()
    })
}

fn cognitive_write_for_authority(
    authority: &AuthorityGrant,
) -> std::io::Result<Option<Authorized<CognitiveWriteCapability>>> {
    if authority.allows(AuthorityAction::WriteCognitiveState) {
        authority
            .authorize::<CognitiveWriteCapability>()
            .map(Some)
            .map_err(|error| std::io::Error::other(error.to_string()))
    } else {
        Ok(None)
    }
}

fn validate_cognitive_write_capability(
    identity: &AgentdIdentity,
    authority: &AuthorityGrant,
    cognitive_write: Option<&Authorized<CognitiveWriteCapability>>,
) -> std::io::Result<()> {
    let expected = authority.allows(AuthorityAction::WriteCognitiveState);
    if expected != cognitive_write.is_some() {
        return Err(std::io::Error::other(
            "typed cognitive-write capability does not match the selected authority profile",
        ));
    }
    if let Some(cognitive_write) = cognitive_write {
        if cognitive_write.is_external() {
            return Err(std::io::Error::other(
                "Agent App Server cannot consume external production cognitive-write authority",
            ));
        }
        if cognitive_write.subject_agent_id() != &identity.agent_id
            || cognitive_write.generation() != identity.spawn_generation
        {
            return Err(std::io::Error::other(
                "typed cognitive-write capability does not match Agent identity/generation",
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use codex_features::Feature;
    use codex_hepta_contracts::AgentId;
    use codex_hepta_contracts::AuthorityAction;
    use codex_hepta_fleet::ResourceBudget;
    use codex_hepta_paths::HeptaFleetRoot;

    use super::app_server_config_overrides;
    use super::app_server_runtime_options;
    use crate::AgentdIdentity;
    use crate::composition::authority_for_identity;

    const AGENT_ID: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12";

    fn identity_with_queue_capacity(capacity: u32) -> AgentdIdentity {
        let agent_id = AgentId::parse(AGENT_ID).expect("valid agent id");
        let fleet_root =
            HeptaFleetRoot::parse("/tmp/hepta-agentd-capacity-test").expect("valid fleet root");
        let layout = fleet_root.layout().agent(&agent_id);
        let mut resources = ResourceBudget::local_default();
        resources.turn_queue_capacity = capacity;
        AgentdIdentity {
            agent_id,
            workspace: "/tmp/hepta-agentd-capacity-workspace".into(),
            home_root: layout.home_root().to_path_buf(),
            run_root: layout.run_root().to_path_buf(),
            control_socket: layout.agentd_control_socket().to_path_buf(),
            app_server_socket: layout.app_server_socket().to_path_buf(),
            layout,
            spawn_generation: 1,
            fleet_root: fleet_root.as_path().to_path_buf(),
            resources,
        }
    }

    #[test]
    fn agentd_forces_hepta_turn_recovery_on() {
        let overrides = app_server_config_overrides(false);
        assert!(
            overrides
                .raw_overrides
                .iter()
                .any(|value| value == "features.hepta_turn_recovery=true")
        );
    }

    #[test]
    fn agentd_derives_cognitive_write_from_typed_capability() {
        let identity = identity_with_queue_capacity(37);
        let authority = authority_for_identity(&identity).expect("valid build authority");
        let expected = authority.allows(AuthorityAction::WriteCognitiveState);
        let overrides = app_server_config_overrides(expected);
        assert!(overrides.raw_overrides.iter().any(|value| {
            value == &format!("features.hepta_cognitive_write={expected}")
        }));
    }

    #[test]
    fn manifest_queue_capacity_reaches_app_server_runtime_options_exactly() {
        let identity = identity_with_queue_capacity(37);
        let options =
            app_server_runtime_options(&identity, codex_hepta_memory::CognitiveRuntime::Absent)
                .expect("valid runtime options");
        let authority = authority_for_identity(&identity).expect("valid build authority");
        let cognitive_write_enabled = authority.allows(AuthorityAction::WriteCognitiveState);
        assert_eq!(
            Some(37),
            options.turn_queue_capacity.map(std::num::NonZeroUsize::get)
        );
        assert_eq!(
            Some(identity.home_root.as_path()),
            options
                .required_sqlite_home
                .as_ref()
                .map(codex_utils_absolute_path::AbsolutePathBuf::as_path)
        );
        assert_eq!(
            Some(&codex_app_server::ThreadStoreConfig::Local),
            options.required_thread_store_mode.as_ref()
        );
        assert!(!options.hepta_local_turn_lifecycle_enabled);
        assert_eq!(
            Some(codex_hepta_memory::LocalDevelopmentLifecyclePolicy::qualification_only()),
            options.hepta_local_development_policy
        );
        assert_eq!(
            Some(&cognitive_write_enabled),
            options
                .required_feature_states
                .get(&Feature::HeptaCognitiveWrite)
        );
    }
}
