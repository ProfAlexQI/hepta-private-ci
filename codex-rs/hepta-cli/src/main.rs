use hepta_native_gateway as native_gateway;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.first().is_some_and(|arg| arg == "gate") {
        println!("{}", native_gateway::gate_command_json(&args[1..])?);
        return Ok(());
    }

    let Some(options) = native_gateway::parse_serve_ui_args(&args)? else {
        anyhow::bail!(
            "hepta-cli supports --serve-ui plus `hepta gate` registry and explicit source compatibility execution; legacy CLI compatibility remains outside the active service binary"
        );
    };

    native_gateway::run_native_gateway(options).await
}
