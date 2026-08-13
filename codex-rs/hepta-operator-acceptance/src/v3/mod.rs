mod builder;
mod evidence;
mod model;
mod profiles;
mod strict_json;

use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;

use crate::AcceptanceError;

pub use model::AggregateBuildPlanV3;
pub use model::AggregateBuildRecordV3;
pub use model::AggregateBuildSpecV3;
pub use model::AggregateQualificationPacketV3;
pub use model::ArtifactBindingV3;
pub use model::CandidateBindingV3;
pub use model::CandidateBundleBindingV3;
pub use model::EvidenceProfileV3;
pub use model::LinuxExactV6InnerResultV3;
pub use model::LinuxExactV6OuterResultV3;
pub use model::ManifestLayerBindingV3;
pub use model::ManifestLayerIdV3;
pub use model::ManifestRootKindV3;
pub use model::ModeManifestBindingV3;
pub use model::ModeManifestFormatV3;
pub use model::ObservedGateV3;
pub use model::ObservedPrerequisiteV3;
pub use model::OriginalReceiptBindingV3;
pub use model::PlatformGateBindingV3;
pub use model::PlatformGateInputV3;
pub use model::PlatformPolicyV3;
pub use model::PrerequisiteInputV3;
pub use model::PrerequisiteReceiptBindingV3;
pub use model::QualificationAssessmentV3;
pub use model::QualificationDecisionV3;
pub use model::ReceiptEvidenceBindingV3;
pub use model::ReceiptProvenanceV3;
pub use model::SealedAggregateV3;
pub use model::VerifiedAggregateV3;

const RECEIPTS_PARENT: &str = "/Volumes/T5/hepta-vnext/artifacts/receipts";

const FORMAL_ENVIRONMENT: [(&str, &str); 5] = [
    ("HEPTA_SSD_ROOT", "/Volumes/T5/hepta-vnext"),
    (
        "HEPTA_SSD_VOLUME_UUID",
        "FB804D1B-24CB-4D6E-AEA7-A9E180807758",
    ),
    ("HEPTA_LANE", "operator-acceptance-52ec-v3"),
    (
        "HEPTA_WORKTREE",
        "/Volumes/T5/hepta-vnext/worktrees/operator-acceptance-52ec-v3",
    ),
    ("HEPTA_ARTIFACTS_DIR", "/Volumes/T5/hepta-vnext/artifacts"),
];

pub struct BuildPlanRequestV3<'a> {
    pub build_spec_path: &'a Path,
    pub expected_build_spec_sha256: &'a str,
    pub output_root: &'a Path,
}

struct ExecuteBuildRequestV3<'a> {
    pub build_spec_path: &'a Path,
    pub expected_build_spec_sha256: &'a str,
    pub output_root: &'a Path,
}

pub struct VerifyAggregateRequestV3<'a> {
    pub aggregate_root: &'a Path,
    pub expected_manifest_sha256: &'a str,
}

pub fn build_plan_v3(
    request: BuildPlanRequestV3<'_>,
) -> Result<AggregateBuildPlanV3, AcceptanceError> {
    require_formal_environment_v3()?;
    builder::plan(
        request.build_spec_path,
        request.expected_build_spec_sha256,
        request.output_root,
        Path::new(RECEIPTS_PARENT),
    )
}

fn build_and_seal_v3(
    request: ExecuteBuildRequestV3<'_>,
) -> Result<SealedAggregateV3, AcceptanceError> {
    require_formal_environment_v3()?;
    builder::build(
        request.build_spec_path,
        request.expected_build_spec_sha256,
        request.output_root,
        Path::new(RECEIPTS_PARENT),
    )
}

pub fn run_cli_v3(arguments: Vec<OsString>) -> Result<String, String> {
    const USAGE: &str = "usage:\n  hepta-operator-acceptance-v3 build-plan       <canonical-build-spec.json> <expected-build-spec-sha256> <new-aggregate-root>\n  hepta-operator-acceptance-v3 build            --execute <canonical-build-spec.json> <expected-build-spec-sha256> <new-aggregate-root>\n  hepta-operator-acceptance-v3 verify-aggregate <aggregate-root> <externally-pinned-SHA256SUMS-sha256>\n  hepta-operator-acceptance-v3 assess          <aggregate-root> <externally-pinned-SHA256SUMS-sha256>";

    if arguments.len() == 2
        && matches!(
            arguments.get(1).and_then(|value| value.to_str()),
            Some("--help" | "-h" | "help")
        )
    {
        return Ok(USAGE.to_string());
    }

    let command = arguments
        .get(1)
        .and_then(|value| value.to_str())
        .ok_or_else(|| USAGE.to_string())?;
    let json = match command {
        "build-plan" if arguments.len() == 5 => {
            let spec = PathBuf::from(&arguments[2]);
            let spec_sha256 = utf8_cli(&arguments[3], "build-spec digest")?;
            let output = PathBuf::from(&arguments[4]);
            serde_json::to_string(
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
            let spec_sha256 = utf8_cli(&arguments[4], "build-spec digest")?;
            let output = PathBuf::from(&arguments[5]);
            serde_json::to_string(
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
            let digest = utf8_cli(&arguments[3], "aggregate manifest digest")?;
            serde_json::to_string(
                &verify_aggregate_v3(VerifyAggregateRequestV3 {
                    aggregate_root: &aggregate,
                    expected_manifest_sha256: digest,
                })
                .map_err(|error| error.to_string())?,
            )
        }
        "assess" if arguments.len() == 4 => {
            let aggregate = PathBuf::from(&arguments[2]);
            let digest = utf8_cli(&arguments[3], "aggregate manifest digest")?;
            serde_json::to_string(
                &assess_v3(VerifyAggregateRequestV3 {
                    aggregate_root: &aggregate,
                    expected_manifest_sha256: digest,
                })
                .map_err(|error| error.to_string())?,
            )
        }
        _ => return Err(USAGE.to_string()),
    }
    .map_err(|error| error.to_string())?;
    Ok(json)
}

fn utf8_cli<'a>(value: &'a OsString, label: &str) -> Result<&'a str, String> {
    value
        .to_str()
        .ok_or_else(|| format!("{label} must be UTF-8"))
}

pub fn verify_aggregate_v3(
    request: VerifyAggregateRequestV3<'_>,
) -> Result<VerifiedAggregateV3, AcceptanceError> {
    require_formal_environment_v3()?;
    builder::verify(
        request.aggregate_root,
        request.expected_manifest_sha256,
        Path::new(RECEIPTS_PARENT),
    )
}

pub fn assess_v3(
    request: VerifyAggregateRequestV3<'_>,
) -> Result<QualificationAssessmentV3, AcceptanceError> {
    Ok(verify_aggregate_v3(request)?.assessment)
}

pub fn require_formal_environment_v3() -> Result<(), AcceptanceError> {
    validate_formal_environment_with(|name| std::env::var_os(name))
}

fn validate_formal_environment_with(
    lookup: impl Fn(&str) -> Option<OsString>,
) -> Result<(), AcceptanceError> {
    for (name, expected) in FORMAL_ENVIRONMENT {
        if lookup(name).as_deref() != Some(OsStr::new(expected)) {
            return Err(AcceptanceError::Invalid(format!(
                "formal 52ec aggregate V3 requires exact {name} from hepta-ssd-run"
            )));
        }
    }
    Ok(())
}

pub fn aggregate_root_prefix_v3() -> &'static str {
    builder::aggregate_prefix()
}

#[cfg(test)]
mod tests;
