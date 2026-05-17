use anyhow::Result;
use predicates::str::contains;
use std::path::Path;
use tempfile::TempDir;

fn hepta_command(hepta_home: &Path) -> Result<assert_cmd::Command> {
    let mut cmd = assert_cmd::Command::new(codex_utils_cargo_bin::cargo_bin("hepta")?);
    cmd.env("HEPTA_HOME", hepta_home);
    Ok(cmd)
}

#[cfg(debug_assertions)]
#[tokio::test]
async fn update_does_not_start_interactive_prompt() -> Result<()> {
    let hepta_home = TempDir::new()?;

    hepta_command(hepta_home.path())?
        .arg("update")
        .assert()
        .failure()
        .stderr(contains("`hepta update` is not available in debug builds"));

    Ok(())
}
