use std::process::ExitCode;

use hepta_runtime::TYPED_COMPAT_REPORT_IDS;
use hepta_runtime::typed_compat_report;

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

    match typed_compat_report(&id)
        .and_then(|report| serde_json::to_string_pretty(&report).map_err(Into::into))
    {
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
