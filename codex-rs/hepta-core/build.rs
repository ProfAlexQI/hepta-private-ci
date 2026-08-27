use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

const CONTROL_UI_BUNDLE_BOUNDARY: &[u8] =
    b"\n/* hepta-ui-v4-runtime-bundle-boundary */\n";
const SHA256_ABC: &str =
    "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";

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

    validate_sha256_implementation()?;

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

fn validate_sha256_implementation() -> Result<(), Box<dyn Error>> {
    if sha256_hex(b"abc") != SHA256_ABC {
        return Err("embedded SHA-256 implementation failed its known-answer test".into());
    }
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

fn sha256_hex(input: &[u8]) -> String {
    const INITIAL: [u32; 8] = [
        0x6A09_E667,
        0xBB67_AE85,
        0x3C6E_F372,
        0xA54F_F53A,
        0x510E_527F,
        0x9B05_688C,
        0x1F83_D9AB,
        0x5BE0_CD19,
    ];
    const K: [u32; 64] = [
        0x428A_2F98, 0x7137_4491, 0xB5C0_FBCF, 0xE9B5_DBA5,
        0x3956_C25B, 0x59F1_11F1, 0x923F_82A4, 0xAB1C_5ED5,
        0xD807_AA98, 0x1283_5B01, 0x2431_85BE, 0x550C_7DC3,
        0x72BE_5D74, 0x80DE_B1FE, 0x9BDC_06A7, 0xC19B_F174,
        0xE49B_69C1, 0xEFBE_4786, 0x0FC1_9DC6, 0x240C_A1CC,
        0x2DE9_2C6F, 0x4A74_84AA, 0x5CB0_A9DC, 0x76F9_88DA,
        0x983E_5152, 0xA831_C66D, 0xB003_27C8, 0xBF59_7FC7,
        0xC6E0_0BF3, 0xD5A7_9147, 0x06CA_6351, 0x1429_2967,
        0x27B7_0A85, 0x2E1B_2138, 0x4D2C_6DFC, 0x5338_0D13,
        0x650A_7354, 0x766A_0ABB, 0x81C2_C92E, 0x9272_2C85,
        0xA2BF_E8A1, 0xA81A_664B, 0xC24B_8B70, 0xC76C_51A3,
        0xD192_E819, 0xD699_0624, 0xF40E_3585, 0x106A_A070,
        0x19A4_C116, 0x1E37_6C08, 0x2748_774C, 0x34B0_BCB5,
        0x391C_0CB3, 0x4ED8_AA4A, 0x5B9C_CA4F, 0x682E_6FF3,
        0x748F_82EE, 0x78A5_636F, 0x84C8_7814, 0x8CC7_0208,
        0x90BE_FFFA, 0xA450_6CEB, 0xBEF9_A3F7, 0xC671_78F2,
    ];

    let bit_length = (input.len() as u64).wrapping_mul(8);
    let mut padded = input.to_vec();
    padded.push(0x80);
    while padded.len() % 64 != 56 {
        padded.push(0);
    }
    padded.extend_from_slice(&bit_length.to_be_bytes());

    let mut hash = INITIAL;
    for chunk in padded.chunks_exact(64) {
        let mut words = [0_u32; 64];
        for (index, bytes) in chunk.chunks_exact(4).take(16).enumerate() {
            words[index] = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        }
        for index in 16..64 {
            let sigma0 = words[index - 15].rotate_right(7)
                ^ words[index - 15].rotate_right(18)
                ^ (words[index - 15] >> 3);
            let sigma1 = words[index - 2].rotate_right(17)
                ^ words[index - 2].rotate_right(19)
                ^ (words[index - 2] >> 10);
            words[index] = words[index - 16]
                .wrapping_add(sigma0)
                .wrapping_add(words[index - 7])
                .wrapping_add(sigma1);
        }

        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = hash;
        for index in 0..64 {
            let sum1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let choose = (e & f) ^ ((!e) & g);
            let temp1 = h
                .wrapping_add(sum1)
                .wrapping_add(choose)
                .wrapping_add(K[index])
                .wrapping_add(words[index]);
            let sum0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let majority = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = sum0.wrapping_add(majority);

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        hash[0] = hash[0].wrapping_add(a);
        hash[1] = hash[1].wrapping_add(b);
        hash[2] = hash[2].wrapping_add(c);
        hash[3] = hash[3].wrapping_add(d);
        hash[4] = hash[4].wrapping_add(e);
        hash[5] = hash[5].wrapping_add(f);
        hash[6] = hash[6].wrapping_add(g);
        hash[7] = hash[7].wrapping_add(h);
    }

    let mut output = String::with_capacity(64);
    for value in hash {
        output.push_str(&format!("{value:08x}"));
    }
    output
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
