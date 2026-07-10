use std::collections::BTreeMap;
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use anyhow::Context;
use anyhow::Result;
use sha2::Digest;
use sha2::Sha256;

#[derive(Debug, Default)]
struct ShellScriptAvailability {
    gate: Option<PathBuf>,
    report: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GateScriptKind {
    Gate,
    Report,
}

pub(crate) fn execute_gate(id: &str) -> Result<String> {
    execute_compatibility_script(id, GateScriptKind::Gate)
}

pub(crate) fn execute_report(id: &str) -> Result<String> {
    execute_compatibility_script(id, GateScriptKind::Report)
}

pub(crate) fn shell_gate_catalog_json() -> Result<String> {
    let repo_root = execution_repo_root()?;
    shell_gate_catalog_json_for_root(&repo_root)
}

pub(crate) fn shell_gate_snapshot_json() -> Result<String> {
    let repo_root = execution_repo_root()?;
    shell_gate_snapshot_json_for_root(&repo_root)
}

fn execute_compatibility_script(id: &str, kind: GateScriptKind) -> Result<String> {
    let repo_root = execution_repo_root()?;
    let script = resolve_compatibility_script(&repo_root, id, kind)?;
    let output = Command::new("/bin/bash")
        .arg(&script)
        .current_dir(&repo_root)
        .env("HEPTA_REPO_ROOT", &repo_root)
        .output()
        .with_context(|| format!("failed to execute {}", script.display()))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    if !output.status.success() {
        anyhow::bail!(
            "Hepta {} failed for {id} with status {}\n{}{}",
            kind.label(),
            output.status,
            stdout,
            stderr
        );
    }

    Ok(format!("{stdout}{stderr}"))
}

fn execution_repo_root() -> Result<PathBuf> {
    let repo_root = env::var_os("HEPTA_REPO_ROOT")
        .context("HEPTA_REPO_ROOT is required for source gate/report execution")?;
    fs::canonicalize(PathBuf::from(repo_root)).context("failed to canonicalize HEPTA_REPO_ROOT")
}

fn resolve_compatibility_script(
    repo_root: &Path,
    id: &str,
    kind: GateScriptKind,
) -> Result<PathBuf> {
    validate_id(id)?;

    let scripts_root = fs::canonicalize(repo_root.join("scripts"))
        .context("HEPTA_REPO_ROOT does not contain a scripts directory")?;
    let mut resolved = Vec::new();
    for candidate_name in kind.candidate_names(id) {
        let candidate = scripts_root.join(candidate_name);
        if !candidate.is_file() {
            continue;
        }
        let canonical = fs::canonicalize(&candidate)
            .with_context(|| format!("failed to canonicalize {}", candidate.display()))?;
        if !canonical.starts_with(&scripts_root) {
            anyhow::bail!(
                "Hepta gate script escapes scripts root: {}",
                canonical.display()
            );
        }
        resolved.push(canonical);
    }

    match resolved.as_slice() {
        [script] => Ok(script.clone()),
        [] => anyhow::bail!(
            "no {} compatibility script found for Hepta gate id: {id}",
            kind.label()
        ),
        scripts => anyhow::bail!(
            "ambiguous {} compatibility scripts for Hepta gate id {id}: {}",
            kind.label(),
            scripts
                .iter()
                .map(|script| script.display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        ),
    }
}

fn shell_gate_catalog_json_for_root(repo_root: &Path) -> Result<String> {
    let catalog = shell_gate_catalog_for_root(repo_root)?;

    let gate_count = catalog
        .values()
        .filter(|availability| availability.gate.is_some())
        .count();
    let report_count = catalog
        .values()
        .filter(|availability| availability.report.is_some())
        .count();
    let exact_pair_count = catalog
        .values()
        .filter(|availability| availability.gate.is_some() && availability.report.is_some())
        .count();
    let entries = catalog
        .into_iter()
        .map(|(id, availability)| {
            let gate = availability.gate.is_some();
            let report = availability.report.is_some();
            serde_json::json!({
                "id": id,
                "gate": gate,
                "report": report,
                "exact_pair": gate && report,
            })
        })
        .collect::<Vec<_>>();

    Ok(json_or_error(&serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": "ready",
        "runner": "hepta gate",
        "mode": "legacy_shell_compatibility_catalog",
        "gate_count": gate_count,
        "report_count": report_count,
        "exact_pair_count": exact_pair_count,
        "entry_count": entries.len(),
        "execution_requires_explicit_flag": true,
        "repo_root_required": true,
        "entries": entries,
    })))
}

fn shell_gate_snapshot_json_for_root(repo_root: &Path) -> Result<String> {
    let catalog = shell_gate_catalog_for_root(repo_root)?;
    let gate_count = catalog
        .values()
        .filter(|availability| availability.gate.is_some())
        .count();
    let report_count = catalog
        .values()
        .filter(|availability| availability.report.is_some())
        .count();
    let exact_pair_count = catalog
        .values()
        .filter(|availability| availability.gate.is_some() && availability.report.is_some())
        .count();
    let mut catalog_hasher = Sha256::new();
    let mut pair_id_hasher = Sha256::new();
    let mut entries = Vec::with_capacity(catalog.len());

    for (id, availability) in catalog {
        let gate = script_snapshot(repo_root, availability.gate.as_deref())?;
        let report = script_snapshot(repo_root, availability.report.as_deref())?;
        let exact_pair = gate.is_some() && report.is_some();
        let gate_path = gate
            .as_ref()
            .map(|snapshot| snapshot.relative_path.as_str())
            .unwrap_or("");
        let gate_sha256 = gate
            .as_ref()
            .map(|snapshot| snapshot.sha256.as_str())
            .unwrap_or("");
        let report_path = report
            .as_ref()
            .map(|snapshot| snapshot.relative_path.as_str())
            .unwrap_or("");
        let report_sha256 = report
            .as_ref()
            .map(|snapshot| snapshot.sha256.as_str())
            .unwrap_or("");

        catalog_hasher.update(id.as_bytes());
        catalog_hasher.update(b"\t");
        catalog_hasher.update(gate_path.as_bytes());
        catalog_hasher.update(b"\t");
        catalog_hasher.update(gate_sha256.as_bytes());
        catalog_hasher.update(b"\t");
        catalog_hasher.update(report_path.as_bytes());
        catalog_hasher.update(b"\t");
        catalog_hasher.update(report_sha256.as_bytes());
        catalog_hasher.update(b"\n");
        if exact_pair {
            pair_id_hasher.update(id.as_bytes());
            pair_id_hasher.update(b"\n");
        }

        entries.push(serde_json::json!({
            "id": id,
            "gate_path": gate.as_ref().map(|snapshot| snapshot.relative_path.as_str()),
            "gate_sha256": gate.as_ref().map(|snapshot| snapshot.sha256.as_str()),
            "report_path": report.as_ref().map(|snapshot| snapshot.relative_path.as_str()),
            "report_sha256": report.as_ref().map(|snapshot| snapshot.sha256.as_str()),
            "exact_pair": exact_pair,
        }));
    }

    Ok(json_or_error(&serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": "ready",
        "runner": "hepta gate",
        "mode": "legacy_shell_compatibility_parity_snapshot",
        "schema_version": "hepta_shell_gate_parity_snapshot_v1",
        "gate_count": gate_count,
        "report_count": report_count,
        "exact_pair_count": exact_pair_count,
        "entry_count": entries.len(),
        "catalog_sha256": hex_digest(catalog_hasher.finalize()),
        "exact_pair_id_sha256": hex_digest(pair_id_hasher.finalize()),
        "script_execution_performed": false,
        "side_effect_free": true,
        "entries": entries,
    })))
}

fn shell_gate_catalog_for_root(
    repo_root: &Path,
) -> Result<BTreeMap<String, ShellScriptAvailability>> {
    let scripts_root = fs::canonicalize(repo_root.join("scripts"))
        .context("HEPTA_REPO_ROOT does not contain a scripts directory")?;
    let mut catalog = BTreeMap::<String, ShellScriptAvailability>::new();
    for entry in fs::read_dir(&scripts_root).context("failed to read Hepta scripts directory")? {
        let entry = entry.context("failed to read Hepta scripts entry")?;
        if !entry
            .file_type()
            .context("failed to read Hepta scripts entry type")?
            .is_file()
        {
            continue;
        }
        let filename = entry.file_name().to_string_lossy().into_owned();
        let (id, kind) = if let Some(id) = filename.strip_suffix("-gate.sh") {
            (id, GateScriptKind::Gate)
        } else if let Some(id) = filename.strip_suffix("-report.sh") {
            (id, GateScriptKind::Report)
        } else {
            continue;
        };
        if validate_id(id).is_err() {
            continue;
        }
        let path = fs::canonicalize(entry.path())
            .with_context(|| format!("failed to canonicalize {filename}"))?;
        if !path.starts_with(&scripts_root) {
            anyhow::bail!(
                "Hepta shell catalog entry escapes scripts root: {}",
                path.display()
            );
        }
        let availability = catalog.entry(id.to_string()).or_default();
        let slot = match kind {
            GateScriptKind::Gate => &mut availability.gate,
            GateScriptKind::Report => &mut availability.report,
        };
        if let Some(existing) = slot {
            anyhow::bail!(
                "duplicate Hepta {} catalog entry for {id}: {} and {}",
                kind.label(),
                existing.display(),
                path.display()
            );
        }
        *slot = Some(path);
    }
    Ok(catalog)
}

#[derive(Debug)]
struct ScriptSnapshot {
    relative_path: String,
    sha256: String,
}

fn script_snapshot(repo_root: &Path, path: Option<&Path>) -> Result<Option<ScriptSnapshot>> {
    let Some(path) = path else {
        return Ok(None);
    };
    let relative_path = path
        .strip_prefix(repo_root)
        .with_context(|| format!("script is outside Hepta repo root: {}", path.display()))?
        .to_string_lossy()
        .into_owned();
    let bytes = fs::read(path).with_context(|| format!("failed to read {}", path.display()))?;
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    Ok(Some(ScriptSnapshot {
        relative_path,
        sha256: hex_digest(hasher.finalize()),
    }))
}

fn hex_digest(bytes: impl AsRef<[u8]>) -> String {
    let bytes = bytes.as_ref();
    let mut digest = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        let _ = write!(&mut digest, "{byte:02x}");
    }
    digest
}

fn validate_id(id: &str) -> Result<()> {
    if id.is_empty()
        || !id
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
    {
        anyhow::bail!("invalid Hepta gate id: {id}");
    }
    Ok(())
}

fn json_or_error(value: &serde_json::Value) -> String {
    serde_json::to_string(value)
        .unwrap_or_else(|err| format!(r#"{{"error":"gate runner serialization failed: {err}"}}"#))
}

impl GateScriptKind {
    fn label(self) -> &'static str {
        match self {
            Self::Gate => "gate",
            Self::Report => "report",
        }
    }

    fn candidate_names(self, id: &str) -> Vec<String> {
        match self {
            Self::Gate => vec![
                format!("{id}-gate.sh"),
                format!("{id}-route-gate.sh"),
                format!("{id}-lane-gate.sh"),
            ],
            Self::Report => vec![format!("{id}-report.sh")],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo_root() -> &'static Path {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("repo root")
    }

    #[test]
    fn resolves_compatibility_scripts_inside_the_repo_only() {
        let gate = resolve_compatibility_script(
            repo_root(),
            "hepta-full-live-activation-closure-index",
            GateScriptKind::Gate,
        )
        .expect("registered route gate");
        let report = resolve_compatibility_script(
            repo_root(),
            "hepta-systems-controlled-live-readiness-audit",
            GateScriptKind::Report,
        )
        .expect("legacy report compatibility wrapper");

        assert!(gate.ends_with("scripts/hepta-full-live-activation-closure-index-route-gate.sh"));
        assert!(
            report.ends_with("scripts/hepta-systems-controlled-live-readiness-audit-report.sh")
        );
        assert!(
            resolve_compatibility_script(repo_root(), "../escape", GateScriptKind::Gate)
                .expect_err("path traversal must fail")
                .to_string()
                .contains("invalid Hepta gate id")
        );
    }

    #[test]
    fn shell_catalog_derives_legacy_pair_counts_from_scripts() {
        let value: serde_json::Value = serde_json::from_str(
            &shell_gate_catalog_json_for_root(repo_root()).expect("shell catalog json"),
        )
        .expect("shell catalog value");

        assert_eq!(value["status"], "ready");
        assert_eq!(value["runner"], "hepta gate");
        assert!(value["gate_count"].as_u64().is_some_and(|count| count > 0));
        assert!(
            value["report_count"]
                .as_u64()
                .is_some_and(|count| count > 0)
        );
        assert!(
            value["exact_pair_count"]
                .as_u64()
                .is_some_and(|count| count > 0)
        );
        assert_eq!(value["execution_requires_explicit_flag"], true);
        assert_eq!(value["repo_root_required"], true);
    }

    #[test]
    fn shell_snapshot_is_deterministic_and_content_addressed() {
        let first: serde_json::Value = serde_json::from_str(
            &shell_gate_snapshot_json_for_root(repo_root()).expect("first shell snapshot"),
        )
        .expect("first shell snapshot value");
        let second: serde_json::Value = serde_json::from_str(
            &shell_gate_snapshot_json_for_root(repo_root()).expect("second shell snapshot"),
        )
        .expect("second shell snapshot value");
        let catalog: serde_json::Value = serde_json::from_str(
            &shell_gate_catalog_json_for_root(repo_root()).expect("shell catalog"),
        )
        .expect("shell catalog value");

        assert_eq!(
            first["schema_version"],
            "hepta_shell_gate_parity_snapshot_v1"
        );
        assert_eq!(first["gate_count"], catalog["gate_count"]);
        assert_eq!(first["report_count"], catalog["report_count"]);
        assert_eq!(first["exact_pair_count"], catalog["exact_pair_count"]);
        assert_eq!(first["entry_count"], catalog["entry_count"]);
        assert_eq!(first["catalog_sha256"], second["catalog_sha256"]);
        assert_eq!(
            first["exact_pair_id_sha256"],
            second["exact_pair_id_sha256"]
        );
        assert_eq!(first["script_execution_performed"], false);
        assert_eq!(first["side_effect_free"], true);
        assert_eq!(first["catalog_sha256"].as_str().map(str::len), Some(64));
        assert_eq!(
            first["exact_pair_id_sha256"].as_str().map(str::len),
            Some(64)
        );
        assert!(first["entries"].as_array().is_some_and(|entries| {
            entries.iter().all(|entry| {
                let gate_ready = entry["gate_path"].is_null()
                    || (entry["gate_path"]
                        .as_str()
                        .is_some_and(|path| path.starts_with("scripts/"))
                        && entry["gate_sha256"].as_str().map(str::len) == Some(64));
                let report_ready = entry["report_path"].is_null()
                    || (entry["report_path"]
                        .as_str()
                        .is_some_and(|path| path.starts_with("scripts/"))
                        && entry["report_sha256"].as_str().map(str::len) == Some(64));
                gate_ready && report_ready
            })
        }));
    }

    #[test]
    fn shell_snapshot_matches_the_frozen_pre_migration_baseline() {
        let snapshot: serde_json::Value = serde_json::from_str(
            &shell_gate_snapshot_json_for_root(repo_root()).expect("shell snapshot"),
        )
        .expect("shell snapshot value");
        let baseline_path =
            repo_root().join("docs/architecture/HEPTA_SHELL_GATE_PARITY_BASELINE_V1.json");
        let baseline: serde_json::Value = serde_json::from_str(
            &fs::read_to_string(&baseline_path)
                .with_context(|| format!("failed to read {}", baseline_path.display()))
                .expect("parity baseline"),
        )
        .expect("parity baseline value");

        for field in [
            "schema_version",
            "gate_count",
            "report_count",
            "exact_pair_count",
            "entry_count",
            "catalog_sha256",
            "exact_pair_id_sha256",
            "script_execution_performed",
            "side_effect_free",
        ] {
            assert_eq!(snapshot[field], baseline[field], "baseline field {field}");
        }
    }
}
