use codex_hepta_shadow_qualification::BROWSER_WORKER_MODE_ARGUMENT;
use codex_hepta_shadow_qualification::run_qualification_browser_worker;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments.len() != 1 || arguments[0] != BROWSER_WORKER_MODE_ARGUMENT {
        eprintln!(
            "hepta-browser-worker-qualification accepts only the private qualification mode"
        );
        std::process::exit(2);
    }
    if let Err(error) = run_qualification_browser_worker().await {
        eprintln!("hepta browser qualification worker failed: {error}");
        std::process::exit(1);
    }
}
