use std::path::Path;

use anyhow::Result;
use predicates::str::contains;
use tempfile::TempDir;

fn hepta_command(hepta_home: &Path) -> Result<assert_cmd::Command> {
    let mut cmd = assert_cmd::Command::new(codex_utils_cargo_bin::cargo_bin("hepta")?);
    cmd.env("HEPTA_HOME", hepta_home);
    Ok(cmd)
}

#[test]
fn strict_config_rejects_unknown_config_fields_for_app_server() -> Result<()> {
    let hepta_home = TempDir::new()?;
    std::fs::write(
        hepta_home.path().join("config.toml"),
        r#"
foo = "bar"
"#,
    )?;

    let mut cmd = hepta_command(hepta_home.path())?;
    cmd.args(["app-server", "--strict-config", "--listen", "off"])
        .assert()
        .failure()
        .stderr(contains("unknown configuration field"));

    Ok(())
}
