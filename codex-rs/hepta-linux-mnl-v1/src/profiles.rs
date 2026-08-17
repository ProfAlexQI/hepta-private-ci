use std::collections::BTreeSet;

use serde::Deserialize;
use serde::Serialize;

use crate::CompositeIdentityV1;
use crate::LinuxMnlError;
use crate::blocked;
use crate::canonical::validate_digest;
use crate::canonical_sha256;
use crate::decode_canonical_json;
use crate::invalid;
use crate::validate_composite_identity;

pub const TRUST_PROFILE_COUNT_V1: usize = 8;
pub const STATE_ROOT_PATH_V1: &str = "/var/lib/hepta-linux-v8";
pub const MACHINE_ID_PATH_V1: &str = "/etc/machine-id";
pub const PRODUCTION_TARGET_ALIAS_V1: &str = "desktop-ts";

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustPurposeV1 {
    InstallV2,
    InstallEpochV1,
    ExternalWatermarkLeaseV1,
    ExternalWatermarkCommitV1,
    ExternalWatermarkCurrentTipV1,
    OneShotV2,
    BreakGlass,
    MacCopyAck,
}

impl TrustPurposeV1 {
    pub const fn all() -> [Self; TRUST_PROFILE_COUNT_V1] {
        [
            Self::InstallV2,
            Self::InstallEpochV1,
            Self::ExternalWatermarkLeaseV1,
            Self::ExternalWatermarkCommitV1,
            Self::ExternalWatermarkCurrentTipV1,
            Self::OneShotV2,
            Self::BreakGlass,
            Self::MacCopyAck,
        ]
    }

    pub const fn schema(self) -> &'static str {
        match self {
            Self::InstallV2 => "hepta_linux_mnl_install_v2_trust_profile_v1",
            Self::InstallEpochV1 => "hepta_linux_mnl_install_epoch_v1_trust_profile_v1",
            Self::ExternalWatermarkLeaseV1 => {
                "hepta_linux_mnl_external_watermark_lease_v1_trust_profile_v1"
            }
            Self::ExternalWatermarkCommitV1 => {
                "hepta_linux_mnl_external_watermark_commit_v1_trust_profile_v1"
            }
            Self::ExternalWatermarkCurrentTipV1 => {
                "hepta_linux_mnl_external_watermark_current_tip_v1_trust_profile_v1"
            }
            Self::OneShotV2 => "hepta_linux_mnl_one_shot_v2_trust_profile_v1",
            Self::BreakGlass => "hepta_linux_mnl_break_glass_trust_profile_v1",
            Self::MacCopyAck => "hepta_linux_mnl_mac_copy_ack_trust_profile_v1",
        }
    }

    pub const fn namespace(self) -> &'static str {
        match self {
            Self::InstallV2 => "hepta-linux-mnl-install-v2",
            Self::InstallEpochV1 => "hepta-linux-mnl-install-epoch-v1",
            Self::ExternalWatermarkLeaseV1 => "hepta-linux-mnl-watermark-lease-v1",
            Self::ExternalWatermarkCommitV1 => "hepta-linux-mnl-watermark-commit-v1",
            Self::ExternalWatermarkCurrentTipV1 => "hepta-linux-mnl-watermark-current-tip-v1",
            Self::OneShotV2 => "hepta-linux-mnl-one-shot-v2",
            Self::BreakGlass => "hepta-linux-mnl-break-glass-v1",
            Self::MacCopyAck => "hepta-linux-mnl-mac-copy-ack-v1",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TrustProfileV1 {
    allowed_signers_sha256: String,
    key_fingerprint: String,
    namespace: String,
    principal: String,
    profile_revision: u64,
    purpose: TrustPurposeV1,
    schema: String,
    schema_version: u32,
    trust_root_id: String,
}

impl TrustProfileV1 {
    pub fn purpose(&self) -> TrustPurposeV1 {
        self.purpose
    }

    pub fn trust_root_id(&self) -> &str {
        &self.trust_root_id
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StateRootProfileV1 {
    gid: u32,
    layout_manifest_sha256: String,
    machine_id_sha256: String,
    mode: u32,
    path: String,
    profile_id: String,
    profile_revision: u64,
    schema: String,
    schema_version: u32,
    uid: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InstallEpochCompletionV1 {
    composite_identity_sha256: String,
    epoch: u64,
    epoch_id_sha256: String,
    install_epoch_trust_profile_sha256: String,
    machine_id_sha256: String,
    schema: String,
    schema_version: u32,
    signed_statement_sha256: String,
    state_root_layout_manifest_sha256: String,
    workload_identity_sha256: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalWatermarkProviderProfileV1 {
    commit_trust_profile_sha256: String,
    current_tip_trust_profile_sha256: String,
    genesis_epoch_binding_sha256: String,
    genesis_revision: u64,
    genesis_tip_sha256: String,
    lease_trust_profile_sha256: String,
    profile_id: String,
    profile_revision: u64,
    schema: String,
    schema_version: u32,
    stream_id_sha256: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalWatermarkCurrentTipV1 {
    current_tip_trust_profile_sha256: String,
    observed_revision: u64,
    observed_tip_sha256: String,
    provider_profile_sha256: String,
    schema: String,
    schema_version: u32,
    signed_statement_sha256: String,
    stream_id_sha256: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetRoleV1 {
    ProductionLinuxMnlTarget,
    QualificationFixtureOnly,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TargetProfileV1 {
    fixture_substitution_allowed: bool,
    host_alias: String,
    machine_id_mode: u32,
    machine_id_path: String,
    machine_id_sha256: String,
    profile_revision: u64,
    role: TargetRoleV1,
    schema: String,
    schema_version: u32,
    workload_identity_sha256: String,
}

impl TargetProfileV1 {
    pub fn host_alias(&self) -> &str {
        &self.host_alias
    }

    pub fn role(&self) -> TargetRoleV1 {
        self.role
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PublishedProfileDocumentsV1 {
    composite_identity: CompositeIdentityV1,
    external_watermark_current_tip: ExternalWatermarkCurrentTipV1,
    external_watermark_provider: ExternalWatermarkProviderProfileV1,
    install_epoch_completion: InstallEpochCompletionV1,
    schema: String,
    schema_version: u32,
    state_root: StateRootProfileV1,
    target: TargetProfileV1,
    trust_profiles: Vec<TrustProfileV1>,
}

impl PublishedProfileDocumentsV1 {
    pub fn composite_identity(&self) -> &CompositeIdentityV1 {
        &self.composite_identity
    }

    pub fn target(&self) -> &TargetProfileV1 {
        &self.target
    }
}

/// Compile-time publication pins. `None` is deliberate: Phase 1 must remain
/// hard blocked until independently reviewed bytes are pinned in source.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CompiledPublishedProfilePinsV1 {
    pub current_tip_sha256: Option<&'static str>,
    pub install_epoch_completion_sha256: Option<&'static str>,
    pub provider_sha256: Option<&'static str>,
    pub state_root_sha256: Option<&'static str>,
    pub target_sha256: Option<&'static str>,
    pub trust_profile_sha256: [Option<&'static str>; TRUST_PROFILE_COUNT_V1],
}

pub const COMPILED_PUBLISHED_PROFILE_PINS_V1: CompiledPublishedProfilePinsV1 =
    CompiledPublishedProfilePinsV1 {
        current_tip_sha256: None,
        install_epoch_completion_sha256: None,
        provider_sha256: None,
        state_root_sha256: None,
        target_sha256: None,
        trust_profile_sha256: [None; TRUST_PROFILE_COUNT_V1],
    };

/// Future independently reviewed canonical bytes must be embedded by a source
/// change. No path, environment variable, or caller value can nominate the
/// production document bundle.
const COMPILED_PUBLISHED_PROFILE_DOCUMENTS_JSON_V1: Option<&[u8]> = None;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CompiledProfilePinStatusV1 {
    pub blocked: bool,
    pub missing_pins: Vec<String>,
    pub production_plan_available: bool,
    pub schema: String,
}

pub fn compiled_profile_status() -> CompiledProfilePinStatusV1 {
    let mut missing_pins = missing_pins(COMPILED_PUBLISHED_PROFILE_PINS_V1);
    if COMPILED_PUBLISHED_PROFILE_DOCUMENTS_JSON_V1.is_none() {
        missing_pins.push("profile_documents_canonical_json".to_string());
    }
    CompiledProfilePinStatusV1 {
        blocked: !missing_pins.is_empty(),
        production_plan_available: missing_pins.is_empty(),
        missing_pins,
        schema: "hepta_linux_mnl_compiled_profile_pin_status_v1".to_string(),
    }
}

pub(crate) fn required_compiled_published_profiles()
-> Result<VerifiedPublishedProfilesV1, LinuxMnlError> {
    let bytes = COMPILED_PUBLISHED_PROFILE_DOCUMENTS_JSON_V1.ok_or_else(|| {
        blocked("independently published canonical profile document bytes are not compiled")
    })?;
    let documents: PublishedProfileDocumentsV1 = decode_canonical_json(bytes)?;
    validate_published_profiles(&documents)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedPublishedProfilesV1 {
    documents_sha256: String,
    target_host_alias: String,
    target_profile_sha256: String,
}

impl VerifiedPublishedProfilesV1 {
    pub fn documents_sha256(&self) -> &str {
        &self.documents_sha256
    }

    pub fn target_host_alias(&self) -> &str {
        &self.target_host_alias
    }

    pub fn target_profile_sha256(&self) -> &str {
        &self.target_profile_sha256
    }
}

/// Production validation cannot be redirected to caller-selected pins.
pub fn validate_published_profiles(
    documents: &PublishedProfileDocumentsV1,
) -> Result<VerifiedPublishedProfilesV1, LinuxMnlError> {
    validate_published_profiles_against(documents, COMPILED_PUBLISHED_PROFILE_PINS_V1)
}

fn validate_published_profiles_against(
    documents: &PublishedProfileDocumentsV1,
    pins: CompiledPublishedProfilePinsV1,
) -> Result<VerifiedPublishedProfilesV1, LinuxMnlError> {
    let missing = missing_pins(pins);
    if !missing.is_empty() {
        return Err(blocked(format!(
            "independently published external profile pins are absent: {}",
            missing.join(",")
        )));
    }

    if documents.schema != "hepta_linux_mnl_published_profile_documents_v1"
        || documents.schema_version != 1
    {
        return Err(invalid("published profile document envelope is not exact"));
    }
    validate_composite_identity(&documents.composite_identity)?;
    validate_trust_profiles(&documents.trust_profiles, pins.trust_profile_sha256)?;
    validate_state_root(&documents.state_root, required(pins.state_root_sha256)?)?;
    let target_profile_sha256 = validate_target(&documents.target, required(pins.target_sha256)?)?;
    if documents.state_root.machine_id_sha256 != documents.target.machine_id_sha256 {
        return Err(invalid(
            "state-root and production-target profiles bind different machine identities",
        ));
    }
    let install_epoch_completion_sha256 = validate_install_epoch_completion(
        &documents.install_epoch_completion,
        &documents.composite_identity,
        &documents.state_root,
        &documents.target,
        &documents.trust_profiles,
        required(pins.install_epoch_completion_sha256)?,
    )?;
    let provider_sha256 = validate_provider(
        &documents.external_watermark_provider,
        &documents.trust_profiles,
        &install_epoch_completion_sha256,
        required(pins.provider_sha256)?,
    )?;
    validate_current_tip(
        &documents.external_watermark_current_tip,
        &documents.external_watermark_provider,
        &documents.trust_profiles,
        &provider_sha256,
        required(pins.current_tip_sha256)?,
    )?;
    Ok(VerifiedPublishedProfilesV1 {
        documents_sha256: canonical_sha256(documents)?,
        target_host_alias: documents.target.host_alias.clone(),
        target_profile_sha256,
    })
}

fn validate_trust_profiles(
    profiles: &[TrustProfileV1],
    expected_pins: [Option<&str>; TRUST_PROFILE_COUNT_V1],
) -> Result<(), LinuxMnlError> {
    if profiles.len() != TRUST_PROFILE_COUNT_V1 {
        return Err(invalid(
            "exactly eight purpose-separated trust profiles are required",
        ));
    }

    let mut roots = BTreeSet::new();
    let mut fingerprints = BTreeSet::new();
    let mut allowed_signers = BTreeSet::new();
    let mut namespaces = BTreeSet::new();
    for ((profile, expected_purpose), expected_pin) in profiles
        .iter()
        .zip(TrustPurposeV1::all())
        .zip(expected_pins)
    {
        if profile.purpose != expected_purpose
            || profile.schema != expected_purpose.schema()
            || profile.namespace != expected_purpose.namespace()
            || profile.schema_version != 1
            || profile.profile_revision == 0
            || profile.trust_root_id.is_empty()
            || profile.principal.is_empty()
        {
            return Err(invalid(
                "trust profile purpose, schema, namespace, identity, or revision is not exact",
            ));
        }
        validate_digest("allowed signers", &profile.allowed_signers_sha256)?;
        validate_fingerprint(&profile.key_fingerprint)?;
        if !roots.insert(profile.trust_root_id.as_str())
            || !fingerprints.insert(profile.key_fingerprint.as_str())
            || !allowed_signers.insert(profile.allowed_signers_sha256.as_str())
            || !namespaces.insert(profile.namespace.as_str())
        {
            return Err(invalid(
                "trust purpose reuses a root, key, allowed-signers set, or namespace",
            ));
        }
        let expected_pin = required(expected_pin)?;
        if canonical_sha256(profile)? != expected_pin {
            return Err(invalid(
                "trust profile bytes differ from their compiled published pin",
            ));
        }
    }
    Ok(())
}

fn validate_state_root(
    profile: &StateRootProfileV1,
    expected_pin: &str,
) -> Result<(), LinuxMnlError> {
    if profile.schema != "hepta_linux_mnl_state_root_profile_v1"
        || profile.schema_version != 1
        || profile.profile_id != "hepta-linux-v8-state-root-profile-v1"
        || profile.profile_revision == 0
        || profile.path != STATE_ROOT_PATH_V1
        || !canonical_absolute_path(&profile.path)
        || profile.uid != 0
        || profile.gid != 0
        || profile.mode != 0o700
    {
        return Err(invalid(
            "state-root profile path, owner, mode, schema, or revision is not exact",
        ));
    }
    validate_digest("state-root layout", &profile.layout_manifest_sha256)?;
    validate_digest("state-root machine", &profile.machine_id_sha256)?;
    if canonical_sha256(profile)? != expected_pin {
        return Err(invalid(
            "state-root profile differs from its compiled published pin",
        ));
    }
    Ok(())
}

fn validate_install_epoch_completion(
    completion: &InstallEpochCompletionV1,
    composite_identity: &CompositeIdentityV1,
    state_root: &StateRootProfileV1,
    target: &TargetProfileV1,
    trust_profiles: &[TrustProfileV1],
    expected_pin: &str,
) -> Result<String, LinuxMnlError> {
    if completion.schema != "hepta_linux_mnl_install_epoch_completion_v1"
        || completion.schema_version != 1
        || completion.epoch == 0
        || completion.composite_identity_sha256 != canonical_sha256(composite_identity)?
        || completion.machine_id_sha256 != target.machine_id_sha256
        || completion.machine_id_sha256 != state_root.machine_id_sha256
        || completion.state_root_layout_manifest_sha256 != state_root.layout_manifest_sha256
        || completion.workload_identity_sha256 != target.workload_identity_sha256
    {
        return Err(invalid(
            "install-epoch completion does not bind the exact identity, target, state root, or epoch",
        ));
    }
    validate_digest("install epoch id", &completion.epoch_id_sha256)?;
    validate_digest(
        "install epoch signed statement",
        &completion.signed_statement_sha256,
    )?;
    let trust = trust_profile_sha256(trust_profiles, TrustPurposeV1::InstallEpochV1)?;
    if completion.install_epoch_trust_profile_sha256 != trust {
        return Err(invalid(
            "install-epoch completion uses the wrong purpose-separated trust profile",
        ));
    }
    let actual = canonical_sha256(completion)?;
    if actual != expected_pin {
        return Err(invalid(
            "install-epoch completion differs from its compiled published pin",
        ));
    }
    Ok(actual)
}

fn validate_provider(
    profile: &ExternalWatermarkProviderProfileV1,
    trust_profiles: &[TrustProfileV1],
    install_epoch_completion_sha256: &str,
    expected_pin: &str,
) -> Result<String, LinuxMnlError> {
    if profile.schema != "hepta_linux_mnl_external_watermark_provider_profile_v1"
        || profile.schema_version != 1
        || profile.profile_id != "hepta-linux-mnl-external-watermark-provider-v1"
        || profile.profile_revision == 0
        || profile.genesis_revision != 0
        || profile.genesis_epoch_binding_sha256 != install_epoch_completion_sha256
    {
        return Err(invalid(
            "external watermark provider identity, schema, or genesis revision is not exact",
        ));
    }
    validate_digest(
        "external watermark genesis epoch",
        &profile.genesis_epoch_binding_sha256,
    )?;
    validate_digest(
        "external watermark genesis tip",
        &profile.genesis_tip_sha256,
    )?;
    validate_digest("external watermark stream", &profile.stream_id_sha256)?;

    let lease = trust_profile_sha256(trust_profiles, TrustPurposeV1::ExternalWatermarkLeaseV1)?;
    let commit = trust_profile_sha256(trust_profiles, TrustPurposeV1::ExternalWatermarkCommitV1)?;
    let current_tip = trust_profile_sha256(
        trust_profiles,
        TrustPurposeV1::ExternalWatermarkCurrentTipV1,
    )?;
    if profile.lease_trust_profile_sha256 != lease
        || profile.commit_trust_profile_sha256 != commit
        || profile.current_tip_trust_profile_sha256 != current_tip
    {
        return Err(invalid(
            "watermark provider does not bind the three purpose-separated trust profiles",
        ));
    }
    let actual = canonical_sha256(profile)?;
    if actual != expected_pin {
        return Err(invalid(
            "external watermark provider differs from its compiled published pin",
        ));
    }
    Ok(actual)
}

fn validate_current_tip(
    current: &ExternalWatermarkCurrentTipV1,
    provider: &ExternalWatermarkProviderProfileV1,
    trust_profiles: &[TrustProfileV1],
    provider_sha256: &str,
    expected_pin: &str,
) -> Result<(), LinuxMnlError> {
    if current.schema != "hepta_linux_mnl_external_watermark_current_tip_v1"
        || current.schema_version != 1
        || current.provider_profile_sha256 != provider_sha256
        || current.stream_id_sha256 != provider.stream_id_sha256
        || current.observed_revision < provider.genesis_revision
        || (current.observed_revision == provider.genesis_revision
            && current.observed_tip_sha256 != provider.genesis_tip_sha256)
    {
        return Err(invalid(
            "external watermark current-tip identity or revision is not exact",
        ));
    }
    validate_digest(
        "external watermark current tip",
        &current.observed_tip_sha256,
    )?;
    validate_digest(
        "external watermark signed statement",
        &current.signed_statement_sha256,
    )?;
    let trust = trust_profile_sha256(
        trust_profiles,
        TrustPurposeV1::ExternalWatermarkCurrentTipV1,
    )?;
    if current.current_tip_trust_profile_sha256 != trust {
        return Err(invalid(
            "external watermark current tip uses the wrong trust purpose",
        ));
    }
    if canonical_sha256(current)? != expected_pin {
        return Err(invalid(
            "external watermark current tip differs from its compiled published pin",
        ));
    }
    Ok(())
}

fn validate_target(profile: &TargetProfileV1, expected_pin: &str) -> Result<String, LinuxMnlError> {
    if profile.schema != "hepta_linux_mnl_target_profile_v1"
        || profile.schema_version != 1
        || profile.profile_revision == 0
        || profile.role != TargetRoleV1::ProductionLinuxMnlTarget
        || profile.fixture_substitution_allowed
        || profile.host_alias != PRODUCTION_TARGET_ALIAS_V1
        || profile.machine_id_path != MACHINE_ID_PATH_V1
        || !canonical_absolute_path(&profile.machine_id_path)
        || profile.machine_id_mode != 0o444
    {
        return Err(invalid(
            "target profile differs from the positive desktop-ts identity or fixed machine-id boundary",
        ));
    }
    validate_digest("target machine", &profile.machine_id_sha256)?;
    validate_digest(
        "target workload identity",
        &profile.workload_identity_sha256,
    )?;
    let actual = canonical_sha256(profile)?;
    if actual != expected_pin {
        return Err(invalid(
            "target profile differs from its compiled published pin",
        ));
    }
    Ok(actual)
}

fn trust_profile_sha256(
    profiles: &[TrustProfileV1],
    purpose: TrustPurposeV1,
) -> Result<String, LinuxMnlError> {
    profiles
        .iter()
        .find(|profile| profile.purpose == purpose)
        .ok_or_else(|| invalid("required purpose-separated trust profile is missing"))
        .and_then(canonical_sha256)
}

fn missing_pins(pins: CompiledPublishedProfilePinsV1) -> Vec<String> {
    let mut missing = Vec::new();
    for (purpose, pin) in TrustPurposeV1::all()
        .into_iter()
        .zip(pins.trust_profile_sha256)
    {
        if pin.is_none() {
            missing.push(format!("trust:{purpose:?}"));
        }
    }
    for (name, pin) in [
        ("state_root", pins.state_root_sha256),
        (
            "install_epoch_completion",
            pins.install_epoch_completion_sha256,
        ),
        ("external_watermark_provider", pins.provider_sha256),
        ("external_watermark_current_tip", pins.current_tip_sha256),
        ("target", pins.target_sha256),
    ] {
        if pin.is_none() {
            missing.push(name.to_string());
        }
    }
    missing
}

fn required(pin: Option<&str>) -> Result<&str, LinuxMnlError> {
    pin.ok_or_else(|| blocked("compiled independently published profile pin is absent"))
}

fn validate_fingerprint(fingerprint: &str) -> Result<(), LinuxMnlError> {
    let encoded = fingerprint
        .strip_prefix("SHA256:")
        .ok_or_else(|| invalid("trust key fingerprint must use the SHA256: form"))?;
    if encoded.len() < 40
        || !encoded.bytes().all(|byte| {
            byte.is_ascii_alphanumeric() || byte == b'+' || byte == b'/' || byte == b'='
        })
    {
        return Err(invalid("trust key fingerprint is malformed"));
    }
    Ok(())
}

fn canonical_absolute_path(path: &str) -> bool {
    path.starts_with('/')
        && path != "/"
        && !path.ends_with('/')
        && !path.contains("//")
        && path
            .split('/')
            .skip(1)
            .all(|component| !component.is_empty() && component != "." && component != "..")
}

#[cfg(test)]
pub(crate) mod test_support {
    use super::*;
    use crate::canonical::sha256;

    pub(crate) fn documents() -> PublishedProfileDocumentsV1 {
        let trust_profiles = TrustPurposeV1::all()
            .into_iter()
            .enumerate()
            .map(|(index, purpose)| TrustProfileV1 {
                allowed_signers_sha256: sha256(format!("allowed-signers-{index}").as_bytes()),
                key_fingerprint: format!(
                    "SHA256:{}",
                    std::iter::repeat_n((b'A' + index as u8) as char, 43).collect::<String>()
                ),
                namespace: purpose.namespace().to_string(),
                principal: format!("hepta-linux-mnl-purpose-{index}"),
                profile_revision: 1,
                purpose,
                schema: purpose.schema().to_string(),
                schema_version: 1,
                trust_root_id: format!("hepta-linux-mnl-root-{index}"),
            })
            .collect::<Vec<_>>();
        let lease_trust_profile_sha256 = canonical_sha256(&trust_profiles[2]).expect("lease hash");
        let commit_trust_profile_sha256 =
            canonical_sha256(&trust_profiles[3]).expect("commit hash");
        let current_tip_trust_profile_sha256 =
            canonical_sha256(&trust_profiles[4]).expect("tip trust hash");
        let install_epoch_trust_profile_sha256 =
            canonical_sha256(&trust_profiles[1]).expect("install epoch trust hash");
        let composite_identity = crate::exact_composite_identity();
        let state_root = StateRootProfileV1 {
            gid: 0,
            layout_manifest_sha256: digest('5'),
            machine_id_sha256: digest('4'),
            mode: 0o700,
            path: STATE_ROOT_PATH_V1.to_string(),
            profile_id: "hepta-linux-v8-state-root-profile-v1".to_string(),
            profile_revision: 1,
            schema: "hepta_linux_mnl_state_root_profile_v1".to_string(),
            schema_version: 1,
            uid: 0,
        };
        let target = TargetProfileV1 {
            fixture_substitution_allowed: false,
            host_alias: PRODUCTION_TARGET_ALIAS_V1.to_string(),
            machine_id_mode: 0o444,
            machine_id_path: MACHINE_ID_PATH_V1.to_string(),
            machine_id_sha256: digest('4'),
            profile_revision: 1,
            role: TargetRoleV1::ProductionLinuxMnlTarget,
            schema: "hepta_linux_mnl_target_profile_v1".to_string(),
            schema_version: 1,
            workload_identity_sha256: digest('3'),
        };
        let install_epoch_completion = InstallEpochCompletionV1 {
            composite_identity_sha256: canonical_sha256(&composite_identity)
                .expect("composite hash"),
            epoch: 1,
            epoch_id_sha256: digest('2'),
            install_epoch_trust_profile_sha256,
            machine_id_sha256: target.machine_id_sha256.clone(),
            schema: "hepta_linux_mnl_install_epoch_completion_v1".to_string(),
            schema_version: 1,
            signed_statement_sha256: digest('1'),
            state_root_layout_manifest_sha256: state_root.layout_manifest_sha256.clone(),
            workload_identity_sha256: target.workload_identity_sha256.clone(),
        };
        let install_epoch_completion_sha256 =
            canonical_sha256(&install_epoch_completion).expect("install epoch hash");
        let external_watermark_provider = ExternalWatermarkProviderProfileV1 {
            commit_trust_profile_sha256,
            current_tip_trust_profile_sha256: current_tip_trust_profile_sha256.clone(),
            genesis_epoch_binding_sha256: install_epoch_completion_sha256,
            genesis_revision: 0,
            genesis_tip_sha256: digest('8'),
            lease_trust_profile_sha256,
            profile_id: "hepta-linux-mnl-external-watermark-provider-v1".to_string(),
            profile_revision: 1,
            schema: "hepta_linux_mnl_external_watermark_provider_profile_v1".to_string(),
            schema_version: 1,
            stream_id_sha256: digest('7'),
        };
        let provider_profile_sha256 =
            canonical_sha256(&external_watermark_provider).expect("provider hash");
        PublishedProfileDocumentsV1 {
            composite_identity,
            external_watermark_current_tip: ExternalWatermarkCurrentTipV1 {
                current_tip_trust_profile_sha256,
                observed_revision: 0,
                observed_tip_sha256: digest('8'),
                provider_profile_sha256,
                schema: "hepta_linux_mnl_external_watermark_current_tip_v1".to_string(),
                schema_version: 1,
                signed_statement_sha256: digest('6'),
                stream_id_sha256: digest('7'),
            },
            external_watermark_provider,
            install_epoch_completion,
            schema: "hepta_linux_mnl_published_profile_documents_v1".to_string(),
            schema_version: 1,
            state_root,
            target,
            trust_profiles,
        }
    }

    pub(crate) fn pins(documents: &PublishedProfileDocumentsV1) -> CompiledPublishedProfilePinsV1 {
        let trust_profile_sha256 = std::array::from_fn(|index| {
            let digest = canonical_sha256(&documents.trust_profiles[index]).expect("trust hash");
            Some(Box::leak(digest.into_boxed_str()) as &'static str)
        });
        CompiledPublishedProfilePinsV1 {
            current_tip_sha256: Some(Box::leak(
                canonical_sha256(&documents.external_watermark_current_tip)
                    .expect("tip hash")
                    .into_boxed_str(),
            )),
            install_epoch_completion_sha256: Some(Box::leak(
                canonical_sha256(&documents.install_epoch_completion)
                    .expect("install epoch hash")
                    .into_boxed_str(),
            )),
            provider_sha256: Some(Box::leak(
                canonical_sha256(&documents.external_watermark_provider)
                    .expect("provider hash")
                    .into_boxed_str(),
            )),
            state_root_sha256: Some(Box::leak(
                canonical_sha256(&documents.state_root)
                    .expect("state hash")
                    .into_boxed_str(),
            )),
            target_sha256: Some(Box::leak(
                canonical_sha256(&documents.target)
                    .expect("target hash")
                    .into_boxed_str(),
            )),
            trust_profile_sha256,
        }
    }

    pub(crate) fn verify(
        documents: &PublishedProfileDocumentsV1,
        pins: CompiledPublishedProfilePinsV1,
    ) -> Result<VerifiedPublishedProfilesV1, LinuxMnlError> {
        validate_published_profiles_against(documents, pins)
    }

    pub(crate) fn rehash(
        documents: &PublishedProfileDocumentsV1,
    ) -> CompiledPublishedProfilePinsV1 {
        pins(documents)
    }

    pub(crate) fn digest(character: char) -> String {
        std::iter::repeat_n(character, 64).collect()
    }

    pub(crate) fn nonempty_sha256(label: &str) -> String {
        sha256(label.as_bytes())
    }
}
