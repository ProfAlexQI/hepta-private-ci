use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use anyhow::Context;
use anyhow::Result;

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
    let scripts_root = fs::canonicalize(repo_root.join("scripts"))
        .context("HEPTA_REPO_ROOT does not contain a scripts directory")?;
    let mut catalog = BTreeMap::<String, (bool, bool)>::new();
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
        let availability = catalog.entry(id.to_string()).or_default();
        match kind {
            GateScriptKind::Gate => availability.0 = true,
            GateScriptKind::Report => availability.1 = true,
        }
    }

    let gate_count = catalog.values().filter(|(gate, _)| *gate).count();
    let report_count = catalog.values().filter(|(_, report)| *report).count();
    let exact_pair_count = catalog
        .values()
        .filter(|(gate, report)| *gate && *report)
        .count();
    let entries = catalog
        .into_iter()
        .map(|(id, (gate, report))| {
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
}
