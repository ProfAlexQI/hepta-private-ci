#[cfg(target_os = "macos")]
fn main() {
    use std::io::Read;

    let mut stdin = Vec::new();
    if let Err(error) = std::io::stdin()
        .take((codex_hepta_mac_mnl_v1::MAX_CANONICAL_BYTES + 1) as u64)
        .read_to_end(&mut stdin)
    {
        eprintln!("failed to read canonical Mac MNL plan bundle: {error}");
        std::process::exit(1);
    }
    match codex_hepta_mac_mnl_v1::client::run_verify_cli(std::env::args_os().collect(), &stdin) {
        Ok(json) => print!("{json}"),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

#[cfg(not(target_os = "macos"))]
fn main() {
    eprintln!("BLOCKED: hepta-mac-mnl-verify-v1 is available only on macOS");
    std::process::exit(1);
}
