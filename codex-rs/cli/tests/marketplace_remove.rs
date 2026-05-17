use anyhow::Result;
use codex_config::MarketplaceConfigUpdate;
use codex_config::record_user_marketplace;
use codex_core_plugins::installed_marketplaces::marketplace_install_root;
use predicates::str::contains;
use std::path::Path;
use tempfile::TempDir;

fn hepta_command(hepta_home: &Path) -> Result<assert_cmd::Command> {
    let mut cmd = assert_cmd::Command::new(codex_utils_cargo_bin::cargo_bin("hepta")?);
    cmd.env("HEPTA_HOME", hepta_home);
    Ok(cmd)
}

fn configured_marketplace_update() -> MarketplaceConfigUpdate<'static> {
    MarketplaceConfigUpdate {
        last_updated: "2026-04-13T00:00:00Z",
        last_revision: None,
        source_type: "git",
        source: "https://github.com/owner/repo.git",
        ref_name: Some("main"),
        sparse_paths: &[],
    }
}

fn write_installed_marketplace(hepta_home: &Path, marketplace_name: &str) -> Result<()> {
    let root = marketplace_install_root(hepta_home).join(marketplace_name);
    std::fs::create_dir_all(root.join(".agents/plugins"))?;
    std::fs::write(root.join(".agents/plugins/marketplace.json"), "{}")?;
    std::fs::write(root.join("marker.txt"), "installed")?;
    Ok(())
}

#[tokio::test]
async fn marketplace_remove_deletes_config_and_installed_root() -> Result<()> {
    let hepta_home = TempDir::new()?;
    record_user_marketplace(hepta_home.path(), "debug", &configured_marketplace_update())?;
    write_installed_marketplace(hepta_home.path(), "debug")?;

    hepta_command(hepta_home.path())?
        .args(["plugin", "marketplace", "remove", "debug"])
        .assert()
        .success()
        .stdout(contains("Removed marketplace `debug`."));

    let config_path = hepta_home.path().join("config.toml");
    let config = std::fs::read_to_string(config_path)?;
    assert!(!config.contains("[marketplaces.debug]"));
    assert!(
        !marketplace_install_root(hepta_home.path())
            .join("debug")
            .exists()
    );
    Ok(())
}

#[tokio::test]
async fn marketplace_remove_rejects_unknown_marketplace() -> Result<()> {
    let hepta_home = TempDir::new()?;

    hepta_command(hepta_home.path())?
        .args(["plugin", "marketplace", "remove", "debug"])
        .assert()
        .failure()
        .stderr(contains(
            "marketplace `debug` is not configured or installed",
        ));

    Ok(())
}
