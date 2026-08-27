#![cfg(unix)]

use std::error::Error;
use std::process::Command;

#[test]
fn artifact_gate_hands_off_to_exact_browser_binding() -> Result<(), Box<dyn Error>> {
    let output = Command::new(env!("CARGO_BIN_EXE_hepta-browser-c1-startup-bridge-trial"))
        .output()?;
    if !output.status.success() {
        return Err(format!(
            "startup bridge trial failed: status={} stderr={}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }
    let stdout = String::from_utf8(output.stdout)?;
    assert!(stdout.contains("ARTIFACT_TO_BROWSER_HANDOFF_QUALIFICATION_PASS"));
    assert!(stdout.contains("\"artifact_binding\":true"));
    assert!(stdout.contains("\"browser_session_binding\":true"));
    assert!(stdout.contains("\"source_pin_binding\":true"));
    assert!(stdout.contains("\"runtime_authority\":false"));
    assert!(stdout.contains("\"servo_linked\":false"));
    Ok(())
}

#[test]
fn hung_browser_stage_is_deadline_killed_after_artifact_binding() -> Result<(), Box<dyn Error>> {
    let output = Command::new(env!("CARGO_BIN_EXE_hepta-browser-c1-startup-bridge-trial"))
        .arg("--force-kill-trial")
        .output()?;
    if !output.status.success() {
        return Err(format!(
            "startup bridge forced-kill trial failed: status={} stderr={}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }
    let stdout = String::from_utf8(output.stdout)?;
    assert!(stdout.contains("ARTIFACT_TO_BROWSER_HANDOFF_FORCE_KILL_REAP_PASS"));
    assert!(stdout.contains("\"deadline_observed\":true"));
    assert!(stdout.contains("\"forced_kill\":true"));
    assert!(stdout.contains("\"reaped\":true"));
    assert!(stdout.contains("\"runtime_authority\":false"));
    Ok(())
}
