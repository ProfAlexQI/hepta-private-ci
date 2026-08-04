use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};

fn main() {
    if let Err(error) = run() {
        eprintln!("hepta-legacy-route-window: {error:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let mut events = None::<PathBuf>;
    let mut expected_head = None::<String>;
    let mut output = None::<PathBuf>;
    let mut authentication_key_file = None::<PathBuf>;
    let mut allow_blocked = false;
    let mut args = env::args().skip(1);
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--events" => {
                events = Some(PathBuf::from(
                    args.next().context("--events requires a path")?,
                ))
            }
            "--expected-head" => {
                expected_head = Some(args.next().context("--expected-head requires a value")?)
            }
            "--output" => {
                output = Some(PathBuf::from(
                    args.next().context("--output requires a path")?,
                ))
            }
            "--authentication-key-file" => {
                authentication_key_file = Some(PathBuf::from(
                    args.next()
                        .context("--authentication-key-file requires a path")?,
                ))
            }
            "--allow-blocked" => allow_blocked = true,
            "--help" => {
                println!(
                    "Usage: hepta-legacy-route-window --events PATH --expected-head SHA [--authentication-key-file PATH] [--output PATH] [--allow-blocked]"
                );
                return Ok(());
            }
            _ => anyhow::bail!("unknown argument: {argument}"),
        }
    }
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("system clock precedes Unix epoch")?
        .as_millis()
        .min(u128::from(u64::MAX)) as u64;
    let events = events.context("--events is required")?;
    let expected_head = expected_head.context("--expected-head is required")?;
    let summary = if let Some(key_path) = authentication_key_file {
        hepta_native_gateway::legacy_route_window::summarize_path_with_authentication_key_file(
            &events,
            &expected_head,
            now,
            &key_path,
        )?
    } else {
        hepta_native_gateway::legacy_route_window::summarize_path(&events, &expected_head, now)?
    };
    let serialized =
        serde_json::to_vec_pretty(&summary).context("serialize legacy route window")?;
    if let Some(path) = output {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).context("create output directory")?;
        }
        fs::write(path, &serialized).context("write legacy route window summary")?;
    }
    println!("{}", String::from_utf8(serialized).expect("JSON is UTF-8"));
    if !allow_blocked && !summary.decision.eligible {
        anyhow::bail!("observation window is not retirement-eligible");
    }
    Ok(())
}
