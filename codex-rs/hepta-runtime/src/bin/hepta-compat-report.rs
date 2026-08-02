use std::process::Command;
use std::process::ExitCode;

use hepta_runtime::DirtyWorktreeObservation;
use hepta_runtime::RETIRED_DIRTY_WORKTREE_COMPAT_REPORT_ID;
use hepta_runtime::TYPED_COMPAT_REPORT_IDS;
use hepta_runtime::is_dirty_worktree_typed_compat_report;
use hepta_runtime::retired_dirty_worktree_owner_decision_source_report;
use hepta_runtime::typed_compat_report;
use hepta_runtime::typed_compat_report_with_dirty_worktree_observation;

const INTERNAL_DIRTY_WORKTREE_OWNER_DECISION_SOURCE: &str =
    "--internal-dirty-worktree-owner-decision-source";

fn main() -> ExitCode {
    let mut arguments = std::env::args().skip(1);
    let Some(id) = arguments.next() else {
        eprintln!("usage: hepta-compat-report <--list|report-id>");
        return ExitCode::from(64);
    };
    if arguments.next().is_some() {
        eprintln!("usage: hepta-compat-report <--list|report-id>");
        return ExitCode::from(64);
    }

    if id == "--list" {
        for report_id in TYPED_COMPAT_REPORT_IDS {
            println!("{report_id}");
        }
        return ExitCode::SUCCESS;
    }

    let report = if id == INTERNAL_DIRTY_WORKTREE_OWNER_DECISION_SOURCE {
        internal_dirty_worktree_owner_decision_source_report()
    } else if is_dirty_worktree_typed_compat_report(&id) {
        dirty_worktree_report(&id)
    } else {
        typed_compat_report(&id)
    };

    match report.and_then(|report| serde_json::to_string_pretty(&report).map_err(Into::into)) {
        Ok(report) => {
            println!("{report}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

fn dirty_worktree_report(
    id: &str,
) -> Result<serde_json::Value, hepta_runtime::TypedCompatReportError> {
    let observation = dirty_worktree_observation()?;
    typed_compat_report_with_dirty_worktree_observation(id, &observation)
}

fn internal_dirty_worktree_owner_decision_source_report()
-> Result<serde_json::Value, hepta_runtime::TypedCompatReportError> {
    let requested_id = std::env::var("HEPTA_TYPED_COMPAT_INTERNAL_REPORT_ID").map_err(|_| {
        hepta_runtime::TypedCompatReportError::ContractViolation(
            "internal dirty-worktree source requires an explicit report id".to_string(),
        )
    })?;
    if requested_id != RETIRED_DIRTY_WORKTREE_COMPAT_REPORT_ID {
        return Err(hepta_runtime::TypedCompatReportError::ContractViolation(
            "unknown internal dirty-worktree source report id".to_string(),
        ));
    }
    let observation = dirty_worktree_observation()?;
    retired_dirty_worktree_owner_decision_source_report(&observation)
        .map_err(hepta_runtime::TypedCompatReportError::ContractViolation)
}

fn dirty_worktree_observation()
-> Result<DirtyWorktreeObservation, hepta_runtime::TypedCompatReportError> {
    let root = std::env::var_os("HEPTA_REPO_ROOT").ok_or_else(|| {
        hepta_runtime::TypedCompatReportError::ContractViolation(
            "dirty-worktree report requires explicit HEPTA_REPO_ROOT".to_string(),
        )
    })?;
    let output = Command::new("git")
        .env("GIT_OPTIONAL_LOCKS", "0")
        .arg("--no-optional-locks")
        .arg("-C")
        .arg(root)
        .args(["status", "--porcelain=v1", "-z", "--untracked-files=all"])
        .output()
        .map_err(|error| {
            hepta_runtime::TypedCompatReportError::ContractViolation(format!(
                "cannot observe dirty worktree: {error}"
            ))
        })?;
    if !output.status.success() {
        return Err(hepta_runtime::TypedCompatReportError::ContractViolation(
            "git status failed while observing dirty worktree".to_string(),
        ));
    }
    DirtyWorktreeObservation::from_porcelain_v1_z(&output.stdout)
        .map_err(hepta_runtime::TypedCompatReportError::ContractViolation)
}
