use std::collections::BTreeSet;

use serde::Deserialize;

const EXPECTED_UPSTREAM_CUTOFF: &str = "74004b5397b24662a87a5264a6ae80664168c7f3";

#[derive(Debug, Deserialize)]
struct CallersManifest {
    schema_version: u32,
    rule: String,
    candidate_base: String,
    frozen_oracle: String,
    surface: Vec<Surface>,
    excluded: Vec<Excluded>,
    qualification: Vec<toml::Value>,
}

#[derive(Debug, Deserialize)]
struct Surface {
    #[serde(rename = "crate")]
    crate_name: String,
    role: String,
    public_items: Vec<String>,
    product_callers: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Excluded {
    #[serde(rename = "crate")]
    crate_name: String,
    reason: String,
}

fn manifest() -> CallersManifest {
    toml::from_str(include_str!("../CALLERS.toml")).expect("CALLERS.toml must parse")
}

fn surface<'a>(manifest: &'a CallersManifest, crate_name: &str) -> &'a Surface {
    manifest
        .surface
        .iter()
        .find(|surface| surface.crate_name == crate_name)
        .unwrap_or_else(|| panic!("missing product surface {crate_name}"))
}

#[test]
fn callers_manifest_schema_and_cutoff_are_frozen() {
    let manifest = manifest();

    assert_eq!(manifest.schema_version, 1);
    assert_eq!(manifest.candidate_base, EXPECTED_UPSTREAM_CUTOFF);
    assert_eq!(manifest.frozen_oracle.len(), 40);
    assert!(
        manifest
            .frozen_oracle
            .chars()
            .all(|ch| ch.is_ascii_hexdigit())
    );
    assert!(manifest.rule.contains("qualification-only"));
    assert!(!manifest.qualification.is_empty());
}

#[test]
fn callers_manifest_entries_are_unique_and_well_formed() {
    let manifest = manifest();
    let mut surfaces = BTreeSet::new();
    let mut excluded = BTreeSet::new();

    for surface in &manifest.surface {
        assert!(
            surfaces.insert(surface.crate_name.as_str()),
            "duplicate surface {}",
            surface.crate_name
        );
        assert!(
            !surface.role.is_empty(),
            "{} has an empty role",
            surface.crate_name
        );
        assert!(
            !surface.public_items.is_empty(),
            "{} has no public items",
            surface.crate_name
        );
        assert!(
            !surface.product_callers.is_empty(),
            "{} has no product callers",
            surface.crate_name
        );

        let mut public_items = BTreeSet::new();
        for item in &surface.public_items {
            assert!(
                public_items.insert(item),
                "{} repeats public item {item}",
                surface.crate_name
            );
        }

        let mut product_callers = BTreeSet::new();
        for caller in &surface.product_callers {
            assert!(
                product_callers.insert(caller),
                "{} repeats caller {caller}",
                surface.crate_name
            );
            assert!(
                caller.starts_with("codex-rs/"),
                "caller is not repo-relative: {caller}"
            );
            assert!(
                caller.ends_with(".rs"),
                "caller is not a Rust source path: {caller}"
            );
            assert!(
                !caller.contains(".."),
                "caller escapes the repository: {caller}"
            );
        }
    }

    for entry in &manifest.excluded {
        assert!(
            excluded.insert(entry.crate_name.as_str()),
            "duplicate exclusion {}",
            entry.crate_name
        );
        assert!(
            !entry.reason.is_empty(),
            "{} has no exclusion reason",
            entry.crate_name
        );
    }

    assert!(
        surfaces.is_disjoint(&excluded),
        "a crate cannot be both product and excluded"
    );
}

#[test]
fn callers_manifest_records_the_live_product_chain() {
    let manifest = manifest();

    let paths = surface(&manifest, "codex-hepta-paths");
    assert_eq!(paths.public_items, ["HeptaStateRoot", "HeptaStateLayout"]);
    assert_eq!(
        paths.product_callers,
        [
            "codex-rs/hepta-runtime/src/lib.rs",
            "codex-rs/hepta-native-gateway/src/lib.rs",
        ]
    );

    let runtime = surface(&manifest, "codex-hepta-runtime");
    assert_eq!(runtime.public_items, ["HeptaRuntime", "RuntimeStatus"]);
    assert_eq!(
        runtime.product_callers,
        ["codex-rs/hepta-native-gateway/src/lib.rs"]
    );

    let gateway = surface(&manifest, "codex-hepta-native-gateway");
    assert_eq!(
        gateway.public_items,
        [
            "print_live_shell_contract_if_requested",
            "run_serve_ui_if_requested",
        ]
    );
    assert_eq!(gateway.product_callers, ["codex-rs/cli/src/main.rs"]);
}

#[test]
fn qualification_crates_are_excluded_from_product_callers() {
    let manifest = manifest();
    let excluded = manifest
        .excluded
        .iter()
        .map(|entry| entry.crate_name.as_str())
        .collect::<BTreeSet<_>>();

    assert!(excluded.contains("codex-hepta-shadow-qualification"));
    assert!(excluded.contains("codex-hepta-operator-acceptance"));
    assert!(manifest.surface.iter().all(|surface| {
        surface.crate_name != "codex-hepta-shadow-qualification"
            && surface.crate_name != "codex-hepta-operator-acceptance"
    }));
}
