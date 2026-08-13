mod builder;
mod evidence;
mod model;

use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::Path;

use crate::AcceptanceError;

pub use model::AggregateBuildPlanV3;
pub use model::AggregateBuildRecordV3;
pub use model::AggregateBuildSpecV3;
pub use model::AggregateQualificationPacketV3;
pub use model::ArtifactAssertionV3;
pub use model::CandidateBindingV3;
pub use model::CandidateBundleBindingV3;
pub use model::EvidenceArtifactBindingV3;
pub use model::EvidenceArtifactFormatV3;
pub use model::ManifestLayerBindingV3;
pub use model::ManifestRootKindV3;
pub use model::ModeManifestBindingV3;
pub use model::ModeManifestFormatV3;
pub use model::ObservedGateV3;
pub use model::ObservedPrerequisiteV3;
pub use model::PlatformGateBindingV3;
pub use model::PlatformGateInputV3;
pub use model::PlatformPolicyV3;
pub use model::PrerequisiteInputV3;
pub use model::PrerequisiteReceiptBindingV3;
pub use model::QualificationAssessmentV3;
pub use model::QualificationDecisionV3;
pub use model::ReceiptEvidenceBindingV3;
pub use model::SealedAggregateV3;
pub use model::SemanticClaimBindingV3;
pub use model::SemanticClaimV3;
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

pub struct ExecuteBuildRequestV3<'a> {
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

pub fn build_and_seal_v3(
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
