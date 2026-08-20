use std::num::NonZeroUsize;

use codex_app_server::AppServerRuntimeOptions;
use codex_app_server::AppServerTransport;
use codex_app_server::AppServerWebsocketAuthSettings;
use codex_app_server::RemoteControlStartupMode;
use codex_app_server::ThreadStoreConfig;
use codex_arg0::Arg0DispatchPaths;
use codex_config::LoaderOverrides;
use codex_hepta_memory::CognitiveRuntime;
use codex_protocol::protocol::SessionSource;
use codex_utils_absolute_path::AbsolutePathBuf;
use codex_utils_cli::CliConfigOverrides;

use crate::AgentdIdentity;

pub(crate) async fn run_app_server(
    identity: AgentdIdentity,
    arg0_paths: Arg0DispatchPaths,
    cognitive_runtime: CognitiveRuntime,
) -> std::io::Result<()> {
    let socket_path = AbsolutePathBuf::from_absolute_path(&identity.app_server_socket)?;
    let config_overrides = app_server_config_overrides();
    let runtime_options = app_server_runtime_options(&identity, cognitive_runtime)?;
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
        ],
    }
}

pub(crate) fn app_server_runtime_options(
    identity: &AgentdIdentity,
    cognitive_runtime: CognitiveRuntime,
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
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use codex_hepta_contracts::AgentId;
    use codex_hepta_fleet::ResourceBudget;
    use codex_hepta_paths::HeptaFleetRoot;

    use super::app_server_config_overrides;
    use super::app_server_runtime_options;
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
                .map(|path| path.as_path())
        );
        assert_eq!(
            Some(&codex_app_server::ThreadStoreConfig::Local),
            options.required_thread_store_mode.as_ref()
        );
    }
}
