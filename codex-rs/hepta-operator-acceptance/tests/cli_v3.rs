use std::process::Command;

const BINARY: &str = env!("CARGO_BIN_EXE_hepta-operator-acceptance-v3");

#[test]
fn help_shapes_exit_zero_on_stdout_only() {
    for argument in ["--help", "-h", "help"] {
        let output = Command::new(BINARY)
            .arg(argument)
            .output()
            .expect("run acceptance v3 binary");
        assert!(output.status.success(), "{argument} must exit zero");
        assert!(output.stderr.is_empty(), "{argument} must not write stderr");
        assert!(output.stdout.starts_with(b"usage:\n"));
        assert!(output.stdout.ends_with(b"\n"));
    }
}

#[test]
fn missing_invalid_and_overlong_help_shapes_fail_on_stderr_only() {
    for arguments in [
        Vec::<&str>::new(),
        vec!["unknown"],
        vec!["--help", "extra"],
        vec!["-h", "extra"],
        vec!["help", "extra"],
    ] {
        let output = Command::new(BINARY)
            .args(&arguments)
            .output()
            .expect("run acceptance v3 binary");
        assert!(!output.status.success(), "{arguments:?} must fail");
        assert!(
            output.stdout.is_empty(),
            "{arguments:?} must not write stdout"
        );
        assert!(output.stderr.starts_with(b"usage:\n"));
        assert!(output.stderr.ends_with(b"\n"));
    }
}
