use codex_hepta_operator_acceptance::v3::run_cli_v3;

fn main() {
    match run_cli_v3(std::env::args_os().collect()) {
        Ok(json) => println!("{json}"),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}
