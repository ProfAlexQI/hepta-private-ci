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
use codex_hepta_contracts::CognitiveWriteCapability;
use codex_hepta_memory::CognitiveRuntime;
use codex_protocol::protocol::SessionSource;
use codex_utils_absolute_path::AbsolutePathBuf;
use codex_utils_cli::CliConfigOverrides;

use crate::AgentdIdentity;
use crate::AgentdState;
use crate::qualification_writer::qualification_turn_writer_host;

#[cfg(feature = "qualification-cognitive-write")]
const COGNITIVE_WRITE_ENABLED: bool = true;
#[cfg(not(feature = "qualification-cognitive-write"))]
const COGNITIVE_WRITE_ENABLED: bool = false;

pub(crate) async fn run_app_server(
    identity: AgentdIdentity,
    arg0_paths: Arg0DispatchPaths,
    cognitive_runtime: CognitiveRuntime,
    authority: AuthorityGrant,
    state: Arc<AgentdState>,
) -> std::io::Result<()> {
    let socket_path = AbsolutePathBuf::from_absolute_path(&identity.app_server_socket)?;
    let config_overrides = app_server_config_overrides();
    let runtime_options = app_server_runtime_options_for_agent_with_authority(
        &identity,
        state,
        cognitive_runtime,
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

fn app_server_config_overrides() -> CliConfigOverrides {
    CliConfigOverrides {
        raw_overrides: vec![
            "features.hepta_governance=true".to_string(),
            "features.hepta_turn_recovery=true".to_string(),
            "features.hepta_memory=true".to_string(),
            "features.hepta_memory_read_only=true".to_string(),
            // The default binary is read-only. The only positive writer
            // profile is an explicit build-time qualification binary and now
            // also requires a typed CognitiveWrite capability from Agentd's
            // validated composition authority.
            format!("features.hepta_cognitive_write={COGNITIVE_WRITE_ENABLED}"),
        ],
    }
}

pub(crate) fn app_server_runtime_options(
    identity: &AgentdIdentity,
    cognitive_runtime: CognitiveRuntime,
) -> std::io::Result<AppServerRuntimeOptions> {
    let authority = expected_authority(identity)?;
    authority.validate_binding(&identity.agent_id, identity.spawn_generation).map_err(
        |error| std::io::Error::other(format!("invalid Agent authority binding: {error}")),
    )?;
    app_server_runtime_options_with_writer(identity, cognitive_runtime, None)
}

/// Compatibility seam used by the existing Agentd tests. Product startup uses
/// `app_server_runtime_options_for_agent_with_authority` with the exact grant
/// carried by the composition root rather than minting a second grant.
pub(crate) fn app_server_runtime_options_for_agent(
    identity: &AgentdIdentity,
    state: Arc<AgentdState>,
    cognitive_runtime: CognitiveRuntime,
) -> std::io::Result<AppServerRuntimeOptions> {
    let authority = expected_authority(identity)?;
    app_server_runtime_options_for_agent_with_authority(
        identity,
        state,
        cognitive_runtime,
        &authority,
    )
}

fn app_server_runtime_options_for_agent_with_authority(
    identity: &AgentdIdentity,
    state: Arc<AgentdState>,
    cognitive_runtime: CognitiveRuntime,
    authority: &AuthorityGrant,
) -> std::io::Result<AppServerRuntimeOptions> {
    authority
        .validate_binding(&identity.agent_id, identity.spawn_generation)
        .map_err(|error| {
            std::io::Error::other(format!("invalid Agent authority binding: {error}"))
        })?;
    if !authority.is_product_closed() {
        return Err(std::io::Error::other(
            "Agent App Server authority escaped the local product profile",
        ));
    }
    let writer = qualification_writer_for_authority(
        identity,
        state,
        &cognitive_runtime,
        authority,
    )?;
    app_server_runtime_options_with_writer(identity, cognitive_runtime, writer)
}

fn qualification_writer_for_authority(
    identity: &AgentdIdentity,
    state: Arc<AgentdState>,
    cognitive_runtime: &CognitiveRuntime,
    authority: &AuthorityGrant,
) -> std::io::Result<Option<codex_hepta_memory_extension::QualificationTurnWriterHost>> {
    if COGNITIVE_WRITE_ENABLED {
        let _cognitive_write = authority
            .authorize::<CognitiveWriteCapability>()
            .map_err(|error| {
                std::io::Error::other(format!(
                    "qualification writer requires typed cognitive write authority: {error}"
                ))
            })?;
        let writer = qualification_turn_writer_host(identity, state, cognitive_runtime);
        if writer.is_none() {
            return Err(std::io::Error::other(
                "typed cognitive write authority was present but the owning writer host was absent",
            ));
        }
        Ok(writer)
    } else {
        if authority.allows(AuthorityAction::WriteCognitiveState) {
            return Err(std::io::Error::other(
                "default Agentd binary received unexpected cognitive write authority",
            ));
        }
        Ok(None)
    }
}

fn expected_authority(identity: &AgentdIdentity) -> std::io::Result<AuthorityGrant> {
    #[cfg(feature = "qualification-cognitive-write")]
    let authority = AuthorityGrant::qualification_cognitive_write(
        identity.agent_id.clone(),
        identity.spawn_generation,
    );

    #[cfg(not(feature = "qualification-cognitive-write"))]
    let authority =
        AuthorityGrant::agent_local(identity.agent_id.clone(), identity.spawn_generation);

    authority.map_err(|error| std::io::Error::other(format!("build Agent authority: {error}")))
}

fn app_server_runtime_options_with_writer(
    identity: &AgentdIdentity,
    cognitive_runtime: CognitiveRuntime,
    qualification_turn_writer: Option<codex_hepta_memory_extension::QualificationTurnWriterHost>,
) -> std::io::Result<AppServerRuntimeOptions> {
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
        // explicit host owner. The legacy turn callback remains disabled:
        // policy-gated local witness writes must be host-invoked and must not
        // create an unbound lease implicitly during turn startup.
        hepta_local_turn_lifecycle_enabled: false,
        hepta_local_development_policy: Some(
            codex_hepta_memory::LocalDevelopmentLifecyclePolicy::qualification_only(),
        ),
        // The qualification build explicitly opts into the writer seam and
        // receives a typed capability only from the owning Agentd composition
        // root. The default and production-facing binaries remain inert.
        hepta_qualification_turn_writer_enabled: COGNITIVE_WRITE_ENABLED,
        hepta_qualification_turn_writer: qualification_turn_writer,
        // This is an embedding-owned capability boundary. It is applied
        // after managed config and per-request overrides, so those layers
        // cannot change the selected profile at runtime. The positive value
        // exists only in the explicit qualification build.
        required_feature_states: BTreeMap::from([(
            Feature::HeptaCognitiveWrite,
            COGNITIVE_WRITE_ENABLED,
        )]),
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use codex_features::Feature;
    use codex_hepta_contracts::AgentId;
    use codex_hepta_contracts::AuthorityAction;
    use codex_hepta_fleet::ResourceBudget;
    use codex_hepta_paths::HeptaFleetRoot;

    use super::COGNITIVE_WRITE_ENABLED;
    use super::app_server_config_overrides;
    use super::app_server_runtime_options;
    use super::expected_authority;
    use crate::AgentdIdentity;

    const AGENT_ID: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12";

    #[test]
    fn agentd_forces_hepta_turn_recovery_on() {
        let overrides = app_server_config_overrides();
        assert!(
            overrides
                .raw_overrides
                .iter()
                .any(|value| value == "features.hepta_turn_recovery=true")
        );
    }

    #[test]
    fn agentd_forces_explicit_cognitive_write_profile_state() {
        let overrides = app_server_config_overrides();
        assert!(overrides.raw_overrides.iter().any(|value| {
            value == &format!("features.hepta_cognitive_write={COGNITIVE_WRITE_ENABLED}")
        }));
    }

    #[test]
    fn manifest_queue_capacity_reaches_app_server_runtime_options_exactly() {
        let agent_id = AgentId::parse(AGENT_ID).expect("valid agent id");
        let fleet_root =
            HeptaFleetRoot::parse("/tmp/hepta-agentd-capacity-test").expect("valid fleet root");
        let layout = fleet_root.layout().agent(&agent_id);
        let mut resources = ResourceBudget::local_default();
        resources.turn_queue_capacity = 37;
        let identity = AgentdIdentity {
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
        };

        let options =
            app_server_runtime_options(&identity, codex_hepta_memory::CognitiveRuntime::Absent)
                .expect("valid runtime options");
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
            Some(&COGNITIVE_WRITE_ENABLED),
            options
                .required_feature_states
                .get(&Feature::HeptaCognitiveWrite)
        );
    }

    #[test]
    fn selected_authority_profile_never_grants_external_effects() {
        let agent_id = AgentId::parse(AGENT_ID).expect("valid agent id");
        let fleet_root =
            HeptaFleetRoot::parse("/tmp/hepta-agentd-authority-test").expect("valid fleet root");
        let layout = fleet_root.layout().agent(&agent_id);
        let identity = AgentdIdentity {
            agent_id,
            workspace: "/tmp/hepta-agentd-authority-workspace".into(),
            home_root: layout.home_root().to_path_buf(),
            run_root: layout.run_root().to_path_buf(),
            control_socket: layout.agentd_control_socket().to_path_buf(),
            app_server_socket: layout.app_server_socket().to_path_buf(),
            layout,
            spawn_generation: 1,
            fleet_root: fleet_root.as_path().to_path_buf(),
            resources: ResourceBudget::local_default(),
        };
        let authority = expected_authority(&identity).expect("valid authority");
        assert!(authority.is_product_closed());
        assert!(!authority.allows(AuthorityAction::ExternalEffect));
        assert!(!authority.allows(AuthorityAction::InvokeModel));
        assert!(!authority.allows(AuthorityAction::PromoteRelease));
        assert_eq!(
            authority.allows(AuthorityAction::WriteCognitiveState),
            COGNITIVE_WRITE_ENABLED
        );
    }
}
