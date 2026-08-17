use serde::Deserialize;
use serde::Serialize;
use sha2::Digest;

use crate::DetachedSignatureRoleV1;
use crate::MnlTrustError;
use crate::VerifiedDetachedSignatureInspectionV1;
use crate::invalid;

pub const PRE_RUN_REPLAY_CLAIM_SCHEMA: &str = "hepta_mnl_pre_run_replay_claim_v1";
pub const COPY_ACK_REPLAY_CLAIM_SCHEMA: &str = "hepta_mnl_copy_ack_replay_claim_v1";
pub const SIGNED_PRE_RUN_REPLAY_PROFILE_SCHEMA: &str = "hepta_mnl_signed_pre_run_replay_profile_v1";
pub const MAX_REPLAY_CLAIM_BYTES: usize = 64 * 1024;
pub const MAX_SIGNED_FRESHNESS_LIFETIME_SECONDS: u64 = 60 * 60;

const PRE_RUN_SLOT_DOMAIN: &[u8] = b"hepta.mnl.replay-slot.pre-run.v1\0";
const COPY_ACK_SLOT_DOMAIN: &[u8] = b"hepta.mnl.replay-slot.copy-ack.v1\0";
const PRE_RUN_FULL_BINDING_DOMAIN: &[u8] = b"hepta.mnl.replay-binding.pre-run.v1\0";
const COPY_ACK_FULL_BINDING_DOMAIN: &[u8] = b"hepta.mnl.replay-binding.copy-ack.v1\0";
const RUN_IDENTITY_DOMAIN: &[u8] = b"hepta.mnl.run-identity.v1\0";
const MAX_IDENTIFIER_BYTES: usize = 128;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ReplayClaimNamespaceV1 {
    #[serde(rename = "pre_run_launch")]
    PreRunLaunch,
    #[serde(rename = "independent_copy_ack")]
    IndependentCopyAck,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ReplayPlatformScopeV1 {
    #[serde(rename = "macos")]
    MacOs,
    #[serde(rename = "linux_phase1")]
    LinuxPhase1,
    #[serde(rename = "nix")]
    Nix,
}

/// Exact semantic payload authenticated by the N2 pre-run profile signature.
///
/// The detached-signature manifest authenticates the exact canonical bytes of
/// this value. N3 reparses those retained bytes and requires every replay
/// claim field below to agree, preventing a valid signature inspection from
/// being transplanted onto caller-authored nonce, clock, host, or store data.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SignedPreRunReplayProfileV1 {
    pub authorized_copy_ack_signer_key_id: String,
    pub boot_id_sha256: String,
    pub challenge_nonce_sha256: String,
    pub copy_replay_store_identity_sha256: String,
    pub copy_session_nonce_sha256: String,
    pub expires_at_unix_seconds: u64,
    pub final_artifact_freeze_manifest_sha256: String,
    pub final_artifact_freeze_payload_sha256: String,
    pub final_artifact_freeze_profile_id: String,
    pub final_artifact_freeze_signature_sha256: String,
    pub final_artifact_freeze_signed_frame_sha256: String,
    pub final_artifact_freeze_signer_key_id: String,
    pub generation_epoch_id: String,
    pub host_identity_sha256: String,
    pub maximum_lifetime_seconds: u64,
    pub not_before_unix_seconds: u64,
    pub platform_scope: ReplayPlatformScopeV1,
    pub pre_run_replay_store_identity_sha256: String,
    pub profile_id: String,
    pub run_identity_sha256: String,
    pub run_nonce_sha256: String,
    pub schema: String,
    pub session_nonce_sha256: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PreRunReplayClaimWireV1 {
    pub authorized_copy_ack_signer_key_id: String,
    pub boot_id_sha256: String,
    pub challenge_nonce_sha256: String,
    pub copy_replay_store_identity_sha256: String,
    pub copy_session_nonce_sha256: String,
    pub expires_at_unix_seconds: u64,
    pub final_artifact_freeze_manifest_sha256: String,
    pub final_artifact_freeze_payload_sha256: String,
    pub final_artifact_freeze_profile_id: String,
    pub final_artifact_freeze_signature_sha256: String,
    pub final_artifact_freeze_signed_frame_sha256: String,
    pub final_artifact_freeze_signer_key_id: String,
    pub generation_epoch_id: String,
    pub host_identity_sha256: String,
    pub maximum_lifetime_seconds: u64,
    pub namespace: ReplayClaimNamespaceV1,
    pub not_before_unix_seconds: u64,
    pub platform_scope: ReplayPlatformScopeV1,
    pub pre_run_profile_manifest_sha256: String,
    pub pre_run_profile_payload_sha256: String,
    pub pre_run_profile_signature_sha256: String,
    pub pre_run_profile_signed_frame_sha256: String,
    pub pre_run_profile_signer_key_id: String,
    pub profile_id: String,
    pub replay_slot_sha256: String,
    pub pre_run_replay_store_identity_sha256: String,
    pub run_identity_sha256: String,
    pub run_nonce_sha256: String,
    pub schema: String,
    pub session_nonce_sha256: String,
    pub trust_policy_sha256: String,
    pub trust_root_id: String,
    pub trust_root_revision: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CopyAckReplayClaimWireV1 {
    pub authorized_copy_ack_signer_key_id: String,
    pub boot_id_sha256: String,
    pub copy_session_nonce_sha256: String,
    pub copy_replay_store_identity_sha256: String,
    pub destination_failure_domain_id: String,
    pub destination_identity_sha256: String,
    pub final_artifact_freeze_manifest_sha256: String,
    pub final_artifact_freeze_payload_sha256: String,
    pub final_artifact_freeze_profile_id: String,
    pub final_artifact_freeze_signature_sha256: String,
    pub final_artifact_freeze_signed_frame_sha256: String,
    pub final_artifact_freeze_signer_key_id: String,
    pub host_identity_sha256: String,
    pub namespace: ReplayClaimNamespaceV1,
    pub platform_scope: ReplayPlatformScopeV1,
    pub pre_run_full_binding_sha256: String,
    pub pre_run_profile_manifest_sha256: String,
    pub pre_run_profile_payload_sha256: String,
    pub pre_run_profile_signature_sha256: String,
    pub pre_run_profile_signed_frame_sha256: String,
    pub pre_run_profile_signer_key_id: String,
    pub pre_run_replay_slot_sha256: String,
    pub profile_id: String,
    pub replay_slot_sha256: String,
    pub run_identity_sha256: String,
    pub run_nonce_sha256: String,
    pub schema: String,
    pub sealed_bundle_byte_count: u64,
    pub sealed_bundle_sha256: String,
    pub trust_policy_sha256: String,
    pub trust_root_id: String,
    pub trust_root_revision: u64,
}

#[derive(Debug)]
pub struct PreparedPreRunReplayClaimV1 {
    authorized_copy_ack_signer_key_id: String,
    binding: PreparedReplayBindingV1,
    boot_id_sha256: String,
    challenge_nonce_sha256: String,
    copy_session_nonce_sha256: String,
    host_identity_sha256: String,
    platform_scope: ReplayPlatformScopeV1,
    profile_id: String,
    copy_replay_store_identity_sha256: String,
    final_artifact_freeze_signature: InspectedSignatureBindingV1,
    pre_run_profile_signature: InspectedSignatureBindingV1,
    pre_run_replay_store_identity_sha256: String,
    run_identity_sha256: String,
    run_nonce_sha256: String,
}

#[derive(Debug)]
pub struct PreparedCopyAckReplayClaimV1 {
    binding: PreparedReplayBindingV1,
    copy_session_nonce_sha256: String,
    platform_scope: ReplayPlatformScopeV1,
    pre_run_full_binding_sha256: String,
    pre_run_replay_slot_sha256: String,
    profile_id: String,
    copy_replay_store_identity_sha256: String,
}

#[derive(Debug)]
struct PreparedReplayBindingV1 {
    canonical_claim_sha256: String,
    final_leaf_name: String,
    full_binding_sha256: String,
    namespace: ReplayClaimNamespaceV1,
    record_bytes: Vec<u8>,
    replay_slot_sha256: String,
    trust_policy_sha256: String,
    trust_root_id: String,
    trust_root_revision: u64,
}

#[derive(Debug)]
struct InspectedSignatureBindingV1 {
    manifest_sha256: String,
    payload_sha256: String,
    profile_id: String,
    signature_sha256: String,
    signed_frame_sha256: String,
    signer_key_id: String,
}

impl InspectedSignatureBindingV1 {
    fn from_inspection(inspection: &VerifiedDetachedSignatureInspectionV1) -> Self {
        Self {
            manifest_sha256: inspection.manifest_sha256().to_string(),
            payload_sha256: inspection.payload_sha256().to_string(),
            profile_id: inspection.profile_id().to_string(),
            signature_sha256: inspection.signature_sha256().to_string(),
            signed_frame_sha256: inspection.signed_frame_sha256().to_string(),
            signer_key_id: inspection.signer_key_id().to_string(),
        }
    }

    fn matches(
        &self,
        profile_id: &str,
        signer_key_id: &str,
        manifest_sha256: &str,
        payload_sha256: &str,
        signed_frame_sha256: &str,
        signature_sha256: &str,
    ) -> bool {
        self.profile_id == profile_id
            && self.signer_key_id == signer_key_id
            && self.manifest_sha256 == manifest_sha256
            && self.payload_sha256 == payload_sha256
            && self.signed_frame_sha256 == signed_frame_sha256
            && self.signature_sha256 == signature_sha256
    }
}

macro_rules! common_prepared_getters {
    () => {
        pub fn canonical_claim_sha256(&self) -> &str {
            &self.binding.canonical_claim_sha256
        }

        pub fn final_leaf_name(&self) -> &str {
            &self.binding.final_leaf_name
        }

        pub fn full_binding_sha256(&self) -> &str {
            &self.binding.full_binding_sha256
        }

        pub fn namespace(&self) -> ReplayClaimNamespaceV1 {
            self.binding.namespace
        }

        pub fn record_bytes(&self) -> &[u8] {
            &self.binding.record_bytes
        }

        pub fn replay_slot_sha256(&self) -> &str {
            &self.binding.replay_slot_sha256
        }

        pub fn trust_root_id(&self) -> &str {
            &self.binding.trust_root_id
        }

        pub fn trust_root_revision(&self) -> u64 {
            self.binding.trust_root_revision
        }

        pub const fn authorizes_live(&self) -> bool {
            false
        }

        pub const fn durable_commit_observed(&self) -> bool {
            false
        }

        pub const fn wall_clock_verified(&self) -> bool {
            false
        }
    };
}

impl PreparedPreRunReplayClaimV1 {
    common_prepared_getters!();

    pub fn boot_id_sha256(&self) -> &str {
        &self.boot_id_sha256
    }

    pub fn challenge_nonce_sha256(&self) -> &str {
        &self.challenge_nonce_sha256
    }

    pub fn copy_session_nonce_sha256(&self) -> &str {
        &self.copy_session_nonce_sha256
    }

    pub fn host_identity_sha256(&self) -> &str {
        &self.host_identity_sha256
    }

    pub fn platform_scope(&self) -> ReplayPlatformScopeV1 {
        self.platform_scope
    }

    pub fn profile_id(&self) -> &str {
        &self.profile_id
    }

    pub fn authorized_copy_ack_signer_key_id(&self) -> &str {
        &self.authorized_copy_ack_signer_key_id
    }

    pub fn copy_replay_store_identity_sha256(&self) -> &str {
        &self.copy_replay_store_identity_sha256
    }

    pub fn pre_run_replay_store_identity_sha256(&self) -> &str {
        &self.pre_run_replay_store_identity_sha256
    }

    pub fn run_identity_sha256(&self) -> &str {
        &self.run_identity_sha256
    }

    pub fn run_nonce_sha256(&self) -> &str {
        &self.run_nonce_sha256
    }
}

impl PreparedCopyAckReplayClaimV1 {
    common_prepared_getters!();

    pub fn copy_session_nonce_sha256(&self) -> &str {
        &self.copy_session_nonce_sha256
    }

    pub fn platform_scope(&self) -> ReplayPlatformScopeV1 {
        self.platform_scope
    }

    pub fn pre_run_full_binding_sha256(&self) -> &str {
        &self.pre_run_full_binding_sha256
    }

    pub fn pre_run_replay_slot_sha256(&self) -> &str {
        &self.pre_run_replay_slot_sha256
    }

    pub fn profile_id(&self) -> &str {
        &self.profile_id
    }

    pub fn copy_replay_store_identity_sha256(&self) -> &str {
        &self.copy_replay_store_identity_sha256
    }
}

pub fn inspect_canonical_pre_run_replay_claim(
    final_artifact_freeze: &VerifiedDetachedSignatureInspectionV1,
    pre_run_profile: &VerifiedDetachedSignatureInspectionV1,
    canonical_claim: &[u8],
) -> Result<PreparedPreRunReplayClaimV1, MnlTrustError> {
    require_signature_role(
        final_artifact_freeze,
        DetachedSignatureRoleV1::FinalArtifactFreeze,
    )?;
    require_signature_role(pre_run_profile, DetachedSignatureRoleV1::PreRunProfile)?;
    require_common_signature_policy(final_artifact_freeze, pre_run_profile)?;
    let signed_profile: SignedPreRunReplayProfileV1 =
        parse_canonical_claim(pre_run_profile.exact_payload_bytes())?;
    validate_signed_pre_run_profile(final_artifact_freeze, pre_run_profile, &signed_profile)?;
    let claim: PreRunReplayClaimWireV1 = parse_canonical_claim(canonical_claim)?;
    if claim.schema != PRE_RUN_REPLAY_CLAIM_SCHEMA
        || claim.namespace != ReplayClaimNamespaceV1::PreRunLaunch
    {
        return Err(invalid(
            "pre-run replay claim schema or namespace is not exact",
        ));
    }
    validate_common_claim_fields(
        &claim.trust_root_id,
        claim.trust_root_revision,
        &claim.trust_policy_sha256,
        &claim.profile_id,
        &claim.pre_run_replay_store_identity_sha256,
    )?;
    validate_sha256(
        &claim.copy_replay_store_identity_sha256,
        "copy replay store identity",
    )?;
    require_signature_bindings(
        final_artifact_freeze,
        &claim.final_artifact_freeze_profile_id,
        &claim.final_artifact_freeze_signer_key_id,
        &claim.final_artifact_freeze_manifest_sha256,
        &claim.final_artifact_freeze_payload_sha256,
        &claim.final_artifact_freeze_signed_frame_sha256,
        &claim.final_artifact_freeze_signature_sha256,
    )?;
    require_signature_bindings(
        pre_run_profile,
        &claim.profile_id,
        &claim.pre_run_profile_signer_key_id,
        &claim.pre_run_profile_manifest_sha256,
        &claim.pre_run_profile_payload_sha256,
        &claim.pre_run_profile_signed_frame_sha256,
        &claim.pre_run_profile_signature_sha256,
    )?;
    if claim.trust_root_id != pre_run_profile.trust_root_id()
        || claim.trust_root_revision != pre_run_profile.trust_root_revision()
        || claim.trust_policy_sha256 != pre_run_profile.trust_policy_sha256()
        || !pre_run_claim_matches_signed_profile(&claim, &signed_profile)
    {
        return Err(invalid(
            "pre-run replay claim differs from its exact signed profile semantics",
        ));
    }
    validate_identifier(&claim.generation_epoch_id, "generation epoch id")?;
    validate_sha256_fields(&[
        (&claim.boot_id_sha256, "boot identity"),
        (&claim.challenge_nonce_sha256, "challenge nonce"),
        (&claim.copy_session_nonce_sha256, "copy session nonce"),
        (&claim.host_identity_sha256, "host identity"),
        (&claim.run_identity_sha256, "run identity"),
        (&claim.run_nonce_sha256, "run nonce"),
        (&claim.session_nonce_sha256, "session nonce"),
    ])?;
    if claim.challenge_nonce_sha256 == claim.session_nonce_sha256
        || claim.challenge_nonce_sha256 == claim.copy_session_nonce_sha256
        || claim.session_nonce_sha256 == claim.copy_session_nonce_sha256
        || claim.session_nonce_sha256 != claim.run_nonce_sha256
    {
        return Err(invalid(
            "pre-run replay nonces are not purpose-separated or session/run-bound",
        ));
    }
    let run_identity = derive_run_identity_sha256(&claim.run_nonce_sha256, &claim.boot_id_sha256)?;
    if claim.run_identity_sha256 != run_identity {
        return Err(invalid("pre-run replay run identity is not exact"));
    }
    validate_freshness_window(
        claim.not_before_unix_seconds,
        claim.expires_at_unix_seconds,
        claim.maximum_lifetime_seconds,
    )?;
    let replay_slot_sha256 =
        derive_pre_run_replay_slot_sha256(&claim.trust_root_id, &claim.challenge_nonce_sha256)?;
    if claim.replay_slot_sha256 != replay_slot_sha256 {
        return Err(invalid("pre-run replay slot digest is not exact"));
    }
    let binding = prepare_binding(
        ReplayClaimNamespaceV1::PreRunLaunch,
        PRE_RUN_FULL_BINDING_DOMAIN,
        canonical_claim,
        replay_slot_sha256,
        &claim.trust_policy_sha256,
        &claim.trust_root_id,
        claim.trust_root_revision,
    )?;
    Ok(PreparedPreRunReplayClaimV1 {
        authorized_copy_ack_signer_key_id: claim.authorized_copy_ack_signer_key_id,
        binding,
        boot_id_sha256: claim.boot_id_sha256,
        challenge_nonce_sha256: claim.challenge_nonce_sha256,
        copy_session_nonce_sha256: claim.copy_session_nonce_sha256,
        host_identity_sha256: claim.host_identity_sha256,
        platform_scope: claim.platform_scope,
        profile_id: claim.profile_id,
        copy_replay_store_identity_sha256: claim.copy_replay_store_identity_sha256,
        final_artifact_freeze_signature: InspectedSignatureBindingV1::from_inspection(
            final_artifact_freeze,
        ),
        pre_run_profile_signature: InspectedSignatureBindingV1::from_inspection(pre_run_profile),
        pre_run_replay_store_identity_sha256: claim.pre_run_replay_store_identity_sha256,
        run_identity_sha256: claim.run_identity_sha256,
        run_nonce_sha256: claim.run_nonce_sha256,
    })
}

pub fn inspect_canonical_copy_ack_replay_claim(
    prepared_pre_run: &PreparedPreRunReplayClaimV1,
    canonical_claim: &[u8],
) -> Result<PreparedCopyAckReplayClaimV1, MnlTrustError> {
    let claim: CopyAckReplayClaimWireV1 = parse_canonical_claim(canonical_claim)?;
    if claim.schema != COPY_ACK_REPLAY_CLAIM_SCHEMA
        || claim.namespace != ReplayClaimNamespaceV1::IndependentCopyAck
    {
        return Err(invalid(
            "copy-ack replay claim schema or namespace is not exact",
        ));
    }
    validate_common_claim_fields(
        &claim.trust_root_id,
        claim.trust_root_revision,
        &claim.trust_policy_sha256,
        &claim.profile_id,
        &claim.copy_replay_store_identity_sha256,
    )?;
    if !prepared_pre_run.final_artifact_freeze_signature.matches(
        &claim.final_artifact_freeze_profile_id,
        &claim.final_artifact_freeze_signer_key_id,
        &claim.final_artifact_freeze_manifest_sha256,
        &claim.final_artifact_freeze_payload_sha256,
        &claim.final_artifact_freeze_signed_frame_sha256,
        &claim.final_artifact_freeze_signature_sha256,
    ) || !prepared_pre_run.pre_run_profile_signature.matches(
        &claim.profile_id,
        &claim.pre_run_profile_signer_key_id,
        &claim.pre_run_profile_manifest_sha256,
        &claim.pre_run_profile_payload_sha256,
        &claim.pre_run_profile_signed_frame_sha256,
        &claim.pre_run_profile_signature_sha256,
    ) {
        return Err(invalid(
            "copy-ack replay claim differs from prepared signature lineage",
        ));
    }
    if claim.trust_root_id != prepared_pre_run.trust_root_id()
        || claim.trust_root_revision != prepared_pre_run.trust_root_revision()
        || claim.profile_id != prepared_pre_run.profile_id()
        || claim.platform_scope != prepared_pre_run.platform_scope()
        || claim.pre_run_replay_slot_sha256 != prepared_pre_run.replay_slot_sha256()
        || claim.pre_run_full_binding_sha256 != prepared_pre_run.full_binding_sha256()
        || claim.copy_session_nonce_sha256 != prepared_pre_run.copy_session_nonce_sha256()
        || claim.run_nonce_sha256 != prepared_pre_run.run_nonce_sha256()
        || claim.run_identity_sha256 != prepared_pre_run.run_identity_sha256()
        || claim.boot_id_sha256 != prepared_pre_run.boot_id_sha256()
        || claim.host_identity_sha256 != prepared_pre_run.host_identity_sha256()
        || claim.trust_policy_sha256 != prepared_pre_run.binding.trust_policy_sha256.as_str()
        || claim.authorized_copy_ack_signer_key_id
            != prepared_pre_run.authorized_copy_ack_signer_key_id()
        || claim.copy_replay_store_identity_sha256
            != prepared_pre_run.copy_replay_store_identity_sha256()
    {
        return Err(invalid(
            "copy-ack replay claim differs from its prepared pre-run binding",
        ));
    }
    validate_identifier(
        &claim.authorized_copy_ack_signer_key_id,
        "authorized copy-ack signer key id",
    )?;
    validate_identifier(
        &claim.destination_failure_domain_id,
        "destination failure-domain id",
    )?;
    validate_sha256_fields(&[
        (&claim.boot_id_sha256, "boot identity"),
        (&claim.copy_session_nonce_sha256, "copy session nonce"),
        (&claim.destination_identity_sha256, "destination identity"),
        (&claim.host_identity_sha256, "host identity"),
        (&claim.pre_run_full_binding_sha256, "pre-run full binding"),
        (&claim.pre_run_replay_slot_sha256, "pre-run replay slot"),
        (&claim.run_identity_sha256, "run identity"),
        (&claim.run_nonce_sha256, "run nonce"),
        (&claim.sealed_bundle_sha256, "sealed bundle"),
    ])?;
    if claim.sealed_bundle_byte_count == 0 {
        return Err(invalid("copy-ack sealed bundle byte count is zero"));
    }
    if claim.copy_replay_store_identity_sha256
        == prepared_pre_run.pre_run_replay_store_identity_sha256()
        || claim.destination_identity_sha256 == prepared_pre_run.host_identity_sha256()
    {
        return Err(invalid(
            "copy-ack replay store or destination is not independent from the pre-run source",
        ));
    }
    let replay_slot_sha256 =
        derive_copy_ack_replay_slot_sha256(&claim.trust_root_id, &claim.copy_session_nonce_sha256)?;
    if claim.replay_slot_sha256 != replay_slot_sha256 {
        return Err(invalid("copy-ack replay slot digest is not exact"));
    }
    let binding = prepare_binding(
        ReplayClaimNamespaceV1::IndependentCopyAck,
        COPY_ACK_FULL_BINDING_DOMAIN,
        canonical_claim,
        replay_slot_sha256,
        &claim.trust_policy_sha256,
        &claim.trust_root_id,
        claim.trust_root_revision,
    )?;
    Ok(PreparedCopyAckReplayClaimV1 {
        binding,
        copy_session_nonce_sha256: claim.copy_session_nonce_sha256,
        platform_scope: claim.platform_scope,
        pre_run_full_binding_sha256: claim.pre_run_full_binding_sha256,
        pre_run_replay_slot_sha256: claim.pre_run_replay_slot_sha256,
        profile_id: claim.profile_id,
        copy_replay_store_identity_sha256: claim.copy_replay_store_identity_sha256,
    })
}

pub fn derive_pre_run_replay_slot_sha256(
    trust_root_id: &str,
    challenge_nonce_sha256: &str,
) -> Result<String, MnlTrustError> {
    derive_replay_slot_sha256(PRE_RUN_SLOT_DOMAIN, trust_root_id, challenge_nonce_sha256)
}

pub fn derive_copy_ack_replay_slot_sha256(
    trust_root_id: &str,
    copy_session_nonce_sha256: &str,
) -> Result<String, MnlTrustError> {
    derive_replay_slot_sha256(
        COPY_ACK_SLOT_DOMAIN,
        trust_root_id,
        copy_session_nonce_sha256,
    )
}

pub fn derive_run_identity_sha256(
    run_nonce_sha256: &str,
    boot_id_sha256: &str,
) -> Result<String, MnlTrustError> {
    let run_nonce = decode_sha256(run_nonce_sha256, "run nonce")?;
    let boot_id = decode_sha256(boot_id_sha256, "boot identity")?;
    let mut frame = Vec::with_capacity(RUN_IDENTITY_DOMAIN.len() + 64);
    frame.extend_from_slice(RUN_IDENTITY_DOMAIN);
    frame.extend_from_slice(&run_nonce);
    frame.extend_from_slice(&boot_id);
    Ok(sha256_hex(&frame))
}

fn derive_replay_slot_sha256(
    domain: &[u8],
    trust_root_id: &str,
    primary_nonce_sha256: &str,
) -> Result<String, MnlTrustError> {
    validate_identifier(trust_root_id, "trust root id")?;
    let nonce = decode_sha256(primary_nonce_sha256, "primary replay nonce")?;
    let mut frame = Vec::new();
    frame.extend_from_slice(domain);
    append_length_prefixed(&mut frame, trust_root_id.as_bytes())?;
    frame.extend_from_slice(&nonce);
    Ok(sha256_hex(&frame))
}

fn prepare_binding(
    namespace: ReplayClaimNamespaceV1,
    domain: &[u8],
    canonical_claim: &[u8],
    replay_slot_sha256: String,
    trust_policy_sha256: &str,
    trust_root_id: &str,
    trust_root_revision: u64,
) -> Result<PreparedReplayBindingV1, MnlTrustError> {
    let mut record_bytes = Vec::new();
    record_bytes.extend_from_slice(domain);
    append_length_prefixed(&mut record_bytes, canonical_claim)?;
    Ok(PreparedReplayBindingV1 {
        canonical_claim_sha256: sha256_hex(canonical_claim),
        final_leaf_name: format!("{replay_slot_sha256}.claim-v1"),
        full_binding_sha256: sha256_hex(&record_bytes),
        namespace,
        record_bytes,
        replay_slot_sha256,
        trust_policy_sha256: trust_policy_sha256.to_string(),
        trust_root_id: trust_root_id.to_string(),
        trust_root_revision,
    })
}

fn require_signature_role(
    inspection: &VerifiedDetachedSignatureInspectionV1,
    expected: DetachedSignatureRoleV1,
) -> Result<(), MnlTrustError> {
    if inspection.role() != expected || inspection.authorizes_live() {
        return Err(invalid(
            "replay claim uses the wrong signature role or an authorizing token",
        ));
    }
    Ok(())
}

fn require_common_signature_policy(
    final_artifact_freeze: &VerifiedDetachedSignatureInspectionV1,
    pre_run_profile: &VerifiedDetachedSignatureInspectionV1,
) -> Result<(), MnlTrustError> {
    if final_artifact_freeze.trust_root_id() != pre_run_profile.trust_root_id()
        || final_artifact_freeze.trust_root_revision() != pre_run_profile.trust_root_revision()
        || final_artifact_freeze.trust_policy_sha256() != pre_run_profile.trust_policy_sha256()
    {
        return Err(invalid(
            "replay claim signature inspections do not share one trust policy",
        ));
    }
    Ok(())
}

fn require_signature_bindings(
    inspection: &VerifiedDetachedSignatureInspectionV1,
    profile_id: &str,
    signer_key_id: &str,
    manifest_sha256: &str,
    payload_sha256: &str,
    signed_frame_sha256: &str,
    signature_sha256: &str,
) -> Result<(), MnlTrustError> {
    if inspection.profile_id() != profile_id
        || inspection.signer_key_id() != signer_key_id
        || inspection.manifest_sha256() != manifest_sha256
        || inspection.payload_sha256() != payload_sha256
        || inspection.signed_frame_sha256() != signed_frame_sha256
        || inspection.signature_sha256() != signature_sha256
    {
        return Err(invalid(
            "replay claim differs from exact detached-signature inspection",
        ));
    }
    Ok(())
}

fn validate_signed_pre_run_profile(
    final_artifact_freeze: &VerifiedDetachedSignatureInspectionV1,
    pre_run_profile: &VerifiedDetachedSignatureInspectionV1,
    profile: &SignedPreRunReplayProfileV1,
) -> Result<(), MnlTrustError> {
    if profile.schema != SIGNED_PRE_RUN_REPLAY_PROFILE_SCHEMA
        || profile.profile_id != pre_run_profile.profile_id()
    {
        return Err(invalid(
            "signed pre-run replay profile schema or profile id is not exact",
        ));
    }
    require_signature_bindings(
        final_artifact_freeze,
        &profile.final_artifact_freeze_profile_id,
        &profile.final_artifact_freeze_signer_key_id,
        &profile.final_artifact_freeze_manifest_sha256,
        &profile.final_artifact_freeze_payload_sha256,
        &profile.final_artifact_freeze_signed_frame_sha256,
        &profile.final_artifact_freeze_signature_sha256,
    )?;
    validate_identifier(
        &profile.authorized_copy_ack_signer_key_id,
        "authorized copy-ack signer key id",
    )?;
    validate_identifier(&profile.generation_epoch_id, "generation epoch id")?;
    validate_sha256_fields(&[
        (&profile.boot_id_sha256, "boot identity"),
        (&profile.challenge_nonce_sha256, "challenge nonce"),
        (
            &profile.copy_replay_store_identity_sha256,
            "copy replay store identity",
        ),
        (&profile.copy_session_nonce_sha256, "copy session nonce"),
        (&profile.host_identity_sha256, "host identity"),
        (
            &profile.pre_run_replay_store_identity_sha256,
            "pre-run replay store identity",
        ),
        (&profile.run_identity_sha256, "run identity"),
        (&profile.run_nonce_sha256, "run nonce"),
        (&profile.session_nonce_sha256, "session nonce"),
    ])?;
    if profile.pre_run_replay_store_identity_sha256 == profile.copy_replay_store_identity_sha256 {
        return Err(invalid(
            "signed pre-run and copy replay stores are not independent",
        ));
    }
    if profile.challenge_nonce_sha256 == profile.session_nonce_sha256
        || profile.challenge_nonce_sha256 == profile.copy_session_nonce_sha256
        || profile.session_nonce_sha256 == profile.copy_session_nonce_sha256
        || profile.session_nonce_sha256 != profile.run_nonce_sha256
    {
        return Err(invalid(
            "signed pre-run replay nonces are not purpose-separated or session/run-bound",
        ));
    }
    if profile.run_identity_sha256
        != derive_run_identity_sha256(&profile.run_nonce_sha256, &profile.boot_id_sha256)?
    {
        return Err(invalid("signed pre-run replay run identity is not exact"));
    }
    validate_freshness_window(
        profile.not_before_unix_seconds,
        profile.expires_at_unix_seconds,
        profile.maximum_lifetime_seconds,
    )
}

fn pre_run_claim_matches_signed_profile(
    claim: &PreRunReplayClaimWireV1,
    profile: &SignedPreRunReplayProfileV1,
) -> bool {
    claim.authorized_copy_ack_signer_key_id == profile.authorized_copy_ack_signer_key_id
        && claim.boot_id_sha256 == profile.boot_id_sha256
        && claim.challenge_nonce_sha256 == profile.challenge_nonce_sha256
        && claim.copy_replay_store_identity_sha256 == profile.copy_replay_store_identity_sha256
        && claim.copy_session_nonce_sha256 == profile.copy_session_nonce_sha256
        && claim.expires_at_unix_seconds == profile.expires_at_unix_seconds
        && claim.final_artifact_freeze_manifest_sha256
            == profile.final_artifact_freeze_manifest_sha256
        && claim.final_artifact_freeze_payload_sha256
            == profile.final_artifact_freeze_payload_sha256
        && claim.final_artifact_freeze_profile_id == profile.final_artifact_freeze_profile_id
        && claim.final_artifact_freeze_signature_sha256
            == profile.final_artifact_freeze_signature_sha256
        && claim.final_artifact_freeze_signed_frame_sha256
            == profile.final_artifact_freeze_signed_frame_sha256
        && claim.final_artifact_freeze_signer_key_id == profile.final_artifact_freeze_signer_key_id
        && claim.generation_epoch_id == profile.generation_epoch_id
        && claim.host_identity_sha256 == profile.host_identity_sha256
        && claim.maximum_lifetime_seconds == profile.maximum_lifetime_seconds
        && claim.not_before_unix_seconds == profile.not_before_unix_seconds
        && claim.platform_scope == profile.platform_scope
        && claim.pre_run_replay_store_identity_sha256
            == profile.pre_run_replay_store_identity_sha256
        && claim.profile_id == profile.profile_id
        && claim.run_identity_sha256 == profile.run_identity_sha256
        && claim.run_nonce_sha256 == profile.run_nonce_sha256
        && claim.session_nonce_sha256 == profile.session_nonce_sha256
}

fn parse_canonical_claim<T>(canonical_claim: &[u8]) -> Result<T, MnlTrustError>
where
    T: for<'de> Deserialize<'de> + Serialize,
{
    if canonical_claim.is_empty() || canonical_claim.len() > MAX_REPLAY_CLAIM_BYTES {
        return Err(invalid("replay claim byte length is outside its bound"));
    }
    let claim: T = serde_json::from_slice(canonical_claim)
        .map_err(|error| MnlTrustError::Serialization(error.to_string()))?;
    let reencoded = serde_json::to_vec(&claim)
        .map_err(|error| MnlTrustError::Serialization(error.to_string()))?;
    if reencoded != canonical_claim {
        return Err(invalid("replay claim is not exact canonical JSON"));
    }
    Ok(claim)
}

fn validate_common_claim_fields(
    trust_root_id: &str,
    trust_root_revision: u64,
    trust_policy_sha256: &str,
    profile_id: &str,
    replay_store_identity_sha256: &str,
) -> Result<(), MnlTrustError> {
    validate_identifier(trust_root_id, "trust root id")?;
    validate_identifier(profile_id, "profile id")?;
    if trust_root_revision == 0 {
        return Err(invalid("trust root revision must be positive"));
    }
    validate_sha256(trust_policy_sha256, "trust policy")?;
    validate_sha256(replay_store_identity_sha256, "replay store identity")
}

fn validate_freshness_window(
    not_before_unix_seconds: u64,
    expires_at_unix_seconds: u64,
    maximum_lifetime_seconds: u64,
) -> Result<(), MnlTrustError> {
    if not_before_unix_seconds > i64::MAX as u64 || expires_at_unix_seconds > i64::MAX as u64 {
        return Err(invalid(
            "signed freshness window is not representable by a 64-bit system clock",
        ));
    }
    let lifetime = expires_at_unix_seconds
        .checked_sub(not_before_unix_seconds)
        .ok_or_else(|| invalid("signed freshness window is reversed"))?;
    if lifetime == 0
        || maximum_lifetime_seconds == 0
        || lifetime > maximum_lifetime_seconds
        || maximum_lifetime_seconds > MAX_SIGNED_FRESHNESS_LIFETIME_SECONDS
    {
        return Err(invalid(
            "signed freshness window exceeds its structural lifetime policy",
        ));
    }
    Ok(())
}

fn validate_sha256_fields(values: &[(&str, &str)]) -> Result<(), MnlTrustError> {
    for (value, label) in values {
        validate_sha256(value, label)?;
    }
    Ok(())
}

fn validate_identifier(value: &str, label: &str) -> Result<(), MnlTrustError> {
    let mut bytes = value.bytes();
    let Some(first) = bytes.next() else {
        return Err(invalid(format!("{label} is empty")));
    };
    if value.len() > MAX_IDENTIFIER_BYTES
        || !(first.is_ascii_lowercase() || first.is_ascii_digit())
        || !bytes.all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || matches!(byte, b'-' | b'_' | b'.' | b':')
        })
    {
        return Err(invalid(format!("{label} is not a canonical identifier")));
    }
    Ok(())
}

fn validate_sha256(value: &str, label: &str) -> Result<(), MnlTrustError> {
    if value.len() != 64
        || value.bytes().all(|byte| byte == b'0')
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(invalid(format!("{label} is not canonical SHA-256")));
    }
    Ok(())
}

fn decode_sha256(value: &str, label: &str) -> Result<[u8; 32], MnlTrustError> {
    validate_sha256(value, label)?;
    let mut output = [0_u8; 32];
    for (index, pair) in value.as_bytes().chunks_exact(2).enumerate() {
        let high = canonical_hex_nibble(pair[0])
            .ok_or_else(|| invalid(format!("{label} is not canonical SHA-256")))?;
        let low = canonical_hex_nibble(pair[1])
            .ok_or_else(|| invalid(format!("{label} is not canonical SHA-256")))?;
        output[index] = (high << 4) | low;
    }
    Ok(output)
}

fn canonical_hex_nibble(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        _ => None,
    }
}

fn append_length_prefixed(output: &mut Vec<u8>, value: &[u8]) -> Result<(), MnlTrustError> {
    let length = u64::try_from(value.len())
        .map_err(|_| invalid("replay frame field length is not representable"))?;
    output.extend_from_slice(&length.to_be_bytes());
    output.extend_from_slice(value);
    Ok(())
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", sha2::Sha256::digest(bytes))
}
