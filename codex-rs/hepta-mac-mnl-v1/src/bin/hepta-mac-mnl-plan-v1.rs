#[cfg(target_os = "macos")]
fn main() {
    match codex_hepta_mac_mnl_v1::client::run_plan_cli(std::env::args_os().collect()) {
        Ok(json) => print!("{json}"),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

#[cfg(not(target_os = "macos"))]
fn main() {
    eprintln!("BLOCKED: hepta-mac-mnl-plan-v1 is available only on macOS");
    std::process::exit(1);
}
