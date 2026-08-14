use codex_hepta_linux_v8_native::run_runtime_preflight_v8;

fn main() {
    let arguments = std::env::args_os()
        .skip(1)
        .map(std::ffi::OsString::into_string)
        .collect::<Result<Vec<_>, _>>();
    let arguments = match arguments {
        Ok(arguments) => arguments,
        Err(_) => {
            eprintln!("invalid Linux v8 native operation: command argument is not valid UTF-8");
            std::process::exit(1);
        }
    };
    match run_runtime_preflight_v8("recover", &arguments) {
        Ok(report) => println!("{report}"),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}
