use std::collections::BTreeSet;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Result;
use codex_hepta_contracts::AgentId;
use pretty_assertions::assert_eq;

use super::HeptaAgentLayout;
use super::HeptaFleetLayout;
use super::HeptaFleetRoot;

const FIRST_AGENT_ID: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12";
const SECOND_AGENT_ID: &str = "019153a4-3088-7e03-a56a-9b1964f75dd3";

#[test]
fn production_layout_is_stable_and_fleet_scoped() -> Result<()> {
    let fleet_root = HeptaFleetRoot::production_default(Path::new("/Users/operator"))?;
    let layout = fleet_root.layout();
    let expected_root = PathBuf::from("/Users/operator/.local/share/hepta-vnext/fleet-v1");

    assert_eq!(
        layout,
        HeptaFleetLayout {
            fleet_root,
            fleet_config: expected_root.join("fleet.toml"),
            state_root: expected_root.join("state"),
            supervisor_database: expected_root.join("state/supervisor.sqlite3"),
            run_root: expected_root.join("run"),
            supervisor_socket: expected_root.join("run/supervisor.sock"),
            supervisor_lock: expected_root.join("run/supervisor.lock"),
            agents_root: expected_root.join("agents"),
        }
    );
    Ok(())
}

#[test]
fn agent_layout_uses_safe_id_component_and_disjoint_owned_roots() -> Result<()> {
    let fleet = HeptaFleetRoot::parse("/srv/hepta/fleet")?.layout();
    let agent_id = AgentId::parse(FIRST_AGENT_ID).map_err(|error| anyhow::anyhow!("{error}"))?;
    let layout = fleet.agent(&agent_id);
    let agent_root = PathBuf::from(format!("/srv/hepta/fleet/agents/{FIRST_AGENT_ID}"));
    let socket_key = "aghu64s7r56mdd2v36p3hkrmci";

    assert_eq!(
        layout,
        HeptaAgentLayout {
            agent_id,
            agent_root: agent_root.clone(),
            agent_config: agent_root.join("agent.toml"),
            home_root: agent_root.join("home"),
            run_root: agent_root.join("run"),
            agentd_control_socket: PathBuf::from(format!("/srv/hepta/fleet/run/a{socket_key}.ctl")),
            app_server_socket: PathBuf::from(format!("/srv/hepta/fleet/run/a{socket_key}.app")),
            writer_lock: agent_root.join("run/writer.lock"),
            generation_cursor: agent_root.join("run/generation.json"),
            logs_root: agent_root.join("logs"),
            releases_root: agent_root.join("releases"),
            active_release: agent_root.join("releases/active"),
            cognitive_root: agent_root.join("cognitive"),
        }
    );

    let owned_roots = BTreeSet::from([
        layout.home_root(),
        layout.run_root(),
        layout.logs_root(),
        layout.releases_root(),
        layout.cognitive_root(),
    ]);
    assert_eq!(owned_roots.len(), 5);
    assert_ne!(layout.agentd_control_socket(), layout.app_server_socket());
    Ok(())
}

#[test]
fn distinct_agent_ids_cannot_alias_one_agent_root() -> Result<()> {
    let fleet = HeptaFleetRoot::parse("/srv/hepta/fleet")?.layout();
    let first =
        fleet.agent(&AgentId::parse(FIRST_AGENT_ID).map_err(|error| anyhow::anyhow!("{error}"))?);
    let second =
        fleet.agent(&AgentId::parse(SECOND_AGENT_ID).map_err(|error| anyhow::anyhow!("{error}"))?);

    assert_ne!(first, second);
    assert_ne!(first.agent_root(), second.agent_root());
    assert_ne!(
        first.agentd_control_socket(),
        second.agentd_control_socket()
    );
    assert_ne!(first.app_server_socket(), second.app_server_socket());
    Ok(())
}

#[test]
fn production_socket_paths_fit_the_darwin_sun_path_limit() -> Result<()> {
    let fleet = HeptaFleetRoot::production_default(Path::new("/Users/operator"))?.layout();
    let agent =
        fleet.agent(&AgentId::parse(FIRST_AGENT_ID).map_err(|error| anyhow::anyhow!("{error}"))?);

    for socket in [agent.agentd_control_socket(), agent.app_server_socket()] {
        assert!(
            socket.as_os_str().len() < 104,
            "socket path exceeds Darwin sun_path: {}",
            socket.display()
        );
    }
    Ok(())
}

#[test]
fn fleet_root_rejects_relative_root_and_dot_segments() {
    for root in ["fleet", "/", "/srv/hepta/../fleet", "/srv/hepta/./fleet"] {
        assert!(HeptaFleetRoot::parse(root).is_err(), "accepted {root:?}");
    }
}
