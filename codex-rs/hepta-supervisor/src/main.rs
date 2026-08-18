use std::path::PathBuf;

use codex_hepta_paths::HeptaFleetRoot;
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let fleet_root = parse_fleet_root()?;
    let cancellation = CancellationToken::new();
    spawn_shutdown_signal(cancellation.clone());
    codex_hepta_supervisor::run_supervisord(fleet_root, cancellation).await?;
    Ok(())
}

fn parse_fleet_root() -> anyhow::Result<HeptaFleetRoot> {
    let mut arguments = std::env::args_os().skip(1);
    let flag = arguments.next();
    let value = arguments.next();
    if flag.as_deref() != Some(std::ffi::OsStr::new("--fleet-root"))
        || value.is_none()
        || arguments.next().is_some()
    {
        anyhow::bail!("usage: hepta-supervisord --fleet-root ABSOLUTE_PATH");
    }
    let value = value.ok_or_else(|| anyhow::anyhow!("fleet root argument is missing"))?;
    HeptaFleetRoot::parse(PathBuf::from(value))
}

fn spawn_shutdown_signal(cancellation: CancellationToken) {
    tokio::spawn(async move {
        wait_for_shutdown_signal().await;
        cancellation.cancel();
    });
}

#[cfg(unix)]
async fn wait_for_shutdown_signal() {
    let terminate = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate());
    let interrupt = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt());
    let (Ok(mut terminate), Ok(mut interrupt)) = (terminate, interrupt) else {
        let _ = tokio::signal::ctrl_c().await;
        return;
    };
    tokio::select! {
        _ = terminate.recv() => {}
        _ = interrupt.recv() => {}
    }
}

#[cfg(not(unix))]
async fn wait_for_shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
}
