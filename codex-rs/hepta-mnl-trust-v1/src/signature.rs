use std::collections::HashSet;

use ed25519_dalek::Signature;
use ed25519_dalek::VerifyingKey;
use sha2::Digest;

use crate::DetachedSignatureManifestV1;
use crate::DetachedSignatureRoleV1;
use crate::MnlTrustError;
use crate::RawDetachedEd25519SignatureV1;
use crate::VerifiedDetachedSignatureInspectionV1;
use crate::invalid;
use crate::model::DetachedSignatureInspectionSealV1;

pub const DETACHED_SIGNATURE_MANIFEST_SCHEMA: &str =
    "hepta_mnl_detached_ed25519_signature_manifest_v1";
pub const DETACHED_SIGNATURE_ALGORITHM: &str = "ed25519-detached-sha256";
pub const MAX_DETACHED_SIGNATURE_MANIFEST_BYTES: usize = 16 * 1024;
pub const MAX_DETACHED_SIGNATURE_PAYLOAD_BYTES: usize = 16 * 1024 * 1024;
pub const PRODUCTION_SIGNATURE_POLICY_AVAILABLE: bool = false;

pub const ALL_DETACHED_SIGNATURE_ROLES: [DetachedSignatureRoleV1; 7] = [
    DetachedSignatureRoleV1::FinalArtifactFreeze,
    DetachedSignatureRoleV1::PreRunProfile,
    DetachedSignatureRoleV1::FreezeManifest,
    DetachedSignatureRoleV1::SupervisorSeal,
    DetachedSignatureRoleV1::IndependentCopyAck,
    DetachedSignatureRoleV1::TerminalManifest,
    DetachedSignatureRoleV1::PostRunResultEnvelope,
];

const SIGNED_FRAME_DOMAIN: &[u8] = b"hepta.mnl.detached-ed25519.v1\0";
const SIGNATURE_POLICY_DOMAIN: &[u8] = b"hepta.mnl.detached-ed25519.policy.v1\0";
const ED25519_PUBLIC_KEY_BYTES: usize = 32;
const ED25519_SIGNATURE_BYTES: usize = 64;
const MAX_IDENTIFIER_BYTES: usize = 128;

impl DetachedSignatureRoleV1 {
    pub const fn role_domain(self) -> &'static str {
        match self {
            Self::FinalArtifactFreeze => "hepta.mnl.role.final-artifact-freeze.v1",
            Self::PreRunProfile => "hepta.mnl.role.pre-run-profile.v1",
            Self::FreezeManifest => "hepta.mnl.role.freeze-manifest.v1",
            Self::SupervisorSeal => "hepta.mnl.role.supervisor-seal.v1",
            Self::IndependentCopyAck => "hepta.mnl.role.independent-copy-ack.v1",
            Self::TerminalManifest => "hepta.mnl.role.terminal-manifest.v1",
            Self::PostRunResultEnvelope => "hepta.mnl.role.post-run-result-envelope.v1",
        }
    }

    pub const fn payload_schema(self) -> &'static str {
        match self {
            Self::FinalArtifactFreeze => "hepta-mnl-v1/final-artifact-freeze",
            Self::PreRunProfile => "hepta-mnl-v1/pre-run-profile",
            Self::FreezeManifest => "hepta-mnl-v1/freeze-manifest",
            Self::SupervisorSeal => "hepta-mnl-v1/supervisor-seal",
            Self::IndependentCopyAck => "hepta-mnl-v1/independent-copy-ack",
            Self::TerminalManifest => "hepta-mnl-v1/terminal-manifest",
            Self::PostRunResultEnvelope => "hepta-mnl-v1/post-run-result-envelope",
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct SignaturePolicyEntryMaterialV1 {
    pub(crate) role: DetachedSignatureRoleV1,
    pub(crate) signer_key_id: String,
    pub(crate) verifying_key_bytes: [u8; ED25519_PUBLIC_KEY_BYTES],
}

#[derive(Clone, Debug)]
pub(crate) struct SignaturePolicyMaterialV1 {
    pub(crate) entries: Vec<SignaturePolicyEntryMaterialV1>,
    pub(crate) trust_root_id: String,
    pub(crate) trust_root_revision: u64,
}

struct ValidatedSignaturePolicyV1 {
    entries: Vec<ValidatedSignaturePolicyEntryV1>,
    policy_sha256: String,
    trust_root_id: String,
    trust_root_revision: u64,
}

struct ValidatedSignaturePolicyEntryV1 {
    role: DetachedSignatureRoleV1,
    signer_key_id: String,
    verifying_key: VerifyingKey,
}

pub fn inspect_final_artifact_freeze_signature(
    canonical_manifest: &[u8],
    exact_payload: &[u8],
    raw_signature: &RawDetachedEd25519SignatureV1<'_>,
) -> Result<VerifiedDetachedSignatureInspectionV1, MnlTrustError> {
    inspect_with_compiled_policy(
        DetachedSignatureRoleV1::FinalArtifactFreeze,
        canonical_manifest,
        exact_payload,
        raw_signature,
    )
}

pub fn inspect_pre_run_profile_signature(
    canonical_manifest: &[u8],
    exact_payload: &[u8],
    raw_signature: &RawDetachedEd25519SignatureV1<'_>,
) -> Result<VerifiedDetachedSignatureInspectionV1, MnlTrustError> {
    inspect_with_compiled_policy(
        DetachedSignatureRoleV1::PreRunProfile,
        canonical_manifest,
        exact_payload,
        raw_signature,
    )
}

pub fn inspect_freeze_manifest_signature(
    canonical_manifest: &[u8],
    exact_payload: &[u8],
    raw_signature: &RawDetachedEd25519SignatureV1<'_>,
) -> Result<VerifiedDetachedSignatureInspectionV1, MnlTrustError> {
    inspect_with_compiled_policy(
        DetachedSignatureRoleV1::FreezeManifest,
        canonical_manifest,
        exact_payload,
        raw_signature,
    )
}

pub fn inspect_supervisor_seal_signature(
    canonical_manifest: &[u8],
    exact_payload: &[u8],
    raw_signature: &RawDetachedEd25519SignatureV1<'_>,
) -> Result<VerifiedDetachedSignatureInspectionV1, MnlTrustError> {
    inspect_with_compiled_policy(
        DetachedSignatureRoleV1::SupervisorSeal,
        canonical_manifest,
        exact_payload,
        raw_signature,
    )
}

pub fn inspect_independent_copy_ack_signature(
    canonical_manifest: &[u8],
    exact_payload: &[u8],
    raw_signature: &RawDetachedEd25519SignatureV1<'_>,
) -> Result<VerifiedDetachedSignatureInspectionV1, MnlTrustError> {
    inspect_with_compiled_policy(
        DetachedSignatureRoleV1::IndependentCopyAck,
        canonical_manifest,
        exact_payload,
        raw_signature,
    )
}

pub fn inspect_terminal_manifest_signature(
    canonical_manifest: &[u8],
    exact_payload: &[u8],
    raw_signature: &RawDetachedEd25519SignatureV1<'_>,
) -> Result<VerifiedDetachedSignatureInspectionV1, MnlTrustError> {
    inspect_with_compiled_policy(
        DetachedSignatureRoleV1::TerminalManifest,
        canonical_manifest,
        exact_payload,
        raw_signature,
    )
}

pub fn inspect_post_run_result_envelope_signature(
    canonical_manifest: &[u8],
    exact_payload: &[u8],
    raw_signature: &RawDetachedEd25519SignatureV1<'_>,
) -> Result<VerifiedDetachedSignatureInspectionV1, MnlTrustError> {
    inspect_with_compiled_policy(
        DetachedSignatureRoleV1::PostRunResultEnvelope,
        canonical_manifest,
        exact_payload,
        raw_signature,
    )
}

fn inspect_with_compiled_policy(
    expected_role: DetachedSignatureRoleV1,
    canonical_manifest: &[u8],
    exact_payload: &[u8],
    raw_signature: &RawDetachedEd25519SignatureV1<'_>,
) -> Result<VerifiedDetachedSignatureInspectionV1, MnlTrustError> {
    let policy = compiled_production_signature_policy()
        .ok_or_else(|| invalid("compiled production detached-signature policy is absent"))?;
    inspect_canonical_detached_signature_with_policy(
        &policy,
        expected_role,
        canonical_manifest,
        exact_payload,
        raw_signature,
    )
}

fn compiled_production_signature_policy() -> Option<SignaturePolicyMaterialV1> {
    None
}

pub(crate) fn inspect_canonical_detached_signature_with_policy(
    policy: &SignaturePolicyMaterialV1,
    expected_role: DetachedSignatureRoleV1,
    canonical_manifest: &[u8],
    exact_payload: &[u8],
    raw_signature: &RawDetachedEd25519SignatureV1<'_>,
) -> Result<VerifiedDetachedSignatureInspectionV1, MnlTrustError> {
    let policy = validate_signature_policy(policy)?;
    if canonical_manifest.is_empty()
        || canonical_manifest.len() > MAX_DETACHED_SIGNATURE_MANIFEST_BYTES
    {
        return Err(invalid(
            "detached-signature manifest byte length is outside its bound",
        ));
    }
    let manifest: DetachedSignatureManifestV1 = serde_json::from_slice(canonical_manifest)
        .map_err(|error| MnlTrustError::Serialization(error.to_string()))?;
    let reencoded = serde_json::to_vec(&manifest)
        .map_err(|error| MnlTrustError::Serialization(error.to_string()))?;
    if reencoded != canonical_manifest {
        return Err(invalid(
            "detached-signature manifest is not exact canonical JSON",
        ));
    }
    if manifest.schema != DETACHED_SIGNATURE_MANIFEST_SCHEMA {
        return Err(invalid("detached-signature manifest schema is not exact"));
    }
    if manifest.algorithm != DETACHED_SIGNATURE_ALGORITHM {
        return Err(invalid("detached-signature algorithm is not exact"));
    }
    if manifest.role != expected_role {
        return Err(invalid(
            "detached-signature role differs from its entrypoint",
        ));
    }
    validate_identifier(&manifest.profile_id, "profile id")?;
    validate_identifier(&manifest.signer_key_id, "signer key id")?;
    validate_identifier(&manifest.trust_root_id, "trust root id")?;
    validate_sha256(&manifest.payload_sha256, "payload sha256")?;
    validate_sha256(&manifest.signature_sha256, "signature sha256")?;
    validate_sha256(&manifest.trust_policy_sha256, "trust policy sha256")?;
    if manifest.payload_schema != expected_role.payload_schema() {
        return Err(invalid(
            "detached-signature payload schema differs from its role",
        ));
    }
    if manifest.trust_root_id != policy.trust_root_id
        || manifest.trust_root_revision != policy.trust_root_revision
        || manifest.trust_policy_sha256 != policy.policy_sha256
    {
        return Err(invalid(
            "detached-signature trust policy differs from the compiled policy",
        ));
    }
    let role_policy = policy
        .entries
        .iter()
        .find(|entry| entry.role == expected_role)
        .ok_or_else(|| invalid("compiled signature policy lacks the expected role"))?;
    if manifest.signer_key_id != role_policy.signer_key_id {
        return Err(invalid(
            "detached-signature signer key differs from the compiled role policy",
        ));
    }
    if exact_payload.is_empty() || exact_payload.len() > MAX_DETACHED_SIGNATURE_PAYLOAD_BYTES {
        return Err(invalid(
            "detached-signature payload byte length is outside its bound",
        ));
    }
    let payload_byte_count = u64::try_from(exact_payload.len())
        .map_err(|_| invalid("detached-signature payload length is not representable"))?;
    let payload_sha256 = sha256_hex(exact_payload);
    if manifest.payload_byte_count != payload_byte_count
        || manifest.payload_sha256 != payload_sha256
    {
        return Err(invalid(
            "detached-signature payload length or digest differs from its exact bytes",
        ));
    }
    if raw_signature.raw_signature.len() != ED25519_SIGNATURE_BYTES {
        return Err(invalid(
            "detached Ed25519 signature must be exactly 64 bytes",
        ));
    }
    let signature_sha256 = sha256_hex(raw_signature.raw_signature);
    if manifest.signature_sha256 != signature_sha256 {
        return Err(invalid(
            "detached Ed25519 signature digest differs from its exact bytes",
        ));
    }
    let signature = Signature::from_slice(raw_signature.raw_signature)
        .map_err(|_| invalid("detached Ed25519 signature cannot be parsed"))?;
    let signed_frame = detached_signature_frame(&manifest)?;
    role_policy
        .verifying_key
        .verify_strict(&signed_frame, &signature)
        .map_err(|_| invalid("detached Ed25519 strict verification failed"))?;

    Ok(VerifiedDetachedSignatureInspectionV1 {
        manifest_sha256: sha256_hex(canonical_manifest),
        payload_bytes: exact_payload.to_vec(),
        payload_byte_count,
        payload_schema: manifest.payload_schema,
        payload_sha256,
        profile_id: manifest.profile_id,
        role: expected_role,
        signature_sha256,
        signed_frame_sha256: sha256_hex(&signed_frame),
        signer_key_id: manifest.signer_key_id,
        trust_policy_sha256: policy.policy_sha256,
        trust_root_id: policy.trust_root_id,
        trust_root_revision: policy.trust_root_revision,
        _seal: DetachedSignatureInspectionSealV1,
    })
}

fn validate_signature_policy(
    policy: &SignaturePolicyMaterialV1,
) -> Result<ValidatedSignaturePolicyV1, MnlTrustError> {
    validate_identifier(&policy.trust_root_id, "compiled trust root id")?;
    if policy.trust_root_revision == 0 {
        return Err(invalid(
            "compiled signature policy revision must be positive",
        ));
    }
    if policy.entries.len() != ALL_DETACHED_SIGNATURE_ROLES.len() {
        return Err(invalid(
            "compiled signature policy does not contain the exact role roster",
        ));
    }

    let mut key_ids = HashSet::with_capacity(policy.entries.len());
    let mut key_bytes = HashSet::with_capacity(policy.entries.len());
    let mut entries = Vec::with_capacity(policy.entries.len());
    for (expected_role, entry) in ALL_DETACHED_SIGNATURE_ROLES.iter().zip(&policy.entries) {
        if entry.role != *expected_role {
            return Err(invalid("compiled signature policy role order is not exact"));
        }
        validate_identifier(&entry.signer_key_id, "compiled signer key id")?;
        if !key_ids.insert(entry.signer_key_id.as_str()) {
            return Err(invalid("compiled signature policy repeats a key id"));
        }
        if !key_bytes.insert(entry.verifying_key_bytes) {
            return Err(invalid(
                "compiled signature policy reuses a role public key",
            ));
        }
        let verifying_key = VerifyingKey::from_bytes(&entry.verifying_key_bytes)
            .map_err(|_| invalid("compiled Ed25519 public key is not a valid point"))?;
        if verifying_key.is_weak() {
            return Err(invalid("compiled Ed25519 public key is weak"));
        }
        entries.push(ValidatedSignaturePolicyEntryV1 {
            role: entry.role,
            signer_key_id: entry.signer_key_id.clone(),
            verifying_key,
        });
    }

    Ok(ValidatedSignaturePolicyV1 {
        entries,
        policy_sha256: signature_policy_sha256(policy)?,
        trust_root_id: policy.trust_root_id.clone(),
        trust_root_revision: policy.trust_root_revision,
    })
}

pub(crate) fn signature_policy_sha256(
    policy: &SignaturePolicyMaterialV1,
) -> Result<String, MnlTrustError> {
    let mut frame = Vec::new();
    frame.extend_from_slice(SIGNATURE_POLICY_DOMAIN);
    append_length_prefixed(&mut frame, policy.trust_root_id.as_bytes())?;
    frame.extend_from_slice(&policy.trust_root_revision.to_be_bytes());
    let entry_count = u64::try_from(policy.entries.len())
        .map_err(|_| invalid("signature policy entry count is not representable"))?;
    frame.extend_from_slice(&entry_count.to_be_bytes());
    for entry in &policy.entries {
        append_length_prefixed(&mut frame, entry.role.role_domain().as_bytes())?;
        append_length_prefixed(&mut frame, entry.signer_key_id.as_bytes())?;
        append_length_prefixed(&mut frame, &entry.verifying_key_bytes)?;
    }
    Ok(sha256_hex(&frame))
}

pub(crate) fn detached_signature_frame(
    manifest: &DetachedSignatureManifestV1,
) -> Result<Vec<u8>, MnlTrustError> {
    let policy_sha256 = decode_sha256(&manifest.trust_policy_sha256, "trust policy sha256")?;
    let payload_sha256 = decode_sha256(&manifest.payload_sha256, "payload sha256")?;
    let mut frame = Vec::new();
    frame.extend_from_slice(SIGNED_FRAME_DOMAIN);
    append_length_prefixed(&mut frame, manifest.role.role_domain().as_bytes())?;
    append_length_prefixed(&mut frame, manifest.trust_root_id.as_bytes())?;
    frame.extend_from_slice(&manifest.trust_root_revision.to_be_bytes());
    frame.extend_from_slice(&policy_sha256);
    append_length_prefixed(&mut frame, manifest.signer_key_id.as_bytes())?;
    append_length_prefixed(&mut frame, manifest.profile_id.as_bytes())?;
    append_length_prefixed(&mut frame, manifest.payload_schema.as_bytes())?;
    frame.extend_from_slice(&manifest.payload_byte_count.to_be_bytes());
    frame.extend_from_slice(&payload_sha256);
    Ok(frame)
}

fn append_length_prefixed(output: &mut Vec<u8>, value: &[u8]) -> Result<(), MnlTrustError> {
    let length = u64::try_from(value.len())
        .map_err(|_| invalid("signature frame field length is not representable"))?;
    output.extend_from_slice(&length.to_be_bytes());
    output.extend_from_slice(value);
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

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", sha2::Sha256::digest(bytes))
}
