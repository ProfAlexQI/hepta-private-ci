use codex_hepta_shadow_qualification::BROWSER_WORKER_MODE_ARGUMENT;
use codex_hepta_shadow_qualification::run_qualification_browser_worker;
#[cfg(unix)]
use codex_hepta_shadow_qualification::BROWSER_WORKER_UNIX_MODE_ARGUMENT;
#[cfg(unix)]
use codex_hepta_shadow_qualification::run_unix_qualification_browser_worker;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut arguments = std::env::args().skip(1);
    let mode = arguments.next();
    if arguments.next().is_some() {
        reject_arguments();
    }

    let result = match mode.as_deref() {
        Some(BROWSER_WORKER_MODE_ARGUMENT) => run_qualification_browser_worker().await,
        #[cfg(unix)]
        Some(BROWSER_WORKER_UNIX_MODE_ARGUMENT) => {
            run_unix_qualification_browser_worker().await
        }
        _ => reject_arguments(),
    };
    if let Err(error) = result {
        eprintln!("hepta browser qualification worker failed: {error}");
        std::process::exit(1);
    }
}

fn reject_arguments() -> ! {
    eprintln!(
        "hepta-browser-worker-qualification accepts only a private qualification mode"
    );
    std::process::exit(2);
}
