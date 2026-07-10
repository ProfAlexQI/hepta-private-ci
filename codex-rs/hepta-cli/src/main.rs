use hepta_native_gateway as native_gateway;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let Some(options) = native_gateway::parse_serve_ui_args_from_env()? else {
        anyhow::bail!(
            "hepta-cli first-class package supports the --serve-ui gateway entrypoint; legacy CLI compatibility remains outside the active service binary"
        );
    };

    native_gateway::run_native_gateway(options).await
}
