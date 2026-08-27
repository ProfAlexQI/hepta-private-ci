#![cfg(unix)]

use std::error::Error;
use std::process::Command;

#[test]
fn artifact_bound_trial_binds_bytes_and_shuts_down_cleanly() -> Result<(), Box<dyn Error>> {
    let output = Command::new(env!("CARGO_BIN_EXE_hepta-browser-c1-artifact-bound-trial"))
        .output()?;
    if !output.status.success() {
        return Err(std::io::Error::other(format!(
            "artifact-bound trial failed: status={} stderr={}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ))
        .into());
    }
    let stdout = String::from_utf8(output.stdout)?;
    assert!(stdout.contains("ARTIFACT_BOUND_QUALIFICATION_TRIAL_PASS"));
    assert!(stdout.contains("\"artifact_binding\":true"));
    assert!(stdout.contains("\"runtime_authority\":false"));
    assert!(stdout.contains("\"servo_linked\":false"));
    Ok(())
}

#[test]
fn hung_artifact_bound_worker_is_deadline_killed_and_reaped() -> Result<(), Box<dyn Error>> {
    let output = Command::new(env!("CARGO_BIN_EXE_hepta-browser-c1-artifact-bound-trial"))
        .arg("--force-kill-trial")
        .output()?;
    if !output.status.success() {
        return Err(std::io::Error::other(format!(
            "forced-kill trial failed: status={} stderr={}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ))
        .into());
    }
    let stdout = String::from_utf8(output.stdout)?;
    assert!(stdout.contains("ARTIFACT_BOUND_FORCE_KILL_REAP_PASS"));
    assert!(stdout.contains("\"deadline_observed\":true"));
    assert!(stdout.contains("\"forced_kill\":true"));
    assert!(stdout.contains("\"reaped\":true"));
    Ok(())
}
