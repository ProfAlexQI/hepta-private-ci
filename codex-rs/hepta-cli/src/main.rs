use hepta_native_gateway as native_gateway;
mod workgraph_cmd;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.first().is_some_and(|arg| arg == "gate") {
        println!("{}", native_gateway::gate_command_json(&args[1..])?);
        return Ok(());
    }

    if args.first().is_some_and(|arg| arg == "manifest") {
        println!("{}", native_gateway::canonical_manifest_json()?);
        return Ok(());
    }

    if args.first().is_some_and(|arg| arg == "workgraph") {
        return workgraph_cmd::run(&args[1..]);
    }

    let Some(options) = native_gateway::parse_serve_ui_args(&args)? else {
        anyhow::bail!(
            "hepta-cli supports --serve-ui, `hepta gate`, `hepta manifest`, and `hepta workgraph`; legacy CLI compatibility remains outside the active service binary"
        );
    };

    native_gateway::run_native_gateway(options).await
}
