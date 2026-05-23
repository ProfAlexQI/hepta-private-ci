#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let Some(options) = codex_cli::native_gateway::parse_serve_ui_args_from_env()? else {
        anyhow::bail!(
            "hepta-cli first-class package currently supports the --serve-ui gateway entrypoint; legacy CLI compatibility still routes through codex-cli"
        );
    };

    codex_cli::native_gateway::run_native_gateway(options).await
}
