use anyhow::Result;
use predicates::str::contains;
use std::path::Path;
use tempfile::TempDir;

fn hepta_command(hepta_home: &Path) -> Result<assert_cmd::Command> {
    let mut cmd = assert_cmd::Command::new(codex_utils_cargo_bin::cargo_bin("hepta")?);
    cmd.env("HEPTA_HOME", hepta_home);
    Ok(cmd)
}

#[tokio::test]
async fn marketplace_upgrade_runs_under_plugin() -> Result<()> {
    let hepta_home = TempDir::new()?;

    hepta_command(hepta_home.path())?
        .args(["plugin", "marketplace", "upgrade"])
        .assert()
        .success()
        .stdout(contains("No configured Git marketplaces to upgrade."));

    Ok(())
}

#[tokio::test]
async fn marketplace_upgrade_no_longer_runs_at_top_level() -> Result<()> {
    let hepta_home = TempDir::new()?;

    hepta_command(hepta_home.path())?
        .args(["marketplace", "upgrade"])
        .assert()
        .failure()
        .stderr(contains("unrecognized subcommand 'upgrade'"));

    Ok(())
}
