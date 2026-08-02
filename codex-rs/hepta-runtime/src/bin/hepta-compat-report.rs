use std::process::Command;
use std::process::ExitCode;

use hepta_runtime::DirtyWorktreeObservation;
use hepta_runtime::TYPED_COMPAT_REPORT_IDS;
use hepta_runtime::is_dirty_worktree_typed_compat_report;
use hepta_runtime::typed_compat_report;
use hepta_runtime::typed_compat_report_with_dirty_worktree_observation;

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

    let report = if is_dirty_worktree_typed_compat_report(&id) {
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
    let observation = DirtyWorktreeObservation::from_porcelain_v1_z(&output.stdout)
        .map_err(hepta_runtime::TypedCompatReportError::ContractViolation)?;
    typed_compat_report_with_dirty_worktree_observation(id, &observation)
}
