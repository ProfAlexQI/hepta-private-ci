use std::collections::BTreeSet;
use std::fs;
use std::io::Write;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;

const CANONICAL_ASSET_MANIFEST: &str = "canonical-assets-v1.tsv";
const CANONICAL_ASSET_CONTRACT: &[(&str, &str, &str, &str, usize)] = &[
    (
        "control_ui_glass_k",
        "apps/hepta-control-ui/assets/k.png",
        "apps/hepta-native/resources/img/hepta-glass-k.png",
        "a54bc0d6352c3130d2d22b7df80f1fabaa94f5098fec12046e4f262e6d0d7c28",
        2_499_731,
    ),
    (
        "google_play_icon_512",
        "apps/hepta-native/resources/icon_512.png",
        "apps/hepta-native/packaging/icon_google_play_512.png",
        "ddaea2f8da2463a99b7c51ff0959ce746daebdee53190e80a78b361c27fc4d9a",
        222_778,
    ),
];

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
    // Note: `#[cfg(windows)]` checks the *host* OS, not the *target*.
    // We must check the target env at runtime to avoid running this
    // when cross-compiling (e.g., building for Android on a Windows CI runner).
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os == "windows" {
        #[cfg(windows)]
        {
            let mut res = winresource::WindowsResource::new();
            res.set_icon("resources/icon.ico");
            res.compile().expect("Failed to compile Windows resources");
        }
    }
}
