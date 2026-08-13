use std::ffi::OsString;

use codex_hepta_operator_acceptance::frozen_tool::run_cli;

fn main() {
    match run_cli(std::env::args_os().collect::<Vec<OsString>>()) {
        Ok(json) => println!("{json}"),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}
