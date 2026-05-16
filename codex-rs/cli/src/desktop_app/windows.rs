use anyhow::Context as _;
use std::path::Path;
use std::path::PathBuf;
use tokio::process::Command;

pub async fn run_windows_app_open_or_install(
    workspace: PathBuf,
    download_url_override: Option<String>,
) -> anyhow::Result<()> {
    if let Some(app_id) = find_hepta_app_id().await? {
        eprintln!("Opening Hepta Desktop...");
        open_installed_hepta_app(&app_id).await?;
        eprintln!(
            "In Hepta Desktop, open workspace {workspace}.",
            workspace = display_workspace_path(&workspace)
        );
        return Ok(());
    }

    let Some(download_url) = download_url_override.as_deref() else {
        anyhow::bail!(
            "Hepta Desktop is not installed and no Hepta Desktop installer URL is configured for this source fork. Install Hepta Desktop manually or pass --download-url."
        );
    };

    eprintln!("Hepta Desktop not found; opening installer from override URL...");
    open_url(download_url).await?;
    eprintln!(
        "After installing Hepta Desktop, open workspace {workspace}.",
        workspace = display_workspace_path(&workspace)
    );
    Ok(())
}

async fn find_hepta_app_id() -> anyhow::Result<Option<String>> {
    let output = Command::new("powershell.exe")
        .arg("-NoProfile")
        .arg("-Command")
        .arg("Get-StartApps -Name 'Hepta' | Select-Object -First 1 -ExpandProperty AppID")
        .output()
        .await
        .context("failed to invoke `powershell.exe`")?;

    if !output.status.success() {
        return Ok(None);
    }

    let app_id = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if app_id.is_empty() {
        Ok(None)
    } else {
        Ok(Some(app_id))
    }
}

async fn open_installed_hepta_app(app_id: &str) -> anyhow::Result<()> {
    let target = format!("shell:AppsFolder\\{app_id}");
    open_shell_target(&target).await
}

async fn open_url(url: &str) -> anyhow::Result<()> {
    let status = Command::new("powershell.exe")
        .arg("-NoProfile")
        .arg("-Command")
        .arg("& { param($target) Start-Process -FilePath $target }")
        .arg(url)
        .status()
        .await
        .with_context(|| format!("failed to open {url}"))?;

    if status.success() {
        Ok(())
    } else {
        anyhow::bail!("failed to open {url} with {status}");
    }
}

async fn open_shell_target(target: &str) -> anyhow::Result<()> {
    // Explorer can successfully hand off shell targets and still return exit code 1.
    let _status = Command::new("explorer.exe")
        .arg(target)
        .status()
        .await
        .with_context(|| format!("failed to open {target}"))?;

    Ok(())
}

fn display_workspace_path(workspace: &Path) -> String {
    let path = workspace.display().to_string();
    if let Some(path) = path.strip_prefix(r"\\?\UNC\") {
        format!(r"\\{path}")
    } else if let Some(path) = path.strip_prefix(r"\\?\") {
        path.to_string()
    } else {
        path
    }
}

#[cfg(test)]
mod tests {
    use super::display_workspace_path;
    use pretty_assertions::assert_eq;
    use std::path::Path;

    #[test]
    fn display_workspace_path_removes_windows_extended_prefix() {
        assert_eq!(
            display_workspace_path(Path::new(r"\\?\C:\Users\fcoury\code\hepta")),
            r"C:\Users\fcoury\code\hepta"
        );
    }

    #[test]
    fn display_workspace_path_preserves_unc_prefix() {
        assert_eq!(
            display_workspace_path(Path::new(r"\\?\UNC\server\share\hepta")),
            r"\\server\share\hepta"
        );
    }

    #[test]
    fn display_workspace_path_leaves_regular_paths_unchanged() {
        assert_eq!(
            display_workspace_path(Path::new(r"C:\Users\fcoury\code\hepta")),
            r"C:\Users\fcoury\code\hepta"
        );
    }
}
