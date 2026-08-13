use std::ffi::OsString;
use std::path::PathBuf;

use codex_hepta_operator_acceptance::v3::BuildPlanRequestV3;
use codex_hepta_operator_acceptance::v3::ExecuteBuildRequestV3;
use codex_hepta_operator_acceptance::v3::VerifyAggregateRequestV3;
use codex_hepta_operator_acceptance::v3::assess_v3;
use codex_hepta_operator_acceptance::v3::build_and_seal_v3;
use codex_hepta_operator_acceptance::v3::build_plan_v3;
use codex_hepta_operator_acceptance::v3::verify_aggregate_v3;

const USAGE: &str = "usage:\n  hepta-operator-acceptance-v3 build-plan       <canonical-build-spec.json> <expected-build-spec-sha256> <new-aggregate-root>\n  hepta-operator-acceptance-v3 build            --execute <canonical-build-spec.json> <expected-build-spec-sha256> <new-aggregate-root>\n  hepta-operator-acceptance-v3 verify-aggregate <aggregate-root> <externally-pinned-SHA256SUMS-sha256>\n  hepta-operator-acceptance-v3 assess          <aggregate-root> <externally-pinned-SHA256SUMS-sha256>\n\nThe V3 CLI implements aggregate build, verification, and read-only assessment only. It intentionally provides no challenge, signature, nonce-claim, acceptance-receipt, ref, production, or GitHub mutation command.";

fn main() {
    if let Err(error) = run(std::env::args_os().collect()) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run(arguments: Vec<OsString>) -> Result<(), String> {
    let command = arguments
        .get(1)
        .and_then(|value| value.to_str())
        .ok_or_else(|| USAGE.to_string())?;
    match command {
        "build-plan" if arguments.len() == 5 => {
            let spec = PathBuf::from(&arguments[2]);
            let spec_sha256 = utf8(&arguments[3], "build-spec digest")?;
            let output = PathBuf::from(&arguments[4]);
            print_json(
                &build_plan_v3(BuildPlanRequestV3 {
                    build_spec_path: &spec,
                    expected_build_spec_sha256: spec_sha256,
                    output_root: &output,
                })
                .map_err(|error| error.to_string())?,
            )
        }
        "build"
            if arguments.len() == 6
                && arguments.get(2).and_then(|value| value.to_str()) == Some("--execute") =>
        {
            let spec = PathBuf::from(&arguments[3]);
            let spec_sha256 = utf8(&arguments[4], "build-spec digest")?;
            let output = PathBuf::from(&arguments[5]);
            print_json(
                &build_and_seal_v3(ExecuteBuildRequestV3 {
                    build_spec_path: &spec,
                    expected_build_spec_sha256: spec_sha256,
                    output_root: &output,
                })
                .map_err(|error| error.to_string())?,
            )
        }
        "verify-aggregate" if arguments.len() == 4 => {
            let aggregate = PathBuf::from(&arguments[2]);
            let manifest_sha256 = utf8(&arguments[3], "aggregate manifest digest")?;
            print_json(
                &verify_aggregate_v3(VerifyAggregateRequestV3 {
                    aggregate_root: &aggregate,
                    expected_manifest_sha256: manifest_sha256,
                })
                .map_err(|error| error.to_string())?,
            )
        }
        "assess" if arguments.len() == 4 => {
            let aggregate = PathBuf::from(&arguments[2]);
            let manifest_sha256 = utf8(&arguments[3], "aggregate manifest digest")?;
            print_json(
                &assess_v3(VerifyAggregateRequestV3 {
                    aggregate_root: &aggregate,
                    expected_manifest_sha256: manifest_sha256,
                })
                .map_err(|error| error.to_string())?,
            )
        }
        _ => Err(USAGE.to_string()),
    }
}

fn utf8<'a>(value: &'a OsString, label: &str) -> Result<&'a str, String> {
    value
        .to_str()
        .ok_or_else(|| format!("{label} must be UTF-8"))
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), String> {
    println!(
        "{}",
        serde_json::to_string(value).map_err(|error| error.to_string())?
    );
    Ok(())
}
