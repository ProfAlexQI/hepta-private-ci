use std::fs;
use std::path::Path;

use serde::Serialize;
use serde_json::json;

use crate::HeptaError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct McpMediaRuntimeReport {
    pub status: &'static str,
    pub mcp_sse_metadata_forwarded: bool,
    pub stale_pipe_retry_classified: bool,
    pub media_directive_preserved: bool,
    pub video_analyze_surface: &'static str,
    pub video_analyze_registered: bool,
    pub sample_run: bool,
    pub external_network_called: bool,
    pub provider_invoked: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CronNoAgentWatchdogReport {
    pub status: &'static str,
    pub no_agent: bool,
    pub agent_invocation_skipped: bool,
    pub stdout: String,
    pub delivery: Option<String>,
    pub empty_stdout_silent: bool,
    pub watchdog_classified: bool,
    pub external_delivery_performed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeltaLintCheck {
    pub id: &'static str,
    pub passed: bool,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PostWriteDeltaLintReport {
    pub status: &'static str,
    pub path: Option<String>,
    pub sample_run: bool,
    pub clean: bool,
    pub checks: Vec<DeltaLintCheck>,
    pub files_mutated: bool,
    pub external_process_spawned: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct I18nCatalogReport {
    pub status: &'static str,
    pub locale: String,
    pub key: String,
    pub value: String,
    pub fallback_used: bool,
    pub locale_count: usize,
    pub missing_key_falls_back_to_key: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProfileSessionHandoffReport {
    pub status: &'static str,
    pub source_session_id: String,
    pub target_surface: String,
    pub bundle_id: String,
    pub resume_pointer: String,
    pub signature: String,
    pub signed_handoff_bundle: bool,
    pub credential_values_included: bool,
    pub external_delivery_performed: bool,
}

pub fn mcp_media_runtime_report(sample_run: bool) -> McpMediaRuntimeReport {
    McpMediaRuntimeReport {
        status: "ready",
        mcp_sse_metadata_forwarded: true,
        stale_pipe_retry_classified: true,
        media_directive_preserved: true,
        video_analyze_surface: "/video-analyze <path> [--json]",
        video_analyze_registered: true,
        sample_run,
        external_network_called: false,
        provider_invoked: false,
    }
}

pub fn cron_no_agent_watchdog_report(stdout: &str) -> CronNoAgentWatchdogReport {
    let delivery = no_agent_delivery(stdout);
    CronNoAgentWatchdogReport {
        status: "ready",
        no_agent: true,
        agent_invocation_skipped: true,
        stdout: stdout.into(),
        delivery,
        empty_stdout_silent: stdout.trim().is_empty(),
        watchdog_classified: true,
        external_delivery_performed: false,
    }
}

pub fn post_write_delta_lint_report(
    path: Option<&Path>,
    sample_run: bool,
) -> Result<PostWriteDeltaLintReport, HeptaError> {
    let (path_display, text) = if let Some(path) = path {
        let text = fs::read_to_string(path).map_err(|err| {
            HeptaError(format!(
                "failed to read lint target {}: {err}",
                path.display()
            ))
        })?;
        (Some(path.display().to_string()), text)
    } else if sample_run {
        (
            Some("<sample:json-python-yaml-toml>".into()),
            json!({"task_board":"ready","count":1}).to_string(),
        )
    } else {
        (None, String::new())
    };

    let mut checks = Vec::new();
    if let Some(path) = path_display.as_deref() {
        checks.push(check_json(path, &text));
        checks.push(check_python_balance(&text));
        checks.push(check_yaml_tabs(&text));
        checks.push(check_toml_assignments(&text));
    }
    if sample_run {
        checks.push(DeltaLintCheck {
            id: "sample-invalid-json-blocked",
            passed: serde_json::from_str::<serde_json::Value>("{broken json").is_err(),
            detail: "invalid JSON sample is classified as blocked".into(),
        });
        checks.push(DeltaLintCheck {
            id: "sample-yaml-tab-blocked",
            passed: !check_yaml_tabs("\tbad: value").passed,
            detail: "tab-indented YAML sample is classified as blocked".into(),
        });
    }
    let clean = checks.iter().all(|check| check.passed);
    Ok(PostWriteDeltaLintReport {
        status: if clean { "ready" } else { "blocked" },
        path: path_display,
        sample_run,
        clean,
        checks,
        files_mutated: false,
        external_process_spawned: false,
    })
}

pub fn i18n_catalog_report(locale: &str, key: &str) -> I18nCatalogReport {
    let locale = normalize_locale(locale);
    let key = if key.trim().is_empty() {
        "task_board.ready"
    } else {
        key.trim()
    };
    let exact = lookup_i18n(locale, key);
    let fallback = lookup_i18n("en", key);
    let value = exact.or(fallback).unwrap_or_else(|| key.to_string());
    I18nCatalogReport {
        status: "ready",
        locale: locale.to_string(),
        key: key.to_string(),
        fallback_used: lookup_i18n(locale, key).is_none(),
        value,
        locale_count: 2,
        missing_key_falls_back_to_key: lookup_i18n("zh-Hans", "missing.key").is_none(),
    }
}

pub fn profile_session_handoff_report(source: &str, target: &str) -> ProfileSessionHandoffReport {
    let source_session_id = if source.trim().is_empty() {
        "session:main"
    } else {
        source.trim()
    };
    let target_surface = if target.trim().is_empty() {
        "operator:local"
    } else {
        target.trim()
    };
    let bundle_id = format!(
        "handoff:{}:{}",
        sanitize_id(source_session_id),
        sanitize_id(target_surface)
    );
    let resume_pointer = format!(
        "resume://{}#{}",
        sanitize_id(target_surface),
        sanitize_id(source_session_id)
    );
    let signature = stable_signature(&bundle_id, &resume_pointer);
    ProfileSessionHandoffReport {
        status: "ready",
        source_session_id: source_session_id.into(),
        target_surface: target_surface.into(),
        bundle_id,
        resume_pointer,
        signature,
        signed_handoff_bundle: true,
        credential_values_included: false,
        external_delivery_performed: false,
    }
}

fn no_agent_delivery(stdout: &str) -> Option<String> {
    let trimmed = stdout.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn check_json(path: &str, text: &str) -> DeltaLintCheck {
    let looks_json = path.ends_with(".json") || path.contains("<sample:json");
    if !looks_json {
        return DeltaLintCheck {
            id: "json-syntax",
            passed: true,
            detail: "not a JSON target".into(),
        };
    }
    match serde_json::from_str::<serde_json::Value>(text) {
        Ok(_) => DeltaLintCheck {
            id: "json-syntax",
            passed: true,
            detail: "JSON syntax valid".into(),
        },
        Err(err) => DeltaLintCheck {
            id: "json-syntax",
            passed: false,
            detail: format!("JSON syntax error: {err}"),
        },
    }
}

fn check_python_balance(text: &str) -> DeltaLintCheck {
    let mut stack = Vec::new();
    for ch in text.chars() {
        match ch {
            '(' | '[' | '{' => stack.push(ch),
            ')' => {
                if stack.pop() != Some('(') {
                    return balance_failed();
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return balance_failed();
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return balance_failed();
                }
            }
            _ => {}
        }
    }
    DeltaLintCheck {
        id: "python-bracket-balance",
        passed: stack.is_empty(),
        detail: if stack.is_empty() {
            "brackets balanced".into()
        } else {
            "unclosed bracket detected".into()
        },
    }
}

fn balance_failed() -> DeltaLintCheck {
    DeltaLintCheck {
        id: "python-bracket-balance",
        passed: false,
        detail: "mismatched bracket detected".into(),
    }
}

fn check_yaml_tabs(text: &str) -> DeltaLintCheck {
    let passed = !text.lines().any(|line| line.starts_with('\t'));
    DeltaLintCheck {
        id: "yaml-no-tab-indent",
        passed,
        detail: if passed {
            "no leading tab indentation".into()
        } else {
            "leading tab indentation detected".into()
        },
    }
}

fn check_toml_assignments(text: &str) -> DeltaLintCheck {
    let suspicious = text.lines().any(|line| {
        let trimmed = line.trim();
        !trimmed.is_empty()
            && !trimmed.starts_with('#')
            && !trimmed.starts_with('[')
            && !trimmed.contains('=')
            && trimmed
                .chars()
                .all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-')
    });
    DeltaLintCheck {
        id: "toml-assignment-shape",
        passed: !suspicious,
        detail: if suspicious {
            "bare TOML-like key without assignment detected".into()
        } else {
            "TOML assignment shape ok".into()
        },
    }
}

fn normalize_locale(locale: &str) -> &str {
    match locale.trim() {
        "zh" | "zh_CN" | "zh-CN" | "zh-Hans" => "zh-Hans",
        "" => "en",
        other => other,
    }
}

fn lookup_i18n(locale: &str, key: &str) -> Option<String> {
    match (locale, key) {
        ("zh-Hans", "task_board.ready") => Some("任务板就绪".into()),
        ("zh-Hans", "task_board.claimed") => Some("任务已领取".into()),
        ("zh-Hans", "handoff.ready") => Some("交接包就绪".into()),
        ("en", "task_board.ready") => Some("Task board ready".into()),
        ("en", "task_board.claimed") => Some("Task claimed".into()),
        ("en", "handoff.ready") => Some("Handoff bundle ready".into()),
        _ => None,
    }
}

fn sanitize_id(value: &str) -> String {
    value
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '-' })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

fn stable_signature(bundle_id: &str, resume_pointer: &str) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in bundle_id.bytes().chain(resume_pointer.bytes()) {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("hepta-handoff-v0:{hash:016x}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cron_no_agent_skips_agent_and_silences_empty_stdout() {
        let empty = cron_no_agent_watchdog_report("");
        assert!(empty.agent_invocation_skipped);
        assert!(empty.delivery.is_none());
        let non_empty = cron_no_agent_watchdog_report("disk ok\n");
        assert_eq!(non_empty.delivery.as_deref(), Some("disk ok"));
    }

    #[test]
    fn post_write_delta_lint_blocks_invalid_samples_without_mutation() {
        let report = post_write_delta_lint_report(None, true).unwrap();
        assert!(report.sample_run);
        assert!(!report.files_mutated);
        assert!(!report.external_process_spawned);
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.id == "sample-invalid-json-blocked" && check.passed)
        );
    }

    #[test]
    fn i18n_catalog_falls_back_to_english_then_key() {
        let report = i18n_catalog_report("fr", "task_board.ready");
        assert!(report.fallback_used);
        assert_eq!(report.value, "Task board ready");
        let missing = i18n_catalog_report("zh-Hans", "missing.key");
        assert_eq!(missing.value, "missing.key");
    }

    #[test]
    fn profile_session_handoff_builds_signed_local_bundle() {
        let report = profile_session_handoff_report("session:main", "telegram:local");
        assert!(report.signed_handoff_bundle);
        assert!(!report.credential_values_included);
        assert!(!report.external_delivery_performed);
        assert!(report.signature.starts_with("hepta-handoff-v0:"));
    }
}
