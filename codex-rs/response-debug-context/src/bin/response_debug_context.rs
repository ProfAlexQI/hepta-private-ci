use std::io::Read;

fn main() -> std::process::ExitCode {
    let strict = std::env::args().any(|arg| arg == "--strict");
    let mut input = String::new();
    if let Err(err) = std::io::stdin().read_to_string(&mut input) {
        eprintln!("failed to read stdin: {err}");
        return std::process::ExitCode::from(2);
    }

    let export = codex_response_debug_context::summarize_rollout_context_debug_jsonl(&input);
    match serde_json::to_string_pretty(&export) {
        Ok(rendered) => println!("{rendered}"),
        Err(err) => {
            eprintln!("failed to serialize response debug export: {err}");
            return std::process::ExitCode::from(2);
        }
    }

    if strict && !export.audit.ok {
        std::process::ExitCode::from(1)
    } else {
        std::process::ExitCode::SUCCESS
    }
}
