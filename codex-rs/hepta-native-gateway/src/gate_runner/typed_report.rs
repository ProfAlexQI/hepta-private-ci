use std::path::Path;

use anyhow::Result;

use super::ShellPairMigrationSpec;
use super::is_sha256;

const REPORT_SOURCE: &str = "codex-rs/hepta-runtime/src/typed_compat_report.rs";

pub(super) fn validate_source_report(spec: &ShellPairMigrationSpec) -> Result<()> {
    let valid = if spec.template == "typed_rust_report_v1" {
        spec.source_report == REPORT_SOURCE
    } else {
        spec.source_report.starts_with("scripts/")
            && (matches!(
                spec.template.as_str(),
                "captured_shell_compat_v1" | "legacy_workgraph_projection_v1"
            ) || spec.source_report.ends_with("-report.sh"))
    };
    if !valid {
        anyhow::bail!(
            "Hepta migrated gate pair {} has invalid source report: {}",
            spec.id,
            spec.source_report
        );
    }
    Ok(())
}

pub(super) fn validate_typed_report_binding(spec: &ShellPairMigrationSpec) -> Result<()> {
    if spec.template != "typed_rust_report_v1" {
        return Ok(());
    }
    let valid = spec.typed_report_runner.as_deref() == Some("scripts/hepta-typed-compat-report")
        && spec
            .typed_report_runner_sha256
            .as_deref()
            .is_some_and(is_sha256)
        && spec.typed_report_registry.as_deref()
            == Some("scripts/hepta-gate-typed-report-bindings-v2.json")
        && spec
            .typed_report_registry_sha256
            .as_deref()
            .is_some_and(is_sha256)
        && spec
            .typed_report_source_sha256
            .as_deref()
            .is_some_and(is_sha256)
        && spec.typed_report_cli_source.as_deref()
            == Some("codex-rs/hepta-runtime/src/bin/hepta-compat-report.rs")
        && spec
            .typed_report_cli_source_sha256
            .as_deref()
            .is_some_and(is_sha256);
    if !valid {
        anyhow::bail!(
            "Hepta typed Rust report pair {} has invalid runner binding",
            spec.id
        );
    }
    Ok(())
}

pub(super) fn source_report_is_bounded(
    repo_root: &Path,
    scripts_root: &Path,
    source_report: &Path,
    spec: &ShellPairMigrationSpec,
) -> bool {
    if spec.template == "typed_rust_report_v1" {
        source_report == repo_root.join(REPORT_SOURCE) && source_report.is_file()
    } else {
        source_report.starts_with(scripts_root)
    }
}
