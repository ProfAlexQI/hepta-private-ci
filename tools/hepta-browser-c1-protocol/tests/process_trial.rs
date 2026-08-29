use std::error::Error;
use std::process::Command;

#[cfg(unix)]
#[test]
fn inherited_pipe_process_trial_completes_without_servo_or_network() -> Result<(), Box<dyn Error>> {
    let output = Command::new(env!("CARGO_BIN_EXE_hepta-browser-c1-process-trial")).output()?;
    if !output.status.success() {
        return Err(std::io::Error::other(format!(
            "C1 process trial failed with {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ))
        .into());
    }
    let stdout = std::str::from_utf8(&output.stdout)?;
    assert_eq!(
        stdout,
        "{\"authority\":false,\"external_network\":false,\"process_boundary\":\"anonymous_inherited_pipes\",\"servo_linked\":false,\"status\":\"QUALIFICATION_ONLY_PROCESS_TRIAL_PASS\"}\n"
    );
    assert!(output.stderr.is_empty());
    Ok(())
}

#[test]
fn process_trial_rejects_unknown_arguments() -> Result<(), Box<dyn Error>> {
    let output = Command::new(env!("CARGO_BIN_EXE_hepta-browser-c1-process-trial"))
        .arg("--unexpected")
        .output()?;
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("unsupported C1 process-trial arguments")
    );
    Ok(())
}
