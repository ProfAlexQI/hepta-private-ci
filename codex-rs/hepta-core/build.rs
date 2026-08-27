use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use sha2::Digest;
use sha2::Sha256;

const CONTROL_UI_BUNDLE_BOUNDARY: &[u8] =
    b"\n/* hepta-ui-v4-runtime-bundle-boundary */\n";

fn main() {
    if let Err(error) = run() {
        panic!("hepta-core build failed: {error}");
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=../../.git/HEAD");
    println!("cargo:rerun-if-changed=../../.git/index");
    println!("cargo:rerun-if-changed=../../.git/packed-refs");
    if let Some(branch) = current_branch() {
        println!("cargo:rerun-if-changed=../../.git/refs/heads/{branch}");
    }

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let repository_root = manifest_dir.join("../..");
    let control_ui_dir = repository_root.join("apps/hepta-control-ui");
    let base_js_path = control_ui_dir.join("control-ui.js");
    let runtime_js_path = control_ui_dir.join("control-ui-v4-runtime.js");

    println!("cargo:rerun-if-changed={}", base_js_path.display());
    println!("cargo:rerun-if-changed={}", runtime_js_path.display());

    generate_control_ui_bundle(
        &base_js_path,
        &runtime_js_path,
        &PathBuf::from(env::var("OUT_DIR")?),
    )?;

    let git_head = Command::new("git")
        .args(["rev-parse", "--short=12", "HEAD"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "unknown".to_owned());

    println!("cargo:rustc-env=HEPTA_GIT_HEAD={git_head}");
    Ok(())
}

fn generate_control_ui_bundle(
    base_js_path: &Path,
    runtime_js_path: &Path,
    out_dir: &Path,
) -> Result<(), Box<dyn Error>> {
    let base_js = fs::read(base_js_path)?;
    let runtime_js = fs::read(runtime_js_path)?;
    if base_js.is_empty() {
        return Err("Control UI base JavaScript is empty".into());
    }
    if runtime_js.is_empty() {
        return Err("Control UI v4 runtime JavaScript is empty".into());
    }

    let mut bundle = Vec::with_capacity(
        base_js.len() + CONTROL_UI_BUNDLE_BOUNDARY.len() + runtime_js.len(),
    );
    bundle.extend_from_slice(&base_js);
    bundle.extend_from_slice(CONTROL_UI_BUNDLE_BOUNDARY);
    bundle.extend_from_slice(&runtime_js);

    let base_sha256 = sha256_hex(&base_js);
    let runtime_sha256 = sha256_hex(&runtime_js);
    let bundle_sha256 = sha256_hex(&bundle);

    fs::write(out_dir.join("control-ui.bundle.js"), &bundle)?;
    fs::write(
        out_dir.join("control_ui_bundle_metadata.rs"),
        format!(
            "pub const CONTROL_UI_BASE_JS_SHA256: &str = \"{base_sha256}\";\n\
             pub const CONTROL_UI_V4_RUNTIME_JS_SHA256: &str = \"{runtime_sha256}\";\n\
             pub const CONTROL_UI_JS_SHA256: &str = \"{bundle_sha256}\";\n\
             pub const CONTROL_UI_JS_ETAG: &str = \"\\\"sha256-{bundle_sha256}\\\"\";\n\
             pub const CONTROL_UI_V4_RUNTIME_BOUND: bool = true;\n"
        ),
    )?;
    Ok(())
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn current_branch() -> Option<String> {
    Command::new("git")
        .args(["symbolic-ref", "--short", "HEAD"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}
