#[cfg(target_os = "macos")]
fn main() {
    if let Err(error) = codex_hepta_operator_acceptance::run_fixed_inert_runner_v3() {
        eprintln!("{error}");
        std::process::exit(101);
    }
}

#[cfg(not(target_os = "macos"))]
fn main() {
    eprintln!("the fixed inert one-shot runner is available only on macOS");
    std::process::exit(101);
}
