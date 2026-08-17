//! Exact, read-only install model. There is no installer in this crate.

use serde::Deserialize;
use serde::Serialize;

use crate::MnlError;
use crate::blocked;
use crate::parse_canonical;
use crate::random_hex;

pub const PLAN_SCHEMA: &str = "hepta_mac_mnl_install_plan_v1";
pub const CORRELATION_SCHEMA: &str = "hepta_mac_mnl_unattested_correlation_v1";
pub const AUTHORITY_SCHEMA: &str = "hepta_mac_mnl_closed_authority_v1";
pub const BACKEND_HEAD: &str = "52ec4b3868fc5272e19ed516d00e11e44c549ea4";
pub const BACKEND_TREE: &str = "247e9e7cfcb41dbfcc8c5b3b531b1e1407c0bd5d";
pub const UI_HEAD: &str = "64612c01de811f647d7f113d3104e2c9d8e17656";
pub const UI_TREE: &str = "7cae3967f9ab878bc67be8083beb9308482725f5";
pub const TOOLING_BASELINE_HEAD: &str = "898628204ff60131b8b015555a3f3a5b2ff80987";
pub const TOOLING_BASELINE_TREE: &str = "4977641b9bf4e91e1f548c73bc7622fc4e2874ee";
pub const T5_VOLUME_UUID: &str = "FB804D1B-24CB-4D6E-AEA7-A9E180807758";
pub const T5_ROOT: &str = "/Volumes/T5/hepta-vnext";
pub const PRODUCER_NAME: &str = "_hepta";
pub const PRODUCER_UID: u32 = 499;
pub const PRODUCER_GID: u32 = 499;
pub const OPERATOR_NAME: &str = "qianqi";
pub const OPERATOR_GROUP: &str = "staff";
pub const OPERATOR_UID: u32 = 501;
pub const OPERATOR_GID: u32 = 20;
pub const BROKER_LABEL: &str = "com.hepta.mnl-broker-v1";
pub const BROKER_BINARY_PATH: &str = "/Library/PrivilegedHelperTools/com.hepta.mnl-broker-v1";
pub const CLIENT_BINARY_PATH: &str = "/Library/PrivilegedHelperTools/com.hepta.mnl-client-v1";
pub const POLICY_PATH: &str = "/Library/PrivilegedHelperTools/com.hepta.mnl-broker-policy-v1.json";
pub const LAUNCHD_PLIST_PATH: &str = "/Library/LaunchDaemons/com.hepta.mnl-broker-v1.plist";
pub const BROKER_SOCKET_PATH: &str = "/private/var/run/com.hepta.mnl-broker-v1.sock";

// All successor and artifact identities stay compile-time-only. There is no
// input field or CLI flag that can replace any of these absent pins.
const SUCCESSOR_FINAL_TOOLING_HEAD: Option<&str> = None;
const SUCCESSOR_FINAL_TOOLING_TREE: Option<&str> = None;
const FROZEN_BROKER_BINARY_SHA256: Option<&str> = None;
const FROZEN_CLIENT_BINARY_SHA256: Option<&str> = None;
const FROZEN_POLICY_JSON_SHA256: Option<&str> = None;
const FROZEN_LAUNCHD_PLIST_SHA256: Option<&str> = None;
const FROZEN_INSTALL_FREEZE_MANIFEST_SHA256: Option<&str> = None;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RepositoryIdentityV1 {
    pub head: String,
    pub tree: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CandidateIdentityV1 {
    pub backend: RepositoryIdentityV1,
    pub successor_final_tooling: Option<RepositoryIdentityV1>,
    pub tooling_baseline: RepositoryIdentityV1,
    pub ui: RepositoryIdentityV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VolumeIdentityV1 {
    pub mount_path: String,
    pub ownership_enabled: bool,
    pub volume_uuid: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProducerIdentityV1 {
    pub gid: u32,
    pub home: String,
    pub name: String,
    pub shell: String,
    pub supplementary_gids: Vec<u32>,
    pub uid: u32,
    pub wheel_member: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OwnerV1 {
    pub gid: u32,
    pub group: String,
    pub uid: u32,
    pub user: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallNodeKindV1 {
    Directory,
    RegularFile,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum InstallModeV1 {
    #[serde(rename = "01755")]
    DirectorySticky01755,
    #[serde(rename = "0755")]
    Directory0755,
    #[serde(rename = "0775")]
    Directory0775,
    #[serde(rename = "0555")]
    Regular0555,
    #[serde(rename = "0444")]
    Regular0444,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallArtifactRoleV1 {
    BrokerBinary,
    BrokerPolicyCanonicalJson,
    ClientBinary,
    LaunchdPlist,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AncestorPolicyV1 {
    pub kind: InstallNodeKindV1,
    pub mode: InstallModeV1,
    pub owner: OwnerV1,
    pub path: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InstallArtifactV1 {
    pub kind: InstallNodeKindV1,
    pub mode: InstallModeV1,
    pub owner: OwnerV1,
    pub path: String,
    pub role: InstallArtifactRoleV1,
    pub sha256: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FrozenArtifactPinsV1 {
    pub broker_binary_sha256: Option<String>,
    pub client_binary_sha256: Option<String>,
    pub freeze_manifest_sha256: Option<String>,
    pub launchd_plist_sha256: Option<String>,
    pub policy_json_sha256: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BrokerInvocationV1 {
    ServeFixedProtocolV1Blocked,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LaunchdContractV1 {
    pub broker_label: String,
    pub invocation: BrokerInvocationV1,
    pub keep_alive: bool,
    pub plist_path: String,
    pub program_path: String,
    pub run_at_load: bool,
    pub socket_path: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CorrelationOriginClaimV1 {
    UnattestedOsCsprngPlannerClaim,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CorrelationDispositionV1 {
    UnverifiedCorrelationOnlyNotFreshnessOrAuthorization,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UnattestedCorrelationV1 {
    pub disposition: CorrelationDispositionV1,
    pub nonce: String,
    pub origin_claim: CorrelationOriginClaimV1,
    pub schema: String,
    pub schema_version: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClosedAuthorityV1 {
    pub automatic_transition: bool,
    pub broker_live: bool,
    pub caller_declared_success: bool,
    pub cutover: bool,
    pub default_ref_change: bool,
    pub deletion: bool,
    pub full_matrix_claim: bool,
    pub ga_claim: bool,
    pub install: bool,
    pub local_ref_change: bool,
    pub mutation: bool,
    pub operator_acceptance: bool,
    pub production: bool,
    pub promotion: bool,
    pub qualification_authority: bool,
    pub recutover: bool,
    pub refs: bool,
    pub remote: bool,
    pub remote_ref_change: bool,
    pub retirement: bool,
    pub rollback: bool,
    pub writer_control: bool,
}

impl ClosedAuthorityV1 {
    pub(crate) const fn exact() -> Self {
        Self {
            automatic_transition: false,
            broker_live: false,
            caller_declared_success: false,
            cutover: false,
            default_ref_change: false,
            deletion: false,
            full_matrix_claim: false,
            ga_claim: false,
            install: false,
            local_ref_change: false,
            mutation: false,
            operator_acceptance: false,
            production: false,
            promotion: false,
            qualification_authority: false,
            recutover: false,
            refs: false,
            remote: false,
            remote_ref_change: false,
            retirement: false,
            rollback: false,
            writer_control: false,
        }
    }

    pub fn is_fully_closed(&self) -> bool {
        self == &Self::exact()
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InstallDispositionV1 {
    BlockedSuccessorFinalToolingUnfrozen,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InstallPlanV1 {
    pub ancestors: Vec<AncestorPolicyV1>,
    pub artifacts: Vec<InstallArtifactV1>,
    pub authority: ClosedAuthorityV1,
    pub authority_schema: String,
    pub candidate: CandidateIdentityV1,
    pub correlation: UnattestedCorrelationV1,
    pub disposition: InstallDispositionV1,
    pub frozen_artifact_pins: FrozenArtifactPinsV1,
    pub launchd: LaunchdContractV1,
    pub producer: ProducerIdentityV1,
    pub schema: String,
    pub schema_version: u32,
    pub target_volume: VolumeIdentityV1,
}

/// Generate a non-authorizing correlation value and the exact read-only plan.
pub fn plan_read_only() -> Result<InstallPlanV1, MnlError> {
    exact_plan_for_correlation(&random_hex::<32>()?)
}

/// Validate bytes, then fail closed because the successor final tooling is absent.
pub fn verify_canonical_plan(bytes: &[u8]) -> Result<InstallPlanV1, MnlError> {
    let plan = parse_canonical(bytes, "Mac MNL install plan")?;
    validate_plan_shape(&plan)?;
    require_activation_inputs(&plan)?;
    Ok(plan)
}

/// There is no installer behind this boundary.
pub fn execute_live_install_v1() -> Result<(), MnlError> {
    Err(blocked(
        "Mac MNL live install has no implementation or authority",
    ))
}

pub(crate) fn exact_plan_for_correlation(nonce: &str) -> Result<InstallPlanV1, MnlError> {
    require_correlation(nonce)?;
    let pins = exact_artifact_pins();
    Ok(InstallPlanV1 {
        ancestors: exact_ancestors(),
        artifacts: exact_artifacts(&pins),
        authority: ClosedAuthorityV1::exact(),
        authority_schema: AUTHORITY_SCHEMA.to_string(),
        candidate: exact_candidate(),
        correlation: UnattestedCorrelationV1 {
            disposition:
                CorrelationDispositionV1::UnverifiedCorrelationOnlyNotFreshnessOrAuthorization,
            nonce: nonce.to_string(),
            origin_claim: CorrelationOriginClaimV1::UnattestedOsCsprngPlannerClaim,
            schema: CORRELATION_SCHEMA.to_string(),
            schema_version: 1,
        },
        disposition: InstallDispositionV1::BlockedSuccessorFinalToolingUnfrozen,
        frozen_artifact_pins: pins,
        launchd: LaunchdContractV1 {
            broker_label: BROKER_LABEL.to_string(),
            invocation: BrokerInvocationV1::ServeFixedProtocolV1Blocked,
            keep_alive: false,
            plist_path: LAUNCHD_PLIST_PATH.to_string(),
            program_path: BROKER_BINARY_PATH.to_string(),
            run_at_load: false,
            socket_path: BROKER_SOCKET_PATH.to_string(),
        },
        producer: ProducerIdentityV1 {
            gid: PRODUCER_GID,
            home: "/var/empty".to_string(),
            name: PRODUCER_NAME.to_string(),
            shell: "/usr/bin/false".to_string(),
            supplementary_gids: Vec::new(),
            uid: PRODUCER_UID,
            wheel_member: false,
        },
        schema: PLAN_SCHEMA.to_string(),
        schema_version: 1,
        target_volume: VolumeIdentityV1 {
            mount_path: T5_ROOT.to_string(),
            ownership_enabled: true,
            volume_uuid: T5_VOLUME_UUID.to_string(),
        },
    })
}

pub(crate) fn validate_plan_shape(plan: &InstallPlanV1) -> Result<(), MnlError> {
    require_correlation(&plan.correlation.nonce)?;
    let expected = exact_plan_for_correlation(&plan.correlation.nonce)?;
    if plan != &expected {
        return Err(crate::invalid(
            "install plan differs from compiled products, tooling baseline, absent successor, paths, owners, modes, inventory, correlation semantics, or closed authority",
        ));
    }
    Ok(())
}

pub(crate) fn require_activation_inputs(plan: &InstallPlanV1) -> Result<(), MnlError> {
    validate_plan_shape(plan)?;
    if plan.candidate.successor_final_tooling.is_none() {
        return Err(blocked(
            "successor final tooling identity is not frozen; tooling baseline is not authority",
        ));
    }
    require_frozen_artifact_pins(plan)
}

fn require_frozen_artifact_pins(plan: &InstallPlanV1) -> Result<(), MnlError> {
    let pins = &plan.frozen_artifact_pins;
    match (
        &pins.broker_binary_sha256,
        &pins.client_binary_sha256,
        &pins.policy_json_sha256,
        &pins.launchd_plist_sha256,
        &pins.freeze_manifest_sha256,
    ) {
        (Some(broker), Some(client), Some(policy), Some(plist), Some(manifest)) => {
            for (digest, label) in [
                (broker, "broker binary"),
                (client, "client binary"),
                (policy, "policy JSON"),
                (plist, "launchd plist"),
                (manifest, "install freeze manifest"),
            ] {
                require_digest(digest, label)?;
            }
            Ok(())
        }
        _ => Err(blocked(
            "broker/client binaries and generated policy/plist are not frozen",
        )),
    }
}

fn exact_candidate() -> CandidateIdentityV1 {
    CandidateIdentityV1 {
        backend: repository(BACKEND_HEAD, BACKEND_TREE),
        successor_final_tooling: match (SUCCESSOR_FINAL_TOOLING_HEAD, SUCCESSOR_FINAL_TOOLING_TREE)
        {
            (Some(head), Some(tree)) => Some(repository(head, tree)),
            _ => None,
        },
        tooling_baseline: repository(TOOLING_BASELINE_HEAD, TOOLING_BASELINE_TREE),
        ui: repository(UI_HEAD, UI_TREE),
    }
}

fn repository(head: &str, tree: &str) -> RepositoryIdentityV1 {
    RepositoryIdentityV1 {
        head: head.to_string(),
        tree: tree.to_string(),
    }
}

pub(crate) fn root_wheel() -> OwnerV1 {
    OwnerV1 {
        gid: 0,
        group: "wheel".to_string(),
        uid: 0,
        user: "root".to_string(),
    }
}

fn root_daemon() -> OwnerV1 {
    OwnerV1 {
        gid: 1,
        group: "daemon".to_string(),
        uid: 0,
        user: "root".to_string(),
    }
}

pub(crate) fn operator_staff() -> OwnerV1 {
    OwnerV1 {
        gid: OPERATOR_GID,
        group: OPERATOR_GROUP.to_string(),
        uid: OPERATOR_UID,
        user: OPERATOR_NAME.to_string(),
    }
}

fn exact_ancestors() -> Vec<AncestorPolicyV1> {
    [
        ("/", InstallModeV1::Directory0755, root_wheel()),
        ("/Library", InstallModeV1::Directory0755, root_wheel()),
        (
            "/Library/PrivilegedHelperTools",
            InstallModeV1::DirectorySticky01755,
            root_wheel(),
        ),
        (
            "/Library/LaunchDaemons",
            InstallModeV1::Directory0755,
            root_wheel(),
        ),
        ("/private", InstallModeV1::Directory0755, root_wheel()),
        ("/private/var", InstallModeV1::Directory0755, root_wheel()),
        (
            "/private/var/run",
            InstallModeV1::Directory0775,
            root_daemon(),
        ),
    ]
    .into_iter()
    .map(|(path, mode, owner)| AncestorPolicyV1 {
        kind: InstallNodeKindV1::Directory,
        mode,
        owner,
        path: path.to_string(),
    })
    .collect()
}

fn exact_artifact_pins() -> FrozenArtifactPinsV1 {
    FrozenArtifactPinsV1 {
        broker_binary_sha256: FROZEN_BROKER_BINARY_SHA256.map(str::to_string),
        client_binary_sha256: FROZEN_CLIENT_BINARY_SHA256.map(str::to_string),
        freeze_manifest_sha256: FROZEN_INSTALL_FREEZE_MANIFEST_SHA256.map(str::to_string),
        launchd_plist_sha256: FROZEN_LAUNCHD_PLIST_SHA256.map(str::to_string),
        policy_json_sha256: FROZEN_POLICY_JSON_SHA256.map(str::to_string),
    }
}

fn exact_artifacts(pins: &FrozenArtifactPinsV1) -> Vec<InstallArtifactV1> {
    [
        (
            InstallArtifactRoleV1::BrokerBinary,
            BROKER_BINARY_PATH,
            InstallModeV1::Regular0555,
            pins.broker_binary_sha256.clone(),
        ),
        (
            InstallArtifactRoleV1::ClientBinary,
            CLIENT_BINARY_PATH,
            InstallModeV1::Regular0555,
            pins.client_binary_sha256.clone(),
        ),
        (
            InstallArtifactRoleV1::BrokerPolicyCanonicalJson,
            POLICY_PATH,
            InstallModeV1::Regular0444,
            pins.policy_json_sha256.clone(),
        ),
        (
            InstallArtifactRoleV1::LaunchdPlist,
            LAUNCHD_PLIST_PATH,
            InstallModeV1::Regular0444,
            pins.launchd_plist_sha256.clone(),
        ),
    ]
    .into_iter()
    .map(|(role, path, mode, sha256)| InstallArtifactV1 {
        kind: InstallNodeKindV1::RegularFile,
        mode,
        owner: root_wheel(),
        path: path.to_string(),
        role,
        sha256,
    })
    .collect()
}

fn require_correlation(value: &str) -> Result<(), MnlError> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(crate::invalid(
            "unattested correlation nonce must be exactly 32 bytes of lowercase hex",
        ));
    }
    Ok(())
}

fn require_digest(value: &str, label: &str) -> Result<(), MnlError> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(crate::invalid(format!(
            "{label} SHA-256 must be 64 lowercase hex characters"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::*;
    use crate::canonical_json;

    const NONCE: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

    fn plan() -> InstallPlanV1 {
        exact_plan_for_correlation(NONCE).expect("exact plan")
    }

    fn canonical(plan: &InstallPlanV1) -> Vec<u8> {
        canonical_json(plan).expect("canonical plan")
    }

    fn rejected(plan: &InstallPlanV1) {
        assert!(verify_canonical_plan(&canonical(plan)).is_err());
    }

    #[test]
    fn plan_binds_products_baseline_absent_successor_and_t5() {
        let plan = plan();
        assert_eq!(plan.candidate.backend.head, BACKEND_HEAD);
        assert_eq!(plan.candidate.ui.tree, UI_TREE);
        assert_eq!(plan.candidate.tooling_baseline.head, TOOLING_BASELINE_HEAD);
        assert!(plan.candidate.successor_final_tooling.is_none());
        assert_eq!(plan.target_volume.volume_uuid, T5_VOLUME_UUID);
        assert!(plan.authority.is_fully_closed());
    }

    #[test]
    fn all_verification_is_blocked_by_absent_successor_before_artifact_pins() {
        let error = verify_canonical_plan(&canonical(&plan())).expect_err("successor absent");
        assert!(matches!(error, MnlError::Blocked(_)));
        assert!(error.to_string().contains("successor final tooling"));
    }

    #[test]
    fn physical_socket_ancestors_are_exact() {
        let plan = plan();
        let run = plan
            .ancestors
            .iter()
            .find(|ancestor| ancestor.path == "/private/var/run")
            .expect("physical run ancestor");
        assert_eq!(run.mode, InstallModeV1::Directory0775);
        assert_eq!(run.owner, root_daemon());
        assert_eq!(plan.launchd.socket_path, BROKER_SOCKET_PATH);
        assert!(!plan.launchd.socket_path.starts_with("/var/run/"));
    }

    #[test]
    fn helper_ancestor_retains_actual_sticky_mode() {
        let helper = plan()
            .ancestors
            .into_iter()
            .find(|ancestor| ancestor.path == "/Library/PrivilegedHelperTools")
            .expect("helper ancestor");
        assert_eq!(helper.mode, InstallModeV1::DirectorySticky01755);
        assert_eq!(helper.owner, root_wheel());
    }

    #[test]
    fn caller_cannot_supply_successor_or_artifact_pins() {
        let mut successor = plan();
        successor.candidate.successor_final_tooling =
            Some(repository(&"a".repeat(40), &"b".repeat(40)));
        rejected(&successor);

        let mut artifact = plan();
        artifact.frozen_artifact_pins.broker_binary_sha256 = Some("a".repeat(64));
        artifact.artifacts[0].sha256 = Some("a".repeat(64));
        rejected(&artifact);
    }

    #[test]
    fn wrong_product_baseline_and_volume_are_rejected() {
        let mut backend = plan();
        backend.candidate.backend.head = "0".repeat(40);
        rejected(&backend);
        let mut ui = plan();
        ui.candidate.ui.tree = "0".repeat(40);
        rejected(&ui);
        let mut baseline = plan();
        baseline.candidate.tooling_baseline.head = "0".repeat(40);
        rejected(&baseline);
        let mut uuid = plan();
        uuid.target_volume.volume_uuid = "00000000-0000-0000-0000-000000000000".to_string();
        rejected(&uuid);
        let mut owners = plan();
        owners.target_volume.ownership_enabled = false;
        rejected(&owners);
    }

    #[test]
    fn wrong_paths_modes_and_owners_are_rejected() {
        let mut path = plan();
        path.artifacts[0].path = "/tmp/helper".to_string();
        rejected(&path);
        let mut mode = plan();
        mode.ancestors[2].mode = InstallModeV1::Directory0755;
        rejected(&mode);
        let mut owner = plan();
        owner.artifacts[0].owner.uid = 501;
        rejected(&owner);
        let mut run_owner = plan();
        run_owner.ancestors[6].owner.gid = 20;
        rejected(&run_owner);
    }

    #[test]
    fn producer_identity_is_exact_and_unprivileged() {
        let plan = plan();
        assert_eq!(plan.producer.uid, 499);
        assert_eq!(plan.producer.gid, 499);
        assert!(!plan.producer.wheel_member);
        let mut wheel = plan;
        wheel.producer.wheel_member = true;
        rejected(&wheel);
    }

    #[test]
    fn every_authority_bit_is_rejected() {
        for index in 0..22 {
            let mut value = plan();
            let authority = &mut value.authority;
            match index {
                0 => authority.automatic_transition = true,
                1 => authority.broker_live = true,
                2 => authority.caller_declared_success = true,
                3 => authority.cutover = true,
                4 => authority.default_ref_change = true,
                5 => authority.deletion = true,
                6 => authority.full_matrix_claim = true,
                7 => authority.ga_claim = true,
                8 => authority.install = true,
                9 => authority.local_ref_change = true,
                10 => authority.mutation = true,
                11 => authority.operator_acceptance = true,
                12 => authority.production = true,
                13 => authority.promotion = true,
                14 => authority.qualification_authority = true,
                15 => authority.recutover = true,
                16 => authority.refs = true,
                17 => authority.remote = true,
                18 => authority.remote_ref_change = true,
                19 => authority.retirement = true,
                20 => authority.rollback = true,
                21 => authority.writer_control = true,
                _ => unreachable!(),
            }
            rejected(&value);
        }
    }

    #[test]
    fn correlation_is_explicitly_unattested_and_not_authority() {
        let plan = plan();
        assert_eq!(
            plan.correlation.disposition,
            CorrelationDispositionV1::UnverifiedCorrelationOnlyNotFreshnessOrAuthorization
        );
        let mut bad = plan;
        bad.correlation.nonce = "A".repeat(64);
        rejected(&bad);
    }

    #[test]
    fn unknown_fields_and_noncanonical_bytes_are_rejected() {
        let mut value = serde_json::to_value(plan()).expect("plan value");
        value
            .as_object_mut()
            .expect("object")
            .insert("success".to_string(), Value::Bool(true));
        assert!(verify_canonical_plan(&serde_json::to_vec(&value).expect("bytes")).is_err());
        let mut bytes = canonical(&plan());
        bytes.push(b'\n');
        assert!(verify_canonical_plan(&bytes).is_err());
    }

    #[test]
    fn live_install_boundary_has_no_authority() {
        assert!(matches!(
            execute_live_install_v1(),
            Err(MnlError::Blocked(_))
        ));
    }
}
