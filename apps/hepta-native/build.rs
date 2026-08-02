use std::collections::BTreeSet;
use std::fs;
use std::io::Write;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;

const ROBRIX_UPSTREAM_FULL_SHA: &str = "a5a664da569c577ab1a3e5a33f45dcc9364954a0";

const CANONICAL_ASSET_MANIFEST: &str = "canonical-assets-v1.tsv";
const CANONICAL_ASSET_CONTRACT: &[(&str, &str, &str, &str, usize)] = &[(
    "google_play_icon_512",
    "apps/hepta-native/resources/icon_512.png",
    "apps/hepta-native/packaging/icon_google_play_512.png",
    "ddaea2f8da2463a99b7c51ff0959ce746daebdee53190e80a78b361c27fc4d9a",
    222_778,
)];

fn safe_relative_path(raw: &str) -> Option<PathBuf> {
    let path = Path::new(raw);
    if raw.is_empty()
        || path
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        return None;
    }
    Some(path.to_path_buf())
}

fn materialize_canonical_assets() {
    let manifest_dir = PathBuf::from(
        std::env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is required"),
    );
    let repo_root = manifest_dir.join("../..");
    let manifest_path = manifest_dir.join(CANONICAL_ASSET_MANIFEST);
    let manifest = fs::read_to_string(&manifest_path).expect("read canonical asset manifest");
    println!("cargo:rerun-if-changed={}", manifest_path.display());
    let mut materialized_keys = BTreeSet::new();

    for (index, line) in manifest.lines().enumerate() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let fields = line.split('\t').collect::<Vec<_>>();
        assert_eq!(fields.len(), 5, "invalid canonical asset row {}", index + 1);
        let expected = CANONICAL_ASSET_CONTRACT
            .iter()
            .find(|record| record.0 == fields[0])
            .expect("known canonical asset key");
        assert!(
            materialized_keys.insert(fields[0]),
            "duplicate canonical asset key"
        );
        assert_eq!(
            (fields[1], fields[2], fields[3]),
            (expected.1, expected.2, expected.3),
            "canonical asset contract drifted"
        );
        let source_relative = safe_relative_path(fields[1]).expect("safe canonical asset path");
        let target_relative = safe_relative_path(fields[2]).expect("safe materialized asset path");
        assert_ne!(
            source_relative, target_relative,
            "asset source and target differ"
        );
        let expected_bytes = fields[4]
            .parse::<usize>()
            .expect("canonical asset byte count is an integer");
        assert_eq!(expected_bytes, expected.4, "canonical asset size drifted");
        let source = repo_root.join(source_relative);
        let target = repo_root.join(target_relative);
        println!("cargo:rerun-if-changed={}", source.display());
        println!("cargo:rerun-if-changed={}", target.display());

        let source_metadata = fs::symlink_metadata(&source).expect("canonical asset metadata");
        assert!(
            source_metadata.is_file() && !source_metadata.file_type().is_symlink(),
            "canonical asset must be a regular non-symlink file"
        );
        let content = fs::read(&source).expect("read canonical asset");
        assert_eq!(
            content.len(),
            expected_bytes,
            "canonical asset byte count drifted"
        );
        if let Ok(metadata) = fs::symlink_metadata(&target) {
            assert!(
                !metadata.file_type().is_symlink(),
                "materialized asset must not be a symlink"
            );
        }
        if fs::read(&target).is_ok_and(|existing| existing == content) {
            continue;
        }
        fs::create_dir_all(target.parent().expect("materialized asset parent"))
            .expect("create materialized asset parent");
        let temporary = target.with_extension(format!("tmp-{}-{index}", std::process::id()));
        let mut output = fs::File::create(&temporary).expect("create temporary materialized asset");
        output
            .write_all(&content)
            .expect("write materialized asset");
        output.sync_all().expect("sync materialized asset");
        drop(output);
        if fs::rename(&temporary, &target).is_err() {
            let _ = fs::remove_file(&target);
            fs::rename(&temporary, &target).expect("install materialized asset");
        }
    }
    assert_eq!(
        materialized_keys.len(),
        CANONICAL_ASSET_CONTRACT.len(),
        "canonical asset manifest is incomplete"
    );
}
fn main() {
    materialize_canonical_assets();

    // Detect `cargo packager` builds using the `CARGO_PACKAGER_FORMAT` env var.
    println!("cargo::rustc-check-cfg=cfg(packaging_build)");
    println!("cargo:rerun-if-env-changed=CARGO_PACKAGER_FORMAT");
    if std::env::var_os("CARGO_PACKAGER_FORMAT").is_some() {
        println!("cargo:rustc-cfg=packaging_build");
    }

    // Note: `#[cfg(windows)]` checks the *host* OS, not the *target*.
    // We must check the target env at runtime to avoid running this
    // when cross-compiling (e.g., building for Android on a Windows CI runner).
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os == "windows" {
        #[cfg(windows)]
        {
            let mut res = winresource::WindowsResource::new();
            res.set_icon("resources/icon.ico");
            // Explicit VERSIONINFO fields. Without these, Windows shows
            // "Unknown publisher" in the UAC/SmartScreen install prompt
            // (CompanyName/LegalCopyright are empty by default), and the
            // ProductName/FileDescription otherwise fall back to the crate name.
            res.set("CompanyName", "Hepta");
            res.set("ProductName", "Hepta");
            res.set(
                "FileDescription",
                "Hepta - Matrix chat and agent collaboration",
            );
            res.set(
                "LegalCopyright",
                "Copyright 2026 Hepta contributors; Robrix portions copyright Project Robius",
            );
            res.compile().expect("Failed to compile Windows resources");
        }
    }

    // Expose the downstream build revision separately from the frozen Robrix baseline.
    println!("cargo:rerun-if-changed=Cargo.lock");
    let (sdk_version, sdk_git_rev, sdk_url) = read_matrix_sdk_info();
    println!("cargo:rustc-env=MATRIX_SDK_VERSION={sdk_version}");
    println!("cargo:rustc-env=MATRIX_SDK_GIT_REV={sdk_git_rev}");
    println!("cargo:rustc-env=MATRIX_SDK_URL={sdk_url}");

    let (hepta_git_rev, hepta_url) = read_hepta_git_info();
    println!("cargo:rustc-env=HEPTA_GIT_COMMIT_HASH={hepta_git_rev}");
    println!("cargo:rustc-env=HEPTA_GIT_COMMIT_URL={hepta_url}");

    let robrix_short_rev: String = ROBRIX_UPSTREAM_FULL_SHA.chars().take(8).collect();
    println!("cargo:rustc-env=ROBRIX_GIT_COMMIT_HASH={robrix_short_rev}");
    println!(
        "cargo:rustc-env=ROBRIX_GIT_COMMIT_URL=https://github.com/project-robius/robrix/commit/{ROBRIX_UPSTREAM_FULL_SHA}"
    );

    println!("cargo:rerun-if-env-changed=TESTFLIGHT_BUILD_NUMBER");
    let testflight_build = std::env::var("TESTFLIGHT_BUILD_NUMBER").unwrap_or_default();
    println!("cargo:rustc-env=TESTFLIGHT_BUILD_NUMBER={testflight_build}");
}

/// Returns the current Hepta revision as a commit hash and permalink.
fn read_hepta_git_info() -> (String, String) {
    let manifest_dir =
        PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap_or_else(|| ".".into()));
    let repo_root = manifest_dir.join("../..");
    let git_output = |arguments: &[&str]| {
        std::process::Command::new("git")
            .arg("-C")
            .arg(&repo_root)
            .args(arguments)
            .output()
            .ok()
            .filter(|output| output.status.success())
            .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
    };

    // Worktrees keep HEAD in their per-worktree git dir and branch refs in the
    // common git dir. Watch both, plus packed-refs, so a reused Cargo target can
    // never retain a revision from another checkout.
    let git_dir = git_output(&["rev-parse", "--absolute-git-dir"]).map(PathBuf::from);
    let common_dir = git_output(&["rev-parse", "--git-common-dir"]).map(|path| {
        let path = PathBuf::from(path);
        if path.is_absolute() {
            path
        } else {
            repo_root.join(path)
        }
    });
    let head_path = git_dir.as_ref().map(|directory| directory.join("HEAD"));
    if let Some(path) = &head_path {
        println!("cargo:rerun-if-changed={}", path.display());
    }
    if let (Some(path), Some(common_dir)) = (&head_path, &common_dir) {
        if let Ok(head) = std::fs::read_to_string(path) {
            if let Some(branch_ref) = head.trim().strip_prefix("ref: ") {
                println!(
                    "cargo:rerun-if-changed={}",
                    common_dir.join(branch_ref).display()
                );
            }
        }
        println!(
            "cargo:rerun-if-changed={}",
            common_dir.join("packed-refs").display()
        );
    } else if let Some(common_dir) = &common_dir {
        println!(
            "cargo:rerun-if-changed={}",
            common_dir.join("packed-refs").display()
        );
    }

    let Some(full_sha) = git_output(&["rev-parse", "HEAD"]) else {
        return (String::new(), String::new());
    };
    if full_sha.len() < 8 {
        return (String::new(), String::new());
    }
    let short_rev: String = full_sha.chars().take(8).collect();
    let url = format!("https://github.com/ProfAlexQI/Hepta/commit/{full_sha}");
    (short_rev, url)
}

/// Parses Cargo.lock to find the resolved version of `matrix-sdk`.
///
/// Returns `(version, short_git_rev, url)`.
fn read_matrix_sdk_info() -> (String, String, String) {
    let Ok(lockfile_text) = std::fs::read_to_string("Cargo.lock") else {
        return (String::new(), String::new(), String::new());
    };
    let Ok(lockfile) = toml::from_str::<toml::Value>(&lockfile_text) else {
        return (String::new(), String::new(), String::new());
    };

    let Some(pkg) = lockfile
        .get("package")
        .and_then(|p| p.as_array())
        .and_then(|pkgs| {
            pkgs.iter()
                .find(|p| p.get("name").and_then(|n| n.as_str()) == Some("matrix-sdk"))
        })
    else {
        return (String::new(), String::new(), String::new());
    };

    let version = pkg
        .get("version")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let source = pkg.get("source").and_then(|s| s.as_str()).unwrap_or("");

    // Git sources look like `git+<repo-url>?<query>#<full-commit>`.
    // The repo URL is the prefix before `?` or `#`; the commit is after `#`.
    let (git_rev, url) = if let Some(rest) = source.strip_prefix("git+") {
        let (left, full_commit) = rest.rsplit_once('#').unwrap_or((rest, ""));
        let base = left.split_once('?').map_or(left, |(b, _)| b);
        let short_rev: String = full_commit.chars().take(8).collect();
        let url = if full_commit.is_empty() {
            base.to_string()
        } else {
            format!("{base}/tree/{full_commit}")
        };
        (short_rev, url)
    } else if !version.is_empty() {
        // Registry/path/other sources: fall back to the crates.io URL.
        (
            String::new(),
            format!("https://crates.io/crates/matrix-sdk/{version}"),
        )
    } else {
        (String::new(), String::new())
    };

    (version, git_rev, url)
}
