use serde::Deserialize;
use serde::Serialize;

use crate::LinuxMnlError;
use crate::blocked;
use crate::canonical::validate_digest;
use crate::invalid;

pub const COMPOSITE_IDENTITY_SCHEMA_V1: &str = "hepta_linux_exact_mnl_composite_identity_v1";
pub const BACKEND_CANDIDATE_HEAD: &str = "52ec4b3868fc5272e19ed516d00e11e44c549ea4";
pub const BACKEND_CANDIDATE_TREE: &str = "247e9e7cfcb41dbfcc8c5b3b531b1e1407c0bd5d";
pub const UI_CANDIDATE_HEAD: &str = "64612c01de811f647d7f113d3104e2c9d8e17656";
pub const UI_CANDIDATE_TREE: &str = "7cae3967f9ab878bc67be8083beb9308482725f5";
pub const TOOLING_BASELINE_HEAD: &str = "898628204ff60131b8b015555a3f3a5b2ff80987";
pub const TOOLING_BASELINE_TREE: &str = "4977641b9bf4e91e1f548c73bc7622fc4e2874ee";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RepositoryIdentityV1 {
    pub head: String,
    pub role: String,
    pub tree: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CompositeIdentityV1 {
    pub backend_product: RepositoryIdentityV1,
    pub schema: String,
    pub schema_version: u32,
    pub tooling_baseline: RepositoryIdentityV1,
    pub ui_product: RepositoryIdentityV1,
}

pub fn exact_composite_identity() -> CompositeIdentityV1 {
    CompositeIdentityV1 {
        backend_product: RepositoryIdentityV1 {
            head: BACKEND_CANDIDATE_HEAD.to_string(),
            role: "product_backend_candidate".to_string(),
            tree: BACKEND_CANDIDATE_TREE.to_string(),
        },
        schema: COMPOSITE_IDENTITY_SCHEMA_V1.to_string(),
        schema_version: 1,
        tooling_baseline: RepositoryIdentityV1 {
            head: TOOLING_BASELINE_HEAD.to_string(),
            role: "tooling_baseline_not_product_or_final_gate".to_string(),
            tree: TOOLING_BASELINE_TREE.to_string(),
        },
        ui_product: RepositoryIdentityV1 {
            head: UI_CANDIDATE_HEAD.to_string(),
            role: "product_ui_candidate".to_string(),
            tree: UI_CANDIDATE_TREE.to_string(),
        },
    }
}

pub fn validate_composite_identity(identity: &CompositeIdentityV1) -> Result<(), LinuxMnlError> {
    if identity != &exact_composite_identity() {
        return Err(invalid(
            "composite identity differs from backend 52ec/tree247, UI 646/tree7ca, or tooling baseline 898/tree497",
        ));
    }
    Ok(())
}

/// Source-level pins for the successor implementation that will actually
/// collect Phase 1 evidence. The 898/tree497 tooling identity above is only a
/// reviewed baseline; it can never stand in for this final implementation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CompiledSuccessorToolingPinsV1 {
    pub collector_binary_sha256: Option<&'static str>,
    pub collector_source_sha256: Option<&'static str>,
    pub successor_final_head: Option<&'static str>,
    pub successor_final_tree: Option<&'static str>,
}

pub const COMPILED_SUCCESSOR_TOOLING_PINS_V1: CompiledSuccessorToolingPinsV1 =
    CompiledSuccessorToolingPinsV1 {
        collector_binary_sha256: None,
        collector_source_sha256: None,
        successor_final_head: None,
        successor_final_tree: None,
    };

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct VerifiedSuccessorToolingIdentityV1 {
    pub(crate) collector_binary_sha256: String,
    pub(crate) collector_source_sha256: String,
    pub(crate) successor_final_tooling: RepositoryIdentityV1,
}

pub(crate) fn missing_successor_tooling_pins() -> Vec<String> {
    let pins = COMPILED_SUCCESSOR_TOOLING_PINS_V1;
    [
        ("successor_final_tooling_head", pins.successor_final_head),
        ("successor_final_tooling_tree", pins.successor_final_tree),
        ("collector_source_sha256", pins.collector_source_sha256),
        ("collector_binary_sha256", pins.collector_binary_sha256),
    ]
    .into_iter()
    .filter_map(|(name, pin)| pin.is_none().then(|| name.to_string()))
    .collect()
}

pub(crate) fn required_compiled_successor_tooling_identity()
-> Result<VerifiedSuccessorToolingIdentityV1, LinuxMnlError> {
    let missing = missing_successor_tooling_pins();
    if !missing.is_empty() {
        return Err(blocked(format!(
            "successor final tooling and collector identity pins are absent: {}",
            missing.join(",")
        )));
    }
    verified_successor_tooling_identity(COMPILED_SUCCESSOR_TOOLING_PINS_V1)
}

fn verified_successor_tooling_identity(
    pins: CompiledSuccessorToolingPinsV1,
) -> Result<VerifiedSuccessorToolingIdentityV1, LinuxMnlError> {
    let head = pins
        .successor_final_head
        .ok_or_else(|| blocked("successor final tooling head is absent"))?;
    let tree = pins
        .successor_final_tree
        .ok_or_else(|| blocked("successor final tooling tree is absent"))?;
    validate_git_oid("successor final tooling head", head)?;
    validate_git_oid("successor final tooling tree", tree)?;
    if head == TOOLING_BASELINE_HEAD || tree == TOOLING_BASELINE_TREE {
        return Err(invalid(
            "successor final tooling cannot reuse the 898/tree497 baseline identity",
        ));
    }
    let collector_source_sha256 = pins
        .collector_source_sha256
        .ok_or_else(|| blocked("collector source identity is absent"))?;
    let collector_binary_sha256 = pins
        .collector_binary_sha256
        .ok_or_else(|| blocked("collector binary identity is absent"))?;
    validate_digest("collector source", collector_source_sha256)?;
    validate_digest("collector binary", collector_binary_sha256)?;
    if collector_source_sha256 == collector_binary_sha256 {
        return Err(invalid(
            "collector source and binary identities must be distinct",
        ));
    }
    Ok(VerifiedSuccessorToolingIdentityV1 {
        collector_binary_sha256: collector_binary_sha256.to_string(),
        collector_source_sha256: collector_source_sha256.to_string(),
        successor_final_tooling: RepositoryIdentityV1 {
            head: head.to_string(),
            role: "successor_final_qualification_tooling".to_string(),
            tree: tree.to_string(),
        },
    })
}

pub(crate) fn validate_successor_tooling_identity(
    identity: &RepositoryIdentityV1,
    collector_source_sha256: &str,
    collector_binary_sha256: &str,
) -> Result<(), LinuxMnlError> {
    validate_git_oid("successor final tooling head", &identity.head)?;
    validate_git_oid("successor final tooling tree", &identity.tree)?;
    if identity.role != "successor_final_qualification_tooling"
        || identity.head == TOOLING_BASELINE_HEAD
        || identity.tree == TOOLING_BASELINE_TREE
    {
        return Err(invalid(
            "successor final tooling identity is not distinct from the reviewed baseline",
        ));
    }
    validate_digest("collector source", collector_source_sha256)?;
    validate_digest("collector binary", collector_binary_sha256)?;
    if collector_source_sha256 == collector_binary_sha256 {
        return Err(invalid(
            "collector source and binary identities must be distinct",
        ));
    }
    Ok(())
}

fn validate_git_oid(label: &str, oid: &str) -> Result<(), LinuxMnlError> {
    if oid.len() != 40
        || !oid
            .as_bytes()
            .iter()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(byte))
    {
        return Err(invalid(format!(
            "{label} must be one lowercase hexadecimal Git object id"
        )));
    }
    Ok(())
}

#[cfg(test)]
pub(crate) mod test_support {
    use super::*;

    pub(crate) fn verified_successor_tooling() -> VerifiedSuccessorToolingIdentityV1 {
        verified_successor_tooling_identity(CompiledSuccessorToolingPinsV1 {
            collector_binary_sha256: Some(
                "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
            ),
            collector_source_sha256: Some(
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            ),
            successor_final_head: Some("1111111111111111111111111111111111111111"),
            successor_final_tree: Some("2222222222222222222222222222222222222222"),
        })
        .expect("test successor tooling")
    }
}
