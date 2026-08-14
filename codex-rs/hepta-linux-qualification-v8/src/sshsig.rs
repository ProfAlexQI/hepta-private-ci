use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use base64::engine::general_purpose::STANDARD_NO_PAD;
use ed25519_dalek::Signature;
use ed25519_dalek::VerifyingKey;
use sha2::Digest as _;

use crate::AuthorityScopeKindV8;
use crate::AuthoritySignatureAlgorithmV8;
use crate::CryptographicSignatureObservation;
use crate::MacCopyAckV8;
use crate::QualificationError;
use crate::SignedAuthorityV8;
use crate::canonical_authority_statement_v8;
use crate::invalid;

#[cfg(test)]
#[path = "sshsig_tests.rs"]
mod tests;

const ARMOR_BEGIN: &[u8] = b"-----BEGIN SSH SIGNATURE-----\n";
const ARMOR_END: &[u8] = b"-----END SSH SIGNATURE-----\n";
const ARMOR_LINE_BYTES: usize = 70;
const SSHSIG_MAGIC: &[u8] = b"SSHSIG";
const SSHSIG_VERSION: u32 = 1;
const ED25519_ALGORITHM: &[u8] = b"ssh-ed25519";
const SHA256_ALGORITHM: &[u8] = b"sha256";
const MAX_ALLOWED_SIGNERS_BYTES: usize = 1_024;
const MAX_NAMESPACE_BYTES: usize = 128;
const MAX_PRINCIPAL_BYTES: usize = 256;
const MAX_SIGNATURE_BYTES: usize = 16 * 1_024;
const MAX_STATEMENT_BYTES: usize = 16 * 1_024 * 1_024;
const ED25519_PUBLIC_KEY_BYTES: usize = 32;
const ED25519_SIGNATURE_BYTES: usize = 64;

/// No Linux v8 trust root has yet been independently published and pinned.
/// Production verification therefore fails closed until a reviewed revision is
/// compiled here; callers cannot substitute an artifact-provided root.
pub const FROZEN_SSHSIG_TRUST_PROFILE_PUBLISHED_V8: bool = false;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SshsigTrustPurposeV8 {
    InstallAuthority,
    InstallAuthorityV2,
    InstallEpochAuthorityV1,
    ExternalWatermarkLeaseV1,
    ExternalWatermarkCommitV1,
    ExternalWatermarkCurrentTipV1,
    OneShotRunAuthority,
    OneShotRunAuthorityV2,
    BreakGlassAuthority,
    MacCopyAck,
}

impl SshsigTrustPurposeV8 {
    pub(crate) const fn namespace(self) -> &'static str {
        match self {
            Self::InstallAuthority => crate::INSTALL_NAMESPACE_V8,
            Self::InstallAuthorityV2 => crate::INSTALL_NAMESPACE_V2,
            Self::InstallEpochAuthorityV1 => crate::INSTALL_EPOCH_AUTHORITY_NAMESPACE_V1,
            Self::ExternalWatermarkLeaseV1 => crate::EXTERNAL_WATERMARK_LEASE_NAMESPACE_V1,
            Self::ExternalWatermarkCommitV1 => crate::EXTERNAL_WATERMARK_COMMIT_NAMESPACE_V1,
            Self::ExternalWatermarkCurrentTipV1 => {
                crate::EXTERNAL_WATERMARK_CURRENT_TIP_NAMESPACE_V1
            }
            Self::OneShotRunAuthority => crate::ONE_SHOT_RUN_NAMESPACE_V8,
            Self::OneShotRunAuthorityV2 => crate::ONE_SHOT_RUN_NAMESPACE_V2,
            Self::BreakGlassAuthority => crate::BREAK_GLASS_NAMESPACE_V8,
            Self::MacCopyAck => crate::COPY_ACK_NAMESPACE_V8,
        }
    }

    const fn canonical_name(self) -> &'static str {
        match self {
            Self::InstallAuthority => "install_authority",
            Self::InstallAuthorityV2 => "install_authority_v2",
            Self::InstallEpochAuthorityV1 => "install_epoch_authority_v1",
            Self::ExternalWatermarkLeaseV1 => "external_watermark_lease_v1",
            Self::ExternalWatermarkCommitV1 => "external_watermark_commit_v1",
            Self::ExternalWatermarkCurrentTipV1 => "external_watermark_current_tip_v1",
            Self::OneShotRunAuthority => "one_shot_run_authority",
            Self::OneShotRunAuthorityV2 => "one_shot_run_authority_v2",
            Self::BreakGlassAuthority => "break_glass_authority",
            Self::MacCopyAck => "mac_copy_ack",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedTrustPolicyBindingV8 {
    allowed_signers_sha256: String,
    key_fingerprint: String,
    namespace: String,
    policy_sha256: String,
    principal: String,
    purpose: SshsigTrustPurposeV8,
    signature_algorithm: AuthoritySignatureAlgorithmV8,
    trust_root_id: String,
    trust_root_revision: u64,
}

impl VerifiedTrustPolicyBindingV8 {
    pub fn allowed_signers_sha256(&self) -> &str {
        &self.allowed_signers_sha256
    }

    pub fn key_fingerprint(&self) -> &str {
        &self.key_fingerprint
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn policy_sha256(&self) -> &str {
        &self.policy_sha256
    }

    pub fn principal(&self) -> &str {
        &self.principal
    }

    pub fn purpose(&self) -> SshsigTrustPurposeV8 {
        self.purpose
    }

    pub fn signature_algorithm(&self) -> AuthoritySignatureAlgorithmV8 {
        self.signature_algorithm
    }

    pub fn trust_root_id(&self) -> &str {
        &self.trust_root_id
    }

    pub fn trust_root_revision(&self) -> u64 {
        self.trust_root_revision
    }
}

#[derive(Clone, Debug)]
pub(crate) struct FrozenSshsigTrustPolicyV8 {
    allowed_signers_bytes: Vec<u8>,
    binding: VerifiedTrustPolicyBindingV8,
}

impl FrozenSshsigTrustPolicyV8 {
    fn binding(&self) -> &VerifiedTrustPolicyBindingV8 {
        &self.binding
    }

    #[cfg(test)]
    fn for_test_only(
        allowed_signers_bytes: Vec<u8>,
        principal: String,
        namespace: String,
        purpose: SshsigTrustPurposeV8,
    ) -> Result<Self, QualificationError> {
        let anchor = parse_allowed_signer(&allowed_signers_bytes, &principal)?;
        let allowed_signers_sha256 = sha256(&allowed_signers_bytes);
        let policy_sha256 = trust_policy_sha256(
            purpose,
            "test-only-not-production",
            1,
            &allowed_signers_sha256,
            &anchor.fingerprint,
            &principal,
            &namespace,
        );
        Ok(Self {
            allowed_signers_bytes,
            binding: VerifiedTrustPolicyBindingV8 {
                allowed_signers_sha256,
                key_fingerprint: anchor.fingerprint,
                namespace,
                policy_sha256,
                principal,
                purpose,
                signature_algorithm: AuthoritySignatureAlgorithmV8::OpenSshSshsigEd25519,
                trust_root_id: "test-only-not-production".to_string(),
                trust_root_revision: 1,
            },
        })
    }
}

pub(crate) fn authority_trust_purpose_v8(kind: AuthorityScopeKindV8) -> SshsigTrustPurposeV8 {
    match kind {
        AuthorityScopeKindV8::Install => SshsigTrustPurposeV8::InstallAuthority,
        AuthorityScopeKindV8::InstallV2 => SshsigTrustPurposeV8::InstallAuthorityV2,
        AuthorityScopeKindV8::OneShotRun => SshsigTrustPurposeV8::OneShotRunAuthority,
        AuthorityScopeKindV8::OneShotRunV2 => SshsigTrustPurposeV8::OneShotRunAuthorityV2,
        AuthorityScopeKindV8::BreakGlass => SshsigTrustPurposeV8::BreakGlassAuthority,
    }
}

pub(crate) fn required_frozen_trust_binding_v8(
    purpose: SshsigTrustPurposeV8,
) -> Result<VerifiedTrustPolicyBindingV8, QualificationError> {
    required_frozen_trust_policy_v8(purpose).map(|policy| policy.binding)
}

fn required_frozen_trust_policy_v8(
    purpose: SshsigTrustPurposeV8,
) -> Result<FrozenSshsigTrustPolicyV8, QualificationError> {
    let _ = purpose;
    Err(invalid(
        "frozen Linux v8 SSHSIG trust policy is not independently published",
    ))
}

#[cfg(test)]
pub(crate) fn test_only_trust_binding_v8(
    purpose: SshsigTrustPurposeV8,
) -> VerifiedTrustPolicyBindingV8 {
    let (trust_root_id, fingerprint_character) = match purpose {
        SshsigTrustPurposeV8::InstallEpochAuthorityV1 => ("test-only-install-epoch-authority", 'A'),
        SshsigTrustPurposeV8::ExternalWatermarkLeaseV1
        | SshsigTrustPurposeV8::ExternalWatermarkCommitV1
        | SshsigTrustPurposeV8::ExternalWatermarkCurrentTipV1 => {
            ("test-only-external-watermark-provider", 'B')
        }
        _ => ("test-only-not-production", 'C'),
    };
    test_only_trust_binding_with_identity_v8(
        purpose,
        trust_root_id,
        &format!("SHA256:{}", fingerprint_character.to_string().repeat(43)),
    )
}

#[cfg(test)]
pub(crate) fn test_only_trust_binding_with_identity_v8(
    purpose: SshsigTrustPurposeV8,
    trust_root_id: &str,
    key_fingerprint: &str,
) -> VerifiedTrustPolicyBindingV8 {
    let allowed_signers_sha256 = sha256(format!("test allowed signers {purpose:?}").as_bytes());
    let key_fingerprint = key_fingerprint.to_string();
    let principal = "linux-v8-operator@example".to_string();
    let namespace = purpose.namespace().to_string();
    let trust_root_id = trust_root_id.to_string();
    let trust_root_revision = 1;
    let policy_sha256 = trust_policy_sha256(
        purpose,
        &trust_root_id,
        trust_root_revision,
        &allowed_signers_sha256,
        &key_fingerprint,
        &principal,
        &namespace,
    );
    VerifiedTrustPolicyBindingV8 {
        allowed_signers_sha256,
        key_fingerprint,
        namespace,
        policy_sha256,
        principal,
        purpose,
        signature_algorithm: AuthoritySignatureAlgorithmV8::OpenSshSshsigEd25519,
        trust_root_id,
        trust_root_revision,
    }
}

pub(crate) fn verify_signed_authority_sshsig_v8(
    signed: &SignedAuthorityV8,
) -> Result<CryptographicSignatureObservation, QualificationError> {
    let purpose = authority_trust_purpose_v8(signed.challenge.scope_kind());
    let policy = required_frozen_trust_policy_v8(purpose)?;
    let statement = canonical_authority_statement_v8(&signed.challenge)?;
    verify_sshsig_ed25519_v8(&statement, &signed.detached_signature_bytes, &policy)
}

pub(crate) fn verify_mac_copy_ack_sshsig_v8(
    ack: &MacCopyAckV8,
) -> Result<CryptographicSignatureObservation, QualificationError> {
    let policy = required_frozen_trust_policy_v8(SshsigTrustPurposeV8::MacCopyAck)?;
    let statement = ack.canonical_statement()?;
    verify_sshsig_ed25519_v8(&statement, &ack.signature_bytes, &policy)
}

pub(crate) fn verify_statement_sshsig_for_purpose_v8(
    statement: &[u8],
    signature_bytes: &[u8],
    purpose: SshsigTrustPurposeV8,
) -> Result<CryptographicSignatureObservation, QualificationError> {
    let policy = required_frozen_trust_policy_v8(purpose)?;
    verify_sshsig_ed25519_v8(statement, signature_bytes, &policy)
}

/// Verifies one canonical OpenSSH SSHSIG Ed25519 signature entirely in-process.
///
/// The trust anchor must be exactly one LF-terminated
/// `principal ssh-ed25519 base64` record. The detached signature must use the
/// canonical OpenSSH armor, the exact requested namespace, an empty reserved
/// field, SHA-256 message hashing, and the identical anchored public-key blob.
fn verify_sshsig_ed25519_v8(
    statement: &[u8],
    signature_bytes: &[u8],
    policy: &FrozenSshsigTrustPolicyV8,
) -> Result<CryptographicSignatureObservation, QualificationError> {
    let binding = policy.binding();
    validate_inputs(
        statement,
        signature_bytes,
        &policy.allowed_signers_bytes,
        binding.principal(),
        binding.namespace(),
    )?;

    if sha256(&policy.allowed_signers_bytes) != binding.allowed_signers_sha256() {
        return Err(invalid(
            "frozen SSHSIG allowed-signers bytes differ from their policy digest",
        ));
    }
    let anchor = parse_allowed_signer(&policy.allowed_signers_bytes, binding.principal())?;
    if anchor.fingerprint != binding.key_fingerprint() {
        return Err(invalid(
            "frozen SSHSIG key differs from its policy fingerprint",
        ));
    }
    let packet = decode_canonical_armor(signature_bytes)?;
    let envelope = parse_envelope(&packet)?;

    if envelope.public_key_blob != anchor.public_key_blob.as_slice() {
        return Err(invalid(
            "SSHSIG embedded public key differs from the allowed signer",
        ));
    }
    if envelope.namespace != binding.namespace().as_bytes() {
        return Err(invalid(
            "SSHSIG namespace differs from the expected namespace",
        ));
    }
    if !envelope.reserved.is_empty() {
        return Err(invalid("SSHSIG reserved field must be empty"));
    }
    if envelope.hash_algorithm != SHA256_ALGORITHM {
        return Err(invalid("SSHSIG hash algorithm must be sha256"));
    }
    if envelope.signature_algorithm != ED25519_ALGORITHM {
        return Err(invalid("SSHSIG signature algorithm must be ssh-ed25519"));
    }

    let signed_data = signed_data(binding.namespace().as_bytes(), statement)?;
    let signature = Signature::from_slice(envelope.signature)
        .map_err(|_| invalid("SSHSIG Ed25519 signature must be exactly 64 bytes"))?;
    anchor
        .verifying_key
        .verify_strict(&signed_data, &signature)
        .map_err(|_| invalid("OpenSSH SSHSIG Ed25519 verification failed"))?;

    Ok(CryptographicSignatureObservation::from_verified_sshsig(
        sha256(signature_bytes),
        sha256(statement),
        binding.clone(),
    ))
}

fn trust_policy_sha256(
    purpose: SshsigTrustPurposeV8,
    trust_root_id: &str,
    trust_root_revision: u64,
    allowed_signers_sha256: &str,
    key_fingerprint: &str,
    principal: &str,
    namespace: &str,
) -> String {
    let mut bytes = b"hepta_linux_v8_frozen_sshsig_trust_policy_v1\0".to_vec();
    for value in [
        purpose.canonical_name(),
        trust_root_id,
        allowed_signers_sha256,
        key_fingerprint,
        principal,
        namespace,
        "openssh_sshsig_ed25519_sha256",
    ] {
        bytes.extend_from_slice(&(value.len() as u64).to_be_bytes());
        bytes.extend_from_slice(value.as_bytes());
    }
    bytes.extend_from_slice(&trust_root_revision.to_be_bytes());
    sha256(&bytes)
}

struct AllowedSigner {
    fingerprint: String,
    public_key_blob: Vec<u8>,
    verifying_key: VerifyingKey,
}

struct SshsigEnvelope<'a> {
    hash_algorithm: &'a [u8],
    namespace: &'a [u8],
    public_key_blob: &'a [u8],
    reserved: &'a [u8],
    signature: &'a [u8],
    signature_algorithm: &'a [u8],
}

fn validate_inputs(
    statement: &[u8],
    signature_bytes: &[u8],
    allowed_signers_bytes: &[u8],
    expected_principal: &str,
    expected_namespace: &str,
) -> Result<(), QualificationError> {
    if statement.len() > MAX_STATEMENT_BYTES {
        return Err(invalid("signed statement exceeds its byte bound"));
    }
    if signature_bytes.is_empty() || signature_bytes.len() > MAX_SIGNATURE_BYTES {
        return Err(invalid("detached SSHSIG exceeds its byte bound"));
    }
    if allowed_signers_bytes.is_empty() || allowed_signers_bytes.len() > MAX_ALLOWED_SIGNERS_BYTES {
        return Err(invalid("allowed_signers exceeds its byte bound"));
    }
    if expected_principal.is_empty()
        || expected_principal.len() > MAX_PRINCIPAL_BYTES
        || !expected_principal.bytes().all(|byte| {
            byte.is_ascii_alphanumeric() || matches!(byte, b'@' | b'.' | b'_' | b'+' | b'-')
        })
    {
        return Err(invalid("expected SSHSIG principal is malformed"));
    }
    if expected_namespace.is_empty()
        || expected_namespace.len() > MAX_NAMESPACE_BYTES
        || !expected_namespace
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
    {
        return Err(invalid("expected SSHSIG namespace is malformed"));
    }
    Ok(())
}

fn parse_allowed_signer(
    bytes: &[u8],
    expected_principal: &str,
) -> Result<AllowedSigner, QualificationError> {
    if bytes.contains(&b'\r')
        || !bytes.ends_with(b"\n")
        || bytes[..bytes.len() - 1].contains(&b'\n')
    {
        return Err(invalid(
            "allowed_signers must be exactly one LF-terminated record",
        ));
    }
    let line = std::str::from_utf8(&bytes[..bytes.len() - 1])
        .map_err(|_| invalid("allowed_signers is not UTF-8"))?;
    let mut fields = line.split(' ');
    let principal = fields.next().unwrap_or_default();
    let algorithm = fields.next().unwrap_or_default();
    let encoded_key = fields.next().unwrap_or_default();
    if fields.next().is_some()
        || principal != expected_principal
        || algorithm != "ssh-ed25519"
        || encoded_key.is_empty()
    {
        return Err(invalid(
            "allowed_signers must map the exact principal to one raw Ed25519 key",
        ));
    }

    let public_key_blob = STANDARD
        .decode(encoded_key)
        .map_err(|_| invalid("allowed_signers public key is not valid base64"))?;
    if STANDARD.encode(&public_key_blob) != encoded_key {
        return Err(invalid(
            "allowed_signers public key base64 is not canonical",
        ));
    }
    let verifying_key = parse_ed25519_public_key_blob(&public_key_blob)?;
    let fingerprint = format!(
        "SHA256:{}",
        STANDARD_NO_PAD.encode(sha2::Sha256::digest(&public_key_blob))
    );

    Ok(AllowedSigner {
        fingerprint,
        public_key_blob,
        verifying_key,
    })
}

fn decode_canonical_armor(bytes: &[u8]) -> Result<Vec<u8>, QualificationError> {
    if bytes.contains(&b'\r') || !bytes.starts_with(ARMOR_BEGIN) || !bytes.ends_with(ARMOR_END) {
        return Err(invalid("detached signature is not canonical SSHSIG armor"));
    }
    let encoded_with_newline = &bytes[ARMOR_BEGIN.len()..bytes.len() - ARMOR_END.len()];
    let encoded = encoded_with_newline
        .strip_suffix(b"\n")
        .ok_or_else(|| invalid("SSHSIG armor body is not LF-terminated"))?;
    if encoded.is_empty()
        || encoded.contains(&b'\n') && encoded.split(|byte| *byte == b'\n').any(<[u8]>::is_empty)
    {
        return Err(invalid("SSHSIG armor contains an empty base64 line"));
    }

    let lines = encoded.split(|byte| *byte == b'\n').collect::<Vec<_>>();
    for (index, line) in lines.iter().enumerate() {
        let is_last = index + 1 == lines.len();
        if line.is_empty()
            || line.len() > ARMOR_LINE_BYTES
            || (!is_last && line.len() != ARMOR_LINE_BYTES)
        {
            return Err(invalid("SSHSIG armor uses non-canonical line wrapping"));
        }
    }
    let encoded_flat = lines.concat();
    let packet = STANDARD
        .decode(&encoded_flat)
        .map_err(|_| invalid("SSHSIG armor body is not valid base64"))?;
    if canonical_armor(&packet)? != bytes {
        return Err(invalid("SSHSIG armor is not canonical"));
    }
    Ok(packet)
}

fn canonical_armor(packet: &[u8]) -> Result<Vec<u8>, QualificationError> {
    let encoded = STANDARD.encode(packet);
    if encoded.is_empty() {
        return Err(invalid("SSHSIG packet is empty"));
    }
    let mut armored = Vec::with_capacity(ARMOR_BEGIN.len() + encoded.len() + ARMOR_END.len() + 8);
    armored.extend_from_slice(ARMOR_BEGIN);
    for line in encoded.as_bytes().chunks(ARMOR_LINE_BYTES) {
        armored.extend_from_slice(line);
        armored.push(b'\n');
    }
    armored.extend_from_slice(ARMOR_END);
    Ok(armored)
}

fn parse_envelope(packet: &[u8]) -> Result<SshsigEnvelope<'_>, QualificationError> {
    let mut cursor = Cursor::new(packet);
    if cursor.take(SSHSIG_MAGIC.len(), "SSHSIG magic")? != SSHSIG_MAGIC {
        return Err(invalid("SSHSIG packet magic is invalid"));
    }
    if cursor.take_u32("SSHSIG version")? != SSHSIG_VERSION {
        return Err(invalid("SSHSIG packet version must be 1"));
    }
    let public_key_blob = cursor.take_string(256, "SSHSIG public key")?;
    let namespace = cursor.take_string(MAX_NAMESPACE_BYTES, "SSHSIG namespace")?;
    let reserved = cursor.take_string(0, "SSHSIG reserved")?;
    let hash_algorithm = cursor.take_string(16, "SSHSIG hash algorithm")?;
    let signature_blob = cursor.take_string(256, "SSHSIG signature blob")?;
    cursor.finish("SSHSIG packet")?;

    // Validate the embedded key independently even though exact anchor-byte
    // equality is checked by the caller.
    parse_ed25519_public_key_blob(public_key_blob)?;

    let mut signature_cursor = Cursor::new(signature_blob);
    let signature_algorithm = signature_cursor.take_string(32, "SSHSIG signature algorithm")?;
    let signature =
        signature_cursor.take_string(ED25519_SIGNATURE_BYTES, "SSHSIG Ed25519 signature")?;
    if signature.len() != ED25519_SIGNATURE_BYTES {
        return Err(invalid("SSHSIG Ed25519 signature must be exactly 64 bytes"));
    }
    signature_cursor.finish("SSHSIG signature blob")?;

    Ok(SshsigEnvelope {
        hash_algorithm,
        namespace,
        public_key_blob,
        reserved,
        signature,
        signature_algorithm,
    })
}

fn parse_ed25519_public_key_blob(blob: &[u8]) -> Result<VerifyingKey, QualificationError> {
    let mut cursor = Cursor::new(blob);
    if cursor.take_string(32, "Ed25519 key algorithm")? != ED25519_ALGORITHM {
        return Err(invalid("SSH public key algorithm must be ssh-ed25519"));
    }
    let key = cursor.take_string(ED25519_PUBLIC_KEY_BYTES, "Ed25519 public key")?;
    if key.len() != ED25519_PUBLIC_KEY_BYTES {
        return Err(invalid("SSH Ed25519 public key must be exactly 32 bytes"));
    }
    cursor.finish("SSH Ed25519 public key blob")?;
    let key: [u8; ED25519_PUBLIC_KEY_BYTES] = key
        .try_into()
        .map_err(|_| invalid("SSH Ed25519 public key length is invalid"))?;
    let verifying_key = VerifyingKey::from_bytes(&key)
        .map_err(|_| invalid("SSH Ed25519 public key is not a valid point"))?;
    if verifying_key.is_weak() {
        return Err(invalid("SSH Ed25519 public key is weak"));
    }
    Ok(verifying_key)
}

fn signed_data(namespace: &[u8], statement: &[u8]) -> Result<Vec<u8>, QualificationError> {
    let statement_hash = sha2::Sha256::digest(statement);
    let mut bytes = Vec::with_capacity(
        SSHSIG_MAGIC.len() + namespace.len() + statement_hash.len() + SHA256_ALGORITHM.len() + 16,
    );
    bytes.extend_from_slice(SSHSIG_MAGIC);
    append_ssh_string(&mut bytes, namespace)?;
    append_ssh_string(&mut bytes, b"")?;
    append_ssh_string(&mut bytes, SHA256_ALGORITHM)?;
    append_ssh_string(&mut bytes, &statement_hash)?;
    Ok(bytes)
}

fn append_ssh_string(output: &mut Vec<u8>, value: &[u8]) -> Result<(), QualificationError> {
    let length = u32::try_from(value.len()).map_err(|_| invalid("SSH string length overflow"))?;
    output.extend_from_slice(&length.to_be_bytes());
    output.extend_from_slice(value);
    Ok(())
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", sha2::Sha256::digest(bytes))
}

struct Cursor<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Cursor<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    fn take(&mut self, length: usize, label: &str) -> Result<&'a [u8], QualificationError> {
        let end = self
            .offset
            .checked_add(length)
            .ok_or_else(|| invalid(format!("{label} length overflow")))?;
        let value = self
            .bytes
            .get(self.offset..end)
            .ok_or_else(|| invalid(format!("{label} is truncated")))?;
        self.offset = end;
        Ok(value)
    }

    fn take_u32(&mut self, label: &str) -> Result<u32, QualificationError> {
        let bytes: [u8; 4] = self
            .take(4, label)?
            .try_into()
            .map_err(|_| invalid(format!("{label} is truncated")))?;
        Ok(u32::from_be_bytes(bytes))
    }

    fn take_string(&mut self, maximum: usize, label: &str) -> Result<&'a [u8], QualificationError> {
        let length = usize::try_from(self.take_u32(label)?)
            .map_err(|_| invalid(format!("{label} length overflow")))?;
        if length > maximum {
            return Err(invalid(format!("{label} exceeds its byte bound")));
        }
        self.take(length, label)
    }

    fn finish(&self, label: &str) -> Result<(), QualificationError> {
        if self.offset != self.bytes.len() {
            return Err(invalid(format!("{label} has trailing bytes")));
        }
        Ok(())
    }
}
